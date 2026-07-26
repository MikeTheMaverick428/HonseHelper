use anyhow::{anyhow, Result};
use std::path::PathBuf;

#[cfg(unix)]
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub start: u64,
    pub end: u64,
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
    pub private: bool,
    pub pathname: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub path: Option<PathBuf>,
}

pub fn find_process_by_name(name: &str) -> Result<ProcessInfo> {
    find_process_by_names(&[name])
}

pub fn find_process_by_names(names: &[&str]) -> Result<ProcessInfo> {
    if names.is_empty() {
        return Err(anyhow!("At least one process name must be provided"));
    }

    #[cfg(unix)]
    {
        find_process_by_names_unix(names)
    }

    #[cfg(windows)]
    {
        find_process_by_names_windows(names)
    }

    #[cfg(not(any(unix, windows)))]
    {
        let _ = names;
        Err(anyhow!("Process discovery is not supported on this target"))
    }
}

#[cfg(unix)]
pub fn list_memory_regions(pid: u32) -> Result<Vec<MemoryRegion>> {
    let maps_path = format!("/proc/{}/maps", pid);
    let content = std::fs::read_to_string(&maps_path)
        .map_err(|e| anyhow!("Failed to read {}: {}", maps_path, e))?;

    let mut regions = Vec::new();
    for line in content.lines() {
        let mut parts = line.split_whitespace();
        let Some(range) = parts.next() else {
            continue;
        };
        let Some(perms) = parts.next() else {
            continue;
        };

        let Some((start_hex, end_hex)) = range.split_once('-') else {
            continue;
        };

        let Ok(start) = u64::from_str_radix(start_hex, 16) else {
            continue;
        };
        let Ok(end) = u64::from_str_radix(end_hex, 16) else {
            continue;
        };
        if end <= start {
            continue;
        }

        let bytes = perms.as_bytes();
        if bytes.len() < 4 {
            continue;
        }

        let pathname = line
            .splitn(6, ' ')
            .nth(5)
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());

        regions.push(MemoryRegion {
            start,
            end,
            readable: bytes[0] == b'r',
            writable: bytes[1] == b'w',
            executable: bytes[2] == b'x',
            private: bytes[3] == b'p',
            pathname,
        });
    }

    Ok(regions)
}

#[cfg(unix)]
pub fn check_read_privileges() -> Result<()> {
    use nix::unistd::Uid;

    if !Uid::effective().is_root() {
        return Err(anyhow!(
            "Root privileges required to read another process memory. Run with sudo."
        ));
    }

    Ok(())
}

#[cfg(unix)]
fn find_process_by_names_unix(names: &[&str]) -> Result<ProcessInfo> {
    let names_lower: Vec<String> = names.iter().map(|n| n.to_ascii_lowercase()).collect();

    let mut candidates: Vec<(u32, String, Option<PathBuf>, usize)> = Vec::new();

    for process in procfs::process::all_processes()? {
        let process = match process {
            Ok(p) => p,
            Err(_) => continue,
        };

        let cmdline = process.cmdline().unwrap_or_default();
        if cmdline.is_empty() {
            continue;
        }

        let cmdline_str = cmdline.join(" ").to_ascii_lowercase();
        let first_arg = cmdline
            .first()
            .map(|s| s.to_ascii_lowercase())
            .unwrap_or_default();

        let has_requested_name = names_lower.iter().any(|needle| {
            cmdline_str.contains(needle)
                || cmdline
                    .iter()
                    .any(|arg| arg.to_ascii_lowercase().ends_with(needle))
        });

        if !has_requested_name {
            continue;
        }

        // Skip Steam/Proton helper wrappers; keep the actual game host process.
        let is_helper = first_arg.contains("steam-launch-wrapper")
            || first_arg.contains("reaper")
            || first_arg.ends_with("steam")
            || first_arg.ends_with("steam.exe")
            || cmdline_str.contains("steamlaunch")
            || cmdline_str.contains("steamlinuxruntime")
            || (cmdline_str.contains("proton") && cmdline_str.contains("waitforexitandrun"));

        if is_helper {
            continue;
        }

        let matched = names_lower.iter().find_map(|needle| {
            cmdline
                .iter()
                .find(|part| {
                    let part_lower = part.to_ascii_lowercase();
                    part_lower.ends_with(needle.as_str()) || part_lower.contains(needle.as_str())
                })
                .cloned()
        });

        let Some(matched_token) = matched else {
            continue;
        };

        let pid = process.pid as u32;
        candidates.push((
            pid,
            matched_token.clone(),
            Some(PathBuf::from(matched_token.clone())),
            matched_token.len(),
        ));
    }

    if candidates.is_empty() {
        return Err(anyhow!("Process not found for names: {}", names.join(", ")));
    }

    candidates.sort_by(|a, b| b.3.cmp(&a.3));
    let (pid, name, path, _) = candidates[0].clone();

    Ok(ProcessInfo { pid, name, path })
}

#[cfg(windows)]
fn find_process_by_names_windows(names: &[&str]) -> Result<ProcessInfo> {
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
        TH32CS_SNAPPROCESS,
    };

    let names_lower: Vec<String> = names.iter().map(|n| n.to_ascii_lowercase()).collect();

    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;

        let mut entry = PROCESSENTRY32W::default();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

        let result = if Process32FirstW(snapshot, &mut entry).is_ok() {
            let mut found: Option<ProcessInfo> = None;
            loop {
                let exe_name = String::from_utf16_lossy(
                    &entry
                        .szExeFile
                        .iter()
                        .take_while(|&&c| c != 0)
                        .copied()
                        .collect::<Vec<u16>>(),
                );

                let exe_lower = exe_name.to_ascii_lowercase();
                let exe_stem = exe_lower.strip_suffix(".exe").unwrap_or(&exe_lower);
                if names_lower.iter().any(|needle| {
                    let n = needle.strip_suffix(".exe").unwrap_or(needle.as_str());
                    exe_lower == needle.as_str() || exe_stem == n
                }) {
                    found = Some(ProcessInfo {
                        pid: entry.th32ProcessID,
                        name: exe_name.clone(),
                        path: Some(PathBuf::from(exe_name)),
                    });
                    break;
                }

                if Process32NextW(snapshot, &mut entry).is_err() {
                    break;
                }
            }
            found
        } else {
            None
        };

        let _ = CloseHandle(snapshot);

        result.ok_or_else(|| anyhow!("Process not found for names: {}", names.join(", ")))
    }
}
