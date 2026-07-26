use crate::{
    styles::{
        app::{
            ButtonGroupStyle, LogEntryStyle, LogErrorStyle, LogInfoStyle, LogLabelStyle,
            LogMessageStyle, LogTimeStyle, LogViewerStyle, LogWarningStyle,
        },
        legacy_planner::SecondaryBtnStyle,
        Style, StyleManager,
    },
    tauri_bridge::invoke_tauri_command,
};
use serde::Deserialize;
use serde_json::{json, Value};
use yew::prelude::*;

#[derive(Clone, Debug, Deserialize)]
struct ProcStats {
    cpu_pct: f32,
    memory_mb: f64,
}

#[derive(Clone, Debug, Deserialize)]
struct ResourceStats {
    app: ProcStats,
    worker: Option<ProcStats>,
}

#[derive(Clone)]
struct LogEntry {
    timestamp: String,
    label: String,
    message: String,
}

#[function_component]
pub fn DevTools() -> Html {
    let log_entries = use_state(Vec::<LogEntry>::new);
    let latest_response = use_state(|| None as Option<Value>);
    let master_db_status = use_state(|| None as Option<shared::MasterDbStatus>);
    let resource_stats = use_state(|| None as Option<ResourceStats>);

    let add_log = {
        let log_entries = log_entries.clone();
        Callback::from(move |(label, message): (String, String)| {
            let now = chrono::Local::now();
            let timestamp = now.format("%H:%M:%S%.3f").to_string();
            let entry = LogEntry {
                timestamp,
                label,
                message,
            };
            let mut entries = (*log_entries).clone();
            entries.push(entry);
            log_entries.set(entries);
        })
    };

    {
        let add_log = add_log.clone();
        let latest_response_for_events = latest_response.clone();
        let resource_stats_for_events = resource_stats.clone();
        use_effect_with((), move |_| {
            let add_log_response = add_log.clone();
            crate::tauri_bridge::listen_to_event("worker-response", move |payload: Value| {
                latest_response_for_events.set(Some(payload.clone()));
                add_log_response.emit(("response".to_string(), format!("{}", payload)));
            });

            let add_log_err = add_log.clone();
            crate::tauri_bridge::listen_to_event("worker-stderr", move |payload: Value| {
                add_log_err.emit((
                    "stderr".to_string(),
                    payload.as_str().unwrap_or("unknown").to_string(),
                ));
            });

            let add_log_parse = add_log.clone();
            crate::tauri_bridge::listen_to_event("worker-parse-error", move |payload: Value| {
                add_log_parse.emit((
                    "error".to_string(),
                    payload.as_str().unwrap_or("parse error").to_string(),
                ));
            });

            let add_log_stream = add_log.clone();
            crate::tauri_bridge::listen_to_event("worker-stream-error", move |payload: Value| {
                add_log_stream.emit((
                    "error".to_string(),
                    payload.as_str().unwrap_or("stream error").to_string(),
                ));
            });

            let master_db_status_for_log = master_db_status.clone();
            let add_log_master = add_log.clone();
            crate::tauri_bridge::listen_to_event("master-db-status", move |payload: Value| {
                match serde_json::from_value::<shared::MasterDbStatus>(payload) {
                    Ok(status) => {
                        if status.found {
                            add_log_master.emit(("info".to_string(), status.message.clone()));
                        } else {
                            add_log_master.emit(("warning".to_string(), status.message.clone()));
                        }
                        master_db_status_for_log.set(Some(status));
                    }
                    Err(err) => add_log_master.emit((
                        "error".to_string(),
                        format!("failed to parse master-db status: {}", err),
                    )),
                }
            });

            let resource_stats_setter = resource_stats_for_events.clone();
            crate::tauri_bridge::listen_to_event("resource-stats", move |payload: Value| {
                if let Ok(stats) = serde_json::from_value::<ResourceStats>(payload) {
                    resource_stats_setter.set(Some(stats));
                }
            });

            || {}
        });
    }

    let on_send_request = {
        let add_log = add_log.clone();
        Callback::from(move |(id, cmd, extra): (u64, String, Value)| {
            let add_log = add_log.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let mut request = json!({
                    "id": id,
                    "command": cmd,
                });
                if let Value::Object(map) = extra {
                    for (k, v) in map {
                        request[k] = v;
                    }
                }
                match invoke_tauri_command("send_worker_request", json!({ "request": request }))
                    .await
                {
                    Ok(_) => {
                        add_log.emit(("request".to_string(), format!("Request {} sent", id)));
                    }
                    Err(e) => add_log.emit((
                        "error".to_string(),
                        format!("Failed to send request: {}", e),
                    )),
                }
            });
        })
    };

    let on_import_support_card_data = {
        let add_log = add_log.clone();
        Callback::from(move |_: yew::MouseEvent| {
            let add_log = add_log.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match invoke_tauri_command(
                    "import_support_card_data",
                    json!({
                        "request": { "command": "get_support_card_data" },
                        "timeoutMs": 10000
                    }),
                )
                .await
                {
                    Ok(_) => {
                        add_log.emit(("info".to_string(), "Support card data imported".to_string()))
                    }
                    Err(err) => add_log.emit((
                        "error".to_string(),
                        format!("Support card import failed: {}", err),
                    )),
                }
            });
        })
    };

    let on_save_race_dump = {
        let add_log = add_log.clone();
        Callback::from(move |_: yew::MouseEvent| {
            let add_log = add_log.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match invoke_tauri_command(
                    "save_race_dump",
                    json!({
                        "request": { "command": "get_race_team_data" },
                        "timeoutMs": 15000
                    }),
                )
                .await
                {
                    Ok(result) => add_log.emit((
                        "info".to_string(),
                        format!(
                            "Race dump saved: id={}, participants={}",
                            result
                                .get("race_dump_id")
                                .and_then(|v| v.as_i64())
                                .unwrap_or(0),
                            result
                                .get("participants")
                                .and_then(|v| v.as_u64())
                                .unwrap_or(0)
                        ),
                    )),
                    Err(err) => {
                        add_log.emit(("error".to_string(), format!("Race dump failed: {}", err)))
                    }
                }
            });
        })
    };

    let on_import_trophy_data = {
        let add_log = add_log.clone();
        Callback::from(move |_: yew::MouseEvent| {
            let add_log = add_log.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match invoke_tauri_command(
                    "import_trophy_data",
                    json!({
                        "request": { "command": "get_trophy_data" },
                        "timeoutMs": 10000
                    }),
                )
                .await
                {
                    Ok(_) => add_log.emit(("info".to_string(), "Trophy data imported".to_string())),
                    Err(err) => add_log.emit((
                        "error".to_string(),
                        format!("Trophy import failed: {}", err),
                    )),
                }
            });
        })
    };

    let on_import_card_data = {
        let add_log = add_log.clone();
        Callback::from(move |_: yew::MouseEvent| {
            let add_log = add_log.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match invoke_tauri_command(
                    "import_card_data",
                    json!({
                        "request": { "command": "get_card_data" },
                        "timeoutMs": 10000
                    }),
                )
                .await
                {
                    Ok(_) => add_log.emit(("info".to_string(), "Card data imported".to_string())),
                    Err(err) => {
                        add_log.emit(("error".to_string(), format!("Card import failed: {}", err)))
                    }
                }
            });
        })
    };

    let on_save_latest_response = {
        let latest_response = latest_response.clone();
        let add_log = add_log.clone();
        Callback::from(move |_| {
            let Some(response) = (*latest_response).clone() else {
                add_log.emit((
                    "warning".to_string(),
                    "no worker response to save yet".to_string(),
                ));
                return;
            };

            let json_text = match serde_json::to_string_pretty(&response) {
                Ok(text) => text,
                Err(err) => {
                    add_log.emit((
                        "error".to_string(),
                        format!("failed to serialize response: {}", err),
                    ));
                    return;
                }
            };

            let add_log = add_log.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match invoke_tauri_command("save_worker_response", json!({"json": json_text})).await
                {
                    Ok(result) => {
                        if result == Value::String("canceled".to_string()) {
                            add_log.emit(("warning".to_string(), "save canceled".to_string()));
                        } else if let Some(path) = result.as_str() {
                            add_log
                                .emit(("info".to_string(), format!("saved response to {}", path)));
                        } else {
                            add_log
                                .emit(("info".to_string(), format!("saved response: {}", result)));
                        }
                    }
                    Err(err) => add_log.emit((
                        "error".to_string(),
                        format!("failed to save response: {}", err),
                    )),
                }
            });
        })
    };

    let on_clear_log = {
        let log_entries = log_entries.clone();
        Callback::from(move |_| {
            log_entries.set(Vec::new());
        })
    };

    let stylesheet = StyleManager::render_stylesheet();

    html! {
        <div style="display: flex; flex-direction: column; height: 100vh; padding: 24px; overflow: hidden; background: #0f172a; color: #e2e8f0;">
            {stylesheet}
            <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 16px;">
                <h1 style="margin: 0; font-size: 20px; font-weight: 700;">{"Dev Tools"}</h1>
            </div>

                <div class={ButtonGroupStyle::CLASS_NAME}>
                    <button onclick={
                        let on_send = on_send_request.clone();
                        Callback::from(move |_| on_send.emit((1, "ping".to_string(), json!({}))))
                    }>{"Ping"}</button>

                    <button onclick={
                        let on_send = on_send_request.clone();
                        Callback::from(move |_| on_send.emit((2, "find_process".to_string(), json!({}))))
                    }>{"Find Process"}</button>

                    <button onclick={
                        let on_send = on_send_request.clone();
                        Callback::from(move |_| on_send.emit((3, "get_view_state".to_string(), json!({}))))
                    }>{"Get View State"}</button>

                    <button onclick={
                        let on_send = on_send_request.clone();
                        Callback::from(move |_| on_send.emit((4, "get_veteran_data".to_string(), json!({}))))
                    }>{"Get Veteran Data"}</button>

                    <button onclick={
                        let on_send = on_send_request.clone();
                        Callback::from(move |_| on_send.emit((5, "get_friend_data".to_string(), json!({}))))
                    }>{"Get Friend Data"}</button>

                    <button onclick={on_import_support_card_data.clone()}>{"Get Support Cards"}</button>

                    <button onclick={on_save_race_dump.clone()}>{"Save Race Dump"}</button>

                    <button onclick={
                        let on_send = on_send_request.clone();
                        Callback::from(move |_| on_send.emit((8, "get_user_data".to_string(), json!({}))))
                    }>{"Get User Data"}</button>

                    <button onclick={on_import_trophy_data.clone()}>{"Get Trophy Data"}</button>

                    <button onclick={on_import_card_data.clone()}>{"Get Card Data"}</button>
                </div>

                <div class={ButtonGroupStyle::CLASS_NAME}>
                    <button class={SecondaryBtnStyle::CLASS_NAME} onclick={on_save_latest_response} disabled={latest_response.is_none()}>{"Save Last Response"}</button>

                    <button class={SecondaryBtnStyle::CLASS_NAME} onclick={on_clear_log}>{"Clear Log"}</button>
                </div>

                <div class={LogViewerStyle::CLASS_NAME}>
                    {
                        log_entries.iter().map(|entry| {
                            let class = match entry.label.as_str() {
                                "error" => LogErrorStyle::CLASS_NAME,
                                "warning" => LogWarningStyle::CLASS_NAME,
                                _ => LogInfoStyle::CLASS_NAME,
                            };
                            html! {
                                <div class={classes!(LogEntryStyle::CLASS_NAME, class)}>
                                    <span class={LogTimeStyle::CLASS_NAME}>{&entry.timestamp}</span>
                                    <span class={LogLabelStyle::CLASS_NAME}>{&entry.label}</span>
                                    <span class={LogMessageStyle::CLASS_NAME}>{&entry.message}</span>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>

                {
                    if let Some(stats) = (*resource_stats).as_ref() {
                        html! {
                            <div style="display: flex; gap: 24px; padding: 8px 12px; margin-top: 8px; border-radius: 6px; background: #1e293b; font-family: monospace; font-size: 12px; color: #94a3b8; flex-shrink: 0;">
                                <span>
                                    {"App: "}
                                    <span style="color: #38bdf8;">{format!("{:.1}%", stats.app.cpu_pct)}</span>
                                    {" CPU | "}
                                    <span style="color: #38bdf8;">{format!("{:.1} MB", stats.app.memory_mb)}</span>
                                    {" MEM"}
                                </span>
                                {
                                    if let Some(ref w) = stats.worker {
                                        html! {
                                            <span>
                                                {"Worker: "}
                                                <span style="color: #a78bfa;">{format!("{:.1}%", w.cpu_pct)}</span>
                                                {" CPU | "}
                                                <span style="color: #a78bfa;">{format!("{:.1} MB", w.memory_mb)}</span>
                                                {" MEM"}
                                            </span>
                                        }
                                    } else {
                                        html! {
                                            <span style="color: #64748b;">{"Worker: not running"}</span>
                                        }
                                    }
                                }
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
        </div>
    }
}
