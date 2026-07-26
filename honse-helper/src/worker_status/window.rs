use crate::{
    styles::{
        db_status::{
            PanelCompactStyle, PanelHeaderStyle, PanelStyle, StatusCardStyle, StatusGridStyle,
            StatusLabelStyle, StatusMessageStyle, StatusPillStyle,
        },
        legacy_planner::SecondaryBtnStyle,
        worker_status::{
            ConfigInputStyle, ConfigLabelStyle, ConfigRowStyle, ConfigUnitStyle, ControlsGridStyle,
            KnownViewBadgeStyle, RetryCountStyle, ToggleCheckboxStyle, ToggleLabelStyle,
            UnknownViewBadgeStyle, ViewCardStyle, ViewRowStyle, WorkerStatusContainerStyle,
            WorkerStatusHeaderStyle, WorkerStatusScrollStyle,
        },
        Style, StyleManager,
    },
    tauri_bridge::invoke_tauri_command,
};
use serde_json::json;
use shared::worker_state::WorkerStatusReport;
use yew::prelude::*;

#[derive(Clone)]
struct ConfigDraft {
    auto_start: bool,
    max_retries: u32,
    retry_interval_secs: u32,
    discovery_interval_secs: u32,
    dirty: bool,
}

impl ConfigDraft {
    fn from_report(r: &WorkerStatusReport) -> Self {
        Self {
            auto_start: r.auto_start,
            max_retries: r.max_retries,
            retry_interval_secs: r.retry_interval_secs,
            discovery_interval_secs: r.discovery_interval_secs,
            dirty: false,
        }
    }
}

#[derive(Clone, PartialEq)]
struct ConfigLabel {
    pub name: &'static str,
    pub subtitle: &'static str,
}

const CFG_AUTO_START: ConfigLabel = ConfigLabel {
    name: "Auto-start",
    subtitle: "Automatically start worker when app launches",
};
const CFG_MAX_RETRIES: ConfigLabel = ConfigLabel {
    name: "Max retries",
    subtitle: "How many times to attempt finding the game process before giving up",
};
const CFG_RETRY_INTERVAL: ConfigLabel = ConfigLabel {
    name: "Retry interval",
    subtitle: "Seconds to wait between each find attempt",
};
const CFG_DISCOVERY_INTERVAL: ConfigLabel = ConfigLabel {
    name: "View discovery interval",
    subtitle: "How often to refresh the current game view (only when process is found)",
};

#[function_component]
pub fn WorkerStatusWindow() -> Html {
    let status = use_state(WorkerStatusReport::default);
    let busy = use_state(|| false);
    let collapsed = use_state(|| vec![false; 4]);
    let config = use_state(|| ConfigDraft::from_report(&WorkerStatusReport::default()));
    let save_feedback = use_state(|| None::<String>);

    let toggle_collapse = {
        let collapsed = collapsed.clone();
        Callback::from(move |idx: usize| {
            let mut c = (*collapsed).clone();
            if idx < c.len() {
                c[idx] = !c[idx];
            }
            collapsed.set(c);
        })
    };

    {
        let status_init = status.clone();
        let config_init = config.clone();
        use_effect_with((), move |_| {
            let status_for_events = status_init.clone();
            let config_for_events = config_init.clone();
            crate::tauri_bridge::listen_to_event(
                "worker-status",
                move |payload: serde_json::Value| {
                    if let Ok(report) = serde_json::from_value::<WorkerStatusReport>(payload) {
                        let mut c = (*config_for_events).clone();
                        if !c.dirty {
                            c = ConfigDraft::from_report(&report);
                            config_for_events.set(c);
                        }
                        status_for_events.set(report);
                    }
                },
            );

            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(result) = invoke_tauri_command("get_worker_status", json!({})).await {
                    if let Ok(report) = serde_json::from_value::<WorkerStatusReport>(result) {
                        config_init.set(ConfigDraft::from_report(&report));
                        status_init.set(report);
                    }
                }
            });

            || {}
        });
    }

    let invoke_and_refresh = {
        let status = status.clone();
        let busy = busy.clone();
        Callback::from(move |cmd: String| {
            let status = status.clone();
            let busy = busy.clone();
            wasm_bindgen_futures::spawn_local(async move {
                busy.set(true);
                let _ = invoke_tauri_command(&cmd, json!({})).await;
                if let Ok(result) = invoke_tauri_command("get_worker_status", json!({})).await {
                    if let Ok(report) = serde_json::from_value::<WorkerStatusReport>(result) {
                        status.set(report);
                    }
                }
                busy.set(false);
            });
        })
    };

    let on_start_worker = {
        let invoke = invoke_and_refresh.clone();
        Callback::from(move |_| invoke.emit("start_worker".to_string()))
    };

    let on_stop_worker = {
        let invoke = invoke_and_refresh.clone();
        Callback::from(move |_| invoke.emit("stop_worker".to_string()))
    };

    let on_find_process = {
        let busy = busy.clone();
        let status = status.clone();
        let config = config.clone();
        Callback::from(move |_| {
            let busy = busy.clone();
            let status = status.clone();
            let config = config.clone();
            wasm_bindgen_futures::spawn_local(async move {
                busy.set(true);
                let _ = invoke_tauri_command(
                    "send_worker_request",
                    json!({
                        "request": {
                            "id": 5100,
                            "command": "find_process"
                        }
                    }),
                )
                .await;
                // The forward_worker_frame interceptor will update the supervisor
                // and emit worker-status, so just wait briefly then refresh
                gloo_timers::future::TimeoutFuture::new(500).await;
                if let Ok(result) = invoke_tauri_command("get_worker_status", json!({})).await {
                    if let Ok(report) = serde_json::from_value::<WorkerStatusReport>(result) {
                        let mut c = (*config).clone();
                        if !c.dirty {
                            c = ConfigDraft::from_report(&report);
                            config.set(c);
                        }
                        status.set(report);
                    }
                }
                busy.set(false);
            });
        })
    };

    let on_refresh_view = {
        let busy = busy.clone();
        let status = status.clone();
        let config = config.clone();
        Callback::from(move |_| {
            let busy = busy.clone();
            let status = status.clone();
            let config = config.clone();
            wasm_bindgen_futures::spawn_local(async move {
                busy.set(true);
                let _ = invoke_tauri_command(
                    "send_worker_request",
                    json!({
                        "request": {
                            "id": 5200,
                            "command": "get_view_state"
                        }
                    }),
                )
                .await;
                gloo_timers::future::TimeoutFuture::new(500).await;
                if let Ok(result) = invoke_tauri_command("get_worker_status", json!({})).await {
                    if let Ok(report) = serde_json::from_value::<WorkerStatusReport>(result) {
                        let mut c = (*config).clone();
                        if !c.dirty {
                            c = ConfigDraft::from_report(&report);
                            config.set(c);
                        }
                        status.set(report);
                    }
                }
                busy.set(false);
            });
        })
    };

    let on_reset_retry = {
        let invoke = invoke_and_refresh.clone();
        Callback::from(move |_| invoke.emit("reset_worker_retry_count".to_string()))
    };

    let on_save_config = {
        let config = config.clone();
        let busy = busy.clone();
        let save_feedback = save_feedback.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            let c = (*config).clone();
            let busy = busy.clone();
            let save_feedback = save_feedback.clone();
            let config = config.clone();
            wasm_bindgen_futures::spawn_local(async move {
                busy.set(true);
                save_feedback.set(Some("Saving\u{2026}".to_string()));

                let mut msg: Option<String> = None;

                let r1 = invoke_tauri_command(
                    "set_worker_auto_start",
                    json!({ "autoStart": c.auto_start }),
                )
                .await;
                if let Err(e) = &r1 {
                    msg = Some(format!("auto-start: {}", e));
                }

                let r2 = invoke_tauri_command(
                    "set_worker_retry_config",
                    json!({
                        "maxRetries": c.max_retries,
                        "intervalSecs": c.retry_interval_secs,
                    }),
                )
                .await;
                if msg.is_none() {
                    if let Err(e) = &r2 {
                        msg = Some(format!("retry config: {}", e));
                    }
                }

                let r3 = invoke_tauri_command(
                    "set_worker_discovery_interval",
                    json!({ "intervalSecs": c.discovery_interval_secs }),
                )
                .await;
                if msg.is_none() {
                    if let Err(e) = &r3 {
                        msg = Some(format!("discovery interval: {}", e));
                    }
                }

                config.set(ConfigDraft { dirty: false, ..c });
                save_feedback.set(Some(msg.unwrap_or_else(|| "Saved".to_string())));
                gloo_timers::future::TimeoutFuture::new(3000).await;
                save_feedback.set(None);
                busy.set(false);
            });
        })
    };

    let set_auto_start = {
        let config = config.clone();
        Callback::from(move |_| {
            let mut c = (*config).clone();
            c.auto_start = !c.auto_start;
            c.dirty = true;
            config.set(c);
        })
    };

    let set_max_retries = {
        let config = config.clone();
        Callback::from(move |e: web_sys::Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            if let Ok(val) = input.value().parse::<u32>() {
                let mut c = (*config).clone();
                c.max_retries = val;
                c.dirty = true;
                config.set(c);
            }
        })
    };

    let set_retry_interval = {
        let config = config.clone();
        Callback::from(move |e: web_sys::Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            if let Ok(val) = input.value().parse::<u32>() {
                let mut c = (*config).clone();
                c.retry_interval_secs = val;
                c.dirty = true;
                config.set(c);
            }
        })
    };

    let set_discovery_interval = {
        let config = config.clone();
        Callback::from(move |e: web_sys::Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            if let Ok(val) = input.value().parse::<u32>() {
                let mut c = (*config).clone();
                c.discovery_interval_secs = val;
                c.dirty = true;
                config.set(c);
            }
        })
    };

    let copy_to_clipboard = Callback::from(|text: String| {
        if let Some(window) = web_sys::window() {
            let _ = window.navigator().clipboard().write_text(&text);
        }
    });

    let stylesheet = StyleManager::render_stylesheet();
    let report = &*status;
    let cfg = &*config;

    let status_label = if report.worker_running {
        if report.process_found {
            "Running \u{2022} Process Found"
        } else if report.retry_count >= report.max_retries {
            "Running \u{2022} Retries Exhausted"
        } else {
            "Running \u{2022} Searching"
        }
    } else {
        "Stopped"
    };

    let status_dot_color = if !report.worker_running {
        "#94a3b8"
    } else if report.process_found {
        "#22c55e"
    } else if report.retry_count >= report.max_retries {
        "#f97316"
    } else {
        "#38bdf8"
    };

    let process_basename = report
        .process_name
        .as_deref()
        .map(|p| p.rsplit(['/', '\\']).next().unwrap_or(p))
        .unwrap_or("\u{2014}");

    let process_full = report.process_path.as_deref().unwrap_or("");

    html! {
        <div class={WorkerStatusContainerStyle::CLASS_NAME}>
            <div class={WorkerStatusHeaderStyle::CLASS_NAME}>
                <h1>{"Worker Status"}</h1>
            </div>

            {stylesheet}

            <div class={WorkerStatusScrollStyle::CLASS_NAME}>

                // ── Status ────────────────────────────────────────────────────
                <section class={classes!(PanelStyle::CLASS_NAME, PanelCompactStyle::CLASS_NAME)}>
                    <div class={PanelHeaderStyle::CLASS_NAME}>
                        <div>
                            <h2>{"Status"}</h2>
                            <p>{"Worker and game process state"}</p>
                        </div>
                        <div style="display: flex; align-items: center; gap: 8px;">
                            <div class={StatusPillStyle::CLASS_NAME}>
                                <span style={format!(
                                    "display: inline-block; width: 8px; height: 8px; border-radius: 50%; background: {}; margin-right: 6px;",
                                    status_dot_color
                                )}></span>
                                {status_label}
                            </div>
                            <button
                                class={SecondaryBtnStyle::CLASS_NAME}
                                style="padding: 4px 8px; font-size: 12px;"
                                onclick={toggle_collapse.reform(move |_| 0)}
                            >
                                {if collapsed[0] { "\u{25B6}" } else { "\u{25BC}" }}
                            </button>
                        </div>
                    </div>

                    if !collapsed[0] {
                        <div class={StatusGridStyle::CLASS_NAME} style="grid-template-columns: repeat(2, minmax(0, 1fr));">
                            <div class={StatusCardStyle::CLASS_NAME}>
                                <span class={StatusLabelStyle::CLASS_NAME}>{"Worker"}</span>
                                <strong style="font-size: 13px;">{if report.worker_running { "Running \u{2713}" } else { "Stopped \u{2717}" }}</strong>
                            </div>
                            <div class={StatusCardStyle::CLASS_NAME}>
                                <span class={StatusLabelStyle::CLASS_NAME}>{"Game Process"}</span>
                                <strong style="font-size: 13px;">{if report.process_found { "Found \u{2713}" } else { "Not found \u{2717}" }}</strong>
                            </div>
                        </div>

                        if report.process_found {
                            <div class={StatusCardStyle::CLASS_NAME} style="margin-bottom: 8px;">
                                <span class={StatusLabelStyle::CLASS_NAME}>{"Process"}</span>
                                <strong
                                    style="font-size: 13px; cursor: pointer; user-select: all;"
                                    title={"Click to copy full path"}
                                    onclick={
                                        let full = process_full.to_string();
                                        copy_to_clipboard.reform(move |_| full.clone())
                                    }
                                >
                                    {process_basename}
                                </strong>
                            </div>
                        }

                        <div class={StatusMessageStyle::CLASS_NAME} style="font-size: 12px;">
                            {
                                if !report.worker_running {
                                    "Worker is not running. Start it manually or enable auto-start.".to_string()
                                } else if report.process_found {
                                    "Game process found and attached.".to_string()
                                } else if report.retry_count >= report.max_retries {
                                    format!(
                                        "Failed to find game process after {} attempts. Reset retry count or adjust config.",
                                        report.max_retries
                                    )
                                } else {
                                    format!(
                                        "Searching for game process (attempt {} of {})\u{2026}",
                                        report.retry_count.saturating_add(1),
                                        report.max_retries
                                    )
                                }
                            }
                        </div>
                    }
                </section>

                // ── Current View ──────────────────────────────────────────────
                <section class={classes!(PanelStyle::CLASS_NAME, PanelCompactStyle::CLASS_NAME)}>
                    <div class={PanelHeaderStyle::CLASS_NAME}>
                        <div>
                            <h2>{"Current View"}</h2>
                            <p>{"Live game view discovery"}</p>
                        </div>
                        <div style="display: flex; align-items: center; gap: 8px;">
                                if let Some(known) = &report.current_known_view {
                                    <div class={KnownViewBadgeStyle::CLASS_NAME}>{known}</div>
                                } else if report.current_view_id_raw.is_some() {
                                    <div class={UnknownViewBadgeStyle::CLASS_NAME}>
                                        {"Unknown View"}
                                    </div>
                                }
                            <button
                                onclick={on_refresh_view}
                                disabled={*busy || !report.process_found}
                            >
                                {"Refresh View"}
                            </button>
                            <button
                                class={SecondaryBtnStyle::CLASS_NAME}
                                style="padding: 4px 8px; font-size: 12px;"
                                onclick={toggle_collapse.reform(move |_| 1)}
                            >
                                {if collapsed[1] { "\u{25B6}" } else { "\u{25BC}" }}
                            </button>
                        </div>
                    </div>

                    if !collapsed[1] {
                        if report.current_view_id_raw.is_some()
                            || report.current_view_kclass.is_some()
                            || report.current_view_class.is_some()
                            || report.current_scene_class.is_some()
                        {
                            <div class={ViewCardStyle::CLASS_NAME}>
                                {
                                    report.current_view_id_raw.map(|id| html! {
                                        <div class={ViewRowStyle::CLASS_NAME}>
                                            <span>{"View ID"}</span>
                                            <span>{id.to_string()}</span>
                                        </div>
                                    })
                                }
                                {
                                    report.current_view_kclass.as_ref().map(|v| html! {
                                        <div class={ViewRowStyle::CLASS_NAME}>
                                            <span>{"kclass"}</span>
                                            <span>{v}</span>
                                        </div>
                                    })
                                }
                                {
                                    report.current_view_class.as_ref().map(|v| html! {
                                        <div class={ViewRowStyle::CLASS_NAME}>
                                            <span>{"View Class"}</span>
                                            <span>{v}</span>
                                        </div>
                                    })
                                }
                                {
                                    report.current_view_ptr.as_ref().map(|v| html! {
                                        <div class={ViewRowStyle::CLASS_NAME}>
                                            <span>{"View Ptr"}</span>
                                            <span>{v}</span>
                                        </div>
                                    })
                                }
                                {
                                    report.current_scene_class.as_ref().map(|v| html! {
                                        <div class={ViewRowStyle::CLASS_NAME}>
                                            <span>{"Scene Class"}</span>
                                            <span>{v}</span>
                                        </div>
                                    })
                                }
                                {
                                    report.current_scene_base_ptr.as_ref().map(|v| html! {
                                        <div class={ViewRowStyle::CLASS_NAME}>
                                            <span>{"Scene Ptr"}</span>
                                            <span>{v}</span>
                                        </div>
                                    })
                                }
                            </div>
                        } else {
                            <div class={StatusMessageStyle::CLASS_NAME} style="font-size: 12px;">
                                {
                                    if report.worker_running && report.process_found {
                                        "Waiting for view discovery\u{2026}"
                                    } else {
                                        "Process not found \u{2014} view discovery unavailable."
                                    }
                                }
                            </div>
                        }
                    }
                </section>

                // ── Configuration ──────────────────────────────────────────────
                <section class={classes!(PanelStyle::CLASS_NAME, PanelCompactStyle::CLASS_NAME)}>
                    <div class={PanelHeaderStyle::CLASS_NAME}>
                        <div>
                            <h2>{"Configuration"}</h2>
                            <p>{"Worker behavior settings"}</p>
                        </div>
                        <div style="display: flex; align-items: center; gap: 8px;">
                            {
                                save_feedback.as_ref().map(|msg| html! {
                                    <span style="font-size: 12px; color: #94a3b8; margin-right: 4px;">
                                        {msg.clone()}
                                    </span>
                                })
                            }
                            <button
                                onclick={on_save_config.clone()}
                                disabled={*busy || !cfg.dirty}
                            >
                                {"Save"}
                            </button>
                            <button
                                class={SecondaryBtnStyle::CLASS_NAME}
                                style="padding: 4px 8px; font-size: 12px;"
                                onclick={toggle_collapse.reform(move |_| 2)}
                            >
                                {if collapsed[2] { "\u{25B6}" } else { "\u{25BC}" }}
                            </button>
                        </div>
                    </div>

                    if !collapsed[2] {
                        <div class={ConfigRowStyle::CLASS_NAME}>
                            <span class={ConfigLabelStyle::CLASS_NAME}>{CFG_AUTO_START.name}</span>
                            <input
                                class={ToggleCheckboxStyle::CLASS_NAME}
                                type="checkbox"
                                checked={cfg.auto_start}
                                onclick={set_auto_start}
                                disabled={*busy}
                            />
                            <span class={ToggleLabelStyle::CLASS_NAME}>
                                {CFG_AUTO_START.subtitle}
                            </span>
                        </div>

                        <div class={ConfigRowStyle::CLASS_NAME}>
                            <span class={ConfigLabelStyle::CLASS_NAME} title={CFG_MAX_RETRIES.subtitle}>
                                {CFG_MAX_RETRIES.name}
                            </span>
                            <input
                                class={ConfigInputStyle::CLASS_NAME}
                                type="number"
                                min="1"
                                max="1000"
                                value={cfg.max_retries.to_string()}
                                onchange={set_max_retries}
                                disabled={*busy}
                            />
                            <span class={ConfigUnitStyle::CLASS_NAME}>{"attempts"}</span>
                            <span class={RetryCountStyle::CLASS_NAME}>
                                {"Current: "}
                                <span class="retry-current">{report.retry_count}</span>
                                {" / "}
                                <span class="retry-max">{cfg.max_retries}</span>
                            </span>
                        </div>
                        <div style="font-size: 11px; color: #64748b; margin: -6px 0 10px 152px;">
                            {CFG_MAX_RETRIES.subtitle}
                        </div>

                        <div class={ConfigRowStyle::CLASS_NAME}>
                            <span class={ConfigLabelStyle::CLASS_NAME} title={CFG_RETRY_INTERVAL.subtitle}>
                                {CFG_RETRY_INTERVAL.name}
                            </span>
                            <input
                                class={ConfigInputStyle::CLASS_NAME}
                                type="number"
                                min="1"
                                max="3600"
                                value={cfg.retry_interval_secs.to_string()}
                                onchange={set_retry_interval}
                                disabled={*busy}
                            />
                            <span class={ConfigUnitStyle::CLASS_NAME}>{"seconds"}</span>
                        </div>
                        <div style="font-size: 11px; color: #64748b; margin: -6px 0 10px 152px;">
                            {CFG_RETRY_INTERVAL.subtitle}
                        </div>

                        <div class={ConfigRowStyle::CLASS_NAME}>
                            <span class={ConfigLabelStyle::CLASS_NAME} title={CFG_DISCOVERY_INTERVAL.subtitle}>
                                {CFG_DISCOVERY_INTERVAL.name}
                            </span>
                            <input
                                class={ConfigInputStyle::CLASS_NAME}
                                type="number"
                                min="1"
                                max="3600"
                                value={cfg.discovery_interval_secs.to_string()}
                                onchange={set_discovery_interval}
                                disabled={*busy}
                            />
                            <span class={ConfigUnitStyle::CLASS_NAME}>{"seconds"}</span>
                            <span style="font-size: 11px; color: #64748b; margin-left: 4px;">
                                {" \u{2190} how often the current view updates"}
                            </span>
                        </div>
                        <div style="font-size: 11px; color: #64748b; margin: -6px 0 10px 152px;">
                            {CFG_DISCOVERY_INTERVAL.subtitle}
                        </div>
                    }
                </section>

                // ── Controls ──────────────────────────────────────────────────
                <section class={classes!(PanelStyle::CLASS_NAME, PanelCompactStyle::CLASS_NAME)}>
                    <div class={PanelHeaderStyle::CLASS_NAME}>
                        <div>
                            <h2>{"Controls"}</h2>
                            <p>{"Manual worker management"}</p>
                        </div>
                        <button
                            class={SecondaryBtnStyle::CLASS_NAME}
                            style="padding: 4px 8px; font-size: 12px;"
                            onclick={toggle_collapse.reform(move |_| 3)}
                        >
                            {if collapsed[3] { "\u{25B6}" } else { "\u{25BC}" }}
                        </button>
                    </div>

                    if !collapsed[3] {
                        <div class={ControlsGridStyle::CLASS_NAME}>
                            <button
                                onclick={on_start_worker}
                                disabled={*busy || report.worker_running}
                            >
                                {"Start Worker"}
                            </button>
                            <button
                                class={SecondaryBtnStyle::CLASS_NAME}
                                onclick={on_stop_worker}
                                disabled={*busy || !report.worker_running}
                            >
                                {"Stop Worker"}
                            </button>
                            <button
                                onclick={on_find_process}
                                disabled={*busy || !report.worker_running}
                            >
                                {"Find Process"}
                            </button>
                            <button
                                class={SecondaryBtnStyle::CLASS_NAME}
                                onclick={on_reset_retry}
                                disabled={*busy || report.retry_count == 0}
                            >
                                {"Reset Retry Count"}
                            </button>
                        </div>
                    }
                </section>
            </div>
        </div>
    }
}
