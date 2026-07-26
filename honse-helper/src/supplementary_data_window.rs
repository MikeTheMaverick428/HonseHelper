use crate::{
    components::notifications::{use_timed_notification, Notification, NotificationOverlay},
    styles::{
        app::ButtonGroupStyle,
        db_status::{
            DbStatusContainerStyle, DbStatusHeaderStyle, DbStatusScrollStyle, PanelCompactStyle,
            PanelHeaderStyle, PanelStyle, StatusCardStyle, StatusGridStyle, StatusLabelStyle,
            StatusMessageStyle, StatusPillStyle,
        },
        Style, StyleManager,
    },
    tauri_bridge::invoke_tauri_command,
};
use serde_json::json;
use shared::{DatasetCheckEntry, SupplementaryDataCheckReport, SupplementaryDataSyncReport};
use yew::prelude::*;

#[function_component]
pub fn SupplementaryDataWindow() -> Html {
    let status = use_state(|| None as Option<SupplementaryDataSyncReport>);
    let check_result = use_state(|| None as Option<SupplementaryDataCheckReport>);
    let busy = use_state(|| false);
    let (state, push, remove) = use_timed_notification(5000);

    {
        let status = status.clone();
        let status_events = status.clone();
        let check_result = check_result.clone();
        use_effect_with((), move |_| {
            let status_init = status.clone();
            let check_init = check_result.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(result) =
                    invoke_tauri_command("get_supplementary_data_status", json!({})).await
                {
                    if let Ok(report) =
                        serde_json::from_value::<Option<SupplementaryDataSyncReport>>(result)
                    {
                        status_init.set(report);
                    }
                }
                if let Ok(result) =
                    invoke_tauri_command("check_supplementary_data_updates", json!({})).await
                {
                    if let Ok(report) =
                        serde_json::from_value::<SupplementaryDataCheckReport>(result)
                    {
                        check_init.set(Some(report));
                    }
                }
            });

            crate::tauri_bridge::listen_to_event(
                "supplementary-data-sync-status",
                move |payload: serde_json::Value| {
                    if let Ok(report) =
                        serde_json::from_value::<SupplementaryDataSyncReport>(payload)
                    {
                        status_events.set(Some(report));
                    }
                },
            );
            || {}
        });
    }

    let on_check = {
        let busy = busy.clone();
        let check_result = check_result.clone();
        let push = push.clone();
        Callback::from(move |_: yew::MouseEvent| {
            let busy = busy.clone();
            let check_result = check_result.clone();
            let push = push.clone();
            wasm_bindgen_futures::spawn_local(async move {
                busy.set(true);
                push(Notification::info("Checking for updates…"));
                match invoke_tauri_command("check_supplementary_data_updates", json!({})).await {
                    Ok(val) => {
                        if let Ok(report) =
                            serde_json::from_value::<SupplementaryDataCheckReport>(val)
                        {
                            let stale: Vec<&DatasetCheckEntry> =
                                report.datasets.iter().filter(|d| d.needs_update).collect();
                            if stale.is_empty() {
                                push(Notification::success("All datasets up to date ✓"));
                            } else {
                                let names: Vec<&str> =
                                    stale.iter().map(|d| d.id.as_str()).collect();
                                push(Notification::info(format!(
                                    "Updates available: {}",
                                    names.join(", ")
                                )));
                            }
                            check_result.set(Some(report));
                        }
                    }
                    Err(e) => push(Notification::error(format!("Check failed: {e}"))),
                }
                busy.set(false);
            });
        })
    };

    let on_sync = {
        let busy = busy.clone();
        let check_result = check_result.clone();
        let status = status.clone();
        let push = push.clone();
        Callback::from(move |_: yew::MouseEvent| {
            let busy = busy.clone();
            let check_result = check_result.clone();
            let status = status.clone();
            let push = push.clone();
            wasm_bindgen_futures::spawn_local(async move {
                busy.set(true);
                push(Notification::info("Syncing supplementary data…"));

                let dataset_ids = {
                    let mut ids = Vec::new();
                    if let Some(check) = &*check_result {
                        for d in &check.datasets {
                            if d.needs_update {
                                ids.push(d.id.clone());
                            }
                        }
                    }
                    if ids.is_empty() {
                        ids.push("support-events".to_string());
                    }
                    ids
                };

                match invoke_tauri_command(
                    "sync_supplementary_data",
                    json!({"datasetIds": dataset_ids}),
                )
                .await
                {
                    Ok(val) => {
                        if let Ok(report) =
                            serde_json::from_value::<SupplementaryDataSyncReport>(val)
                        {
                            status.set(Some(report));
                            push(Notification::success("Sync completed."));
                        }
                    }
                    Err(e) => push(Notification::error(format!("Sync failed: {e}"))),
                }
                check_result.set(None);
                busy.set(false);
            });
        })
    };

    let stylesheet = StyleManager::render_stylesheet();

    let needs_sync = check_result
        .as_ref()
        .map(|r| r.datasets.iter().any(|d| d.needs_update))
        .unwrap_or(false);

    let pill_text = if *busy {
        "Working…"
    } else if needs_sync {
        "Update available"
    } else {
        match &*status {
            Some(r) if r.synced => "Loaded ✓",
            _ => "Not loaded",
        }
    };

    html! {
        <div class={DbStatusContainerStyle::CLASS_NAME}>
            <div class={DbStatusHeaderStyle::CLASS_NAME}>
                <h1>{"Supplementary Data"}</h1>
            </div>

            {stylesheet}

            <NotificationOverlay notifications={state.0.clone()} on_close={{
                let remove = remove.clone();
                Callback::from(move |id: u32| remove(id))
            }} />

            <div class={DbStatusScrollStyle::CLASS_NAME}>
                <section class={classes!(PanelStyle::CLASS_NAME, PanelCompactStyle::CLASS_NAME)}>
                    <div class={PanelHeaderStyle::CLASS_NAME}>
                        <div>
                            <h2>{"Support Events"}</h2>
                            <p>{"Training event choice rewards — automatically synced from remote"}</p>
                        </div>
                        <div class={StatusPillStyle::CLASS_NAME}>{ pill_text }</div>
                    </div>

                    {
                        if let Some(report) = &*status {
                            html! {
                                <>
                                    <div class={StatusGridStyle::CLASS_NAME}>
                                        <div class={StatusCardStyle::CLASS_NAME}>
                                            <span class={StatusLabelStyle::CLASS_NAME}>{"State"}</span>
                                            <strong>{ if report.synced { "Loaded ✓" } else { "Not loaded ✗" } }</strong>
                                        </div>
                                        <div class={StatusCardStyle::CLASS_NAME}>
                                            <span class={StatusLabelStyle::CLASS_NAME}>{"Version"}</span>
                                            <strong>{
                                                report.datasets.first()
                                                    .map(|d| d.version.to_string())
                                                    .unwrap_or_else(|| "—".to_string())
                                            }</strong>
                                        </div>
                                        <div class={StatusCardStyle::CLASS_NAME}>
                                            <span class={StatusLabelStyle::CLASS_NAME}>{"Events"}</span>
                                            <strong>{ report.event_count }</strong>
                                        </div>
                                        <div class={StatusCardStyle::CLASS_NAME}>
                                            <span class={StatusLabelStyle::CLASS_NAME}>{"Choices"}</span>
                                            <strong>{ report.choice_count }</strong>
                                        </div>
                                        <div class={StatusCardStyle::CLASS_NAME}>
                                            <span class={StatusLabelStyle::CLASS_NAME}>{"Rewards"}</span>
                                            <strong>{ report.reward_count }</strong>
                                        </div>
                                    </div>

                                    <div class={StatusMessageStyle::CLASS_NAME}>{ &report.message }</div>
                                </>
                            }
                        } else {
                            html! { <div class={StatusMessageStyle::CLASS_NAME}>{"No supplementary data loaded."}</div> }
                        }
                    }

                    {
                        if let Some(check) = &*check_result {
                            html! {
                                <div style="margin: 8px 0; font-size: 0.85em; opacity: 0.8;">
                                    {
                                        check.datasets.iter().map(|d| {
                                            let local = d.local_version.map(|v| v.to_string()).unwrap_or_else(|| "—".to_string());
                                            html! {
                                                <div key={d.id.clone()}>
                                                    { &d.id }{ ": remote v" }{ d.available_version }{ " / local v" }{ local }
                                                    { if d.needs_update { " ⬆" } else { " ✓" } }
                                                </div>
                                            }
                                        }).collect::<Html>()
                                    }
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }

                    <div class={ButtonGroupStyle::CLASS_NAME}>
                        <button onclick={on_check} disabled={*busy}>
                            { "Check for updates" }
                        </button>
                        <button onclick={on_sync} disabled={*busy || !needs_sync}>
                            { if *busy { "Syncing…" } else { "Sync" } }
                        </button>
                    </div>
                </section>
            </div>
        </div>
    }
}
