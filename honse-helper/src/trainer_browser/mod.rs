use crate::{
    components::notifications::{use_timed_notification, Notification, NotificationOverlay},
    styles::{trainer_browser::*, Style, StyleManager},
    support_card_browser::components::support_card_detail_modal::SupportCardDetailModal,
    tauri_bridge::invoke_tauri_command,
    veteran_browser::components::detail_modal::DetailModal,
    veteran_browser::components::pagination::Pagination,
    veteran_browser::components::preset_manager::PresetManager,
};
use serde_json::json;
use shared::{
    models::PaginationResponse,
    support_card_browser::SupportCardPageItem,
    trainer_browser::*,
    veteran_browser::{
        MajorWinRow, ParentRow, PresetData, SparkGroupRow, VeteranRow, VeteranSkillRow,
        VeteranSupportCardRow,
    },
};
use std::rc::Rc;
use yew::prelude::*;

pub mod components;

use components::add_trainer::AddTrainerById;
use components::filter_panel::TrainerFilterPanel;
use components::gather_button::GatherFollowedTrainersButton;
use components::sort_selector::TrainerSortSelector;
use components::trainer_card::TrainerCard;

const PAGE_SIZE: u32 = 30;
const BROWSER_TYPE: &str = "trainer";

#[function_component]
pub fn TrainerBrowser() -> Html {
    let filters = use_state(Vec::<TrainerFilter>::new);
    let sort = use_state(|| TrainerSortConfig::default());
    let page = use_state(|| 1u32);
    let trainers = use_state(Vec::<TrainerPageItem>::new);
    let total = use_state(|| 0u32);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let filter_options = use_state(|| TrainerFilterOptions {
        characters: Vec::new(),
        card_types: Vec::new(),
        rarities: Vec::new(),
    });
    let presets = use_state(Vec::<String>::new);

    let (notification_state, push, remove) = use_timed_notification(3000);

    let detail_hash = use_state(|| None::<i64>);
    let detail_veteran = use_state(|| None::<VeteranRow>);
    let detail_sparks = use_state(Vec::<SparkGroupRow>::new);
    let detail_wins = use_state(Vec::<MajorWinRow>::new);
    let detail_parents = use_state(Vec::<ParentRow>::new);
    let detail_skills = use_state(Vec::<VeteranSkillRow>::new);
    let detail_support_cards = use_state(Vec::<VeteranSupportCardRow>::new);
    let detail_loading = use_state(|| false);

    let selected_support_card = use_state(|| None::<SupportCardPageItem>);

    let api_key_configured = use_state(|| false);
    let refreshing_ids = use_state(Vec::<i64>::new);

    // Load api key status on mount
    {
        let api_key_configured = api_key_configured.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(val) = invoke_tauri_command("get_api_key_status", json!({})).await {
                    if let Ok(status) = serde_json::from_value::<shared::ApiKeyStatus>(val) {
                        api_key_configured.set(status.configured);
                    }
                }
            });
            || {}
        });
    }

    let last_gather_time = use_state(|| None::<String>);

    // Load filter options on mount
    {
        let filter_options = filter_options.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(val) =
                    invoke_tauri_command("get_trainer_filter_options", json!({})).await
                {
                    if let Ok(opts) = serde_json::from_value::<TrainerFilterOptions>(val) {
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
                if let Ok(val) = invoke_tauri_command(
                    "list_presets",
                    json!({"browserType": BROWSER_TYPE}),
                )
                .await
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
                    json!({"key": "last_trainers_gathered"}),
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
        let trainers = trainers.clone();
        let total = total.clone();
        let loading = loading.clone();
        let error = error.clone();
        let push = push.clone();
        Rc::new(
            move |flt: Vec<TrainerFilter>, srt: TrainerSortConfig, p: u32| {
                let filters = filters.clone();
                let sort = sort.clone();
                let page = page.clone();
                let trainers = trainers.clone();
                let total = total.clone();
                let loading = loading.clone();
                let error = error.clone();
                let push = push.clone();
                filters.set(flt.clone());
                sort.set(srt.clone());
                page.set(p);
                loading.set(true);
                error.set(None);
                wasm_bindgen_futures::spawn_local(async move {
                    let query = TrainerBrowserQuery {
                        filters: flt,
                        sort: srt,
                        page: p,
                        page_size: PAGE_SIZE,
                    };
                    match invoke_tauri_command(
                        "query_trainer_page",
                        json!({ "query": query }),
                    )
                    .await
                    {
                        Ok(val) => {
                            match serde_json::from_value::<
                                PaginationResponse<TrainerPageItem>,
                            >(val)
                            {
                                Ok(resp) => {
                                    trainers.set(resp.results);
                                    total.set(resp.total);
                                }
                                Err(e) => {
                                    push(Notification::error(format!("Parse error: {e}")));
                                }
                            }
                            loading.set(false);
                        }
                        Err(e) => {
                            push(Notification::error(format!("Query failed: {e}")));
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
                    "load_preset_active",
                    json!({"browserType": BROWSER_TYPE}),
                )
                .await
                {
                    if let Ok(Some(data)) = serde_json::from_value::<Option<PresetData>>(val) {
                        if let Some(ref filters_json) = data.filters {
                            if let Ok(f) = serde_json::from_str::<Vec<TrainerFilter>>(filters_json)
                            {
                                loaded_filters = f.clone();
                                filters.set(f);
                            }
                        }
                        if let Some(ref sort_json) = data.sort {
                            if let Ok(s) = serde_json::from_str::<TrainerSortConfig>(sort_json) {
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
        Callback::from(move |flt: Vec<TrainerFilter>| {
            run_query(flt, (*sort).clone(), 1);
        })
    };

    let set_sort = {
        let run_query = run_query.clone();
        let filters = filters.clone();
        Callback::from(move |srt: TrainerSortConfig| {
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

    let open_detail = {
        let detail_hash = detail_hash.clone();
        let detail_veteran = detail_veteran.clone();
        let detail_sparks = detail_sparks.clone();
        let detail_wins = detail_wins.clone();
        let detail_parents = detail_parents.clone();
        let detail_skills = detail_skills.clone();
        let detail_support_cards = detail_support_cards.clone();
        let detail_loading = detail_loading.clone();
        Callback::from(move |hash: i64| {
            let detail_hash = detail_hash.clone();
            let detail_veteran = detail_veteran.clone();
            let detail_sparks = detail_sparks.clone();
            let detail_wins = detail_wins.clone();
            let detail_parents = detail_parents.clone();
            let detail_skills = detail_skills.clone();
            let detail_support_cards = detail_support_cards.clone();
            let detail_loading = detail_loading.clone();
            detail_hash.set(Some(hash));
            detail_veteran.set(None);
            detail_sparks.set(Vec::new());
            detail_wins.set(Vec::new());
            detail_parents.set(Vec::new());
            detail_skills.set(Vec::new());
            detail_support_cards.set(Vec::new());
            detail_loading.set(true);
            let h = hash.to_string();
            wasm_bindgen_futures::spawn_local(async move {
                let vet_fut =
                    invoke_tauri_command("get_veteran_detail", json!({"hash": h}));
                let sparks_fut =
                    invoke_tauri_command("get_veteran_sparks", json!({"hash": h.clone()}));
                let wins_fut =
                    invoke_tauri_command("get_veteran_wins", json!({"hash": h.clone()}));
                let parents_fut =
                    invoke_tauri_command("get_veteran_parents", json!({"hash": h.clone()}));
                let skills_fut =
                    invoke_tauri_command("get_veteran_skills", json!({"hash": h.clone()}));
                let sc_fut =
                    invoke_tauri_command("get_veteran_support_cards", json!({"hash": h.clone()}));

                if let Ok(result) = vet_fut.await {
                    if let Ok(v) = serde_json::from_value::<Option<VeteranRow>>(result) {
                        detail_veteran.set(v);
                    }
                }
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
                    if let Ok(sc) = serde_json::from_value::<Vec<VeteranSupportCardRow>>(result) {
                        detail_support_cards.set(sc);
                    }
                }
                detail_loading.set(false);
            });
        })
    };

    let close_detail = {
        let detail_hash = detail_hash.clone();
        Callback::from(move |_| {
            detail_hash.set(None);
        })
    };

    let open_support_card_detail = {
        let selected = selected_support_card.clone();
        Callback::from(move |sc: BorrowSupportCardInfo| {
            selected.set(Some(SupportCardPageItem {
                support_card_id: sc.id,
                name: sc.name,
                rarity: sc.card_rarity,
                card_type: sc.card_type,
                level: sc.level,
                max_level: max_level_for_limit_break(sc.card_rarity, sc.limit_break_count),
                limit_break_count: sc.limit_break_count,
                exp: 0,
                favorite_flag: false,
                stock: 0,
                character_id: sc.character_id,
                owned: false,
                borrow_available: false,
                borrow_level: 0,
                borrow_limit_break_count: 0,
            }));
        })
    };

    let close_support_card_detail = {
        let selected = selected_support_card.clone();
        Callback::from(move |_| selected.set(None))
    };

    let refresh_trainer = {
        let refreshing_ids = refreshing_ids.clone();
        let run_query = run_query.clone();
        let filters = filters.clone();
        let sort = sort.clone();
        let page = page.clone();
        let push = push.clone();
        Callback::from(move |trainer_id: i64| {
            let mut ids = (*refreshing_ids).clone();
            if ids.contains(&trainer_id) {
                return;
            }
            ids.push(trainer_id);
            refreshing_ids.set(ids);
            let refreshing_ids = refreshing_ids.clone();
            let run_query = run_query.clone();
            let filters = filters.clone();
            let sort = sort.clone();
            let page = page.clone();
            let push = push.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let result = invoke_tauri_command(
                    "refresh_uma_moe_trainer",
                    json!({ "trainerId": trainer_id }),
                )
                .await;
                let mut ids = (*refreshing_ids).clone();
                ids.retain(|id| *id != trainer_id);
                refreshing_ids.set(ids);
                match result {
                    Ok(name) => {
                        push(Notification::success(format!("Refreshed trainer {}", name)));
                    }
                    Err(e) => {
                        push(Notification::error(format!("Refresh failed: {}", e)));
                    }
                }
                run_query((*filters).clone(), (*sort).clone(), *page);
            });
        })
    };

    let total_pages = (*total + PAGE_SIZE - 1) / PAGE_SIZE;
    let first_item = if *total == 0 { 0 } else { (*page - 1) * PAGE_SIZE + 1 };
    let last_item = ((*page) * PAGE_SIZE).min(*total);

    // Preset callbacks (shared preset commands, browserType = "trainer")
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
                match invoke_tauri_command(
                    "load_preset",
                    json!({ "name": name, "browserType": BROWSER_TYPE }),
                )
                .await
                {
                    Ok(val) => {
                        if let Ok(Some(data)) = serde_json::from_value::<Option<PresetData>>(val) {
                            let new_filters: Vec<TrainerFilter> = data
                                .filters
                                .as_deref()
                                .and_then(|s| serde_json::from_str(s).ok())
                                .unwrap_or_else(|| (*filters).clone());
                            let new_sort: TrainerSortConfig = data
                                .sort
                                .as_deref()
                                .and_then(|s| serde_json::from_str(s).ok())
                                .unwrap_or_else(|| (*sort).clone());
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
                    "save_preset",
                    json!({
                        "name": name,
                        "filters": filters_json,
                        "sort": sort_json,
                        "browserType": BROWSER_TYPE,
                    }),
                )
                .await
                {
                    Ok(_) => {
                        push(Notification::success("Preset saved".to_string()));
                        if let Ok(val) = invoke_tauri_command(
                            "list_presets",
                            json!({"browserType": BROWSER_TYPE}),
                        )
                        .await
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
                match invoke_tauri_command(
                    "delete_preset",
                    json!({ "name": name, "browserType": BROWSER_TYPE }),
                )
                .await
                {
                    Ok(_) => {
                        push(Notification::success("Preset deleted".to_string()));
                        if let Ok(val) = invoke_tauri_command(
                            "list_presets",
                            json!({"browserType": BROWSER_TYPE}),
                        )
                        .await
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
        <div class={TrainerBrowserRootStyle::CLASS_NAME}>
            <div class={TrainerBrowserHeaderStyle::CLASS_NAME}>
                <h1>{"Trainer Browser"}</h1>
                <div class={TrainerBrowserHeaderControlsStyle::CLASS_NAME}>
                    <PresetManager
                        presets={(*presets).clone()}
                        on_load={load_preset}
                        on_save={save_as_preset}
                        on_delete={delete_preset}
                    />
                    <TrainerSortSelector sort={(*sort).clone()} on_change={set_sort} />
                    <AddTrainerById on_complete={{
                        let run_query = run_query.clone();
                        let push = push.clone();
                        let filters = filters.clone();
                        let sort = sort.clone();
                        Callback::from(move |result: Result<String, String>| {
                            match result {
                                Ok(name) => {
                                    push(Notification::success(format!("Added trainer {}", name)));
                                    run_query((*filters).clone(), (*sort).clone(), 1);
                                }
                                Err(e) => {
                                    push(Notification::error(format!("Add failed: {}", e)));
                                }
                            }
                        })
                    }} />
                    <div style="display: flex; flex-direction: column; align-items: flex-end;">
                        <GatherFollowedTrainersButton
                            on_complete={{
                                let run_query = run_query.clone();
                                let push = push.clone();
                                let filters = filters.clone();
                                let sort = sort.clone();
                                let last_gather_time = last_gather_time.clone();
                                Callback::from(move |result: Result<(), String>| {
                                    match result {
                                        Ok(()) => {
                                            push(Notification::success("Followed trainers gathered"));
                                            run_query((*filters).clone(), (*sort).clone(), 1);
                                            let last_gather_time = last_gather_time.clone();
                                            wasm_bindgen_futures::spawn_local(async move {
                                                if let Ok(val) = invoke_tauri_command("get_last_gather_time", json!({"key": "last_trainers_gathered"})).await {
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

            <div class={TrainerBrowserBodyStyle::CLASS_NAME}>
                <aside class={TrainerBrowserSidebarStyle::CLASS_NAME}>
                    <TrainerFilterPanel
                        filters={(*filters).clone()}
                        on_change={set_filters}
                        options={(*filter_options).clone()}
                    />
                </aside>

                <main class={TrainerBrowserMainStyle::CLASS_NAME}>
                    <div style="display:flex;align-items:center;gap:14px;font-size:11px;color:#94a3b8;margin-bottom:8px;flex-shrink:0;">
                        <span style="display:inline-flex;align-items:center;gap:5px;">
                            <span style="width:5px;height:5px;border-radius:50%;background:#34d399;"></span>
                            {"Game follow list"}
                        </span>
                        <span style="display:inline-flex;align-items:center;gap:5px;">
                            <span style="width:5px;height:5px;border-radius:50%;background:#60a5fa;"></span>
                            {"uma.moe API"}
                        </span>
                        <span style="display:inline-flex;align-items:center;gap:5px;">
                            <span style="width:5px;height:5px;border-radius:50%;background:#64748b;"></span>
                            {"Both sources"}
                        </span>
                    </div>
                    if *total > 0 {
                        <div style="display: flex; align-items: center; justify-content: space-between; margin-top: 2px; margin-bottom: 8px; flex-shrink: 0;">
                            <span style="font-size: 13px; color: #9ca3af;">{"Showing trainers "}{first_item}{" - "}{last_item}{" out of "}{*total}</span>
                            <Pagination
                                page={*page}
                                total_pages={total_pages}
                                on_page_change={go_to_page}
                            />
                        </div>
                    }
                    if *loading {
                        <div class={TrainerBrowserLoadingStyle::CLASS_NAME}>{"Loading..."}</div>
                    } else if let Some(err) = &*error {
                        <div class={TrainerBrowserErrorStyle::CLASS_NAME}>{ err }</div>
                    } else if trainers.is_empty() {
                        <div class={TrainerBrowserEmptyStyle::CLASS_NAME}>
                            {"No trainers found. Gather followed trainers (game on the Single Mode start screen) or add a trainer by ID via uma.moe."}
                        </div>
                    } else {
                        <div style="flex: 1; overflow-y: auto; min-height: 0;">
                            { for (*trainers).iter().map(|t| {
                                html! {
                                    <TrainerCard
                                        trainer={t.clone()}
                                        on_view_veteran={open_detail.clone()}
                                        on_view_support_card={open_support_card_detail.clone()}
                                        api_key_configured={*api_key_configured}
                                        refreshing_ids={(*refreshing_ids).clone()}
                                        on_refresh_trainer={refresh_trainer.clone()}
                                    />
                                }
                            }) }
                        </div>
                    }
                </main>
            </div>

            { (*detail_hash).map(|_| {
                html! {
                    <DetailModal
                        veteran={(*detail_veteran).clone()}
                        sparks={(*detail_sparks).clone()}
                        wins={(*detail_wins).clone()}
                        parents={(*detail_parents).clone()}
                        skills={(*detail_skills).clone()}
                        support_cards={(*detail_support_cards).clone()}
                        loading={*detail_loading}
                        on_close={close_detail.clone()}
                        on_refresh={Callback::from(|_| {})}
                    />
                }
            }) }

            { (*selected_support_card).as_ref().map(|card| {
                html! {
                    <SupportCardDetailModal card={card.clone()} borrow={true} on_close={close_support_card_detail.clone()} />
                }
            }) }
        </div>
    }
}
