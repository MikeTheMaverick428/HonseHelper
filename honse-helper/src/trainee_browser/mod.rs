use crate::{
    components::gather_trainees::GatherTraineesButton,
    components::notifications::{use_timed_notification, Notification, NotificationOverlay},
    styles::{trainee_browser::*, Style, StyleManager},
    tauri_bridge::{get_window_label, invoke_tauri_command},
    veteran_browser::components::pagination::Pagination,
    veteran_browser::components::preset_manager::PresetManager,
};
use serde_json::json;
use shared::models::PaginationResponse;
use shared::trainee_browser::*;
use shared::legacy_planner::LegacyPlannerSlot;
use std::rc::Rc;
use yew::prelude::*;

pub mod components;

use components::filter_panel::TrFilterPanel;
use components::sort_selector::TrSortSelector;
use components::trainee_card::TraineeCard;
use components::trainee_detail_modal::TraineeDetailModal;

const PAGE_SIZE: u32 = 30;

#[derive(Clone, PartialEq)]
pub enum BrowserMode {
    Browse,
    SelectTrainee,
    SelectTraineeChar { slot_label: String },
}

#[function_component]
pub fn TraineeBrowser() -> Html {
    let filters = use_state(Vec::<TraineeFilter>::new);
    let sort = use_state(|| TraineeSortConfig {
        key: "name".to_string(),
        direction: "Asc".to_string(),
    });
    let page = use_state(|| 1u32);
    let cards = use_state(Vec::<TraineePageItem>::new);
    let total = use_state(|| 0u32);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let presets = use_state(Vec::<String>::new);

    let detail_trainee = use_state(|| None::<TraineePageItem>);
    let detail_data = use_state(|| None::<TraineeDetail>);
    let detail_loading = use_state(|| false);

    let filter_options = use_state(|| None::<TraineeFilterOptions>);

    let last_gather_time = use_state(|| None::<String>);

    let mode = use_state(|| BrowserMode::Browse);
    let planner_slot = use_state(|| None::<LegacyPlannerSlot>);

    let (notification_state, push, remove) = use_timed_notification(3000);

    let run_query = {
        let filters = filters.clone();
        let sort = sort.clone();
        let page = page.clone();
        let cards = cards.clone();
        let total = total.clone();
        let loading = loading.clone();
        let error = error.clone();
        let planner_slot = planner_slot.clone();
        let mode = mode.clone();
        Rc::new(
            move |flt: Vec<TraineeFilter>, srt: TraineeSortConfig, p: u32, name: Option<String>, slot_override: Option<Option<LegacyPlannerSlot>>, planner_ctx: Option<bool>| {
                let filters = filters.clone();
                let sort = sort.clone();
                let page = page.clone();
                let cards = cards.clone();
                let total = total.clone();
                let loading = loading.clone();
                let error = error.clone();
                let planner_slot = planner_slot.clone();
                let mode = mode.clone();
                filters.set(flt.clone());
                sort.set(srt.clone());
                page.set(p);
                loading.set(true);
                error.set(None);
                let slot_val = slot_override.unwrap_or_else(|| (*planner_slot).clone());
                let planner_context = planner_ctx.unwrap_or_else(|| !matches!(*mode, BrowserMode::Browse));
                wasm_bindgen_futures::spawn_local(async move {
                    let query = TraineeBrowserQuery {
                        filters: flt.clone(),
                        sort: srt.clone(),
                        page: p,
                        page_size: PAGE_SIZE,
                        legacy_planner_slot: slot_val,
                        planner_context,
                    };
                    match invoke_tauri_command("query_trainee_cards", json!({ "query": query }))
                        .await
                    {
                        Ok(val) => {
                            if let Ok(resp) =
                                serde_json::from_value::<PaginationResponse<TraineePageItem>>(val)
                            {
                                cards.set(resp.results);
                                total.set(resp.total);
                                if let Some(name) = name {
                                    let filters_json =
                                        serde_json::to_string(&flt).unwrap_or_default();
                                    let is_planner = !matches!(*mode, BrowserMode::Browse);
                                    let sort_json = if is_planner {
                                        None
                                    } else {
                                        Some(serde_json::to_string(&srt).unwrap_or_default())
                                    };
                                    let _ = invoke_tauri_command(
                                    "save_trainee_preset",
                                    json!({ "name": name, "filters": filters_json, "sort": sort_json }),
                                ).await;
                                }
                            }
                            loading.set(false);
                        }
                        Err(e) => {
                            error.set(Some(e));
                            loading.set(false);
                        }
                    }
                });
            },
        )
    };

    // Initial load: restore active preset, then query
    {
        let run_query = run_query.clone();
        let filters = filters.clone();
        let sort = sort.clone();
        let presets = presets.clone();
        let filter_options = filter_options.clone();
        let last_gather_time = last_gather_time.clone();
        let mode = mode.clone();
        let planner_slot = planner_slot.clone();
        use_effect_with((), move |_| {
            let run_query = run_query.clone();
            let filters = filters.clone();
            let sort = sort.clone();
            let presets = presets.clone();
            let filter_options = filter_options.clone();
            let last_gather_time = last_gather_time.clone();
            let mode = mode.clone();
            let planner_slot = planner_slot.clone();
            wasm_bindgen_futures::spawn_local(async move {
                // Fetch browser mode from backend
                let label = get_window_label().unwrap_or_default();
                let mut is_select_mode = false;
                let mut initial_planner_slot: Option<LegacyPlannerSlot> = None;
                let mut initial_planner_context = false;
                if let Ok(result) =
                    invoke_tauri_command("get_trainee_browser_mode", json!({"windowLabel": label})).await
                {
                    if let Ok(Some(mode_str)) = serde_json::from_value::<Option<String>>(result) {
                        if mode_str == "select_trainee" {
                            mode.set(BrowserMode::SelectTrainee);
                            is_select_mode = true;
                            initial_planner_context = true;
                        } else if let Some(slot_label) = mode_str.strip_prefix("select_trainee_char:") {
                            if let Some(slot) = LegacyPlannerSlot::from_label(slot_label) {
                                mode.set(BrowserMode::SelectTraineeChar { slot_label: slot_label.to_string() });
                                initial_planner_slot = Some(slot);
                                planner_slot.set(Some(slot));
                                is_select_mode = true;
                                initial_planner_context = true;
                            }
                        }
                    }
                }

                if let Ok(val) = invoke_tauri_command(
                    "get_last_gather_time",
                    json!({"key": "last_trainee_gathered"}),
                )
                .await
                {
                    if let Ok(time) = serde_json::from_value::<Option<String>>(val) {
                        last_gather_time.set(time);
                    }
                }
                if let Ok(val) = invoke_tauri_command("list_trainee_presets", json!({})).await {
                    if let Ok(list) = serde_json::from_value::<Vec<String>>(val) {
                        presets.set(list);
                    }
                }

                if let Ok(val) = invoke_tauri_command("get_trainee_filter_options", json!({})).await
                {
                    if let Ok(opts) = serde_json::from_value::<TraineeFilterOptions>(val) {
                        filter_options.set(Some(opts));
                    }
                }

                let mut loaded_filters: Option<Vec<TraineeFilter>> = None;
                let mut loaded_sort: Option<TraineeSortConfig> = None;
                if let Ok(val) = invoke_tauri_command("load_trainee_preset_active", json!({})).await
                {
                    if let Ok(Some(data)) =
                        serde_json::from_value::<Option<shared::veteran_browser::PresetData>>(val)
                    {
                        if let Some(ref filters_json) = data.filters {
                            if let Ok(f) = serde_json::from_str::<Vec<TraineeFilter>>(filters_json) {
                                filters.set(f.clone());
                                loaded_filters = Some(f);
                            }
                        }
                        if !is_select_mode {
                            if let Some(ref sort_json) = data.sort {
                                if let Ok(s) = serde_json::from_str::<TraineeSortConfig>(sort_json) {
                                    sort.set(s.clone());
                                    loaded_sort = Some(s);
                                }
                            }
                        }
                    }
                }

                run_query(
                    loaded_filters.unwrap_or_default(),
                    loaded_sort.unwrap_or_else(|| {
                        if is_select_mode {
                            TraineeSortConfig {
                                key: "Affinity".to_string(),
                                direction: "Desc".to_string(),
                            }
                        } else {
                            TraineeSortConfig {
                                key: "name".to_string(),
                                direction: "Asc".to_string(),
                            }
                        }
                    }),
                    1,
                    None,
                    Some(initial_planner_slot),
                    Some(initial_planner_context),
                );
            });
            || {}
        });
    }

    let set_filters = {
        let run_query = run_query.clone();
        let sort = sort.clone();
        Callback::from(move |flt: Vec<TraineeFilter>| {
            run_query(flt, (*sort).clone(), 1, Some("__active__".to_string()), None, None);
        })
    };

    let set_sort = {
        let run_query = run_query.clone();
        let filters = filters.clone();
        Callback::from(move |srt: TraineeSortConfig| {
            run_query((*filters).clone(), srt, 1, Some("__active__".to_string()), None, None);
        })
    };

    let go_to_page = {
        let run_query = run_query.clone();
        let filters = filters.clone();
        let sort = sort.clone();
        Callback::from(move |p: u32| {
            run_query((*filters).clone(), (*sort).clone(), p, None, None, None);
        })
    };

    let open_detail = {
        let detail_trainee = detail_trainee.clone();
        let detail_data = detail_data.clone();
        let detail_loading = detail_loading.clone();
        Callback::from(move |card: TraineePageItem| {
            let detail_trainee = detail_trainee.clone();
            let detail_data = detail_data.clone();
            let detail_loading = detail_loading.clone();
            let rarity = if card.owned_rarity > 0 {
                card.owned_rarity
            } else {
                card.base_rarity
            };
            detail_trainee.set(Some(card.clone()));
            detail_data.set(None);
            detail_loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                match invoke_tauri_command(
                    "get_trainee_detail",
                    json!({ "traineeId": card.id, "rarity": rarity }),
                )
                .await
                {
                    Ok(val) => {
                        if let Ok(d) = serde_json::from_value::<TraineeDetail>(val) {
                            detail_data.set(Some(d));
                        }
                        detail_loading.set(false);
                    }
                    Err(_) => {
                        detail_loading.set(false);
                    }
                }
            });
        })
    };

    let close_detail = {
        let detail_trainee = detail_trainee.clone();
        Callback::from(move |_: ()| {
            detail_trainee.set(None);
        })
    };

    let on_select_trainee = {
        let push = push.clone();
        let mode = mode.clone();
        Callback::from(move |trainee_id: i64| {
            let push = push.clone();
            let slot_label = match &*mode {
                BrowserMode::SelectTraineeChar { slot_label } => Some(slot_label.clone()),
                _ => None,
            };
            wasm_bindgen_futures::spawn_local(async move {
                let mut args = json!({"traineeId": trainee_id});
                if let Some(label) = slot_label {
                    args["slotLabel"] = json!(label);
                }
                match invoke_tauri_command("return_trainee_selection", args).await {
                    Ok(_) => {}
                    Err(e) => {
                        push(Notification::error(format!("Error: {}", e)));
                    }
                }
            });
        })
    };

    let total_pages = (*total + PAGE_SIZE - 1) / PAGE_SIZE;
    let first_item = if *total == 0 { 0 } else { (*page - 1) * PAGE_SIZE + 1 };
    let last_item = ((*page) * PAGE_SIZE).min(*total);

    let load_preset = {
        let run_query = run_query.clone();
        let push = push.clone();
        let mode = mode.clone();
        Callback::from(move |name: String| {
            let run_query = run_query.clone();
            let push = push.clone();
            let mode = mode.clone();
            let name_clone = name.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match invoke_tauri_command("load_trainee_preset", json!({ "name": name })).await {
                    Ok(val) => {
                        if let Some(data) = val.as_object() {
                            let new_filters: Vec<TraineeFilter> = data
                                .get("filters")
                                .and_then(|v| v.as_str())
                                .and_then(|s| serde_json::from_str(s).ok())
                                .unwrap_or_default();
                            let is_planner = !matches!(*mode, BrowserMode::Browse);
                            let new_sort = if is_planner {
                                TraineeSortConfig {
                                    key: "Affinity".to_string(),
                                    direction: "Desc".to_string(),
                                }
                            } else {
                                data.get("sort")
                                    .and_then(|v| v.as_str())
                                    .and_then(|s| serde_json::from_str(s).ok())
                                    .unwrap_or_default()
                            };
                            run_query(new_filters, new_sort, 1, Some(name_clone), None, None);
                        }
                    }
                    Err(e) => push(Notification::error(format!("Load failed: {}", e))),
                }
            });
        })
    };

    let save_as_preset = {
        let filters = filters.clone();
        let sort = sort.clone();
        let presets = presets.clone();
        let push = push.clone();
        let mode = mode.clone();
        Callback::from(move |name: String| {
            let filters = filters.clone();
            let sort = sort.clone();
            let presets = presets.clone();
            let push = push.clone();
            let mode = mode.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let filters_json = serde_json::to_string(&*filters).unwrap_or_default();
                let is_planner = !matches!(*mode, BrowserMode::Browse);
                let sort_json = if is_planner {
                    None
                } else {
                    Some(serde_json::to_string(&*sort).unwrap_or_default())
                };
                match invoke_tauri_command(
                    "save_trainee_preset",
                    json!({
                        "name": name,
                        "filters": filters_json,
                        "sort": sort_json,
                    }),
                )
                .await
                {
                    Ok(_) => {
                        push(Notification::success("Preset saved".to_string()));
                        if let Ok(val) =
                            invoke_tauri_command("list_trainee_presets", json!({})).await
                        {
                            if let Ok(list) = serde_json::from_value::<Vec<String>>(val) {
                                presets.set(list);
                            }
                        }
                    }
                    Err(e) => push(Notification::error(format!("Save failed: {}", e))),
                }
            });
        })
    };

    let delete_preset = {
        let presets = presets.clone();
        let push = push.clone();
        Callback::from(move |name: String| {
            let presets = presets.clone();
            let push = push.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match invoke_tauri_command("delete_trainee_preset", json!({ "name": name })).await {
                    Ok(_) => {
                        push(Notification::success("Preset deleted".to_string()));
                        if let Ok(val) =
                            invoke_tauri_command("list_trainee_presets", json!({})).await
                        {
                            if let Ok(list) = serde_json::from_value::<Vec<String>>(val) {
                                presets.set(list);
                            }
                        }
                    }
                    Err(e) => push(Notification::error(format!("Delete failed: {}", e))),
                }
            });
        })
    };

    let stylesheet = StyleManager::render_stylesheet();

    html! {
        <div class={TraineeBrowserRootStyle::CLASS_NAME}>
            <div class={TrBrowserHeaderStyle::CLASS_NAME}>
                <h1>{
                    match &*mode {
                        BrowserMode::Browse => "Trainee Browser".to_string(),
                        BrowserMode::SelectTrainee => "Select a trainee for Legacy Planner".to_string(),
                        BrowserMode::SelectTraineeChar { slot_label } => format!("Select character for {slot_label}"),
                    }
                }</h1>
                <div class={TrBrowserHeaderControlsStyle::CLASS_NAME}>
                    <PresetManager
                        presets={(*presets).clone()}
                        on_load={load_preset}
                        on_save={save_as_preset}
                        on_delete={delete_preset}
                    />
                    <TrSortSelector sort={(*sort).clone()} on_change={set_sort} show_affinity={matches!(&*mode, BrowserMode::SelectTrainee | BrowserMode::SelectTraineeChar { .. })} />
                    <div style="display: flex; flex-direction: column; align-items: flex-end;">
                        <GatherTraineesButton
                        on_complete={{
                            let run_query = run_query.clone();
                            let push = push.clone();
                            let filters = filters.clone();
                            let sort = sort.clone();
                            let last_gather_time = last_gather_time.clone();
                            Callback::from(move |result: Result<(), String>| {
                                match result {
                                    Ok(()) => {
                                        push(Notification::success("Trainee data gathered"));
                                        run_query((*filters).clone(), (*sort).clone(), 1, Some("__active__".to_string()), None, None);
                                        let last_gather_time = last_gather_time.clone();
                                        wasm_bindgen_futures::spawn_local(async move {
                                            if let Ok(val) = invoke_tauri_command("get_last_gather_time", json!({"key": "last_trainee_gathered"})).await {
                                                if let Ok(time) = serde_json::from_value::<Option<String>>(val) {
                                                    last_gather_time.set(time);
                                                }
                                            }
                                        });
                                    }
                                    Err(e) => {
                                        push(Notification::error(format!("Gather failed: {}", e)));
                                    }
                                }
                            })
                        }}
                    />
                    {crate::components::render_gather_time(&last_gather_time)}
                </div>
            </div>
        </div>

            {stylesheet}

            <NotificationOverlay notifications={notification_state.0.clone()} on_close={{
                let remove = remove.clone();
                Callback::from(move |id: u32| remove(id))
            }} />

            <div class={TrBrowserBodyStyle::CLASS_NAME}>
                <aside class={TrBrowserSidebarStyle::CLASS_NAME}>
                    <TrFilterPanel
                        filters={(*filters).clone()}
                        on_change={set_filters}
                        options={(*filter_options).clone()}
                    />
                </aside>

                <main class={TrBrowserMainStyle::CLASS_NAME}>
                    if *total > 0 {
                        <div style="display: flex; align-items: center; justify-content: space-between; margin-top: 2px; margin-bottom: 8px; flex-shrink: 0;">
                            <span style="font-size: 13px; color: #9ca3af;">{"Showing items "}{first_item}{" - "}{last_item}{" out of "}{*total}</span>
                            <Pagination
                                page={*page}
                                total_pages={total_pages}
                                on_page_change={go_to_page}
                            />
                        </div>
                    }
                    if *loading {
                        <div class={TrBrowserLoadingStyle::CLASS_NAME}>{"Loading..."}</div>
                    } else if let Some(err) = &*error {
                        <div class={TrBrowserErrorStyle::CLASS_NAME}>{ err }</div>
                    } else if cards.is_empty() {
                        <div class={TrBrowserEmptyStyle::CLASS_NAME}>{"No trainee cards found. Try adjusting filters or import card data first."}</div>
                    } else {
                        <div style="flex: 1; overflow-y: auto; min-height: 0;">
                            <div class={TrCardGridStyle::CLASS_NAME}>
                                { for (*cards).iter().map(|c| {
                                    let card = c.clone();
                                    let on_click = {
                                        let open_detail = open_detail.clone();
                                        let card = card.clone();
                                        Callback::from(move |()| open_detail.emit(card.clone()))
                                    };
                                    let on_select = match &*mode {
                                        BrowserMode::SelectTrainee | BrowserMode::SelectTraineeChar { .. } => Some(on_select_trainee.clone()),
                                        BrowserMode::Browse => None,
                                    };
                                    html! { <TraineeCard card={card} on_click={on_click} on_select={on_select} /> }
                                }) }
                            </div>
                        </div>
                    }
                </main>
            </div>

            if let Some(_) = &*detail_trainee {
                <TraineeDetailModal
                    trainee={(*detail_data).clone()}
                    loading={*detail_loading}
                    on_close={close_detail}
                />
            }
        </div>
    }
}
