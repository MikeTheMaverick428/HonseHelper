use serde::Serialize;
use std::time::Instant;
use tauri::{AppHandle, Emitter, Manager};

use crate::worker::WorkerState;

const CLK_TCK: f64 = 100.0;

#[derive(Clone, Serialize)]
pub struct ProcStats {
    pub cpu_pct: f32,
    pub memory_mb: f64,
}

#[derive(Clone, Serialize)]
pub struct ResourceStats {
    pub app: ProcStats,
    pub worker: Option<ProcStats>,
}

fn parse_stat_ticks(pid: u32) -> Option<u64> {
    let path = if pid == std::process::id() as u32 {
        "/proc/self/stat".into()
    } else {
        format!("/proc/{}/stat", pid)
    };
    let content = std::fs::read_to_string(&path).ok()?;
    let after_comm = content.split(") ").nth(1)?;
    let fields: Vec<&str> = after_comm.split_whitespace().collect();
    let utime: u64 = fields.get(11)?.parse().ok()?;
    let stime: u64 = fields.get(12)?.parse().ok()?;
    Some(utime + stime)
}

fn parse_proc_vmrss(pid: u32) -> Option<f64> {
    let path = if pid == std::process::id() as u32 {
        "/proc/self/status".into()
    } else {
        format!("/proc/{}/status", pid)
    };
    let content = std::fs::read_to_string(&path).ok()?;
    let kb: f64 = content
        .lines()
        .find(|l| l.starts_with("VmRSS:"))?
        .split_whitespace()
        .nth(1)?
        .parse()
        .ok()?;
    Some(kb / 1024.0)
}

fn sample_proc(pid: u32) -> Option<(u64, f64)> {
    let ticks = parse_stat_ticks(pid)?;
    let mem = parse_proc_vmrss(pid).unwrap_or(0.0);
    Some((ticks, mem))
}

fn compute_cpu_pct(prev_ticks: u64, cur_ticks: u64, delta_secs: f64) -> f32 {
    if delta_secs <= 0.0 {
        return 0.0;
    }
    let delta_ticks = cur_ticks.saturating_sub(prev_ticks) as f64;
    ((delta_ticks / CLK_TCK) / delta_secs * 100.0) as f32
}

pub fn spawn_resource_monitor(app: AppHandle) {
    std::thread::spawn(move || {
        let own_pid = std::process::id() as u32;
        let mut prev_own: Option<(u64, Instant)> = None;
        let mut prev_worker: Option<(u64, Instant)> = None;

        loop {
            if app.get_webview_window("dev-tools").is_none() {
                break;
            }

            let now = Instant::now();

            let app_cpu = if let Some((ticks, mem)) = sample_proc(own_pid) {
                let cpu = if let Some((prev_ticks, prev_time)) = prev_own {
                    let delta = now.duration_since(prev_time).as_secs_f64();
                    compute_cpu_pct(prev_ticks, ticks, delta)
                } else {
                    0.0
                };
                prev_own = Some((ticks, now));
                ProcStats {
                    cpu_pct: cpu,
                    memory_mb: mem,
                }
            } else {
                ProcStats {
                    cpu_pct: 0.0,
                    memory_mb: 0.0,
                }
            };

            let worker = {
                let ws = app.state::<WorkerState>();
                if let Some(pid) = ws.get_pid_if_running() {
                    sample_proc(pid).map(|(ticks, mem)| {
                        let cpu = if let Some((prev_ticks, prev_time)) = prev_worker {
                            let delta = now.duration_since(prev_time).as_secs_f64();
                            compute_cpu_pct(prev_ticks, ticks, delta)
                        } else {
                            0.0
                        };
                        prev_worker = Some((ticks, now));
                        ProcStats {
                            cpu_pct: cpu,
                            memory_mb: mem,
                        }
                    })
                } else {
                    prev_worker = None;
                    None
                }
            };

            let stats = ResourceStats {
                app: app_cpu,
                worker,
            };

            let _ = app.emit("resource-stats", stats);

            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    });
}
