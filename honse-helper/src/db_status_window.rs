use crate::{
    components::notifications::{use_timed_notification, Notification, NotificationOverlay},
    styles::{
        app::ButtonGroupStyle,
        db_status::{
            DbStatusContainerStyle, DbStatusHeaderStyle, DbStatusScrollStyle, DetectedPathStyle,
            PanelCompactStyle, PanelHeaderStyle, PanelStyle, PathInputStyle, PathRowStyle,
            StatusCardStyle, StatusGridStyle, StatusLabelStyle, StatusMessageStyle,
            StatusPillStyle, SyncTableStyle, TableWrapperStyle, TextMonoStyle, TextRightStyle,
        },
        legacy_planner::SecondaryBtnStyle,
        Style, StyleManager,
    },
    tauri_bridge::invoke_tauri_command,
};
use chrono::{DateTime, Local};
use serde_json::json;
use shared::{AppDbSyncReport, MasterDbStatus};
use yew::prelude::*;

fn format_db_size_mb(bytes: Option<u64>) -> String {
    bytes
        .map(|b| format!("{:.1} MB", b as f64 / 1_048_576.0))
        .unwrap_or_else(|| "N/A".to_string())
}

fn format_local_datetime(input: Option<&str>) -> String {
    input
        .and_then(|value| {
            DateTime::parse_from_rfc3339(value).ok().map(|dt| {
                dt.with_timezone(&Local)
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
            })
        })
        .unwrap_or_else(|| "not yet".to_string())
}

#[function_component]
pub fn DbStatusWindow() -> Html {
    let master_status = use_state(|| None as Option<MasterDbStatus>);
    let app_db_report = use_state(|| None as Option<AppDbSyncReport>);
    let master_db_input = use_state(String::new);
    let master_db_busy = use_state(|| false);
    let app_db_busy = use_state(|| false);
    let (state, push, remove) = use_timed_notification(5000);

    // Load initial state
    {
        let master_status = master_status.clone();
        let app_db_report = app_db_report.clone();
        let master_db_input = master_db_input.clone();
        let app_db_report_events = app_db_report.clone();
        let master_status_events = master_status.clone();
        let master_db_input_events = master_db_input.clone();
        use_effect_with((), move |_| {
            let master_status_init = master_status.clone();
            let app_db_report_init = app_db_report.clone();
            let master_db_input_init = master_db_input.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(result) = invoke_tauri_command("get_master_db_status", json!({})).await {
                    if let Ok(status) = serde_json::from_value::<MasterDbStatus>(result) {
                        if let Some(path) = &status.path {
                            master_db_input_init.set(path.clone());
                        }
                        master_status_init.set(Some(status));
                    }
                }
                if let Ok(result) = invoke_tauri_command("get_app_db_status", json!({})).await {
                    if let Ok(report) = serde_json::from_value::<Option<AppDbSyncReport>>(result) {
                        app_db_report_init.set(report);
                    }
                }
            });

            crate::tauri_bridge::listen_to_event(
                "master-db-status",
                move |payload: serde_json::Value| {
                    if let Ok(status) = serde_json::from_value::<MasterDbStatus>(payload) {
                        if let Some(path) = &status.path {
                            master_db_input_events.set(path.clone());
                        }
                        master_status_events.set(Some(status));
                    }
                },
            );
            crate::tauri_bridge::listen_to_event(
                "app-db-sync-status",
                move |payload: serde_json::Value| {
                    if let Ok(report) = serde_json::from_value::<AppDbSyncReport>(payload) {
                        app_db_report_events.set(Some(report));
                    }
                },
            );
            || {}
        });
    }

    let on_master_db_input = {
        let master_db_input = master_db_input.clone();
        Callback::from(move |event: InputEvent| {
            let input = event.target_unchecked_into::<web_sys::HtmlInputElement>();
            master_db_input.set(input.value());
        })
    };

    let on_apply_master_db = {
        let master_db_input = master_db_input.clone();
        let master_db_busy = master_db_busy.clone();
        let push = push.clone();
        Callback::from(move |_: yew::MouseEvent| {
            let path = (*master_db_input).clone();
            let master_db_busy = master_db_busy.clone();
            let push = push.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if path.trim().is_empty() {
                    push(Notification::error("Enter a master.mdb path first."));
                    return;
                }
                master_db_busy.set(true);
                push(Notification::info("Saving master DB path..."));
                match invoke_tauri_command("set_master_db_path", json!({"path": path})).await {
                    Ok(_) => push(Notification::success("Master DB path saved successfully.")),
                    Err(e) => push(Notification::error(format!("Error: {e}"))),
                }
                master_db_busy.set(false);
            });
        })
    };

    let on_refresh_master_db = {
        let push = push.clone();
        let master_db_busy = master_db_busy.clone();
        let master_status = master_status.clone();
        let master_db_input = master_db_input.clone();
        Callback::from(move |_: yew::MouseEvent| {
            let push = push.clone();
            let master_db_busy = master_db_busy.clone();
            let master_status = master_status.clone();
            let master_db_input = master_db_input.clone();
            wasm_bindgen_futures::spawn_local(async move {
                master_db_busy.set(true);
                push(Notification::info("Scanning for Master DB..."));
                match invoke_tauri_command("discover_master_db_path", json!({})).await {
                    Ok(result) => {
                        if let Ok(status) = serde_json::from_value::<MasterDbStatus>(result) {
                            if let Some(path) = &status.path {
                                master_db_input.set(path.clone());
                            }
                            master_status.set(Some(status));
                        }
                        push(Notification::success("Master DB scan finished."));
                    }
                    Err(e) => push(Notification::error(format!("Master DB scan failed: {e}"))),
                }
                master_db_busy.set(false);
            });
        })
    };

    let on_sync_app_db = {
        let app_db_busy = app_db_busy.clone();
        let push = push.clone();
        Callback::from(move |_: yew::MouseEvent| {
            let app_db_busy = app_db_busy.clone();
            let push = push.clone();
            wasm_bindgen_futures::spawn_local(async move {
                app_db_busy.set(true);
                push(Notification::info("Syncing app database..."));
                match invoke_tauri_command("sync_app_database", json!({})).await {
                    Ok(_) => push(Notification::success("App DB sync completed successfully.")),
                    Err(e) => push(Notification::error(format!("Sync failed: {e}"))),
                }
                app_db_busy.set(false);
            });
        })
    };

    let stylesheet = StyleManager::render_stylesheet();

    html! {
        <div class={DbStatusContainerStyle::CLASS_NAME}>
            <div class={DbStatusHeaderStyle::CLASS_NAME}>
                <h1>{"DB Status"}</h1>
            </div>

            {stylesheet}

            <NotificationOverlay notifications={state.0.clone()} on_close={{
                let remove = remove.clone();
                Callback::from(move |id: u32| remove(id))
            }} />

            <div class={DbStatusScrollStyle::CLASS_NAME}>
                // ── Master DB ──────────────────────────────────────────────────
                <section class={classes!(PanelStyle::CLASS_NAME, PanelCompactStyle::CLASS_NAME)}>
                    <div class={PanelHeaderStyle::CLASS_NAME}>
                        <div>
                            <h2>{"Master DB"}</h2>
                            <p>{"Auto-detection + manual fallback path"}</p>
                        </div>
                        <div class={StatusPillStyle::CLASS_NAME}>{ if *master_db_busy { "Scanning…" } else { "Idle" } }</div>
                    </div>

                    <div class={StatusGridStyle::CLASS_NAME}>
                        <div class={StatusCardStyle::CLASS_NAME}>
                            <span class={StatusLabelStyle::CLASS_NAME}>{"State"}</span>
                            <strong>{ master_status.as_ref().map(|s| if s.found { "Found ✓" } else { "Missing ✗" }).unwrap_or("Unknown") }</strong>
                        </div>
                        <div class={StatusCardStyle::CLASS_NAME}>
                            <span class={StatusLabelStyle::CLASS_NAME}>{"Source"}</span>
                            <strong>{ master_status.as_ref().map(|s| s.source.as_str()).unwrap_or("pending") }</strong>
                        </div>
                        <div class={StatusCardStyle::CLASS_NAME}>
                            <span class={StatusLabelStyle::CLASS_NAME}>{"Last checked"}</span>
                            <strong>{ format_local_datetime(master_status.as_ref().and_then(|s| s.last_checked.as_deref())) }</strong>
                        </div>
                    </div>

                    <div class={StatusMessageStyle::CLASS_NAME}>
                        {
                            match master_status.as_ref() {
                                Some(s) if s.found => "Master DB available.",
                                Some(_) => "Master DB not found.",
                                None => "Waiting for discovery…",
                            }
                        }
                    </div>

                    <div class={PathRowStyle::CLASS_NAME}>
                        <input
                            class={PathInputStyle::CLASS_NAME}
                            type="text"
                            value={(*master_db_input).clone()}
                            oninput={on_master_db_input}
                            placeholder="/home/.../master.mdb"
                        />
                        <button onclick={on_apply_master_db} disabled={*master_db_busy}>{"Use path"}</button>
                        <button class={SecondaryBtnStyle::CLASS_NAME} onclick={on_refresh_master_db} disabled={*master_db_busy}>{"Auto-detect again"}</button>
                    </div>
                </section>

                // ── App DB sync ───────────────────────────────────────────────
                <section class={classes!(PanelStyle::CLASS_NAME, PanelCompactStyle::CLASS_NAME)}>
                    <div class={PanelHeaderStyle::CLASS_NAME}>
                        <div>
                            <h2>{"App DB Sync"}</h2>
                            <p>{"Local application database synced from Master DB"}</p>
                        </div>
                        <div class={StatusPillStyle::CLASS_NAME}>{ if *app_db_busy { "Syncing…" } else { "Idle" } }</div>
                    </div>

                    {
                        if let Some(report) = &*app_db_report {
                            html! {
                                <>
                                    <div class={StatusGridStyle::CLASS_NAME}>
                                        <div class={StatusCardStyle::CLASS_NAME}>
                                            <span class={StatusLabelStyle::CLASS_NAME}>{"Synced"}</span>
                                            <strong>{ if report.up_to_date { "Up to date ✓" } else { "Needs sync ✗" } }</strong>
                                        </div>
                                        <div class={StatusCardStyle::CLASS_NAME}>
                                            <span class={StatusLabelStyle::CLASS_NAME}>{"App version"}</span>
                                            <strong>{ &report.app_version }</strong>
                                        </div>
                                        <div class={StatusCardStyle::CLASS_NAME}>
                                            <span class={StatusLabelStyle::CLASS_NAME}>{"Checked at"}</span>
                                            <strong>{ format_local_datetime(Some(report.checked_at.as_str())) }</strong>
                                        </div>
                                        </div>

                                    <div class={StatusMessageStyle::CLASS_NAME}>
                                        { format!("DB size: {}", format_db_size_mb(report.db_size_bytes)) }
                                    </div>

                                    {
                                        report.source_db_path.as_ref().map(|p| html! {
                                            <div class={DetectedPathStyle::CLASS_NAME}>{ format!("Source: {}", p) }</div>
                                        }).unwrap_or_default()
                                    }

                                    <div class={StatusMessageStyle::CLASS_NAME}>{ &report.message }</div>

                                    {
                                        if !report.table_states.is_empty() {
                                            html! {
                                                <div class={TableWrapperStyle::CLASS_NAME}>
                                                    <table class={SyncTableStyle::CLASS_NAME}>
                                                        <thead>
                                                            <tr>
                                                                <th>{"Table"}</th>
                                                                <th>{"Rows"}</th>
                                                                <th>{"Synced at"}</th>
                                                            </tr>
                                                        </thead>
                                                        <tbody>
                                                            { report.table_states.iter().map(|ts| html! {
                                                                <tr>
                                                                    <td>{ &ts.table_name }</td>
                                                                    <td class={TextRightStyle::CLASS_NAME}>{ ts.row_count }</td>
                                                                    <td class={TextMonoStyle::CLASS_NAME}>{ format_local_datetime(Some(ts.synced_at.as_str())) }</td>
                                                                </tr>
                                                            }).collect::<Html>() }
                                                        </tbody>
                                                    </table>
                                                </div>
                                            }
                                        } else {
                                            html! { <div class={StatusMessageStyle::CLASS_NAME}>{"No table sync records yet."}</div> }
                                        }
                                    }
                                </>
                            }
                        } else {
                            html! { <div class={StatusMessageStyle::CLASS_NAME}>{"App DB status not available — run a sync first."}</div> }
                        }
                    }

                    <div class={ButtonGroupStyle::CLASS_NAME}>
                        <button onclick={on_sync_app_db} disabled={*app_db_busy}>
                            { if *app_db_busy { "Syncing…" } else { "Sync now" } }
                        </button>
                    </div>
                </section>
            </div>
        </div>
    }
}
