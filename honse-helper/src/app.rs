use crate::{
    components::external_link::ExternalLink,
    styles::{
        app::{
            ApiConfiguredStyle, ApiDotStyle, ApiIndicatorStyle, ApiUnconfiguredStyle,
            AppContainerStyle, FeatureCardStyle, FooterStyle, HeaderStatusGroupStyle, MasterDbBusyStyle,
            MasterDbDotStyle, MasterDbIndicatorStyle, MasterDbMissingStyle, MasterDbPartialStyle,
            MasterDbReadyStyle, SupplDotStyle, SupplIndicatorStyle, SupplMissingStyle,
            SupplReadyStyle, SupplUpdateStyle, TopRowStyle, VersionPillStyle, WorkerDotStyle, WorkerErrorStyle,
            WorkerIndicatorStyle, WorkerReadyStyle, WorkerSearchingStyle, WorkerStoppedStyle,
        },
        Style, StyleManager,
    },
    tauri_bridge::invoke_tauri_command,
};
use serde_json::json;
use serde_json::Value;
use shared::worker_state::WorkerStatusReport;
use shared::{
    ApiKeyStatus, AppDbSyncReport, MasterDbStatus, SupplementaryDataCheckReport,
    SupplementaryDataSyncReport,
};
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let master_db_status = use_state(|| None as Option<MasterDbStatus>);
    let master_db_busy = use_state(|| false);
    let app_db_sync_report = use_state(|| None as Option<AppDbSyncReport>);
    let app_db_sync_busy = use_state(|| false);
    let worker_status = use_state(WorkerStatusReport::default);
    let api_config_status = use_state(|| ApiKeyStatus {
        configured: false,
        status: String::new(),
    });
    let suppl_status = use_state(|| None as Option<SupplementaryDataSyncReport>);
    let suppl_check = use_state(|| None as Option<SupplementaryDataCheckReport>);

    // Set up event listeners for header status indicators
    {
        let master_db_status_for_events = master_db_status.clone();
        let master_db_status_for_initial = master_db_status.clone();
        let app_db_sync_report = app_db_sync_report.clone();
        let worker_status_for_events = worker_status.clone();
        let api_config_status_for_events = api_config_status.clone();
        let api_config_status_init = api_config_status.clone();
        let suppl_status_for_events = suppl_status.clone();
        let suppl_status_init = suppl_status.clone();
        let suppl_check_init = suppl_check.clone();
        let suppl_check_for_events = suppl_check.clone();
        use_effect_with((), move |_| {
            crate::tauri_bridge::listen_to_event("master-db-status", move |payload: Value| {
                if let Ok(status) = serde_json::from_value::<MasterDbStatus>(payload) {
                    master_db_status_for_events.set(Some(status));
                }
            });

            let app_db_report_for_events = app_db_sync_report.clone();
            crate::tauri_bridge::listen_to_event("app-db-sync-status", move |payload: Value| {
                if let Ok(report) = serde_json::from_value::<AppDbSyncReport>(payload) {
                    app_db_report_for_events.set(Some(report));
                }
            });

            crate::tauri_bridge::listen_to_event("worker-status", move |payload: Value| {
                if let Ok(report) = serde_json::from_value::<WorkerStatusReport>(payload) {
                    worker_status_for_events.set(report);
                }
            });

            crate::tauri_bridge::listen_to_event("api-key-status", move |payload: Value| {
                if let Ok(config) = serde_json::from_value::<ApiKeyStatus>(payload) {
                    api_config_status_for_events.set(config);
                }
            });

            crate::tauri_bridge::listen_to_event(
                "supplementary-data-sync-status",
                move |payload: Value| {
                    if let Ok(report) =
                        serde_json::from_value::<SupplementaryDataSyncReport>(payload)
                    {
                        suppl_status_for_events.set(Some(report));
                        suppl_check_for_events.set(None);
                    }
                },
            );

            let app_db_report_init = app_db_sync_report.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(result) = invoke_tauri_command("get_app_db_status", json!({})).await {
                    if let Ok(report) = serde_json::from_value::<Option<AppDbSyncReport>>(result) {
                        app_db_report_init.set(report);
                    }
                }
                if let Ok(result) = invoke_tauri_command("get_master_db_status", json!({})).await {
                    if let Ok(status) = serde_json::from_value::<MasterDbStatus>(result) {
                        master_db_status_for_initial.set(Some(status));
                    }
                }
                if let Ok(result) = invoke_tauri_command("get_api_key_status", json!({})).await {
                    if let Ok(status) = serde_json::from_value::<ApiKeyStatus>(result) {
                        api_config_status_init.set(status);
                    }
                }
                if let Ok(result) =
                    invoke_tauri_command("get_supplementary_data_status", json!({})).await
                {
                    if let Ok(report) =
                        serde_json::from_value::<Option<SupplementaryDataSyncReport>>(result)
                    {
                        suppl_status_init.set(report);
                    }
                }
                if let Ok(result) =
                    invoke_tauri_command("check_supplementary_data_updates", json!({})).await
                {
                    if let Ok(report) =
                        serde_json::from_value::<SupplementaryDataCheckReport>(result)
                    {
                        suppl_check_init.set(Some(report));
                    }
                }
            });

            || {}
        });
    }

    let on_open_db_status_window = {
        Callback::from(move |_: yew::MouseEvent| {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke_tauri_command("open_db_status_window", json!({})).await;
            });
        })
    };

    let on_open_worker_status_window = {
        Callback::from(move |_: yew::MouseEvent| {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke_tauri_command("open_worker_status_window", json!({})).await;
            });
        })
    };

    let on_open_api_config_window = {
        Callback::from(move |_: yew::MouseEvent| {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke_tauri_command("open_api_config_window", json!({})).await;
            });
        })
    };

    let on_open_veteran_browser = {
        Callback::from(move |_: yew::MouseEvent| {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke_tauri_command("open_veteran_browser", json!({})).await;
            });
        })
    };

    let on_open_api_veteran_browser = {
        Callback::from(move |_: yew::MouseEvent| {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke_tauri_command("open_veteran_browser", json!({"source": "uma_moe"}))
                    .await;
            });
        })
    };

    let on_open_support_card_browser = {
        Callback::from(move |_: yew::MouseEvent| {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke_tauri_command("open_support_card_browser", json!({})).await;
            });
        })
    };

    let on_open_trainee_browser = {
        Callback::from(move |_: yew::MouseEvent| {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke_tauri_command("open_trainee_browser", json!({})).await;
            });
        })
    };

    let on_open_legacy_planner = {
        Callback::from(move |_: yew::MouseEvent| {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke_tauri_command("open_legacy_planner_window", json!({})).await;
            });
        })
    };

    let on_open_race_dump = {
        Callback::from(move |_: yew::MouseEvent| {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke_tauri_command("open_race_dump_window", json!({})).await;
            });
        })
    };

    let on_import_suppl = {
        Callback::from(move |_: yew::MouseEvent| {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke_tauri_command("open_supplementary_data_window", json!({})).await;
            });
        })
    };

    let on_open_dev_tools = {
        Callback::from(move |_: yew::MouseEvent| {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke_tauri_command("open_dev_tools_window", json!({})).await;
            });
        })
    };

    let stylesheet = StyleManager::render_stylesheet();

    html! {
        <div class={AppContainerStyle::CLASS_NAME}>
            {stylesheet}
            <div class={TopRowStyle::CLASS_NAME}>
                <div class={HeaderStatusGroupStyle::CLASS_NAME}>
                <button
                    class={classes!(
                        MasterDbIndicatorStyle::CLASS_NAME,
                        if *master_db_busy || *app_db_sync_busy {
                            MasterDbBusyStyle::CLASS_NAME
                        } else if master_db_status.as_ref().map(|s| s.found).unwrap_or(false)
                            && app_db_sync_report.as_ref().map(|r| r.up_to_date).unwrap_or(false) {
                            MasterDbReadyStyle::CLASS_NAME
                        } else if master_db_status.as_ref().map(|s| s.found).unwrap_or(false) {
                            MasterDbPartialStyle::CLASS_NAME
                        } else {
                            MasterDbMissingStyle::CLASS_NAME
                        }
                    )}
                    title={
                        if *master_db_busy || *app_db_sync_busy {
                            "DB: busy"
                        } else if master_db_status.as_ref().map(|s| s.found).unwrap_or(false)
                            && app_db_sync_report.as_ref().map(|r| r.up_to_date).unwrap_or(false) {
                            "DB: ready"
                        } else if master_db_status.as_ref().map(|s| s.found).unwrap_or(false) {
                            "DB: needs sync"
                        } else {
                            "DB: missing"
                        }
                    }
                    onclick={on_open_db_status_window.clone()}
                >
                    <span class={MasterDbDotStyle::CLASS_NAME}></span>
                    {"DB"}
                </button>
                <button
                    class={classes!(
                        WorkerIndicatorStyle::CLASS_NAME,
                        if !worker_status.worker_running {
                            WorkerStoppedStyle::CLASS_NAME
                        } else if worker_status.process_found {
                            WorkerReadyStyle::CLASS_NAME
                        } else if worker_status.retry_count >= worker_status.max_retries {
                            WorkerErrorStyle::CLASS_NAME
                        } else {
                            WorkerSearchingStyle::CLASS_NAME
                        }
                    )}
                    title={
                        if !worker_status.worker_running {
                            "Worker: stopped".to_string()
                        } else if worker_status.process_found {
                            "Worker: ready".to_string()
                        } else if worker_status.retry_count >= worker_status.max_retries {
                            "Worker: error".to_string()
                        } else {
                            format!(
                                "Worker: searching ({}/{})",
                                worker_status.retry_count.saturating_add(1),
                                worker_status.max_retries
                            )
                        }
                    }
                    onclick={on_open_worker_status_window.clone()}
                >
                    <span class={WorkerDotStyle::CLASS_NAME}></span>
                    {"Worker"}
                </button>
                <button
                    class={classes!(
                        ApiIndicatorStyle::CLASS_NAME,
                        if api_config_status.clone().configured {
                            ApiConfiguredStyle::CLASS_NAME
                        } else {
                            ApiUnconfiguredStyle::CLASS_NAME
                        }
                    )}
                    title={
                        if api_config_status.clone().configured {
                            "API: ready"
                        } else {
                            "API: not set"
                        }
                    }
                    onclick={on_open_api_config_window.clone()}
                >
                    <span class={ApiDotStyle::CLASS_NAME}></span>
                    {"API"}
                </button>
                <button
                    class={classes!(
                        SupplIndicatorStyle::CLASS_NAME,
                        if suppl_check.as_ref().map(|c| c.datasets.iter().any(|d| d.needs_update)).unwrap_or(false) {
                            SupplUpdateStyle::CLASS_NAME
                        } else if suppl_status.as_ref().map(|s| s.synced).unwrap_or(false) {
                            SupplReadyStyle::CLASS_NAME
                        } else {
                            SupplMissingStyle::CLASS_NAME
                        }
                    )}
                    title={
                        if suppl_check.as_ref().map(|c| c.datasets.iter().any(|d| d.needs_update)).unwrap_or(false) {
                            "Data: update available"
                        } else if suppl_status.as_ref().map(|s| s.synced).unwrap_or(false) {
                            "Data: loaded"
                        } else {
                            "Data: not loaded"
                        }
                    }
                    onclick={on_import_suppl.clone()}
                >
                    <span class={SupplDotStyle::CLASS_NAME}></span>
                    {"Data"}
                </button>
                    <button class={VersionPillStyle::CLASS_NAME}>{ concat!("v", env!("CARGO_PKG_VERSION")) }</button>
                </div>
            </div>

                <div class={FeatureCardStyle::CLASS_NAME}>
                    <h2>{"Veteran Browser"}</h2>
                    <p>{"Browse and search veteran legacy data (local database / uma.moe API)"}</p>
                    <div style="display: flex; gap: 8px;">
                        <button onclick={on_open_veteran_browser}>{"Local"}</button>
                        <button onclick={on_open_api_veteran_browser} disabled={!api_config_status.configured}>{"API"}</button>
                    </div>
                </div>

                <div class={FeatureCardStyle::CLASS_NAME}>
                    <h2>{"Collection Browsers"}</h2>
                    <p>{"Browse and search your support card and trainee collections"}</p>
                    <div style="display: flex; gap: 8px;">
                        <button onclick={on_open_support_card_browser}>{"Support Cards"}</button>
                        <button onclick={on_open_trainee_browser}>{"Trainees"}</button>
                    </div>
                </div>

                {
                    if shared::DEV_VIEW {
                        html! {
                            <div class={FeatureCardStyle::CLASS_NAME}>
                                <h2>{"Friend Browser"}</h2>
                                <p>{"Browse and search friend data"}</p>
                                <button>{"Open"}</button>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }

                <div class={FeatureCardStyle::CLASS_NAME}>
                    <h2>{"Legacy Planner"}</h2>
                    <p>{"Plan and optimize legacy horse breeding"}</p>
                    <button onclick={on_open_legacy_planner}>{"Open"}</button>
                </div>

                <div class={FeatureCardStyle::CLASS_NAME}>
                    <h2>{"Race Dump Viewer"}</h2>
                    <p>{"View and analyze saved race dumps"}</p>
                    <button onclick={on_open_race_dump}>{"Open"}</button>
                </div>

                {
                    if shared::DEV_VIEW {
                        html! {
                            <div class={FeatureCardStyle::CLASS_NAME}>
                                <h2>{"Dev Tools"}</h2>
                                <p>{"Worker commands, data imports, and log viewer"}</p>
                                <button onclick={on_open_dev_tools}>{"Open"}</button>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }

                <div class={FooterStyle::CLASS_NAME}>
                    <ExternalLink
                        url={"https://mikethemaverick428.github.io/HonseHelper/".to_string()}
                        label={"Docs".to_string()}
                        title={"Honse Helper manual / docs".to_string()}
                    />
                    <span class="footer-sep">{"·"}</span>
                    <ExternalLink
                        url={"https://discord.gg/NMhG48AZx".to_string()}
                        label={"Discord".to_string()}
                        title={"Join the Honse Helper Discord".to_string()}
                    />
                </div>
        </div>
    }
}
