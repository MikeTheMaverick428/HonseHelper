use crate::styles::{
    race_dump_detail::{CanvasContainerStyle, ControlBarStyle},
    Style,
};
use gloo_timers::callback::Interval;
use shared::models::{ReplayEvent, ReplayEventData, ReplayFrame, ReplayHorseData};
use shared::{RaceDumpParticipant, RaceDumpSummary};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlSelectElement};
use yew::prelude::*;

const TEAM_COLORS: [&str; 12] = [
    "#2563EB",
    "#DC2626",
    "#9333EA",
    "#c9a91bff",
    "#EA580C",
    "#0891B2",
    "#DB2777",
    "#4F46E5",
    "#059669",
    "#B45309",
    "#0EA5E9",
    "#C026D3",
];
const PLAYER_COLOR: &str = "#00fa0cff";
const CAMERA_WINDOW: f64 = 80.0;

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}
fn clamp01(x: f64) -> f64 {
    x.clamp(0.0, 1.0)
}

fn interpolate_frames(frames: &[ReplayFrame], time: f64) -> (usize, Vec<InterpHorse>) {
    if frames.is_empty() {
        return (0, Vec::new());
    }
    let last = frames.len() - 1;
    if time <= frames[0].time {
        return (0, frames[0].horse_data_array.iter().map(interp).collect());
    }
    if time >= frames[last].time {
        return (
            last,
            frames[last].horse_data_array.iter().map(interp).collect(),
        );
    }
    let mut lo = 0usize;
    let mut hi = last;
    while lo <= hi {
        let mid = (lo + hi) >> 1;
        let tm = frames[mid].time;
        if tm <= time {
            if mid + 1 < frames.len() && time < frames[mid + 1].time {
                lo = mid;
                break;
            }
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    let i = lo.min(last.saturating_sub(1));
    let f0 = &frames[i];
    let f1 = &frames[i + 1];
    let a = clamp01((time - f0.time) / (f1.time - f0.time).max(1e-9));
    let cnt = f0.horse_data_array.len().min(f1.horse_data_array.len());
    let horses: Vec<InterpHorse> = (0..cnt)
        .map(|idx| {
            let h0 = &f0.horse_data_array[idx];
            let h1 = &f1.horse_data_array[idx];
            InterpHorse {
                distance: lerp(h0.distance, h1.distance, a),
                lane_position: lerp(h0.lane_position, h1.lane_position, a),
                speed: lerp(h0.speed, h1.speed, a),
                hp: lerp(h0.hp, h1.hp, a),
                is_tempted: if a >= 0.5 {
                    h1.is_tempted
                } else {
                    h0.is_tempted
                },
                is_blocked: if a >= 0.5 {
                    h1.is_blocked
                } else {
                    h0.is_blocked
                },
            }
        })
        .collect();
    (i, horses)
}

fn interp(h: &ReplayHorseData) -> InterpHorse {
    InterpHorse {
        distance: h.distance,
        lane_position: h.lane_position,
        speed: h.speed,
        hp: h.hp,
        is_tempted: h.is_tempted,
        is_blocked: h.is_blocked,
    }
}

#[derive(Debug, Clone, Copy)]
struct InterpHorse {
    distance: f64,
    lane_position: f64,
    speed: f64,
    hp: f64,
    is_tempted: bool,
    is_blocked: bool,
}

fn draw_round_rect(ctx: &CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64, r: f64) {
    ctx.begin_path();
    let _ = ctx.move_to(x + r, y);
    let _ = ctx.line_to(x + w - r, y);
    let _ = ctx.arc(x + w - r, y + r, r, -std::f64::consts::FRAC_PI_2, 0.0);
    let _ = ctx.line_to(x + w, y + h - r);
    let _ = ctx.arc(x + w - r, y + h - r, r, 0.0, std::f64::consts::FRAC_PI_2);
    let _ = ctx.line_to(x + r, y + h);
    let _ = ctx.arc(
        x + r,
        y + h - r,
        r,
        std::f64::consts::FRAC_PI_2,
        std::f64::consts::PI,
    );
    let _ = ctx.line_to(x, y + r);
    let _ = ctx.arc(
        x + r,
        y + r,
        r,
        std::f64::consts::PI,
        3.0 * std::f64::consts::FRAC_PI_2,
    );
    ctx.close_path();
}

fn render_frame(
    ctx: &CanvasRenderingContext2d,
    cw: f64,
    ch: f64,
    frames: &[ReplayFrame],
    events: &[ReplayEvent],
    participants: &[RaceDumpParticipant],
    rt: f64,
    all_frames_for_hp: &[ReplayFrame],
    race_distance: Option<i64>,
) -> (usize, usize, Vec<InterpHorse>) {
    ctx.save();
    ctx.clear_rect(0.0, 0.0, cw, ch);
    ctx.set_fill_style_str("#080c14");
    ctx.fill_rect(0.0, 0.0, cw, ch);

    let (fi, horses) = interpolate_frames(frames, rt);
    if horses.is_empty() {
        ctx.restore();
        return (fi, frames.len(), horses);
    }

    let goal = horses.iter().map(|h| h.distance).fold(0.0, f64::max);
    let max_lane = horses
        .iter()
        .map(|h| h.lane_position)
        .fold(0.0, f64::max)
        .max(0.6);
    let lead = CAMERA_WINDOW * 0.1;
    let front = goal.min(goal + 10.0) + lead;
    let x_min = (0.0f64).max(CAMERA_WINDOW.max(front) - CAMERA_WINDOW);
    let x_max = CAMERA_WINDOW.max(front);

    let mx = 40.0;
    let myt = 100.0;
    let myb = 45.0;
    let pw = cw - mx * 2.0;
    let ph = ch - myt - myb;
    let to_x = |d: f64| -> f64 { mx + ((d - x_min) / (x_max - x_min).max(1.0)) * pw };
    let to_y =
        |lane: f64| -> f64 { myt + (1.0 - (lane / max_lane.max(0.01)).clamp(0.0, 1.0)) * ph };

    for li in 0..=8 {
        let ly = to_y((li as f64 / 8.0) * max_lane);
        ctx.set_stroke_style_str("#1a2530");
        ctx.set_line_width(1.0);
        ctx.begin_path();
        let _ = ctx.move_to(mx, ly);
        let _ = ctx.line_to(mx + pw, ly);
        ctx.stroke();
    }
    if let Some(dist) = race_distance {
        let finish_x = to_x(dist as f64);
        if finish_x >= mx - 10.0 && finish_x <= mx + pw + 10.0 {
            let fw = 6.0;
            let check_h = 6.0;
            let ly0 = to_y(max_lane);
            let ly1 = to_y(0.0);
            let h = (ly1 - ly0) + 4.0;
            let n = ((h / check_h).ceil() as usize).max(1);
            for ci in 0..n {
                let cy = ly0 - 2.0 + ci as f64 * check_h;
                let ch = check_h.min(ly0 - 2.0 + h - cy);
                ctx.set_fill_style_str(if ci % 2 == 0 { "#ffffff" } else { "#000000" });
                ctx.fill_rect(finish_x - fw / 2.0, cy, fw, ch);
            }
            let label = "FINISH";
            let lw = 48.0;
            let lh = 14.0;
            let lx = finish_x - lw / 2.0;
            let ly = ly0 - lh - 2.0;
            draw_round_rect(ctx, lx, ly, lw, lh, 3.0);
            ctx.set_fill_style_str("rgba(0,0,0,0.7)");
            ctx.fill();
            ctx.set_fill_style_str("#ffffff");
            ctx.set_font("bold 8px sans-serif");
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            let _ = ctx.fill_text(label, finish_x, ly + lh / 2.0);
        }
    }

    let _total = goal.max(1.0);

    let max_hp = all_frames_for_hp
        .iter()
        .flat_map(|f| f.horse_data_array.iter())
        .filter(|h| h.hp > 0.0)
        .map(|h| h.hp as i64)
        .max()
        .unwrap_or(1) as f64;
    let hc = participants.len();

    for (idx, horse) in horses.iter().enumerate() {
        if idx >= hc {
            break;
        }
        let is_player = participants
            .get(idx)
            .map(|p| p.is_player == 1)
            .unwrap_or(false);
        let hx = to_x(horse.distance);
        let hy = to_y(horse.lane_position.max(0.0));
        let r = if is_player { 10.0 } else { 7.0 };
        let color = if is_player {
            PLAYER_COLOR
        } else {
            TEAM_COLORS[idx % TEAM_COLORS.len()]
        };

        ctx.begin_path();
        let _ = ctx.arc(hx, hy, r, 0.0, std::f64::consts::TAU);
        ctx.set_fill_style_str(color);
        ctx.fill();
        let outline_color = if horse.is_blocked {
            "#ef4444"
        } else if horse.is_tempted {
            "#ffaa00"
        } else {
            "#fff"
        };
        ctx.set_stroke_style_str(outline_color);
        ctx.set_line_width(if horse.is_blocked || horse.is_tempted {
            2.5
        } else {
            1.5
        });
        ctx.stroke();

        let name = participants
            .get(idx)
            .and_then(|p| p.chara_name.as_deref())
            .unwrap_or("?");
        ctx.set_fill_style_str(if is_player { "#c4b5fd" } else { "#94a3b8" });
        ctx.set_font("10px sans-serif");
        ctx.set_text_align("center");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text(name, hx, hy + r + 2.0);

        let speed_text = format!("{:.2}", horse.speed);
        let sw = 38.0;
        let sh = 15.0;
        let sx = hx - sw / 2.0;
        let sy = hy - r - sh - 2.0;
        draw_round_rect(ctx, sx, sy, sw, sh, 4.0);
        ctx.set_fill_style_str("rgba(255,255,255,0.85)");
        ctx.fill();
        ctx.set_stroke_style_str("#000");
        ctx.set_line_width(1.0);
        ctx.stroke();
        ctx.set_fill_style_str("#000");
        ctx.set_font("bold 10px sans-serif");
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        let _ = ctx.fill_text(&speed_text, hx, sy + sh / 2.0);

        let hr = (horse.hp / max_hp.max(1.0)).clamp(0.0, 1.0);
        let hpw = 38.0;
        let hph = 5.0;
        let hpx = hx - hpw / 2.0;
        let hpy = hy + r + 18.0;
        ctx.set_fill_style_str("rgba(100,0,0,0.8)");
        ctx.fill_rect(hpx, hpy, hpw, hph);
        ctx.set_fill_style_str("rgba(0,255,0,0.7)");
        ctx.fill_rect(hpx, hpy, hpw * hr, hph);
    }

    let ew = 3.0;
    let mut horse_events: std::collections::BTreeMap<usize, Vec<&ReplayEvent>> =
        std::collections::BTreeMap::new();
    for evt in events.iter() {
        if matches!(&evt.event_data, Some(ReplayEventData::Score)) {
            continue;
        }
        let Some(horse_idx) = evt.horse_idx else {
            continue;
        };
        let idx = horse_idx as usize;
        if idx >= hc {
            continue;
        }
        if !(rt >= evt.frame_time && rt <= evt.frame_time + ew) {
            continue;
        }
        horse_events.entry(idx).or_default().push(evt);
    }
    for (idx, evts) in horse_events.iter() {
        let hx = to_x(horses[*idx].distance);
        let hy = to_y(horses[*idx].lane_position.max(0.0));
        let is_player = participants
            .get(*idx)
            .map(|p| p.is_player == 1)
            .unwrap_or(false);
        let r = if is_player { 10.0 } else { 7.0 };
        for (li, evt) in evts.iter().take(4).enumerate() {
            let label = event_data_label(&evt.event_data);
            let lw = (label.len() as f64 * 8.0 + 12.0).clamp(40.0, 100.0);
            let lx = hx - lw / 2.0;
            let ly = hy - r - 15.0 - 16.0 - 2.0 - (li as f64) * 18.0;
            draw_round_rect(ctx, lx, ly, lw, 14.0, 4.0);
            let is_skill = matches!(&evt.event_data, Some(ReplayEventData::Skill(_)));
            ctx.set_fill_style_str(if is_skill {
                "rgba(139,92,246,0.85)"
            } else {
                "rgba(30,41,59,0.9)"
            });
            ctx.fill();
            ctx.set_stroke_style_str("#000");
            ctx.set_line_width(1.0);
            ctx.stroke();
            ctx.set_fill_style_str("#fff");
            ctx.set_font("bold 9px sans-serif");
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            let _ = ctx.fill_text(&label, hx, ly + 7.0);
        }
    }

    ctx.restore();
    (fi, frames.len(), horses)
}

fn event_data_label(data: &Option<ReplayEventData>) -> String {
    match data {
        Some(ReplayEventData::Score) => "Score".into(),
        Some(ReplayEventData::Skill(name)) => format!("{}", name),
        Some(ReplayEventData::CompTop) => "CompTop".into(),
        Some(ReplayEventData::CompFight) => "CompFight".into(),
        Some(ReplayEventData::RelCons) => "RelCons".into(),
        Some(ReplayEventData::StamBrk) => "StamBrk".into(),
        Some(ReplayEventData::CompSpurt) => "CompSpurt".into(),
        Some(ReplayEventData::StamKeep) => "StamKeep".into(),
        Some(ReplayEventData::SecLead) => "SecLead".into(),
        None => "Event".into(),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HorseSnapshot {
    pub horse_index: usize,
    pub distance: f64,
    pub speed: f64,
    pub hp: f64,
    pub is_tempted: bool,
    pub is_blocked: bool,
    pub active_event_labels: Vec<String>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct ReplayPanelProps {
    pub summary: RaceDumpSummary,
    pub participants: Vec<RaceDumpParticipant>,
    pub frames: Vec<ReplayFrame>,
    pub events: Vec<ReplayEvent>,
    #[prop_or_default]
    pub on_frame: Callback<Vec<HorseSnapshot>>,
    #[prop_or(false)]
    pub paused: bool,
}

#[derive(Clone)]
struct ReplayState {
    render_time: f64,
    frame_count: usize,
    total_frames: usize,
    start_time: f64,
    end_time: f64,
    is_playing: bool,
    playback_rate: f64,
}

impl PartialEq for ReplayState {
    fn eq(&self, other: &Self) -> bool {
        self.render_time == other.render_time
            && self.frame_count == other.frame_count
            && self.total_frames == other.total_frames
            && self.start_time == other.start_time
            && self.end_time == other.end_time
            && self.is_playing == other.is_playing
            && self.playback_rate == other.playback_rate
    }
}

#[function_component]
pub fn ReplayPanel(props: &ReplayPanelProps) -> Html {
    let canvas_ref = use_node_ref();

    let frames = props.frames.clone();
    let events = props.events.clone();
    let participants = props.participants.clone();
    let start_time = frames.first().map(|f| f.time).unwrap_or(0.0);
    let end_time = frames.last().map(|f| f.time).unwrap_or(0.0);

    let state = use_state(|| ReplayState {
        render_time: start_time,
        frame_count: 0,
        total_frames: frames.len(),
        start_time,
        end_time,
        is_playing: false,
        playback_rate: 1.0,
    });

    let race_distance = props.summary.distance;

    let playing_ref = use_mut_ref(|| false);
    let rate_ref = use_mut_ref(|| 1.0f64);
    let rt_ref = use_mut_ref(|| start_time);
    let frames_ref = use_mut_ref(|| frames.clone());
    let events_ref = use_mut_ref(|| events.clone());
    let participants_ref = use_mut_ref(|| participants);

    {
        let canvas_ref = canvas_ref.clone();
        let state = state.clone();
        let playing_ref = playing_ref.clone();
        let rate_ref = rate_ref.clone();
        let rt_ref = rt_ref.clone();
        let frames_ref = frames_ref.clone();
        let events_ref = events_ref.clone();
        let participants_ref = participants_ref.clone();
        let distance = race_distance;
        let on_frame = props.on_frame.clone();

        use_effect_with((), move |_| {
            let interval = Interval::new(16, move || {
                let canvas = match canvas_ref.cast::<HtmlCanvasElement>() {
                    Some(c) => c,
                    None => return,
                };
                let ctx_val = match canvas.get_context("2d") {
                    Ok(Some(c)) => c,
                    _ => return,
                };
                let ctx = match ctx_val.dyn_into::<CanvasRenderingContext2d>() {
                    Ok(c) => c,
                    Err(_) => return,
                };

                let frames = frames_ref.borrow();
                let events = events_ref.borrow();
                let participants = participants_ref.borrow();
                if frames.is_empty() {
                    return;
                }

                let mut rt = *rt_ref.borrow();
                let playing = *playing_ref.borrow();
                let rate = *rate_ref.borrow();
                let st = frames.first().map(|f| f.time).unwrap_or(0.0);
                let et = frames.last().map(|f| f.time).unwrap_or(0.0);

                if playing {
                    rt += 0.016 * rate;
                    if rt >= et {
                        rt = et;
                        *playing_ref.borrow_mut() = false;
                    }
                    rt = rt.clamp(st, et);
                    *rt_ref.borrow_mut() = rt;
                }

                let dpr = web_sys::window()
                    .map(|w| w.device_pixel_ratio())
                    .unwrap_or(1.0);
                let cw = canvas.offset_width().max(1) as f64;
                let ch = canvas.offset_height().max(1) as f64;
                canvas.set_width((cw * dpr) as u32);
                canvas.set_height((ch * dpr) as u32);
                ctx.save();
                let _ = ctx.scale(dpr, dpr);
                let (fi, total, horses) = render_frame(
                    &ctx,
                    cw,
                    ch,
                    &frames,
                    &events,
                    &participants,
                    rt,
                    &frames,
                    distance,
                );
                ctx.restore();

                let snapshots: Vec<HorseSnapshot> = horses
                    .iter()
                    .enumerate()
                    .map(|(idx, h)| {
                        let active: Vec<String> = events
                            .iter()
                            .filter(|e| e.horse_idx == Some(idx as i64))
                            .filter(|e| rt >= e.frame_time && rt <= e.frame_time + 3.0)
                            .filter(|e| !matches!(&e.event_data, Some(ReplayEventData::Score)))
                            .map(|e| event_data_label(&e.event_data))
                            .collect();
                        HorseSnapshot {
                            horse_index: idx,
                            distance: h.distance,
                            speed: h.speed,
                            hp: h.hp,
                            is_tempted: h.is_tempted,
                            is_blocked: h.is_blocked,
                            active_event_labels: active,
                        }
                    })
                    .collect();
                on_frame.emit(snapshots);

                state.set(ReplayState {
                    render_time: rt,
                    frame_count: fi,
                    total_frames: total,
                    start_time: st,
                    end_time: et,
                    is_playing: playing,
                    playback_rate: rate,
                });
            });

            move || drop(interval)
        });
    }

    {
        let playing_ref = playing_ref.clone();
        let state = state.clone();
        let paused = props.paused;
        use_effect_with(paused, move |paused| {
            if *paused {
                *playing_ref.borrow_mut() = false;
                let s = (*state).clone();
                state.set(ReplayState {
                    is_playing: false,
                    ..s
                });
            }
            || {}
        });
    }

    let on_play_pause = {
        let state = state.clone();
        let playing_ref = playing_ref.clone();
        Callback::from(move |_| {
            let s = (*state).clone();
            if !s.is_playing {
                *playing_ref.borrow_mut() = true;
            } else {
                *playing_ref.borrow_mut() = false;
            }
            state.set(ReplayState {
                is_playing: !s.is_playing,
                ..s
            });
        })
    };

    let on_scrub = {
        let state = state.clone();
        let rt_ref = rt_ref.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(v) = input.value().parse::<f64>() {
                    *rt_ref.borrow_mut() = v;
                    let s = (*state).clone();
                    state.set(ReplayState {
                        render_time: v,
                        ..s
                    });
                }
            }
        })
    };

    let on_speed_change = {
        let state = state.clone();
        let rate_ref = rate_ref.clone();
        Callback::from(move |e: Event| {
            if let Some(select) = e.target_dyn_into::<HtmlSelectElement>() {
                if let Ok(v) = select.value().parse::<f64>() {
                    *rate_ref.borrow_mut() = v;
                    let s = (*state).clone();
                    state.set(ReplayState {
                        playback_rate: v,
                        ..s
                    });
                }
            }
        })
    };

    let s = (*state).clone();
    html! {
        <>
            <div class={CanvasContainerStyle::CLASS_NAME}>
                <canvas ref={canvas_ref} />
            </div>
            <div class={ControlBarStyle::CLASS_NAME}>
                <button onclick={on_play_pause}>
                    {if s.is_playing { "⏸ Pause" } else { "▶ Play" }}
                </button>
                <input type="range"
                    min={format!("{}", s.start_time)}
                    max={format!("{}", s.end_time)}
                    step="0.01"
                    value={format!("{}", s.render_time)}
                    oninput={on_scrub}
                />
                <span class="rdd-control-label">{format!("{:.2}s / {:.2}s", s.render_time, s.end_time)}</span>
                <select onchange={on_speed_change} value={s.playback_rate.to_string()}>
                    <option value="0.25">{"0.25x"}</option>
                    <option value="0.5">{"0.5x"}</option>
                    <option value="1.0" selected=true>{"1x"}</option>
                    <option value="2.0">{"2x"}</option>
                    <option value="4.0">{"4x"}</option>
                </select>
                <span class="rdd-frame-counter">{format!("{} / {}", s.frame_count, s.total_frames)}</span>
            </div>
        </>
    }
}
