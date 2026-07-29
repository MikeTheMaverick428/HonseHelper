pub mod batch_ops;
pub mod components;

use std::rc::Rc;

use crate::components::delete_confirmation_modal::DeleteConfirmationModal;
use crate::components::notifications::{use_timed_notification, Notification, NotificationOverlay};
use crate::components::tag_modal::TagModal;
use crate::race_dump::batch_ops::BatchOperationsModal;
use crate::styles::{
    gather_veterans::GatherVeteransBtnStyle,
    race_dump::{
        RaceDumpBodyStyle, RaceDumpCardGridStyle, RaceDumpEmptyStyle, RaceDumpErrorStyle,
        RaceDumpHeaderControlsStyle, RaceDumpHeaderStyle, RaceDumpLoadingStyle, RaceDumpMainStyle,
        RaceDumpRootStyle, RaceDumpSidebarStyle,
    },
    Style, StyleManager,
};
use crate::tauri_bridge::invoke_tauri_command;
use crate::veteran_browser::components::pagination::Pagination;
use crate::veteran_browser::components::preset_manager::PresetManager;
use components::filter_panel::RaceFilterPanel;
use components::race_dump_card::RaceDumpCard;
use components::sort_selector::RaceSortSelector;
use serde_json::json;
use shared::models::PaginationResponse;
use shared::veteran_browser::{SortConfig, TagRow};
use shared::{RaceDumpBrowserQuery, RaceDumpFilter, RaceDumpFilterOptions, RaceDumpPageItem};
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
enum DeleteState {
    Idle,
    Deleting(i64),
}

#[function_component]
pub fn RaceDumpBrowser() -> Html {
    let filters = use_state(Vec::<RaceDumpFilter>::new);
    let sort = use_state(|| SortConfig {
        key: "capture_time".into(),
        direction: "desc".into(),
    });
    let page = use_state(|| 1u32);
    let page_size = use_state(|| 20u32);
    let items = use_state(Vec::<RaceDumpPageItem>::new);
    let total = use_state(|| 0u32);
    let loading = use_state(|| true);
    let error = use_state(|| None as Option<String>);
    let filter_options = use_state(|| None as Option<RaceDumpFilterOptions>);
    let presets = use_state(Vec::<String>::new);
    let (state, push, remove) = use_timed_notification(5000);
    let delete_state = use_state(|| DeleteState::Idle);
    let tag_modal_open = use_state(|| false);
    let tag_modal_dump_id = use_state(|| 0i64);
    let dump_tags = use_state(Vec::<TagRow>::new);
    let tag_search_results = use_state(Vec::<TagRow>::new);
    let saving = use_state(|| false);
    let batch_modal_open = use_state(|| false);
    let pending_delete = use_state(|| None as Option<RaceDumpPageItem>);

    let do_query = {
        let items = items.clone();
        let total = total.clone();
        let loading = loading.clone();
        let error = error.clone();
        let page_size = page_size.clone();
        Rc::new(move |f: Vec<RaceDumpFilter>, s: SortConfig, p: u32| {
            let items = items.clone();
            let total = total.clone();
            let loading = loading.clone();
            let error = error.clone();
            let ps = *page_size;
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                error.set(None);
                let query = RaceDumpBrowserQuery {
                    filters: f,
                    sort: s,
                    page: p,
                    page_size: ps,
                };
                match invoke_tauri_command("query_race_dump_page", json!({ "query": query })).await
                {
                    Ok(result) => {
                        match serde_json::from_value::<PaginationResponse<RaceDumpPageItem>>(result)
                        {
                            Ok(qr) => {
                                items.set(qr.results);
                                total.set(qr.total);
                                error.set(None);
                            }
                            Err(e) => error.set(Some(format!("parse error: {}", e))),
                        }
                    }
                    Err(e) => error.set(Some(e)),
                }
                loading.set(false);
            });
        })
    };

    {
        let do_query = do_query.clone();
        let filter_options = filter_options.clone();
        let presets = presets.clone();
        let filters = filters.clone();
        let sort = sort.clone();
        use_effect_with((), move |_| {
            let filter_options = filter_options.clone();
            let presets = presets.clone();
            let filters = filters.clone();
            let sort = sort.clone();
            let do_query = do_query.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(result) =
                    invoke_tauri_command("get_race_dump_filter_options", json!({})).await
                {
                    if let Ok(opts) = serde_json::from_value::<RaceDumpFilterOptions>(result) {
                        filter_options.set(Some(opts));
                    }
                }
                if let Ok(result) =
                    invoke_tauri_command("list_presets", json!({"browserType": "race_dump"})).await
                {
                    if let Ok(list) = serde_json::from_value::<Vec<String>>(result) {
                        presets.set(list);
                    }
                }
                let mut loaded_f: Option<Vec<RaceDumpFilter>> = None;
                let mut loaded_s: Option<SortConfig> = None;
                if let Ok(result) =
                    invoke_tauri_command("load_preset_active", json!({"browserType": "race_dump"}))
                        .await
                {
                    if let Ok(Some(data)) = serde_json::from_value::<
                        Option<shared::veteran_browser::PresetData>,
                    >(result)
                    {
                        if let Some(ref filters_json) = data.filters {
                            if let Ok(f) = serde_json::from_str::<Vec<RaceDumpFilter>>(filters_json) {
                                filters.set(f.clone());
                                loaded_f = Some(f);
                            }
                        }
                        if let Some(ref sort_json) = data.sort {
                            if let Ok(s) = serde_json::from_str::<SortConfig>(sort_json) {
                                sort.set(s.clone());
                                loaded_s = Some(s);
                            }
                        }
                    }
                }
                let f = loaded_f.unwrap_or_default();
                let s = loaded_s.unwrap_or_else(|| SortConfig {
                    key: "capture_time".into(),
                    direction: "desc".into(),
                });
                do_query(f, s, 1);
            });
            || {}
        });
    }

    {
        let do_query = do_query.clone();
        let filters = filters.clone();
        let sort = sort.clone();
        let page = page.clone();
        let filter_options = filter_options.clone();
        use_effect_with((), move |_| {
            let do_query = do_query.clone();
            let filters = filters.clone();
            let sort = sort.clone();
            let page = page.clone();
            let filter_options = filter_options.clone();
            crate::tauri_bridge::listen_to_event("race-dump-tags-changed", move |_| {
                do_query((*filters).clone(), (*sort).clone(), *page);
                let filter_options = filter_options.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(result) =
                        invoke_tauri_command("get_race_dump_filter_options", json!({})).await
                    {
                        if let Ok(opts) = serde_json::from_value::<RaceDumpFilterOptions>(result) {
                            filter_options.set(Some(opts));
                        }
                    }
                });
            });
            || {}
        });
    }

    let set_filters = {
        let filters = filters.clone();
        let sort = sort.clone();
        let page = page.clone();
        let do_query = do_query.clone();
        Callback::from(move |new_filters: Vec<RaceDumpFilter>| {
            let new_sort = (*sort).clone();
            filters.set(new_filters.clone());
            page.set(1);
            do_query(new_filters.clone(), new_sort.clone(), 1);
            let fjson = serde_json::to_string(&new_filters).unwrap_or_default();
            let sjson = serde_json::to_string(&new_sort).unwrap_or_default();
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke_tauri_command(
                    "save_preset",
                    json!({
                        "name": "__active__",
                        "filters": fjson,
                        "sort": sjson,
                        "browserType": "race_dump",
                    }),
                )
                .await;
            });
        })
    };

    let set_sort = {
        let sort = sort.clone();
        let page = page.clone();
        let do_query = do_query.clone();
        let filters = filters.clone();
        Callback::from(move |new_sort: SortConfig| {
            let new_filters = (*filters).clone();
            sort.set(new_sort.clone());
            page.set(1);
            do_query(new_filters.clone(), new_sort.clone(), 1);
            let fjson = serde_json::to_string(&new_filters).unwrap_or_default();
            let sjson = serde_json::to_string(&new_sort).unwrap_or_default();
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke_tauri_command(
                    "save_preset",
                    json!({
                        "name": "__active__",
                        "filters": fjson,
                        "sort": sjson,
                        "browserType": "race_dump",
                    }),
                )
                .await;
            });
        })
    };

    let go_to_page = {
        let page = page.clone();
        let do_query = do_query.clone();
        let filters = filters.clone();
        let sort = sort.clone();
        Callback::from(move |p: u32| {
            page.set(p);
            do_query((*filters).clone(), (*sort).clone(), p);
        })
    };

    let on_save_race_dump = {
        let saving = saving.clone();
        let push = push.clone();
        let do_query = do_query.clone();
        let filters = filters.clone();
        let sort = sort.clone();
        let page = page.clone();
        Callback::from(move |_: MouseEvent| {
            let saving = saving.clone();
            let push = push.clone();
            let do_query = do_query.clone();
            let filters = filters.clone();
            let sort = sort.clone();
            let current_page = *page;
            wasm_bindgen_futures::spawn_local(async move {
                saving.set(true);
                match invoke_tauri_command(
                    "save_race_dump",
                    json!({
                        "request": { "command": "get_race_team_data" },
                        "timeoutMs": 15000
                    }),
                )
                .await
                {
                    Ok(result) => {
                        let id = result
                            .get("race_dump_id")
                            .and_then(|v| v.as_i64())
                            .unwrap_or(0);
                        let participants = result
                            .get("participants")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(0);
                        push(Notification::success(format!(
                            "Race dump saved: id={}, participants={}",
                            id, participants
                        )));
                        do_query((*filters).clone(), (*sort).clone(), current_page);
                    }
                    Err(e) => {
                        push(Notification::error(format!("Save failed: {}", e)));
                    }
                }
                saving.set(false);
            });
        })
    };

    let on_open_detail = Callback::from(move |id: i64| {
        wasm_bindgen_futures::spawn_local(async move {
            let _ = invoke_tauri_command("open_race_dump_detail_window", json!({"id": id})).await;
        });
    });

    let on_delete = {
        let items = items.clone();
        let pending_delete = pending_delete.clone();
        Callback::from(move |id: i64| {
            if let Some(item) = items.iter().find(|i| i.summary.id == id) {
                pending_delete.set(Some(item.clone()));
            }
        })
    };

    let on_confirm_delete = {
        let pending_delete = pending_delete.clone();
        let delete_state = delete_state.clone();
        let do_query = do_query.clone();
        let filters = filters.clone();
        let sort = sort.clone();
        let page = page.clone();
        Callback::from(move |_: ()| {
            let id = (*pending_delete).as_ref().map(|item| item.summary.id);
            if let Some(id) = id {
                let delete_state = delete_state.clone();
                let do_query = do_query.clone();
                let filters = filters.clone();
                let sort = sort.clone();
                let current_page = *page;
                wasm_bindgen_futures::spawn_local(async move {
                    delete_state.set(DeleteState::Deleting(id));
                    let _ = invoke_tauri_command("delete_race_dump", json!({"id": id})).await;
                    delete_state.set(DeleteState::Idle);
                    do_query((*filters).clone(), (*sort).clone(), current_page);
                });
            }
        })
    };

    let on_close_delete_modal = {
        let pending_delete = pending_delete.clone();
        Callback::from(move |_: ()| pending_delete.set(None))
    };

    let on_close_tag_modal = {
        let tag_modal_open = tag_modal_open.clone();
        Callback::from(move |_: ()| tag_modal_open.set(false))
    };

    let on_tag_search = {
        let tag_search_results = tag_search_results.clone();
        Callback::from(move |query: String| {
            if query.trim().is_empty() {
                tag_search_results.set(Vec::new());
                return;
            }
            let tag_search_results = tag_search_results.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(result) =
                    invoke_tauri_command("search_tags", json!({"query": query})).await
                {
                    if let Ok(tags) = serde_json::from_value::<Vec<TagRow>>(result) {
                        tag_search_results.set(tags);
                    }
                }
            });
        })
    };

    let on_tag_save = {
        let dump_tags = dump_tags.clone();
        let tag_modal_dump_id = tag_modal_dump_id.clone();
        let do_query = do_query.clone();
        let filters = filters.clone();
        let sort = sort.clone();
        let page = page.clone();
        Callback::from(move |saved_tags: Vec<TagRow>| {
            let dump_id = *tag_modal_dump_id;
            let current = (*dump_tags).clone();
            let dump_tags = dump_tags.clone();
            let do_query = do_query.clone();
            let filters = filters.clone();
            let sort = sort.clone();
            let current_page = *page;
            wasm_bindgen_futures::spawn_local(async move {
                for tag in &current {
                    if !saved_tags.iter().any(|t| t.id == tag.id) {
                        let _ = invoke_tauri_command(
                            "untag_race_dump",
                            json!({"tagId": tag.id, "raceDumpId": dump_id}),
                        )
                        .await;
                    }
                }
                let mut final_tags = current.clone();
                for tag in &saved_tags {
                    if current.iter().any(|t| t.id == tag.id && t.id != 0) {
                        continue;
                    }
                    if tag.id == 0 {
                        if let Ok(result) =
                            invoke_tauri_command("add_tag", json!({"tagValue": tag.tag_value}))
                                .await
                        {
                            if let Ok(new_tag) = serde_json::from_value::<TagRow>(result) {
                                let _ = invoke_tauri_command(
                                    "tag_race_dump",
                                    json!({"tagId": new_tag.id, "raceDumpId": dump_id}),
                                )
                                .await;
                                if !final_tags.iter().any(|t| t.id == new_tag.id) {
                                    final_tags.push(new_tag);
                                }
                            }
                        }
                    } else {
                        let _ = invoke_tauri_command(
                            "tag_race_dump",
                            json!({"tagId": tag.id, "raceDumpId": dump_id}),
                        )
                        .await;
                        if !final_tags.iter().any(|t| t.id == tag.id) {
                            final_tags.push(tag.clone());
                        }
                    }
                }
                dump_tags.set(final_tags);
                do_query((*filters).clone(), (*sort).clone(), current_page);
            });
        })
    };

    let load_preset = {
        let filters = filters.clone();
        let sort = sort.clone();
        let page = page.clone();
        let do_query = do_query.clone();
        let push = push.clone();
        Callback::from(move |name: String| {
            let filters = filters.clone();
            let sort = sort.clone();
            let page = page.clone();
            let do_query = do_query.clone();
            let push = push.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match invoke_tauri_command(
                    "load_preset",
                    json!({"name": name, "browserType": "race_dump"}),
                )
                .await
                {
                    Ok(result) => {
                        if let Ok(Some(data)) = serde_json::from_value::<
                            Option<shared::veteran_browser::PresetData>,
                        >(result)
                        {
                            let new_f = data.filters.as_ref()
                                .and_then(|s| serde_json::from_str::<Vec<RaceDumpFilter>>(s).ok())
                                .unwrap_or_default();
                            let new_s = data.sort.as_ref()
                                .and_then(|s| serde_json::from_str::<SortConfig>(s).ok())
                                .unwrap_or_else(|| SortConfig {
                                    key: "capture_time".into(),
                                    direction: "desc".into(),
                                });
                            filters.set(new_f.clone());
                            sort.set(new_s.clone());
                            page.set(1);
                            do_query(new_f, new_s, 1);
                        }
                    }
                    Err(e) => {
                        push(Notification::error(format!("Error loading preset: {}", e)));
                    }
                }
            });
        })
    };

    let save_as_preset = {
        let presets = presets.clone();
        let push = push.clone();
        let filters = filters.clone();
        let sort = sort.clone();
        Callback::from(move |name: String| {
            let filters_json = serde_json::to_string(&(*filters)).unwrap_or_default();
            let sort_json = serde_json::to_string(&(*sort)).unwrap_or_default();
            let presets = presets.clone();
            let push = push.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match invoke_tauri_command(
                    "save_preset",
                    json!({
                        "name": name,
                        "filters": filters_json,
                        "sort": sort_json,
                        "browserType": "race_dump",
                    }),
                )
                .await
                {
                    Ok(_) => {
                        if let Ok(result) = invoke_tauri_command(
                            "list_presets",
                            json!({"browserType": "race_dump"}),
                        )
                        .await
                        {
                            if let Ok(list) = serde_json::from_value::<Vec<String>>(result) {
                                presets.set(list);
                            }
                        }
                        push(Notification::success(format!("Saved preset: {}", name)));
                    }
                    Err(e) => {
                        push(Notification::error(format!("Error: {}", e)));
                    }
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
                match invoke_tauri_command(
                    "delete_preset",
                    json!({"name": name, "browserType": "race_dump"}),
                )
                .await
                {
                    Ok(_) => {
                        if let Ok(result) = invoke_tauri_command(
                            "list_presets",
                            json!({"browserType": "race_dump"}),
                        )
                        .await
                        {
                            if let Ok(list) = serde_json::from_value::<Vec<String>>(result) {
                                presets.set(list);
                            }
                        }
                        push(Notification::success(format!("Deleted preset: {}", name)));
                    }
                    Err(e) => {
                        push(Notification::error(format!("Error: {}", e)));
                    }
                }
            });
        })
    };

    let delete_modal_show = (*pending_delete).is_some();
    let delete_item_name = (*pending_delete)
        .as_ref()
        .map(|item| {
            item.race_name
                .clone()
                .unwrap_or_else(|| format!("Race dump #{}", item.summary.id))
        })
        .unwrap_or_default();

    let total_pages = if *page_size == 0 {
        1
    } else {
        (*total).div_ceil(*page_size)
    };
    let first_item = if *total == 0 { 0 } else { (*page - 1) * *page_size + 1 };
    let last_item = ((*page) * (*page_size)).min(*total);

    let stylesheet = StyleManager::render_stylesheet();

    html! {
        <div class={RaceDumpRootStyle::CLASS_NAME}>
            <div class={RaceDumpHeaderStyle::CLASS_NAME}>
                <h1>{"Race Dumps"}</h1>
                <div class={RaceDumpHeaderControlsStyle::CLASS_NAME}>
                    <PresetManager
                        presets={(*presets).clone()}
                        on_load={load_preset}
                        on_save={save_as_preset}
                        on_delete={delete_preset}
                    />
                    <RaceSortSelector
                        sort={(*sort).clone()}
                        on_change={set_sort.clone()}
                    />
                    <button
                        class={GatherVeteransBtnStyle::CLASS_NAME}
                        onclick={on_save_race_dump}
                        disabled={*saving}
                    >
                        if *saving { { "Saving Race Dump..." } } else { { "Save Race Dump" } }
                    </button>
                    <button
                        class={GatherVeteransBtnStyle::CLASS_NAME}
                        onclick={{
                            let bmo = batch_modal_open.clone();
                            Callback::from(move |_: MouseEvent| bmo.set(true))
                        }}
                    >
                        {"Batch Operations"}
                    </button>
                </div>
            </div>

            {stylesheet}

            <NotificationOverlay notifications={state.0.clone()} on_close={{
                let remove = remove.clone();
                Callback::from(move |id: u32| remove(id))
            }} />

            <div class={RaceDumpBodyStyle::CLASS_NAME}>
                <aside class={RaceDumpSidebarStyle::CLASS_NAME}>
                    <RaceFilterPanel
                        filters={(*filters).clone()}
                        on_change={set_filters}
                        options={(*filter_options).clone()}
                    />
                </aside>

                <main class={RaceDumpMainStyle::CLASS_NAME}>
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
                    if *loading && items.is_empty() {
                        <div class={RaceDumpLoadingStyle::CLASS_NAME}>{"Loading..."}</div>
                    } else if let Some(err) = &*error {
                        <div class={RaceDumpErrorStyle::CLASS_NAME}>{err}</div>
                    } else if items.is_empty() {
                        <div class={RaceDumpEmptyStyle::CLASS_NAME}>{"No race dumps found."}</div>
                    } else {
                        <div style="flex: 1; overflow-y: auto; min-height: 0;">
                            <div class={RaceDumpCardGridStyle::CLASS_NAME}>
                                {for items.iter().map(|item| {
                                    let id = item.summary.id;
                                    let deleting = matches!(*delete_state, DeleteState::Deleting(x) if x == id);
                                    html! {
                                        <RaceDumpCard
                                            key={id}
                                            item={item.clone()}
                                            on_click={on_open_detail.clone()}
                                            on_delete={on_delete.clone()}
                                            {deleting}
                                        />
                                    }
                                })}
                            </div>
                        </div>
                    }
                </main>
            </div>

            <TagModal
                show={*tag_modal_open}
                title={"Manage Tags".to_string()}
                current_tags={(*dump_tags).clone()}
                search_results={(*tag_search_results).clone()}
                on_search={on_tag_search}
                on_save={on_tag_save}
                on_close={on_close_tag_modal}
            />

            <DeleteConfirmationModal
                show={delete_modal_show}
                title={"Delete Race Dump".to_string()}
                item_name={delete_item_name}
                on_confirm={on_confirm_delete}
                on_close={on_close_delete_modal}
            />
            <BatchOperationsModal
                show={*batch_modal_open}
                filters={(*filters).clone()}
                total={*total}
                on_close={
                    let bmo = batch_modal_open.clone();
                    Callback::from(move |()| bmo.set(false))
                }
                on_export_done={{
                    let push = push.clone();
                    let bmo = batch_modal_open.clone();
                    Callback::from(move |result: Result<String, String>| {
                        match result {
                            Ok(msg) if msg == "canceled" => {}
                            Ok(msg) => { push(Notification::success(msg)); }
                            Err(e) => { push(Notification::error(format!("Export failed: {}", e))); }
                        }
                    })
                }}
            />
        </div>
    }
}
