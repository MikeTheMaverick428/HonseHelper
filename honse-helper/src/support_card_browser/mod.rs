use crate::{
    components::gather_support_cards::GatherSupportCardsButton,
    components::notifications::{use_timed_notification, Notification, NotificationOverlay},
    styles::{support_card_browser::*, Style, StyleManager},
    tauri_bridge::invoke_tauri_command,
    veteran_browser::components::pagination::Pagination,
    veteran_browser::components::preset_manager::PresetManager,
};
use serde_json::json;
use shared::models::PaginationResponse;
use shared::support_card_browser::*;
use shared::veteran_browser::PresetData;
use std::rc::Rc;
use yew::prelude::*;

pub mod components;

use components::filter_panel::ScFilterPanel;
use components::sort_selector::ScSortSelector;
use components::support_card_card::SupportCardCard;
use components::support_card_detail_modal::SupportCardDetailModal;

const PAGE_SIZE: u32 = 30;

#[function_component]
pub fn SupportCardBrowser() -> Html {
    let filters = use_state(Vec::<SupportCardFilter>::new);
    let sort = use_state(|| SupportCardSortConfig {
        key: "Name".to_string(),
        direction: "Asc".to_string(),
    });
    let page = use_state(|| 1u32);
    let cards = use_state(Vec::<SupportCardPageItem>::new);
    let total = use_state(|| 0u32);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let filter_options = use_state(|| SupportCardFilterOptions {
        rarities: Vec::new(),
        card_types: Vec::new(),
        effect_types: Vec::new(),
        characters: Vec::new(),
        skills: Vec::new(),
    });
    let presets = use_state(Vec::<String>::new);

    let detail_selected = use_state(|| None::<SupportCardPageItem>);
    let detail_loading = use_state(|| false);

    let (notification_state, push, remove) = use_timed_notification(3000);

    let last_gather_time = use_state(|| None::<String>);

    // Load filter options on mount
    {
        let filter_options = filter_options.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(val) =
                    invoke_tauri_command("get_support_card_filter_options", json!({})).await
                {
                    if let Ok(opts) = serde_json::from_value::<SupportCardFilterOptions>(val) {
                        filter_options.set(opts);
                    }
                }
            });
            || {}
        });
    }

    // Load presets on mount
    {
        let presets = presets.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(val) = invoke_tauri_command("list_support_card_presets", json!({})).await
                {
                    if let Ok(list) = serde_json::from_value::<Vec<String>>(val) {
                        presets.set(list);
                    }
                }
            });
            || {}
        });
    }

    // Load last gather time on mount
    {
        let last_gather_time = last_gather_time.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(val) = invoke_tauri_command(
                    "get_last_gather_time",
                    json!({"key": "last_support_card_gathered"}),
                )
                .await
                {
                    if let Ok(time) = serde_json::from_value::<Option<String>>(val) {
                        last_gather_time.set(time);
                    }
                }
            });
            || {}
        });
    }

    // Query function
    let run_query = {
        let filters = filters.clone();
        let sort = sort.clone();
        let page = page.clone();
        let cards = cards.clone();
        let total = total.clone();
        let loading = loading.clone();
        let error = error.clone();
        Rc::new(
            move |flt: Vec<SupportCardFilter>, srt: SupportCardSortConfig, p: u32| {
                let filters = filters.clone();
                let sort = sort.clone();
                let page = page.clone();
                let cards = cards.clone();
                let total = total.clone();
                let loading = loading.clone();
                let error = error.clone();
                filters.set(flt.clone());
                sort.set(srt.clone());
                page.set(p);
                loading.set(true);
                error.set(None);
                wasm_bindgen_futures::spawn_local(async move {
                    let query = SupportCardBrowserQuery {
                        filters: flt,
                        sort: srt,
                        page: p,
                        page_size: PAGE_SIZE,
                    };
                    match invoke_tauri_command(
                        "query_support_card_store_page",
                        json!({ "query": query }),
                    )
                    .await
                    {
                        Ok(val) => {
                            if let Ok(resp) = serde_json::from_value::<
                                PaginationResponse<SupportCardPageItem>,
                            >(val)
                            {
                                cards.set(resp.results);
                                total.set(resp.total);
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

    // Initial load — load active preset first, then query
    {
        let run_query = run_query.clone();
        let filters = filters.clone();
        let sort = sort.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let mut loaded_filters = (*filters).clone();
                let mut loaded_sort = (*sort).clone();
                if let Ok(val) = invoke_tauri_command(
                    "load_support_card_preset_active",
                    json!({}),
                )
                .await
                {
                    if let Ok(Some(data)) =
                        serde_json::from_value::<Option<PresetData>>(val)
                    {
                        if let Some(ref filters_json) = data.filters {
                            if let Ok(f) =
                                serde_json::from_str::<Vec<SupportCardFilter>>(filters_json)
                            {
                                loaded_filters = f.clone();
                                filters.set(f);
                            }
                        }
                        if let Some(ref sort_json) = data.sort {
                            if let Ok(s) =
                                serde_json::from_str::<SupportCardSortConfig>(sort_json)
                            {
                                loaded_sort = s.clone();
                                sort.set(s);
                            }
                        }
                    }
                }
                run_query(loaded_filters, loaded_sort, 1);
            });
            || {}
        });
    }

    let set_filters = {
        let run_query = run_query.clone();
        let sort = sort.clone();
        Callback::from(move |flt: Vec<SupportCardFilter>| {
            run_query(flt, (*sort).clone(), 1);
        })
    };

    let set_sort = {
        let run_query = run_query.clone();
        let filters = filters.clone();
        Callback::from(move |srt: SupportCardSortConfig| {
            run_query((*filters).clone(), srt, 1);
        })
    };

    let go_to_page = {
        let run_query = run_query.clone();
        let filters = filters.clone();
        let sort = sort.clone();
        Callback::from(move |p: u32| {
            run_query((*filters).clone(), (*sort).clone(), p);
        })
    };

    let total_pages = (*total + PAGE_SIZE - 1) / PAGE_SIZE;
    let first_item = if *total == 0 { 0 } else { (*page - 1) * PAGE_SIZE + 1 };
    let last_item = ((*page) * PAGE_SIZE).min(*total);

    // Preset callbacks
    let load_preset = {
        let filters = filters.clone();
        let sort = sort.clone();
        let run_query = run_query.clone();
        let push = push.clone();
        Callback::from(move |name: String| {
            let filters = filters.clone();
            let sort = sort.clone();
            let run_query = run_query.clone();
            let push = push.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match invoke_tauri_command("load_support_card_preset", json!({ "name": name }))
                    .await
                {
                    Ok(val) => {
                        if let Some(data) = val.as_object() {
                            let new_filters: Vec<SupportCardFilter> = data
                                .get("filters")
                                .and_then(|v| v.as_str())
                                .and_then(|s| serde_json::from_str(s).ok())
                                .unwrap_or_default();
                            let new_sort: SupportCardSortConfig = data
                                .get("sort")
                                .and_then(|v| v.as_str())
                                .and_then(|s| serde_json::from_str(s).ok())
                                .unwrap_or_default();
                            run_query(new_filters, new_sort, 1);
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
        Callback::from(move |name: String| {
            let filters = filters.clone();
            let sort = sort.clone();
            let presets = presets.clone();
            let push = push.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let filters_json = serde_json::to_string(&*filters).unwrap_or_default();
                let sort_json = serde_json::to_string(&*sort).unwrap_or_default();
                match invoke_tauri_command(
                    "save_support_card_preset",
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
                            invoke_tauri_command("list_support_card_presets", json!({})).await
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
                match invoke_tauri_command("delete_support_card_preset", json!({ "name": name }))
                    .await
                {
                    Ok(_) => {
                        push(Notification::success("Preset deleted".to_string()));
                        if let Ok(val) =
                            invoke_tauri_command("list_support_card_presets", json!({})).await
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

    let open_detail = {
        let detail_selected = detail_selected.clone();
        Callback::from(move |card: SupportCardPageItem| {
            detail_selected.set(Some(card));
        })
    };

    let close_detail = {
        let detail_selected = detail_selected.clone();
        Callback::from(move |_: ()| {
            detail_selected.set(None);
        })
    };

    let stylesheet = StyleManager::render_stylesheet();

    html! {
        <div class={SupportCardBrowserRootStyle::CLASS_NAME}>
            <div class={ScBrowserHeaderStyle::CLASS_NAME}>
                <h1>{"Support Card Browser"}</h1>
                <div class={ScBrowserHeaderControlsStyle::CLASS_NAME}>
                    <PresetManager
                        presets={(*presets).clone()}
                        on_load={load_preset}
                        on_save={save_as_preset}
                        on_delete={delete_preset}
                    />
                    <ScSortSelector sort={(*sort).clone()} on_change={set_sort} />
                    <div style="display: flex; flex-direction: column; align-items: flex-end;">
                        <GatherSupportCardsButton
                        on_complete={{
                            let run_query = run_query.clone();
                            let push = push.clone();
                            let filters = filters.clone();
                            let sort = sort.clone();
                            let last_gather_time = last_gather_time.clone();
                            Callback::from(move |result: Result<(), String>| {
                                match result {
                                    Ok(()) => {
                                        push(Notification::success("Support card data gathered"));
                                        run_query((*filters).clone(), (*sort).clone(), 1);
                                        let last_gather_time = last_gather_time.clone();
                                        wasm_bindgen_futures::spawn_local(async move {
                                            if let Ok(val) = invoke_tauri_command("get_last_gather_time", json!({"key": "last_support_card_gathered"})).await {
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

            <div class={ScBrowserBodyStyle::CLASS_NAME}>
                <aside class={ScBrowserSidebarStyle::CLASS_NAME}>
                    <ScFilterPanel
                        filters={(*filters).clone()}
                        on_change={set_filters}
                        options={(*filter_options).clone()}
                    />
                </aside>

                <main class={ScBrowserMainStyle::CLASS_NAME}>
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
                        <div class={ScBrowserLoadingStyle::CLASS_NAME}>{"Loading..."}</div>
                    } else if let Some(err) = &*error {
                        <div class={ScBrowserErrorStyle::CLASS_NAME}>{ err }</div>
                    } else if cards.is_empty() {
                        <div class={ScBrowserEmptyStyle::CLASS_NAME}>{"No support cards found. Try adjusting filters or import support card data first."}</div>
                    } else {
                        <div style="flex: 1; overflow-y: auto; min-height: 0;">
                            <div class={ScCardGridStyle::CLASS_NAME}>
                                { for (*cards).iter().map(|c| {
                                    let open_detail = open_detail.clone();
                                    let card = c.clone();
                                    let on_click = Callback::from(move |()| open_detail.emit(card.clone()));
                                    html! {
                                        <SupportCardCard card={c.clone()} on_click={on_click} />
                                    }
                                }) }
                            </div>
                        </div>
                    }
                </main>
            </div>
            if let Some(card) = &*detail_selected {
                <SupportCardDetailModal
                    card={card.clone()}
                    on_close={close_detail}
                />
            }
        </div>
    }
}
