use std::collections::HashMap;

use shared::{
    legacy_planner::{
        lookup_dtos::AffinityResult, LegacyPlannerSlot, LegacyPlannerState, LegacySlotValue,
        PlannerAffinities, SparkGroupInfo,
    },
    models::SparkType,
};
use yew::prelude::*;

use crate::{
    styles::{
        legacy_affinity::{
            LegacyAffinityTotalsBarStyle, LegacySparkPillStyle, LegacySparkPillsRowStyle,
        },
        legacy_planner::{
            AffinityBaseStyle, AffinityBonusStyle, AffinityPlusStyle, LegacyPlannerRootStyle,
            PlannerHeaderStyle, PlannerSectionLabelStyle, PlannerSlotStyle, PlannerTreeGridStyle,
            SecondaryBtnStyle, TraineeSelectorRowStyle, TreeAffinityBoxStyle,
            TreeAffinityCenterStyle, TreeAffinityEmptyStyle, TreeAffinityStyle, TreeTraineeStyle,
        },
        shared_components::HeaderActionButtonStyle,
        Style, StyleManager,
    },
    tauri_bridge::invoke_tauri_command,
};
use serde_json::json;

use self::{
    spark_modals::*, stats_aptitudes_modal::StatsAptitudesModal,
};

mod detail_modal;
mod spark_modals;
mod stats_aptitudes_modal;
mod veteran_slot;

use self::veteran_slot::LegacyVeteranSlot;

#[function_component]
pub fn LegacyPlanner() -> Html {
    let state = use_state(LegacyPlannerState::default);
    let affinities = use_state(PlannerAffinities::default);
    let loaded = use_state(|| false);

    let show_stats_aptitudes = use_state(|| false);
    let show_sparks_list = use_state(|| false);
    let show_white_spark = use_state(|| false);
    let show_inspiration = use_state(|| false);

    let load_state = {
        let state = state.clone();
        let affinities = affinities.clone();
        let loaded = loaded.clone();
        Callback::from(move |_| {
            let state = state.clone();
            let affinities = affinities.clone();
            let loaded = loaded.clone();
            wasm_bindgen_futures::spawn_local(async move {
                gloo_console::log!("[LegacyPlanner] load_state started");
                if let Ok(result) =
                    invoke_tauri_command("get_legacy_planner_state", json!({})).await
                {
                    if let Ok(s) = serde_json::from_value::<LegacyPlannerState>(result) {
                        gloo_console::log!(format!(
                            "[LegacyPlanner] get_legacy_planner_state returned chosen_character={:?}, parent_a={:?}, parent_b={:?}",
                            s.chosen_character,
                            s.parent_a,
                            s.parent_b
                        ));
                        state.set(s);
                    } else {
                        gloo_console::error!(
                            "[LegacyPlanner] Failed to deserialize LegacyPlannerState"
                        );
                    }
                } else {
                    gloo_console::error!("[LegacyPlanner] get_legacy_planner_state command failed");
                }
                if let Ok(result) =
                    invoke_tauri_command("compute_planner_affinities", json!({})).await
                {
                    if let Ok(a) = serde_json::from_value::<PlannerAffinities>(result) {
                        gloo_console::log!(format!(
                            "[LegacyPlanner] compute_planner_affinities returned trainee_parent_a={:?}, trainee_parent_b={:?}, parent_a_parent_b={:?}",
                            a.trainee_parent_a,
                            a.trainee_parent_b,
                            a.parent_a_parent_b
                        ));
                        affinities.set(a);
                    } else {
                        gloo_console::error!(
                            "[LegacyPlanner] Failed to deserialize PlannerAffinities"
                        );
                    }
                } else {
                    gloo_console::error!(
                        "[LegacyPlanner] compute_planner_affinities command failed"
                    );
                }
                loaded.set(true);
                gloo_console::log!("[LegacyPlanner] load_state finished");
            });
        })
    };

    {
        let load_state = load_state.clone();
        let state_for_listener = state.clone();
        use_effect_with((), move |_| {
            load_state.emit(());
            crate::tauri_bridge::listen_to_event("legacy-planner-state-changed", move |payload| {
                gloo_console::log!(format!(
                    "[LegacyPlanner] legacy-planner-state-changed payload: {:?}",
                    payload
                ));
                if let Ok(new_state) = serde_json::from_value::<LegacyPlannerState>(payload) {
                    gloo_console::log!(format!(
                        "[LegacyPlanner] Applying direct state update: chosen_character={:?}, parent_a={:?}, parent_b={:?}",
                        new_state.chosen_character,
                        new_state.parent_a,
                        new_state.parent_b
                    ));
                    state_for_listener.set(new_state);
                } else {
                    gloo_console::error!(
                        "[LegacyPlanner] Failed to deserialize legacy-planner-state-changed payload; falling back to reload"
                    );
                    load_state.emit(());
                }
            });
            || {}
        });
    }

    let on_clear_all = {
        let load_state = load_state.clone();
        Callback::from(move |_: yew::MouseEvent| {
            let load_state = load_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke_tauri_command("clear_legacy_planner", json!({})).await;
                load_state.emit(());
            });
        })
    };

    let on_trainee_select_from_browser = {
        let load_state = load_state.clone();
        Callback::from(move |payload: serde_json::Value| {
            let load_state = load_state.clone();
            let trainee_id = payload["trainee_id"].as_i64();
            let trainee_name = payload["trainee_name"].as_str().map(|s| s.to_string());
            let character_id = payload["character_id"].as_i64();
            let character_name = payload["character_name"].as_str().map(|s| s.to_string());
            let slot_label = payload["slot_label"].as_str().map(|s| s.to_string());
            if let (Some(cid), Some(cname)) = (character_id, character_name) {
                wasm_bindgen_futures::spawn_local(async move {
                    if let Some(label) = slot_label {
                        let _ = invoke_tauri_command(
                            "set_legacy_planner_slot_character",
                            json!({
                                "slotLabel": label,
                                "characterOption": {
                                    "character_id": cid,
                                    "name": cname,
                                },
                            }),
                        )
                        .await;
                    } else if let (Some(tid), Some(tname)) = (trainee_id, trainee_name) {
                        let _ = invoke_tauri_command(
                            "set_legacy_planner_chosen",
                            json!({
                                "traineeId": tid,
                                "characterId": cid,
                                "traineeName": tname,
                                "characterName": cname,
                            }),
                        )
                        .await;
                    }
                    load_state.emit(());
                });
            }
        })
    };

    {
        let load_state = load_state.clone();
        let on_trainee_select_from_browser = on_trainee_select_from_browser.clone();
        use_effect_with((), move |_| {
            let load_state = load_state.clone();
            let on_trainee_select_from_browser = on_trainee_select_from_browser.clone();
            gloo_console::log!("[LegacyPlanner] Setting up event listeners");
            crate::tauri_bridge::listen_to_event("veteran-selected", move |payload| {
                gloo_console::log!(format!(
                    "[LegacyPlanner] Received veteran-selected event: {:?}",
                    payload
                ));
                let hash_str = payload["hash"].as_str().map(|s| s.to_string());
                let slot_label = payload["slot_label"].as_str().map(|s| s.to_string());
                gloo_console::log!(format!(
                    "[LegacyPlanner] Parsed hash: {:?}, slot_label: {:?}",
                    hash_str, slot_label
                ));
                let source = payload["source"]
                    .as_str()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "local".to_string());
                if let (Some(hash), Some(label)) = (hash_str, slot_label) {
                    let load_state = load_state.clone();
                    if source == "uma_moe" {
                        gloo_console::log!(format!(
                            "[LegacyPlanner] Calling set_legacy_planner_slot_uma_moe_veteran with label={}, hash={}",
                            label, hash
                        ));
                        wasm_bindgen_futures::spawn_local(async move {
                            let result = invoke_tauri_command(
                                "set_legacy_planner_slot_uma_moe_veteran",
                                json!({
                                    "slotLabel": label.clone(),
                                    "hash": hash.clone(),
                                }),
                            )
                            .await;
                            match result {
                                Ok(_) => gloo_console::log!(
                                    "[LegacyPlanner] set_legacy_planner_slot_uma_moe_veteran succeeded"
                                ),
                                Err(e) => gloo_console::error!(format!(
                                    "[LegacyPlanner] set_legacy_planner_slot_uma_moe_veteran error: {}",
                                    e
                                )),
                            }
                            load_state.emit(());
                        });
                    } else {
                        gloo_console::log!(format!(
                            "[LegacyPlanner] Calling set_legacy_planner_slot_veteran with label={}, hash={}",
                            label, hash
                        ));
                        wasm_bindgen_futures::spawn_local(async move {
                            let result = invoke_tauri_command(
                                "set_legacy_planner_slot_veteran",
                                json!({
                                    "slotLabel": label.clone(),
                                    "hash": hash.clone(),
                                }),
                            )
                            .await;
                            match result {
                                Ok(_) => gloo_console::log!(
                                    "[LegacyPlanner] set_legacy_planner_slot_veteran succeeded"
                                ),
                                Err(e) => gloo_console::error!(format!(
                                    "[LegacyPlanner] set_legacy_planner_slot_veteran error: {}",
                                    e
                                )),
                            }
                            load_state.emit(());
                        });
                    }
                } else {
                    gloo_console::warn!("[LegacyPlanner] Missing hash or slot_label in event");
                }
            });

            let on_trainee_select_from_browser = on_trainee_select_from_browser.clone();
            crate::tauri_bridge::listen_to_event("trainee-selected", move |payload| {
                gloo_console::log!(format!(
                    "[LegacyPlanner] Received trainee-selected event: {:?}",
                    payload
                ));
                on_trainee_select_from_browser.emit(payload);
            });

            || {}
        });
    }

    let on_clear_trainee = {
        let load_state = load_state.clone();
        Callback::from(move |_: yew::MouseEvent| {
            let load_state = load_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke_tauri_command("clear_legacy_planner_chosen", json!({})).await;
                load_state.emit(());
            });
        })
    };

    let make_on_open_browser = {
        let chosen_id = state.chosen_character.as_ref().map(|c| c.character_id);
        Callback::from(move |slot_label: String| {
            let chosen_id = chosen_id;
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke_tauri_command(
                    "open_veteran_browser",
                    json!({
                        "mode": format!("select_veteran:{}", slot_label),
                        "chosenCharacterId": chosen_id,
                    }),
                )
                .await;
            });
        })
    };

    let make_on_open_api_browser = {
        let chosen_id = state.chosen_character.as_ref().map(|c| c.character_id);
        Callback::from(move |slot_label: String| {
            let chosen_id = chosen_id;
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke_tauri_command(
                    "open_veteran_browser",
                    json!({
                        "mode": format!("select_veteran:{}", slot_label),
                        "source": "uma_moe",
                        "chosenCharacterId": chosen_id,
                    }),
                )
                .await;
            });
        })
    };

    let on_clear_slot = {
        let load_state = load_state.clone();
        Callback::from(move |slot_label: String| {
            let load_state = load_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke_tauri_command(
                    "clear_legacy_planner_slot",
                    json!({"slotLabel": slot_label}),
                )
                .await;
                load_state.emit(());
            });
        })
    };

    let on_set_slot_character = {
        Callback::from(move |slot_label: String| {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke_tauri_command(
                    "open_trainee_browser",
                    json!({
                        "mode": format!("select_trainee_char:{}", slot_label),
                    }),
                )
                .await;
            });
        })
    };

    let individual_spark_groups = {
        let state = state.clone();
        use_memo((state,), |(state,)| {
            let slots = [
                &state.parent_a,
                &state.parent_b,
                &state.grandparent_aa,
                &state.grandparent_ab,
                &state.grandparent_ba,
                &state.grandparent_bb,
            ];
            let mut all = Vec::new();
            for slot in &slots {
                match slot {
                    Some(LegacySlotValue::LegacyUma(u)) => {
                        for sg in &u.spark_groups {
                            if sg.trainee_stars_veteran > 0 {
                                let mut cloned = sg.clone();
                                cloned.total_stars = sg.trainee_stars_veteran;
                                all.push(cloned);
                            }
                        }
                    }
                    Some(LegacySlotValue::ParentUma(u)) => {
                        for sg in &u.spark_groups {
                            if sg.total_stars > 0 {
                                all.push(sg.clone());
                            }
                        }
                    }
                    _ => {}
                }
            }
            all
        })
    };

    let all_spark_groups = {
        let state = state.clone();
        use_memo((state,), |(state,)| {
            let slots = [
                &state.parent_a,
                &state.parent_b,
                &state.grandparent_aa,
                &state.grandparent_ab,
                &state.grandparent_ba,
                &state.grandparent_bb,
            ];
            let mut groups: HashMap<i64, SparkGroupInfo> = HashMap::new();
            for slot in &slots {
                match slot {
                    Some(LegacySlotValue::LegacyUma(u)) => {
                        for sg in &u.spark_groups {
                            if sg.trainee_stars_veteran > 0 {
                                let entry = groups.entry(sg.spark_group_id).or_insert_with(|| {
                                    SparkGroupInfo {
                                        spark_group_id: sg.spark_group_id,
                                        name: sg.name.clone(),
                                        spark_type: sg.spark_type,
                                        total_stars: 0,
                                        trainee_stars_veteran: 0,
                                        uma_count: 0,
                                    }
                                });
                                entry.total_stars += sg.trainee_stars_veteran;
                                entry.uma_count += 1;
                            }
                        }
                    }
                    Some(LegacySlotValue::ParentUma(u)) => {
                        for sg in &u.spark_groups {
                            if sg.total_stars > 0 {
                                let entry = groups.entry(sg.spark_group_id).or_insert_with(|| {
                                    SparkGroupInfo {
                                        spark_group_id: sg.spark_group_id,
                                        name: sg.name.clone(),
                                        spark_type: sg.spark_type,
                                        total_stars: 0,
                                        trainee_stars_veteran: 0,
                                        uma_count: 0,
                                    }
                                });
                                entry.total_stars += sg.total_stars;
                                entry.uma_count += 1;
                            }
                        }
                    }
                    _ => {}
                }
            }
            groups.into_values().collect::<Vec<_>>()
        })
    };

    let spark_pills = {
        let all = (*all_spark_groups).clone();
        use_memo((all,), |(all,)| {
            let mut pills: Vec<SparkGroupInfo> = all
                .iter()
                .filter(|s| s.spark_type == SparkType::Stat || s.spark_type == SparkType::Aptitude)
                .cloned()
                .collect();
            pills.sort_by(|a, b| a.spark_group_id.cmp(&b.spark_group_id));
            pills
        })
    };

    let has_any_slot_filled = {
        let state = state.clone();
        use_memo((state,), |(state,)| {
            state.parent_a.is_some()
                || state.parent_b.is_some()
                || state.grandparent_aa.is_some()
                || state.grandparent_ab.is_some()
                || state.grandparent_ba.is_some()
                || state.grandparent_bb.is_some()
        })
    };

    let on_open_stats_aptitudes = {
        let show = show_stats_aptitudes.clone();
        Callback::from(move |_: yew::MouseEvent| show.set(true))
    };
    let on_close_stats_aptitudes = {
        let show = show_stats_aptitudes.clone();
        Callback::from(move |_| show.set(false))
    };

    let on_open_sparks_list = {
        let show = show_sparks_list.clone();
        Callback::from(move |_: yew::MouseEvent| show.set(true))
    };
    let on_close_sparks_list = {
        let show = show_sparks_list.clone();
        Callback::from(move |_| show.set(false))
    };

    let on_open_white_spark = {
        let show = show_white_spark.clone();
        Callback::from(move |_: yew::MouseEvent| show.set(true))
    };
    let on_close_white_spark = {
        let show = show_white_spark.clone();
        Callback::from(move |_| show.set(false))
    };

    let on_open_inspiration = {
        let show = show_inspiration.clone();
        Callback::from(move |_: yew::MouseEvent| show.set(true))
    };
    let on_close_inspiration = {
        let show = show_inspiration.clone();
        Callback::from(move |_| show.set(false))
    };

    let on_open_trainee_select = {
        Callback::from(move |_: yew::MouseEvent| {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke_tauri_command(
                    "open_trainee_browser",
                    json!({
                        "mode": "select_trainee",
                    }),
                )
                .await;
            });
        })
    };

    let stylesheet = StyleManager::render_stylesheet();

    let has_trainee = state.chosen_character.is_some();
    let affinity_total = affinities.total();
    let has_affinity = affinity_total.base > 0 || affinity_total.bonus > 0;

    html! {
        <div class={LegacyPlannerRootStyle::CLASS_NAME}>
            <div class={PlannerHeaderStyle::CLASS_NAME}>
                <h2>{"Legacy Planner"}</h2>
                <button class={SecondaryBtnStyle::CLASS_NAME} onclick={on_clear_all}>{"Clear All"}</button>
            </div>

            {stylesheet}

            <div class={PlannerTreeGridStyle::CLASS_NAME}>
                // ROW 1: Chosen Trainee centered
                <div class={TreeTraineeStyle::CLASS_NAME} style="grid-column: 2 / span 3;">
                    <div class={PlannerSectionLabelStyle::CLASS_NAME}>{"Chosen Trainee"}</div>
                    <div class={TraineeSelectorRowStyle::CLASS_NAME}>
                        if let Some(chosen) = &state.chosen_character {
                            <span style="font-size: 14px; font-weight: 600; color: #f3f4f6; margin-right: 12px;">
                                {chosen.trainee_name.clone()}
                            </span>
                            <button class={HeaderActionButtonStyle::CLASS_NAME} onclick={on_open_trainee_select.clone()}>{"Replace"}</button>
                            <button class={HeaderActionButtonStyle::CLASS_NAME} onclick={on_clear_trainee}>{"Clear"}</button>
                        } else {
                            <button class={SecondaryBtnStyle::CLASS_NAME} onclick={on_open_trainee_select.clone()}>{"Select Trainee"}</button>
                        }
                    </div>

                    if has_trainee && has_affinity {
                        <div class={LegacyAffinityTotalsBarStyle::CLASS_NAME}>
                            <div class="affinity-bar-header">{"Affinity"}</div>
                            <div class="affinity-bar-content">
                                <div class="affinity-stat">
                                    <span class="affinity-stat-label">{"Base"}</span>
                                    <span class={classes!("affinity-value", "affinity-value-base")}>{affinity_total.base}</span>
                                </div>
                                <div class="affinity-divider"></div>
                                <div class="affinity-stat">
                                    <span class="affinity-stat-label">{"Bonus"}</span>
                                    <span class={classes!("affinity-value", "affinity-value-bonus")}>{affinity_total.bonus}</span>
                                </div>
                                <div class="affinity-divider"></div>
                                <div class="affinity-stat">
                                    <span class="affinity-stat-label">{"Total"}</span>
                                    <span class={classes!("affinity-value", "affinity-value-total")}>{affinity_total.total()}</span>
                                </div>
                            </div>
                        </div>
                    }

                    if has_trainee && has_affinity && !spark_pills.is_empty() {
                        {{
                            let stat_pills: Vec<_> = spark_pills.iter().filter(|s| s.spark_type == SparkType::Stat).collect();
                            let apt_pills: Vec<_> = spark_pills.iter().filter(|s| s.spark_type != SparkType::Stat).collect();
                            let mut rows = Vec::new();
                            if !stat_pills.is_empty() {
                                rows.push(html! {
                                    <div class={LegacySparkPillsRowStyle::CLASS_NAME}>
                                        <div class={classes!("spark-section-header", "blue")}>{"Stat Sparks"}</div>
                                        <div class="spark-pills-list">
                                            {for stat_pills.iter().map(|s| {
                                                html! {
                                                    <span class={format!("{} {}", LegacySparkPillStyle::CLASS_NAME, "spark-blue")}>
                                                        <span class="spark-name">{&s.name}</span>
                                                        <span class="spark-stars">{format!("{}★", s.total_stars)}</span>
                                                    </span>
                                                }
                                            })}
                                        </div>
                                    </div>
                                });
                            }
                            if !apt_pills.is_empty() {
                                rows.push(html! {
                                    <div class={LegacySparkPillsRowStyle::CLASS_NAME}>
                                        <div class={classes!("spark-section-header", "pink")}>{"Aptitude Sparks"}</div>
                                        <div class="spark-pills-list">
                                            {for apt_pills.iter().map(|s| {
                                                html! {
                                                    <span class={format!("{} {}", LegacySparkPillStyle::CLASS_NAME, "spark-pink")}>
                                                        <span class="spark-name">{&s.name}</span>
                                                        <span class="spark-stars">{format!("{}★", s.total_stars)}</span>
                                                    </span>
                                                }
                                            })}
                                        </div>
                                    </div>
                                });
                            }
                            rows
                        }}
                    }

                    if has_trainee {
                        <div style="margin-top: 12px;">
                            <div class={PlannerSectionLabelStyle::CLASS_NAME}>{"Details"}</div>
                            <div style="display: flex; flex-wrap: wrap; gap: 6px; padding: 4px 0 4px;">
                            <button class={HeaderActionButtonStyle::CLASS_NAME} onclick={on_open_stats_aptitudes}>
                                {"Stats + Aptitudes"}
                            </button>
                            if *has_any_slot_filled {
                                <button class={HeaderActionButtonStyle::CLASS_NAME} onclick={on_open_sparks_list}>
                                    {"Sparks"}
                                </button>
                            }
                            if *has_any_slot_filled {
                                <button class={HeaderActionButtonStyle::CLASS_NAME} onclick={on_open_white_spark}>
                                    {"White Spark Generating Chance"}
                                </button>
                            }
                            if *has_any_slot_filled {
                                <button class={HeaderActionButtonStyle::CLASS_NAME} onclick={on_open_inspiration}>
                                    {"Inspiration Spark Chance"}
                                </button>
                            }
                            </div>
                        </div>
                    }
                </div>

                // ROW 2: Trainee ↔ Parent affinities
                <div class={TreeAffinityStyle::CLASS_NAME} style="grid-column: 2;">
                    {render_affinity_result(&affinities.trainee_parent_a)}
                </div>
                <div class={TreeAffinityStyle::CLASS_NAME} style="grid-column: 4;">
                    {render_affinity_result(&affinities.trainee_parent_b)}
                </div>

                // ROW 3: Parent A, center affinity, Parent B
                <div class={PlannerSlotStyle::CLASS_NAME} style="grid-column: 1 / span 2;">
                    {render_veteran_slot(
                        LegacyPlannerSlot::ParentA,
                        slot_value(&state, LegacyPlannerSlot::ParentA).cloned(),
                        &make_on_open_browser,
                        &make_on_open_api_browser,
                        &on_clear_slot,
                        &on_set_slot_character,
                    )}
                </div>
                <div class={classes!(TreeAffinityStyle::CLASS_NAME, TreeAffinityCenterStyle::CLASS_NAME)} style="grid-column: 3;">
                    {render_affinity_result(&affinities.parent_a_parent_b)}
                </div>
                <div class={PlannerSlotStyle::CLASS_NAME} style="grid-column: 4 / span 2;">
                    {render_veteran_slot(
                        LegacyPlannerSlot::ParentB,
                        slot_value(&state, LegacyPlannerSlot::ParentB).cloned(),
                        &make_on_open_browser,
                        &make_on_open_api_browser,
                        &on_clear_slot,
                        &on_set_slot_character,
                    )}
                </div>

                // ROW 4: Parent ↔ Grandparent affinities
                <div class={TreeAffinityStyle::CLASS_NAME} style="grid-column: 1;">
                    {render_affinity_result(&affinities.parent_a_grandparent_aa)}
                </div>
                <div class={TreeAffinityStyle::CLASS_NAME} style="grid-column: 2;">
                    {render_affinity_result(&affinities.parent_a_grandparent_ab)}
                </div>
                <div class={TreeAffinityStyle::CLASS_NAME} style="grid-column: 4;">
                    {render_affinity_result(&affinities.parent_b_grandparent_ba)}
                </div>
                <div class={TreeAffinityStyle::CLASS_NAME} style="grid-column: 5;">
                    {render_affinity_result(&affinities.parent_b_grandparent_bb)}
                </div>

                // ROW 5: Grandparent slots
                <div class={PlannerSlotStyle::CLASS_NAME} style="grid-column: 1;">
                    {render_veteran_slot(
                        LegacyPlannerSlot::GrandparentAA,
                        slot_value(&state, LegacyPlannerSlot::GrandparentAA).cloned(),
                        &make_on_open_browser,
                        &make_on_open_api_browser,
                        &on_clear_slot,
                        &on_set_slot_character,
                    )}
                </div>
                <div class={PlannerSlotStyle::CLASS_NAME} style="grid-column: 2;">
                    {render_veteran_slot(
                        LegacyPlannerSlot::GrandparentAB,
                        slot_value(&state, LegacyPlannerSlot::GrandparentAB).cloned(),
                        &make_on_open_browser,
                        &make_on_open_api_browser,
                        &on_clear_slot,
                        &on_set_slot_character,
                    )}
                </div>
                <div class={PlannerSlotStyle::CLASS_NAME} style="grid-column: 4;">
                    {render_veteran_slot(
                        LegacyPlannerSlot::GrandparentBA,
                        slot_value(&state, LegacyPlannerSlot::GrandparentBA).cloned(),
                        &make_on_open_browser,
                        &make_on_open_api_browser,
                        &on_clear_slot,
                        &on_set_slot_character,
                    )}
                </div>
                <div class={PlannerSlotStyle::CLASS_NAME} style="grid-column: 5;">
                    {render_veteran_slot(
                        LegacyPlannerSlot::GrandparentBB,
                        slot_value(&state, LegacyPlannerSlot::GrandparentBB).cloned(),
                        &make_on_open_browser,
                        &make_on_open_api_browser,
                        &on_clear_slot,
                        &on_set_slot_character,
                    )}
                </div>
            </div>

            if *show_stats_aptitudes {
                if let Some(chosen) = &state.chosen_character {
                    <StatsAptitudesModal
                        trainee_id={chosen.trainee_id}
                        individual_spark_groups={(*individual_spark_groups).clone()}
                        on_close={on_close_stats_aptitudes}
                    />
                }
            }

            if *show_sparks_list {
                <SparksListModal
                    all_spark_groups={(*all_spark_groups).clone()}
                    on_close={on_close_sparks_list}
                />
            }

            if *show_white_spark {
                <WhiteSparkChanceModal on_close={on_close_white_spark} />
            }

            if *show_inspiration {
                <InspirationChanceModal on_close={on_close_inspiration} />
            }
        </div>
    }
}

fn render_veteran_slot(
    slot: LegacyPlannerSlot,
    value: Option<LegacySlotValue>,
    on_open_browser: &Callback<String>,
    on_open_api_browser: &Callback<String>,
    on_clear_slot: &Callback<String>,
    on_open_char_select: &Callback<String>,
) -> Html {
    let slot_label = slot.label().to_string();
    let is_locked = matches!(&value, Some(LegacySlotValue::ParentUma(_)));

    let on_select_slot_veteran = if is_locked {
        None
    } else {
        let on_open_browser = on_open_browser.clone();
        let slot_label = slot_label.clone();
        Some(Callback::from(move |_: yew::MouseEvent| {
            on_open_browser.emit(slot_label.clone());
        }))
    };

    let on_select_slot_veteran_api = if is_locked {
        None
    } else {
        let on_open_api_browser = on_open_api_browser.clone();
        let slot_label = slot_label.clone();
        Some(Callback::from(move |_: yew::MouseEvent| {
            on_open_api_browser.emit(slot_label.clone());
        }))
    };

    let can_clear = value.is_some() && !is_locked;
    let clear_disabled_title = if is_locked {
        Some("Locked — clear parent to remove".to_string())
    } else if value.is_none() {
        Some("No veteran selected".to_string())
    } else {
        None
    };

    let on_clear = {
        let on_clear_slot = on_clear_slot.clone();
        let slot_label = slot_label.clone();
        Callback::from(move |_: yew::MouseEvent| {
            on_clear_slot.emit(slot_label.clone());
        })
    };

    html! {
        <LegacyVeteranSlot
            title={slot_label}
            slot_type={slot}
            selected={value.clone()}
            on_select_slot_veteran={on_select_slot_veteran}
            on_select_slot_veteran_api={on_select_slot_veteran_api}
            on_clear={on_clear}
            can_clear={can_clear}
            clear_disabled_title={clear_disabled_title}
            on_open_char_select={on_open_char_select.clone()}
        />
    }
}

fn slot_value<'a>(
    state: &'a LegacyPlannerState,
    slot: LegacyPlannerSlot,
) -> Option<&'a LegacySlotValue> {
    match slot {
        LegacyPlannerSlot::ParentA => state.parent_a.as_ref(),
        LegacyPlannerSlot::ParentB => state.parent_b.as_ref(),
        LegacyPlannerSlot::GrandparentAA => state.grandparent_aa.as_ref(),
        LegacyPlannerSlot::GrandparentAB => state.grandparent_ab.as_ref(),
        LegacyPlannerSlot::GrandparentBA => state.grandparent_ba.as_ref(),
        LegacyPlannerSlot::GrandparentBB => state.grandparent_bb.as_ref(),
    }
}

fn render_affinity_result(value: &Option<AffinityResult>) -> Html {
    match value {
        Some(aff) => {
            let has_bonus = aff.bonus > 0;
            html! {
                <div class={TreeAffinityBoxStyle::CLASS_NAME}>
                    if has_bonus {
                        <span class={AffinityBaseStyle::CLASS_NAME}>{aff.base}</span>
                        <span class={AffinityPlusStyle::CLASS_NAME}>{"+"}</span>
                        <span class={AffinityBonusStyle::CLASS_NAME}>{aff.bonus}</span>
                    } else {
                        <span class={AffinityBaseStyle::CLASS_NAME}>{aff.total()}</span>
                    }
                </div>
            }
        }
        None => {
            html! {
                <div
                    class={classes!(TreeAffinityBoxStyle::CLASS_NAME, TreeAffinityEmptyStyle::CLASS_NAME)}
                >
                    <span class={AffinityBaseStyle::CLASS_NAME}>{"-"}</span>
                </div>
            }
        }
    }
}
