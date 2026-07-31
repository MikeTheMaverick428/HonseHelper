use crate::{
    components::delete_confirmation_modal::DeleteConfirmationModal,
    components::gather_veterans::GatherVeteransButton,
    components::notifications::{use_timed_notification, Notification, NotificationOverlay},
    styles::{
        gather_veterans::GatherVeteransBtnStyle,
        veteran_browser::{
            BrowserBodyStyle, BrowserEmptyStyle, BrowserErrorStyle, BrowserHeaderControlsStyle,
            BrowserHeaderStyle, BrowserLoadingStyle, BrowserMainStyle, BrowserSidebarStyle, CardGridStyle, VeteranBrowserRootStyle,
        },
        Style, StyleManager,
    },
    tauri_bridge::{get_window_label, invoke_tauri_command},
};
use serde_json::json;
use shared::filters::Filter;
use shared::legacy_planner::LegacyPlannerSlot;
use shared::models::PaginationResponse;
use shared::veteran_browser::*;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct ExportVeteransButtonProps {
    #[prop_or(Callback::noop())]
    on_complete: Callback<Result<String, String>>,
}

#[function_component(ExportVeteransButton)]
fn export_veterans_button(props: &ExportVeteransButtonProps) -> Html {
    let busy = use_state(|| false);

    let onclick = {
        let busy = busy.clone();
        let on_complete = props.on_complete.clone();
        Callback::from(move |_: MouseEvent| {
            let busy = busy.clone();
            let on_complete = on_complete.clone();
            wasm_bindgen_futures::spawn_local(async move {
                busy.set(true);
                let result = invoke_tauri_command("export_veterans_to_json", json!({})).await;
                match result {
                    Ok(val) => {
                        if let Some(s) = val.as_str() {
                            on_complete.emit(Ok(s.to_string()));
                        } else {
                            on_complete.emit(Ok(val.to_string()));
                        }
                    }
                    Err(e) => on_complete.emit(Err(e)),
                }
                busy.set(false);
            });
        })
    };

    html! {
        <button
            class={GatherVeteransBtnStyle::CLASS_NAME}
            onclick={onclick}
            disabled={*busy}
        >
            if *busy {
                {"Exporting..."}
            } else {
                {"Export data.json"}
            }
        </button>
    }
}

pub mod components;

use components::detail_modal::DetailModal;
use components::filter_panel::FilterPanel;
use components::pagination::Pagination;
use components::preset_manager::PresetManager;
use components::sort_selector::SortSelector;
use components::veteran_card::VeteranCard;

#[derive(Clone, PartialEq)]
pub enum BrowserMode {
    Browse,
    SelectVeteran { slot: LegacyPlannerSlot },
}

#[derive(serde::Deserialize)]
struct UmaMoeDetailPayload {
    sparks: Vec<SparkGroupRow>,
    wins: Vec<MajorWinRow>,
    parents: Vec<ParentRow>,
}

fn browser_type(source: &str) -> &str {
    if source == "uma_moe" {
        "uma_moe"
    } else {
        "veteran"
    }
}

impl BrowserMode {
    fn slot(&self) -> Option<LegacyPlannerSlot> {
        match self {
            BrowserMode::Browse => None,
            BrowserMode::SelectVeteran { slot } => Some(*slot),
        }
    }
}

#[function_component]
pub fn VeteranBrowser() -> Html {
    let filters = use_state(Vec::<Filter>::new);
    let sort = use_state(|| SortConfig {
        key: "CreatedAt".to_string(),
        direction: "Desc".to_string(),
    });
    let page = use_state(|| 1u32);
    let page_size = use_state(|| 20u32);
    let veterans = use_state(Vec::<VeteranPageItem>::new);
    let total = use_state(|| 0u32);
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);

    let detail_veteran = use_state(|| None::<i64>);
    let detail_sparks = use_state(Vec::<SparkGroupRow>::new);
    let detail_wins = use_state(Vec::<MajorWinRow>::new);
    let detail_parents = use_state(Vec::<ParentRow>::new);
    let detail_skills = use_state(Vec::<VeteranSkillRow>::new);
    let detail_support_cards = use_state(Vec::<VeteranSupportCardRow>::new);
    let detail_loading = use_state(|| false);

    let filter_options = use_state(|| None::<FilterOptions>);
    let presets = use_state(Vec::<String>::new);
    let last_gather_time = use_state(|| None::<String>);
    let (state, push, remove) = use_timed_notification(5000);

    let mode = use_state(|| BrowserMode::Browse);
    let source = use_state(|| "local".to_string());
    let pending_delete = use_state(|| None::<VeteranPageItem>);

    // ── Reusable query runner ───────────────────────────────────
    // Takes explicit values (not state) so timing is never an issue.
    let pn = push.clone();
    let run_query: Rc<
        dyn Fn(
            Vec<Filter>,
            SortConfig,
            u32,
            u32,
            Option<LegacyPlannerSlot>,
            Option<String>,
            String,
        ),
    > = {
        let v = veterans.clone();
        let t = total.clone();
        let l = loading.clone();
        let e = error.clone();
        Rc::new(move |flt, srt, p, psz, slot, name, src| {
            let v = v.clone();
            let t = t.clone();
            let l = l.clone();
            let e = e.clone();
            let is_api = src == "uma_moe";
            wasm_bindgen_futures::spawn_local(async move {
                l.set(true);
                let cmd = if is_api {
                    "query_uma_moe_veterans"
                } else {
                    "query_veteran_store_page"
                };
                let raw = invoke_tauri_command(
                    cmd,
                    json!({
                        "query": {
                            "filters": flt.clone(),
                            "sort": srt.clone(),
                            "page": p,
                            "page_size": psz,
                            "legacy_planner_slot": slot,
                        }
                    }),
                )
                .await;
                match raw {
                    Ok(result) => {
                        match serde_json::from_value::<PaginationResponse<VeteranPageItem>>(result)
                        {
                            Ok(qr) => {
                                v.set(qr.results);
                                t.set(qr.total);
                                e.set(None);
                                if let Some(name) = name {
                                    let filters_json =
                                        serde_json::to_string(&(*flt)).unwrap_or_default();
                                    let sort_json =
                                        serde_json::to_string(&(srt)).unwrap_or_default();
                                    let bt = browser_type(&src);
                                    let _ = invoke_tauri_command(
                                        "save_preset",
                                        json!({
                                            "name": name,
                                            "filters": filters_json,
                                            "sort": sort_json,
                                            "browserType": bt,
                                        }),
                                    )
                                    .await;
                                }
                            }
                            Err(err) => {
                                e.set(Some(format!("parse error: {}", err)));
                            }
                        }
                    }
                    Err(err) => e.set(Some(err)),
                }
                l.set(false);
            });
        })
    };

    // Initial load
    {
        let run_query = run_query.clone();
        let page_size = page_size.clone();
        let filter_options = filter_options.clone();
        let presets = presets.clone();
        let filters = filters.clone();
        let sort = sort.clone();
        let mode = mode.clone();
        let source = source.clone();
        let last_gather_time = last_gather_time.clone();

        use_effect_with((), move |_| {
            let run_query = run_query.clone();
            let page_size = page_size.clone();
            let filter_options = filter_options.clone();
            let presets = presets.clone();
            let filters = filters.clone();
            let sort = sort.clone();
            let mode = mode.clone();
            let source = source.clone();
            let last_gather_time = last_gather_time.clone();

            wasm_bindgen_futures::spawn_local(async move {
                // Fetch browser mode from backend
                let mut initial_slot: Option<LegacyPlannerSlot> = None;
                let label = get_window_label().unwrap_or_default();
                if let Ok(result) =
                    invoke_tauri_command("get_browser_mode", json!({"windowLabel": label})).await
                {
                    if let Ok(Some(mode_str)) = serde_json::from_value::<Option<String>>(result) {
                        if let Some(slot_label) = mode_str.strip_prefix("select_veteran:") {
                            if let Some(slot) = LegacyPlannerSlot::from_label(slot_label) {
                                initial_slot = Some(slot);
                                mode.set(BrowserMode::SelectVeteran { slot });
                            }
                        }
                    }
                }

                // Fetch source (local vs uma_moe)
                let current_source: String = if let Ok(result) =
                    invoke_tauri_command("get_browser_source", json!({"windowLabel": label})).await
                {
                    if let Ok(Some(src)) = serde_json::from_value::<Option<String>>(result) {
                        source.set(src.clone());
                        src
                    } else {
                        (*source).clone()
                    }
                } else {
                    (*source).clone()
                };

                // Load filter options
                if let Ok(result) = invoke_tauri_command("get_filter_options", json!({})).await {
                    if let Ok(opts) = serde_json::from_value::<FilterOptions>(result) {
                        filter_options.set(Some(opts));
                    }
                }

                // Load last gather time
                if let Ok(val) = invoke_tauri_command(
                    "get_last_gather_time",
                    json!({"key": "last_veterans_gathered"}),
                )
                .await
                {
                    if let Ok(time) = serde_json::from_value::<Option<String>>(val) {
                        last_gather_time.set(time);
                    }
                }

                let bt = browser_type(&current_source);

                // Load presets list
                if let Ok(result) =
                    invoke_tauri_command("list_presets", json!({"browserType": bt})).await
                {
                    if let Ok(list) = serde_json::from_value::<Vec<String>>(result) {
                        presets.set(list);
                    }
                }

                // Load active state
                let mut loaded_filters: Option<Vec<Filter>> = None;
                let mut loaded_sort: Option<SortConfig> = None;
                if let Ok(result) =
                    invoke_tauri_command("load_preset_active", json!({"browserType": bt})).await
                {
                    if let Ok(Some(data)) = serde_json::from_value::<Option<PresetData>>(result) {
                        if let Some(ref filters_json) = data.filters {
                            if let Ok(f) = serde_json::from_str::<Vec<Filter>>(filters_json) {
                                filters.set(f.clone());
                                loaded_filters = Some(f);
                            }
                        }
                        if let Some(ref sort_json) = data.sort {
                            if let Ok(s) = serde_json::from_str::<SortConfig>(sort_json) {
                                sort.set(s.clone());
                                loaded_sort = Some(s);
                            }
                        }
                    }
                }

                run_query(
                    loaded_filters.unwrap_or_default(),
                    loaded_sort.unwrap_or(SortConfig {
                        key: "CreatedAt".to_string(),
                        direction: "Desc".to_string(),
                    }),
                    1u32,
                    *page_size,
                    initial_slot,
                    None,
                    current_source,
                );
            });

            || {}
        });
    }

    let set_filters = {
        let filters = filters.clone();
        let sort = sort.clone();
        let page = page.clone();
        let page_size = page_size.clone();
        let run_query = run_query.clone();
        let mode = mode.clone();
        let source = source.clone();
        Callback::from(move |new_filters: Vec<Filter>| {
            filters.set(new_filters.clone());
            page.set(1u32);
            run_query(
                new_filters,
                (*sort).clone(),
                1u32,
                *page_size,
                (*mode).slot(),
                Some("__active__".to_string()),
                (*source).clone(),
            );
        })
    };

    let set_sort = {
        let filters = filters.clone();
        let sort = sort.clone();
        let page = page.clone();
        let page_size = page_size.clone();
        let run_query = run_query.clone();
        let mode = mode.clone();
        let source = source.clone();
        Callback::from(move |new_sort: SortConfig| {
            sort.set(new_sort.clone());
            page.set(1u32);
            run_query(
                (*filters).clone(),
                new_sort,
                1u32,
                *page_size,
                (*mode).slot(),
                Some("__active__".to_string()),
                (*source).clone(),
            );
        })
    };

    let go_to_page = {
        let filters = filters.clone();
        let sort = sort.clone();
        let page = page.clone();
        let page_size = page_size.clone();
        let run_query = run_query.clone();
        let mode = mode.clone();
        let source = source.clone();
        Callback::from(move |new_page: u32| {
            page.set(new_page);
            run_query(
                (*filters).clone(),
                (*sort).clone(),
                new_page,
                *page_size,
                (*mode).slot(),
                None,
                (*source).clone(),
            );
        })
    };

    let open_detail = {
        let detail_veteran = detail_veteran.clone();
        let detail_sparks = detail_sparks.clone();
        let detail_wins = detail_wins.clone();
        let detail_parents = detail_parents.clone();
        let detail_skills = detail_skills.clone();
        let detail_support_cards = detail_support_cards.clone();
        let detail_loading = detail_loading.clone();
        let source = source.clone();
        Callback::from(move |hash: i64| {
            let detail_veteran = detail_veteran.clone();
            let detail_sparks = detail_sparks.clone();
            let detail_wins = detail_wins.clone();
            let detail_parents = detail_parents.clone();
            let detail_skills = detail_skills.clone();
            let detail_support_cards = detail_support_cards.clone();
            let detail_loading = detail_loading.clone();
            let src = (*source).clone();
            detail_veteran.set(Some(hash));
            detail_support_cards.set(Vec::new());
            detail_loading.set(true);
            let h_hex = format!("{:016x}", hash as u64);
            wasm_bindgen_futures::spawn_local(async move {
                if src == "uma_moe" {
                    detail_skills.set(Vec::new());
                    if let Ok(result) =
                        invoke_tauri_command("get_uma_moe_veteran_detail", json!({"hash": h_hex}))
                            .await
                    {
                        if let Ok(d) = serde_json::from_value::<UmaMoeDetailPayload>(result) {
                            detail_sparks.set(d.sparks);
                            detail_wins.set(d.wins);
                            detail_parents.set(d.parents);
                        }
                    }
                } else {
                    let h1 = hash.to_string();
                    let h2 = hash.to_string();
                    let h3 = hash.to_string();
                    let h4 = hash.to_string();
                    let h5 = hash.to_string();
                    let sparks_fut =
                        invoke_tauri_command("get_veteran_sparks", json!({"hash": h1}));
                    let wins_fut = invoke_tauri_command("get_veteran_wins", json!({"hash": h2}));
                    let parents_fut =
                        invoke_tauri_command("get_veteran_parents", json!({"hash": h3}));
                    let skills_fut =
                        invoke_tauri_command("get_veteran_skills", json!({"hash": h4}));
                    let sc_fut =
                        invoke_tauri_command("get_veteran_support_cards", json!({"hash": h5}));

                    if let Ok(result) = sparks_fut.await {
                        if let Ok(s) = serde_json::from_value::<Vec<SparkGroupRow>>(result) {
                            detail_sparks.set(s);
                        }
                    }
                    if let Ok(result) = wins_fut.await {
                        if let Ok(w) = serde_json::from_value::<Vec<MajorWinRow>>(result) {
                            detail_wins.set(w);
                        }
                    }
                    if let Ok(result) = parents_fut.await {
                        if let Ok(p) = serde_json::from_value::<Vec<ParentRow>>(result) {
                            detail_parents.set(p);
                        }
                    }
                    if let Ok(result) = skills_fut.await {
                        if let Ok(sk) = serde_json::from_value::<Vec<VeteranSkillRow>>(result) {
                            detail_skills.set(sk);
                        }
                    }
                    if let Ok(result) = sc_fut.await {
                        if let Ok(sc) = serde_json::from_value::<Vec<VeteranSupportCardRow>>(result)
                        {
                            detail_support_cards.set(sc);
                        }
                    }
                }
                detail_loading.set(false);
            });
        })
    };

    let close_detail = {
        let detail_veteran = detail_veteran.clone();
        Callback::from(move |_| {
            detail_veteran.set(None);
        })
    };

    let on_refresh = {
        let filters = filters.clone();
        let sort = sort.clone();
        let page = page.clone();
        let page_size = page_size.clone();
        let run_query = run_query.clone();
        let mode = mode.clone();
        let filter_options = filter_options.clone();
        let source = source.clone();
        Callback::from(move |_| {
            run_query(
                (*filters).clone(),
                (*sort).clone(),
                *page,
                *page_size,
                (*mode).slot(),
                None,
                (*source).clone(),
            );
            // refresh filter options so new tags appear in filter autocomplete
            let filter_options = filter_options.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(result) = invoke_tauri_command("get_filter_options", json!({})).await {
                    if let Ok(opts) = serde_json::from_value::<FilterOptions>(result) {
                        filter_options.set(Some(opts));
                    }
                }
            });
        })
    };

    let load_preset = {
        let filters = filters.clone();
        let sort = sort.clone();
        let page = page.clone();
        let page_size = page_size.clone();
        let run_query = run_query.clone();
        let push = push.clone();
        let mode = mode.clone();
        let source = source.clone();
        Callback::from(move |name: String| {
            let filters = filters.clone();
            let sort = sort.clone();
            let page = page.clone();
            let page_size = page_size.clone();
            let run_query = run_query.clone();
            let push = push.clone();
            let mode = mode.clone();
            let source = source.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let bt = browser_type(&source);
                match invoke_tauri_command("load_preset", json!({"name": name, "browserType": bt}))
                    .await
                {
                    Ok(result) => {
                        if let Ok(Some(data)) = serde_json::from_value::<Option<PresetData>>(result)
                        {
                            let mut new_filters: Vec<Filter> = Vec::new();
                            let mut new_sort = SortConfig {
                                key: "CreatedAt".to_string(),
                                direction: "Desc".to_string(),
                            };
                            if let Some(ref filters_json) = data.filters {
                                if let Ok(f) = serde_json::from_str::<Vec<Filter>>(filters_json) {
                                    new_filters = f;
                                }
                            }
                            if let Some(ref sort_json) = data.sort {
                                if let Ok(s) = serde_json::from_str::<SortConfig>(sort_json) {
                                    new_sort = s;
                                }
                            }
                            if !new_filters.is_empty() {
                                filters.set(new_filters.clone());
                            }
                            sort.set(new_sort.clone());
                            page.set(1u32);
                            run_query(
                                new_filters,
                                new_sort,
                                1u32,
                                *page_size,
                                (*mode).slot(),
                                Some(name.clone()),
                                (*source).clone(),
                            );
                            push(Notification::success(format!("Loaded preset: {}", name)));
                        }
                    }
                    Err(e) => push(Notification::error(format!("Error: {}", e))),
                }
            });
        })
    };

    let save_as_preset = {
        let filters = filters.clone();
        let sort = sort.clone();
        let presets = presets.clone();
        let push = push.clone();
        let source = source.clone();
        Callback::from(move |name: String| {
            let filters = filters.clone();
            let sort = sort.clone();
            let presets = presets.clone();
            let push = push.clone();
            let source = source.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let filters_json = serde_json::to_string(&(*filters)).unwrap_or_default();
                let sort_json = serde_json::to_string(&(*sort)).unwrap_or_default();
                let bt = browser_type(&source);
                match invoke_tauri_command(
                    "save_preset",
                    json!({
                        "name": name,
                        "filters": filters_json,
                        "sort": sort_json,
                        "browserType": bt,
                    }),
                )
                .await
                {
                    Ok(_) => {
                        if let Ok(result) =
                            invoke_tauri_command("list_presets", json!({"browserType": bt})).await
                        {
                            if let Ok(list) = serde_json::from_value::<Vec<String>>(result) {
                                presets.set(list);
                            }
                        }
                        push(Notification::success(format!("Saved preset: {}", name)));
                    }
                    Err(e) => push(Notification::error(format!("Error: {}", e))),
                }
            });
        })
    };

    let delete_preset = {
        let presets = presets.clone();
        let push = push.clone();
        let source = source.clone();
        Callback::from(move |name: String| {
            let presets = presets.clone();
            let push = push.clone();
            let source = source.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let bt = browser_type(&source);
                match invoke_tauri_command(
                    "delete_preset",
                    json!({"name": name, "browserType": bt}),
                )
                .await
                {
                    Ok(_) => {
                        if let Ok(result) =
                            invoke_tauri_command("list_presets", json!({"browserType": bt})).await
                        {
                            if let Ok(list) = serde_json::from_value::<Vec<String>>(result) {
                                presets.set(list);
                            }
                        }
                        push(Notification::success(format!("Deleted preset: {}", name)));
                    }
                    Err(e) => push(Notification::error(format!("Error: {}", e))),
                }
            });
        })
    };

    let on_select_veteran = {
        let push = push.clone();
        Callback::from(move |hash: String| {
            let push = push.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match invoke_tauri_command(
                    "return_veteran_selection",
                    json!({"hash": hash.clone()}),
                )
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        push(Notification::error(format!("Error: {}", e)));
                    }
                }
            });
        })
    };

    let on_save_veteran = {
        let push = push.clone();
        Callback::from(move |hash: String| {
            let push = push.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match invoke_tauri_command("save_uma_moe_veteran", json!({"hash": hash.clone()}))
                    .await
                {
                    Ok(_) => {
                        push(Notification::success("Saved to local DB"));
                    }
                    Err(e) => {
                        push(Notification::error(format!("Save error: {}", e)));
                    }
                }
            });
        })
    };

    let on_delete = {
        let veterans = veterans.clone();
        let pending_delete = pending_delete.clone();
        Callback::from(move |hash: String| {
            if let Some(item) = veterans
                .iter()
                .find(|v| format!("{:016x}", v.veteran.hash as u64) == hash)
            {
                pending_delete.set(Some(item.clone()));
            }
        })
    };

    let on_confirm_delete = {
        let pending_delete = pending_delete.clone();
        let push = push.clone();
        let run_query = run_query.clone();
        let filters = filters.clone();
        let sort = sort.clone();
        let page = page.clone();
        let page_size = page_size.clone();
        let mode = mode.clone();
        let source = source.clone();
        Callback::from(move |_: ()| {
            let hash = (*pending_delete).as_ref().map(|item| item.veteran.hash);
            if let Some(hash) = hash {
                let h_hex = format!("{:016x}", hash as u64);
                let push = push.clone();
                let run_query = run_query.clone();
                let filters = filters.clone();
                let sort = sort.clone();
                let page = page.clone();
                let page_size = page_size.clone();
                let mode = mode.clone();
                let source = source.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match invoke_tauri_command("delete_veteran", json!({"hash": h_hex})).await {
                        Ok(_) => {
                            push(Notification::success("Veteran removed"));
                            run_query(
                                (*filters).clone(),
                                (*sort).clone(),
                                *page,
                                *page_size,
                                (*mode).slot(),
                                None,
                                (*source).clone(),
                            );
                        }
                        Err(e) => {
                            push(Notification::error(format!("Delete error: {}", e)));
                        }
                    }
                });
            }
        })
    };

    let on_close_delete_modal = {
        let pending_delete = pending_delete.clone();
        Callback::from(move |_: ()| pending_delete.set(None))
    };

    let total_pages = ((*total + *page_size - 1) / *page_size).max(1);
    let first_item = if *total == 0 { 0 } else { (*page - 1) * *page_size + 1 };
    let last_item = ((*page) * (*page_size)).min(*total);

    let active_spark_group_ids = {
        let mut ids = Vec::new();
        for f in filters.iter() {
            match f {
                Filter::Spark(sp) => {
                    let gid = sp.group_id as i64;
                    if !ids.contains(&gid) {
                        ids.push(gid);
                    }
                }
                Filter::WhiteSpark(wsf) => {
                    for gid in &wsf.group_ids {
                        if !ids.contains(gid) {
                            ids.push(*gid);
                        }
                    }
                }
                _ => {}
            }
        }
        ids
    };

    let stylesheet = StyleManager::render_stylesheet();
    let api_mode = *source == "uma_moe";
    let delete_modal_show = (*pending_delete).is_some();
    let delete_item_name = (*pending_delete)
        .as_ref()
        .map(|item| {
            item.veteran
                .trainee_name
                .clone()
                .unwrap_or_else(|| format!("Veteran #{:016x}", item.veteran.hash as u64))
        })
        .unwrap_or_default();

    html! {
        <div class={VeteranBrowserRootStyle::CLASS_NAME}>
            <div class={BrowserHeaderStyle::CLASS_NAME}>
                <h1>{
                    match &*mode {
                        BrowserMode::Browse => {
                            if api_mode { "Veteran Browser (API)".to_string() } else { "Veteran Browser".to_string() }
                        }
                        BrowserMode::SelectVeteran { slot } => {
                            format!("Assign to {} — Select a veteran", slot.label())
                        }
                    }
                }</h1>
                <div class={BrowserHeaderControlsStyle::CLASS_NAME}>
                    <PresetManager
                        presets={(*presets).clone()}
                        on_load={load_preset}
                        on_save={save_as_preset}
                        on_delete={delete_preset}
                    />
                    <SortSelector sort={(*sort).clone()} on_change={set_sort} api_mode={api_mode} />
                    <div style="display: flex; flex-direction: column; align-items: flex-end;">
                        <div style="display: flex; gap: 8px;">
                    if !api_mode {
                        <GatherVeteransButton
                            on_complete={{
                                let run_query = run_query.clone();
                                let push = push.clone();
                                let filters = filters.clone();
                                let sort = sort.clone();
                                let mode = mode.clone();
                                let page_size = page_size.clone();
                                let source = source.clone();
                                let last_gather_time = last_gather_time.clone();
                                Callback::from(move |result: Result<shared::GatherVeteransResult, String>| {
                                    match result {
                                        Ok(counts) => {
                                            push(Notification::success(format!(
                                                "{} added, {} removed ({} total)",
                                                counts.added, counts.removed, counts.total
                                            )));
                                            run_query((*filters).clone(), (*sort).clone(), 1, *page_size, (*mode).slot(), None, (*source).clone());
                                            let last_gather_time = last_gather_time.clone();
                                            wasm_bindgen_futures::spawn_local(async move {
                                                if let Ok(val) = invoke_tauri_command("get_last_gather_time", json!({"key": "last_veterans_gathered"})).await {
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
                    }
                    if !api_mode {
                        <ExportVeteransButton
                            on_complete={{
                                let push = push.clone();
                                Callback::from(move |result: Result<String, String>| {
                                    match result {
                                        Ok(msg) if msg == "canceled" => {}
                                        Ok(path) => {
                                            push(Notification::success(format!("Saved: {}", path)));
                                        }
                                        Err(e) => {
                                            push(Notification::error(format!("Export failed: {}", e)));
                                        }
                                    }
                                })
                            }}
                        />
                    }
                    </div>
                    {crate::components::render_gather_time(&last_gather_time)}
                </div>
            </div>
        </div>

            {stylesheet}

            <NotificationOverlay notifications={state.0.clone()} on_close={{
                let remove = remove.clone();
                Callback::from(move |id: u32| remove(id))
            }} />

            <div class={BrowserBodyStyle::CLASS_NAME}>
                <aside class={BrowserSidebarStyle::CLASS_NAME}>
                    <FilterPanel
                        filters={(*filters).clone()}
                        on_change={set_filters}
                        options={(*filter_options).clone()}
                        api_mode={api_mode}
                    />
                </aside>

                <main class={BrowserMainStyle::CLASS_NAME}>
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
                        <div class={BrowserLoadingStyle::CLASS_NAME}>{"Loading..."}</div>
                    } else if let Some(err) = &*error {
                        <div class={BrowserErrorStyle::CLASS_NAME}>{ err }</div>
                    } else if veterans.is_empty() {
                        <div class={BrowserEmptyStyle::CLASS_NAME}>{"No veterans found. Try adjusting filters or gather veteran data first."}</div>
                    } else {
                        <div style="flex: 1; overflow-y: auto; min-height: 0;">
                            <div class={CardGridStyle::CLASS_NAME}>
                                { for (*veterans).iter().map(|v| {
                                    let hash = v.veteran.hash;
                                    let on_click = open_detail.clone();
                                    let on_select = match &*mode {
                                        BrowserMode::SelectVeteran { .. } => Some(on_select_veteran.clone()),
                                        BrowserMode::Browse => None,
                                    };
                                    let on_save = if api_mode && matches!(*mode, BrowserMode::Browse) {
                                        Some(on_save_veteran.clone())
                                    } else {
                                        None
                                    };
                                    let on_delete = if !v.veteran.owned && matches!(*mode, BrowserMode::Browse) {
                                        Some(on_delete.clone())
                                    } else {
                                        None
                                    };
                                    let scenarios = filter_options.as_ref().map(|o| o.scenarios.clone()).unwrap_or_default();
                                    let affinity = v.affinity;
                                    let tags = v.tags.clone();
                                    html! {
                                        <VeteranCard
                                            veteran={v.veteran.clone()}
                                            on_click={Callback::from(move |_| on_click.emit(hash))}
                                            on_select={on_select}
                                            on_save={on_save}
                                            {on_delete}
                                            active_spark_group_ids={active_spark_group_ids.clone()}
                                            scenarios={scenarios}
                                            {affinity}
                                            {tags}
                            />
                        }
                                }) }
                            </div>
                        </div>
                    }
                </main>
            </div>

            { detail_veteran.as_ref().map(|hash| {
                let veteran = (*veterans).iter().find(|v| v.veteran.hash == *hash).map(|p| p.veteran.clone());
                html! {
                    <DetailModal
                        veteran={veteran}
                        sparks={(*detail_sparks).clone()}
                        wins={(*detail_wins).clone()}
                        parents={(*detail_parents).clone()}
                        skills={(*detail_skills).clone()}
                        support_cards={(*detail_support_cards).clone()}
                        loading={*detail_loading}
                        on_close={close_detail}
                        on_refresh={on_refresh}
                        api_mode={api_mode}
                        active_spark_group_ids={active_spark_group_ids.clone()}
                    />
                }
            }) }

            <DeleteConfirmationModal
                show={delete_modal_show}
                title={"Remove Veteran".to_string()}
                item_name={delete_item_name}
                on_confirm={on_confirm_delete}
                on_close={on_close_delete_modal}
            />
        </div>
    }
}
