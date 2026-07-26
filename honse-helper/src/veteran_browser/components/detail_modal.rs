use crate::styles::{
    detail_modal::*,
    skill_pill::*,
    tag_modal::{TagPillListStyle, TagPillRemoveStyle, TagPillStyle},
    veteran_card::{
        CardFooterStyle, CardHashStyle, CardHeaderStyle, CardMetaStyle, CardNameStyle,
        CardRankStyle, CardSparksStyle, CardStatsRowStyle, IndepTrainBadgeStyle,
        OwnerIdBadgeStyle, OwnerIdPrefixStyle, StatLabelStyle, StatValueStyle,
        VeteranVariantStyle,
    },
    Style,
};
use crate::support_card_browser::components::support_card_card::parse_card_name;
use crate::veteran_browser::components::skill_detail_modal::SkillDetailModal;
use crate::veteran_browser::components::skill_pill::SkillPill;
use shared::models::{INDEPENDENT_LEARNER_NICKNAME, UmaRank};
use shared::veteran_browser::{
    MajorWinRow, ParentRow, SparkGroupRow, TagRow, VeteranRow, VeteranSkillRow,
    VeteranSupportCardRow,
};
use yew::prelude::*;

use crate::components::tag_modal::TagModal;
use crate::tauri_bridge::invoke_tauri_command;
use serde_json::json;

use super::rank_badge::RankBadge;
use super::spark_item::SparkItem;
use super::veteran_card::parse_veteran_name;
use crate::components::wins_list::WinsList;

#[derive(Properties, PartialEq)]
pub struct DetailModalProps {
    pub veteran: Option<VeteranRow>,
    pub sparks: Vec<SparkGroupRow>,
    pub wins: Vec<MajorWinRow>,
    pub parents: Vec<ParentRow>,
    pub loading: bool,
    pub on_close: Callback<()>,
    pub on_refresh: Callback<()>,
    #[prop_or(false)]
    pub api_mode: bool,
    #[prop_or_default]
    pub skills: Vec<VeteranSkillRow>,
    #[prop_or_default]
    pub support_cards: Vec<VeteranSupportCardRow>,
}

#[derive(PartialEq, Clone, Copy)]
enum Tab {
    Stats,
    Sparks,
    Parents,
    Wins,
    Skills,
    SupportCards,
    Tags,
}

fn skill_category_label(cat: Option<i64>) -> (&'static str, &'static str) {
    match cat {
        Some(0) => ("Passive", "#22c55e"),
        Some(1) => ("Early race", "#3b82f6"),
        Some(2) => ("Mid race", "#eab308"),
        Some(3) => ("Late race", "#ef4444"),
        Some(4) => ("Anytime", "#f97316"),
        Some(5) => ("Unique", "#ec4899"),
        _ => ("Other", "#6b7280"),
    }
}

fn render_skills_tab(skills: &[VeteranSkillRow], on_skill_click: Callback<(i64, i64)>) -> Html {
    let mut grouped: Vec<(i64, Vec<&VeteranSkillRow>)> = {
        let mut map: std::collections::BTreeMap<i64, Vec<&VeteranSkillRow>> =
            std::collections::BTreeMap::new();
        for s in skills {
            let key = s.category.unwrap_or(0);
            map.entry(key).or_default().push(s);
        }
        map.into_iter().collect()
    };
    grouped.sort_by_key(|(cat, _)| if *cat == 5 { -1 } else { *cat });
    for (_, rows) in &mut grouped {
        rows.sort_by_key(|s| s.skill_id);
    }

    html! {
        <div class={DetailTabStyle::CLASS_NAME}>
            if grouped.is_empty() {
                <p>{"No skills data."}</p>
            } else {
                <div class={SkillPillListStyle::CLASS_NAME}>
                    { for grouped.iter().map(|(cat, rows)| {
                        let (label, color) = skill_category_label(Some(*cat));
                        html! {
                            <div class={SkillPillGroupStyle::CLASS_NAME}>
                                <div class={SkillPillCategoryStyle::CLASS_NAME} style={format!("color: {}", color)}>
                                    { label }
                                </div>
                                { for rows.iter().map(|s| {
                                    let on_click = {
                                        let cb = on_skill_click.clone();
                                        let skill_id = s.skill_id;
                                        let level = s.level;
                                        Callback::from(move |_| cb.emit((skill_id, level)))
                                    };
                                    html! {
                                        <SkillPill
                                            skill_id={s.skill_id}
                                            name={s.name.clone()}
                                            level={s.level}
                                            skill_type={s.skill_type.clone()}
                                            rarity={s.rarity}
                                            on_click={on_click}
                                        />
                                    }
                                })}
                            </div>
                        }
                    }) }
                </div>
            }
        </div>
    }
}

fn render_parent_detail(
    selected_parent: &Option<ParentRow>,
    parent_sparks: &[SparkGroupRow],
    parent_wins: &[MajorWinRow],
    parent_loading: bool,
    on_close: &Callback<()>,
) -> Html {
    let Some(sp) = selected_parent else {
        return html! {};
    };
    let pname = sp.trainee_name.as_deref().unwrap_or("Unknown").to_string();
    let parent_spark_groups: Vec<Vec<SparkGroupRow>> = {
        let blue = parent_sparks
            .iter()
            .filter(|s| s.spark_type == 1)
            .cloned()
            .collect::<Vec<_>>();
        let pink = parent_sparks
            .iter()
            .filter(|s| s.spark_type == 2)
            .cloned()
            .collect::<Vec<_>>();
        let green = parent_sparks
            .iter()
            .filter(|s| s.spark_type == 3)
            .cloned()
            .collect::<Vec<_>>();
        let other = parent_sparks
            .iter()
            .filter(|s| s.spark_type != 1 && s.spark_type != 2 && s.spark_type != 3)
            .cloned()
            .collect::<Vec<_>>();
        vec![blue, pink, green, other]
    };
    let mut sorted_parent_wins = parent_wins.to_vec();
    sorted_parent_wins.sort_by(|a, b| a.priority.cmp(&b.priority));

    let overlay_click = {
        let cb = on_close.clone();
        Callback::from(move |_| cb.emit(()))
    };
    let close_click = {
        let cb = on_close.clone();
        Callback::from(move |_| cb.emit(()))
    };

    html! {
        <div class={ParentDetailOverlayStyle::CLASS_NAME} onclick={overlay_click}>
            <div class={ParentDetailContentStyle::CLASS_NAME} onclick={|e: yew::MouseEvent| e.stop_propagation()}>
                <div class={ModalHeaderStyle::CLASS_NAME}>
                    <h2>{ pname }</h2>
                    <button class={ModalCloseStyle::CLASS_NAME} onclick={close_click}>{"\u{00D7}"}</button>
                </div>
                <div class={ModalBodyStyle::CLASS_NAME}>
                    if parent_loading {
                        <p>{"Loading..."}</p>
                    } else {
                        if parent_sparks.is_empty() {
                            <p><em>{"No spark data."}</em></p>
                        } else {
                            <h3>{"Sparks"}</h3>
                            <div class={SparkDetailListStyle::CLASS_NAME}>
                                { for parent_spark_groups.iter().filter(|g| !g.is_empty()).map(|group| {
                                    html! {
                                        <div class={SparkColorRowStyle::CLASS_NAME}>
                                            { for group.iter().map(|s| html! { <SparkItem spark={s.clone()} /> }) }
                                        </div>
                                    }
                                })}
                            </div>
                        }
                            <h3>{"Major Wins"}</h3>
                            <WinsList wins={sorted_parent_wins} />
                    }
                </div>
            </div>
        </div>
    }
}

#[function_component]
pub fn DetailModal(props: &DetailModalProps) -> Html {
    let show_stats_tab = props.veteran.as_ref().map_or(false, |v| {
        v.stat_speed.is_some()
            || v.stat_stamina.is_some()
            || v.stat_power.is_some()
            || v.stat_guts.is_some()
            || v.stat_wit.is_some()
            || v.aptitude_turf.is_some()
            || v.aptitude_dirt.is_some()
            || v.aptitude_sprint.is_some()
            || v.aptitude_mile.is_some()
            || v.aptitude_medium.is_some()
            || v.aptitude_long.is_some()
            || v.aptitude_front.is_some()
            || v.aptitude_pace_chaser.is_some()
            || v.aptitude_late_surger.is_some()
            || v.aptitude_end_closer.is_some()
    });
    let show_skills_tab = !props.skills.is_empty();
    let show_support_cards_tab = !props.support_cards.is_empty();
    let tab = use_state(|| {
        if show_stats_tab {
            Tab::Stats
        } else {
            Tab::Sparks
        }
    });

    {
        let tab = tab.clone();
        let show = show_stats_tab;
        let api = props.api_mode;
        let show_skills = show_skills_tab;
        let show_sc = show_support_cards_tab;
        use_effect_with(
            (*tab, show, api, show_skills, show_sc),
            move |(current, show, api, show_skills, show_sc)| {
                if *current == Tab::Stats && !(*show) {
                    tab.set(Tab::Sparks);
                }
                if *api && *current == Tab::Tags {
                    tab.set(Tab::Sparks);
                }
                if *current == Tab::Skills && !(*show_skills) {
                    tab.set(Tab::Sparks);
                }
                if *current == Tab::SupportCards && !(*show_sc) {
                    tab.set(Tab::Sparks);
                }
                || {}
            },
        );
    }

    let selected_parent = use_state(|| None::<ParentRow>);
    let parent_sparks = use_state(Vec::<SparkGroupRow>::new);
    let parent_wins = use_state(Vec::<MajorWinRow>::new);
    let parent_loading = use_state(|| false);
    let parent_hash_copied = use_state(|| None::<i64>);
    let selected_skill = use_state(|| None::<(i64, i64)>);
    let parent_owner_copied = use_state(|| false);

    let tag_modal_open = use_state(|| false);
    let veteran_tags = use_state(Vec::<TagRow>::new);
    let tag_search_results = use_state(Vec::<TagRow>::new);

    // Load veteran tags when Tags tab becomes active
    {
        let tab = tab.clone();
        let veteran_tags = veteran_tags.clone();
        let veteran = props.veteran.clone();
        use_effect_with((*tab).clone(), move |current_tab| {
            if *current_tab == Tab::Tags {
                if let Some(v) = &veteran {
                    let hash = v.hash;
                    let veteran_tags = veteran_tags.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Ok(result) = invoke_tauri_command(
                            "get_veteran_tags",
                            json!({"veteranHash": hash.to_string()}),
                        )
                        .await
                        {
                            if let Ok(tags) = serde_json::from_value::<Vec<TagRow>>(result) {
                                veteran_tags.set(tags);
                            }
                        }
                    });
                }
            }
            || {}
        });
    }

    let on_close = {
        let cb = props.on_close.clone();
        Callback::from(move |_| cb.emit(()))
    };

    let on_open_tag_modal = {
        let tag_modal_open = tag_modal_open.clone();
        let veteran_tags = veteran_tags.clone();
        let tag_search_results = tag_search_results.clone();
        let veteran = props.veteran.clone();
        Callback::from(move |_| {
            if let Some(v) = &veteran {
                let hash = v.hash;
                let veteran_tags = veteran_tags.clone();
                let tag_search_results = tag_search_results.clone();
                let tag_modal_open = tag_modal_open.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(result) = invoke_tauri_command(
                        "get_veteran_tags",
                        json!({"veteranHash": hash.to_string()}),
                    )
                    .await
                    {
                        if let Ok(tags) = serde_json::from_value::<Vec<TagRow>>(result) {
                            veteran_tags.set(tags);
                        }
                    }
                    tag_search_results.set(Vec::new());
                    tag_modal_open.set(true);
                });
            }
        })
    };

    let on_close_tag_modal = {
        let tag_modal_open = tag_modal_open.clone();
        Callback::from(move |_| tag_modal_open.set(false))
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

    let veteran = props.veteran.clone();

    let on_tag_save = {
        let veteran_tags = veteran_tags.clone();
        let veteran = veteran.clone();
        let on_refresh = props.on_refresh.clone();
        Callback::from(move |saved_tags: Vec<TagRow>| {
            if let Some(v) = &veteran {
                let hash = v.hash;
                let current = (*veteran_tags).clone();
                let veteran_tags = veteran_tags.clone();
                let on_refresh = on_refresh.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    for tag in &current {
                        if !saved_tags.iter().any(|t| t.id == tag.id) {
                            let _ = invoke_tauri_command(
                                "untag_veteran",
                                json!({"tagId": tag.id, "veteranHash": hash.to_string()}),
                            )
                            .await;
                        }
                    }
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
                                    let _ = invoke_tauri_command("tag_veteran", json!({"tagId": new_tag.id, "veteranHash": hash.to_string()})).await;
                                }
                            }
                        } else {
                            let _ = invoke_tauri_command(
                                "tag_veteran",
                                json!({"tagId": tag.id, "veteranHash": hash.to_string()}),
                            )
                            .await;
                        }
                    }
                    if let Ok(result) = invoke_tauri_command(
                        "get_veteran_tags",
                        json!({"veteranHash": hash.to_string()}),
                    )
                    .await
                    {
                        if let Ok(tags) = serde_json::from_value::<Vec<TagRow>>(result) {
                            veteran_tags.set(tags);
                        }
                    }
                    on_refresh.emit(());
                });
            }
        })
    };

    let on_remove_tag = {
        let veteran_tags = veteran_tags.clone();
        let veteran = veteran.clone();
        let on_refresh = props.on_refresh.clone();
        Callback::from(move |tag_id: i64| {
            if let Some(v) = &veteran {
                let hash = v.hash;
                let veteran_tags = veteran_tags.clone();
                let on_refresh = on_refresh.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let _ = invoke_tauri_command(
                        "untag_veteran",
                        json!({"tagId": tag_id, "veteranHash": hash.to_string()}),
                    )
                    .await;
                    if let Ok(result) = invoke_tauri_command(
                        "get_veteran_tags",
                        json!({"veteranHash": hash.to_string()}),
                    )
                    .await
                    {
                        if let Ok(tags) = serde_json::from_value::<Vec<TagRow>>(result) {
                            veteran_tags.set(tags);
                        }
                    }
                    on_refresh.emit(());
                });
            }
        })
    };

    let on_tab_click = {
        let tab = tab.clone();
        Callback::from(move |t: Tab| tab.set(t))
    };

    let on_parent_click = {
        let selected_parent = selected_parent.clone();
        let parent_sparks = parent_sparks.clone();
        let parent_wins = parent_wins.clone();
        let parent_loading = parent_loading.clone();
        let api_mode = props.api_mode;
        Callback::from(move |p: ParentRow| {
            let hash = p.hash.to_string();
            let selected_parent = selected_parent.clone();
            let parent_sparks = parent_sparks.clone();
            let parent_wins = parent_wins.clone();
            let parent_loading = parent_loading.clone();
            selected_parent.set(Some(p));
            parent_loading.set(true);
            let h1 = hash.clone();
            let sparks_cmd = if api_mode {
                "get_uma_moe_parent_sparks"
            } else {
                "get_parent_sparks"
            };
            let wins_cmd = if api_mode {
                "get_uma_moe_parent_wins"
            } else {
                "get_parent_wins"
            };
            wasm_bindgen_futures::spawn_local(async move {
                let sparks_fut = invoke_tauri_command(sparks_cmd, json!({"hash": h1}));
                let wins_fut = invoke_tauri_command(wins_cmd, json!({"hash": hash}));

                if let Ok(result) = sparks_fut.await {
                    if let Ok(s) = serde_json::from_value::<Vec<SparkGroupRow>>(result) {
                        parent_sparks.set(s);
                    }
                }
                if let Ok(result) = wins_fut.await {
                    if let Ok(w) = serde_json::from_value::<Vec<MajorWinRow>>(result) {
                        parent_wins.set(w);
                    }
                }
                parent_loading.set(false);
            });
        })
    };

    let close_parent_detail = {
        let selected_parent = selected_parent.clone();
        Callback::from(move |_| selected_parent.set(None))
    };

    let Some(v) = &props.veteran else {
        return html! { <div class={ModalOverlayStyle::CLASS_NAME}><div class={ModalContentStyle::CLASS_NAME}><p>{"Loading..."}</p></div></div> };
    };

    let (variant, character_name) = v.trainee_name.as_deref().map(parse_veteran_name).unwrap_or((None, "Unknown"));

    let has_any_stat = v.stat_speed.is_some()
        || v.stat_stamina.is_some()
        || v.stat_power.is_some()
        || v.stat_guts.is_some()
        || v.stat_wit.is_some();
    let has_any_aptitude = v.aptitude_turf.is_some()
        || v.aptitude_dirt.is_some()
        || v.aptitude_sprint.is_some()
        || v.aptitude_mile.is_some()
        || v.aptitude_medium.is_some()
        || v.aptitude_long.is_some()
        || v.aptitude_front.is_some()
        || v.aptitude_pace_chaser.is_some()
        || v.aptitude_late_surger.is_some()
        || v.aptitude_end_closer.is_some();

    let aptitude_level = |val: i64| -> &'static str {
        match val {
            8 => "S",
            7 => "A",
            6 => "B",
            5 => "C",
            4 => "D",
            3 => "E",
            2 => "F",
            1 => "G",
            _ => "H",
        }
    };

    let aptitude_style = |val: i64| -> (&'static str, &'static str) {
        match val {
            8 => ("#7c3aed", "800"),
            7 => ("#f59e0b", "700"),
            6 => ("#3b82f6", "700"),
            5 => ("#22c55e", "700"),
            4 => ("#a3e635", "700"),
            3 => ("#fb923c", "700"),
            2 => ("#fca5a5", "700"),
            _ => ("#e2e8f0", "700"),
        }
    };

    let mut sorted_wins = props.wins.clone();
    sorted_wins.sort_by(|a, b| {
        a.priority
            .cmp(&b.priority)
            .then(a.on_veteran.cmp(&b.on_veteran))
            .then(a.name.cmp(&b.name))
    });

    let spark_groups: Vec<Vec<SparkGroupRow>> = {
        let blue = props
            .sparks
            .iter()
            .filter(|s| s.spark_type == 1)
            .cloned()
            .collect::<Vec<_>>();
        let pink = props
            .sparks
            .iter()
            .filter(|s| s.spark_type == 2)
            .cloned()
            .collect::<Vec<_>>();
        let green = props
            .sparks
            .iter()
            .filter(|s| s.spark_type == 3)
            .cloned()
            .collect::<Vec<_>>();
        let white = props
            .sparks
            .iter()
            .filter(|s| s.spark_type != 1 && s.spark_type != 2 && s.spark_type != 3)
            .cloned()
            .collect::<Vec<_>>();
        vec![blue, pink, green, white]
    };

    html! {
        <div class={ModalOverlayStyle::CLASS_NAME} onclick={on_close.clone()}>
            <div class={ModalContentStyle::CLASS_NAME} onclick={|e: yew::MouseEvent| e.stop_propagation()}>
                <div class={ModalHeaderStyle::CLASS_NAME}>
                    <div>
                        {if let Some(v) = &variant {
                            html! { <div class={VeteranVariantStyle::CLASS_NAME}>{v}</div> }
                        } else { html! {} }}
                        <h2>{ character_name }</h2>
                    </div>
                    <div style="display:flex;align-items:center;gap:12px;">
                        { if v.nickname_id == Some(INDEPENDENT_LEARNER_NICKNAME) {
                            html! { <span class={IndepTrainBadgeStyle::CLASS_NAME}>{"Indep. Training"}</span> }
                        } else { html! {} } }
                        <button class={ModalCloseStyle::CLASS_NAME} onclick={on_close}>{"\u{00D7}"}</button>
                    </div>
                </div>

                <div class={ModalTabsStyle::CLASS_NAME}>
                    { if show_stats_tab {
                        let is_stats = *tab == Tab::Stats;
                        html! { <button class={classes!(TabBtnStyle::CLASS_NAME, is_stats.then_some(TabActiveStyle::CLASS_NAME))}
                            onclick={let t = on_tab_click.clone(); Callback::from(move |_| t.emit(Tab::Stats))}>
                            {"Stats"}
                        </button> }
                    } else { html! {} } }
                    {{
                        let is_sparks = *tab == Tab::Sparks;
                        html! { <button class={classes!(TabBtnStyle::CLASS_NAME, is_sparks.then_some(TabActiveStyle::CLASS_NAME))}
                            onclick={let t = on_tab_click.clone(); Callback::from(move |_| t.emit(Tab::Sparks))}>
                            {"Sparks"}
                        </button> }
                    }}
                    {{
                        let is_parents = *tab == Tab::Parents;
                        html! { <button class={classes!(TabBtnStyle::CLASS_NAME, is_parents.then_some(TabActiveStyle::CLASS_NAME))}
                            onclick={let t = on_tab_click.clone(); Callback::from(move |_| t.emit(Tab::Parents))}>
                            {"Parents"}
                        </button> }
                    }}
                    {{
                        let is_wins = *tab == Tab::Wins;
                        html! { <button class={classes!(TabBtnStyle::CLASS_NAME, is_wins.then_some(TabActiveStyle::CLASS_NAME))}
                            onclick={let t = on_tab_click.clone(); Callback::from(move |_| t.emit(Tab::Wins))}>
                            {"Major Wins"}
                        </button> }
                    }}
                    { if show_skills_tab {
                        let is_skills = *tab == Tab::Skills;
                        html! { <button class={classes!(TabBtnStyle::CLASS_NAME, is_skills.then_some(TabActiveStyle::CLASS_NAME))}
                            onclick={let t = on_tab_click.clone(); Callback::from(move |_| t.emit(Tab::Skills))}>
                            {"Skills"}
                        </button> }
                    } else { html! {} } }
                     { if show_support_cards_tab {
                        let is_sc = *tab == Tab::SupportCards;
                        html! { <button class={classes!(TabBtnStyle::CLASS_NAME, is_sc.then_some(TabActiveStyle::CLASS_NAME))}
                            onclick={let t = on_tab_click.clone(); Callback::from(move |_| t.emit(Tab::SupportCards))}>
                            {"Support Cards"}
                        </button> }
                    } else { html! {} } }
                     { if !props.api_mode {
                        let is_tags = *tab == Tab::Tags;
                        html! { <button class={classes!(TabBtnStyle::CLASS_NAME, is_tags.then_some(TabActiveStyle::CLASS_NAME))}
                            onclick={let t = on_tab_click.clone(); Callback::from(move |_| t.emit(Tab::Tags))}>
                            {"Tags"}
                        </button> }
                    } else { html! {} } }
                </div>

                <div class={ModalBodyStyle::CLASS_NAME}>
                    { match *tab {
                        Tab::Stats => html! {
                            <div class={DetailTabStyle::CLASS_NAME}>
                                { if has_any_stat {
                                    html! { <>
                                        <div style="display: grid; grid-template-columns: repeat(5, 1fr); gap: 8px; margin-top: 8px;">
                                            { v.stat_speed.map(|val| html! {
                                                <div style="background: #1e293b; border-top: 3px solid #06b6d4; border-radius: 6px; padding: 8px; text-align: center;">
                                                    <div style="font-size: 10px; text-transform: uppercase; color: #64748b; letter-spacing: 0.5px; margin-bottom: 4px;">{"SPD"}</div>
                                                    <div style="font-size: 16px; font-weight: 700; color: #f3f4f6;">{ val }</div>
                                                </div>
                                            }) }
                                            { v.stat_stamina.map(|val| html! {
                                                <div style="background: #1e293b; border-top: 3px solid #f59e0b; border-radius: 6px; padding: 8px; text-align: center;">
                                                    <div style="font-size: 10px; text-transform: uppercase; color: #64748b; letter-spacing: 0.5px; margin-bottom: 4px;">{"STA"}</div>
                                                    <div style="font-size: 16px; font-weight: 700; color: #f3f4f6;">{ val }</div>
                                                </div>
                                            }) }
                                            { v.stat_power.map(|val| html! {
                                                <div style="background: #1e293b; border-top: 3px solid #ef4444; border-radius: 6px; padding: 8px; text-align: center;">
                                                    <div style="font-size: 10px; text-transform: uppercase; color: #64748b; letter-spacing: 0.5px; margin-bottom: 4px;">{"PWR"}</div>
                                                    <div style="font-size: 16px; font-weight: 700; color: #f3f4f6;">{ val }</div>
                                                </div>
                                            }) }
                                            { v.stat_guts.map(|val| html! {
                                                <div style="background: #1e293b; border-top: 3px solid #f97316; border-radius: 6px; padding: 8px; text-align: center;">
                                                    <div style="font-size: 10px; text-transform: uppercase; color: #64748b; letter-spacing: 0.5px; margin-bottom: 4px;">{"GUT"}</div>
                                                    <div style="font-size: 16px; font-weight: 700; color: #f3f4f6;">{ val }</div>
                                                </div>
                                            }) }
                                            { v.stat_wit.map(|val| html! {
                                                <div style="background: #1e293b; border-top: 3px solid #22c55e; border-radius: 6px; padding: 8px; text-align: center;">
                                                    <div style="font-size: 10px; text-transform: uppercase; color: #64748b; letter-spacing: 0.5px; margin-bottom: 4px;">{"WIT"}</div>
                                                    <div style="font-size: 16px; font-weight: 700; color: #f3f4f6;">{ val }</div>
                                                </div>
                                            }) }
                                        </div>
                                    </> }
                                } else { html! {} } }
                                { if has_any_aptitude {
                                    html! { <>
                                        <div style="margin-top: 16px;">
                                            <div style="font-size: 11px; text-transform: uppercase; color: #64748b; letter-spacing: 0.5px; margin-bottom: 6px;">{"Ground"}</div>
                                            <div style="display: flex; flex-wrap: wrap; gap: 6px;">
                                                { v.aptitude_turf.map(|val| {
                                                    let (color, weight) = aptitude_style(val);
                                                    html! {
                                                        <div style={format!("background: #1e293b; border-top: 3px solid {}; border-radius: 6px; padding: 6px 12px; text-align: center; min-width: 70px;", color)}>
                                                            <div style="font-size: 10px; text-transform: uppercase; color: #64748b; margin-bottom: 2px;">{"Turf"}</div>
                                                            <div style={format!("font-size: 18px; font-weight: {}; color: {};", weight, color)}>{ aptitude_level(val) }</div>
                                                        </div>
                                                    }
                                                }) }
                                                { v.aptitude_dirt.map(|val| {
                                                    let (color, weight) = aptitude_style(val);
                                                    html! {
                                                        <div style={format!("background: #1e293b; border-top: 3px solid {}; border-radius: 6px; padding: 6px 12px; text-align: center; min-width: 70px;", color)}>
                                                            <div style="font-size: 10px; text-transform: uppercase; color: #64748b; margin-bottom: 2px;">{"Dirt"}</div>
                                                            <div style={format!("font-size: 18px; font-weight: {}; color: {};", weight, color)}>{ aptitude_level(val) }</div>
                                                        </div>
                                                    }
                                                }) }
                                            </div>
                                        </div>
                                        <div style="margin-top: 12px;">
                                            <div style="font-size: 11px; text-transform: uppercase; color: #64748b; letter-spacing: 0.5px; margin-bottom: 6px;">{"Distance"}</div>
                                            <div style="display: flex; flex-wrap: wrap; gap: 6px;">
                                                { v.aptitude_sprint.map(|val| {
                                                    let (color, weight) = aptitude_style(val);
                                                    html! {
                                                        <div style={format!("background: #1e293b; border-top: 3px solid {}; border-radius: 6px; padding: 6px 12px; text-align: center; min-width: 70px;", color)}>
                                                            <div style="font-size: 10px; text-transform: uppercase; color: #64748b; margin-bottom: 2px;">{"Sprint"}</div>
                                                            <div style={format!("font-size: 18px; font-weight: {}; color: {};", weight, color)}>{ aptitude_level(val) }</div>
                                                        </div>
                                                    }
                                                }) }
                                                { v.aptitude_mile.map(|val| {
                                                    let (color, weight) = aptitude_style(val);
                                                    html! {
                                                        <div style={format!("background: #1e293b; border-top: 3px solid {}; border-radius: 6px; padding: 6px 12px; text-align: center; min-width: 70px;", color)}>
                                                            <div style="font-size: 10px; text-transform: uppercase; color: #64748b; margin-bottom: 2px;">{"Mile"}</div>
                                                            <div style={format!("font-size: 18px; font-weight: {}; color: {};", weight, color)}>{ aptitude_level(val) }</div>
                                                        </div>
                                                    }
                                                }) }
                                                { v.aptitude_medium.map(|val| {
                                                    let (color, weight) = aptitude_style(val);
                                                    html! {
                                                        <div style={format!("background: #1e293b; border-top: 3px solid {}; border-radius: 6px; padding: 6px 12px; text-align: center; min-width: 70px;", color)}>
                                                            <div style="font-size: 10px; text-transform: uppercase; color: #64748b; margin-bottom: 2px;">{"Medium"}</div>
                                                            <div style={format!("font-size: 18px; font-weight: {}; color: {};", weight, color)}>{ aptitude_level(val) }</div>
                                                        </div>
                                                    }
                                                }) }
                                                { v.aptitude_long.map(|val| {
                                                    let (color, weight) = aptitude_style(val);
                                                    html! {
                                                        <div style={format!("background: #1e293b; border-top: 3px solid {}; border-radius: 6px; padding: 6px 12px; text-align: center; min-width: 70px;", color)}>
                                                            <div style="font-size: 10px; text-transform: uppercase; color: #64748b; margin-bottom: 2px;">{"Long"}</div>
                                                            <div style={format!("font-size: 18px; font-weight: {}; color: {};", weight, color)}>{ aptitude_level(val) }</div>
                                                        </div>
                                                    }
                                                }) }
                                            </div>
                                        </div>
                                        <div style="margin-top: 12px;">
                                            <div style="font-size: 11px; text-transform: uppercase; color: #64748b; letter-spacing: 0.5px; margin-bottom: 6px;">{"Running Style"}</div>
                                            <div style="display: flex; flex-wrap: wrap; gap: 6px;">
                                                { v.aptitude_front.map(|val| {
                                                    let (color, weight) = aptitude_style(val);
                                                    html! {
                                                        <div style={format!("background: #1e293b; border-top: 3px solid {}; border-radius: 6px; padding: 6px 12px; text-align: center; min-width: 70px;", color)}>
                                                            <div style="font-size: 10px; text-transform: uppercase; color: #64748b; margin-bottom: 2px;">{"Front"}</div>
                                                            <div style={format!("font-size: 18px; font-weight: {}; color: {};", weight, color)}>{ aptitude_level(val) }</div>
                                                        </div>
                                                    }
                                                }) }
                                                { v.aptitude_pace_chaser.map(|val| {
                                                    let (color, weight) = aptitude_style(val);
                                                    html! {
                                                        <div style={format!("background: #1e293b; border-top: 3px solid {}; border-radius: 6px; padding: 6px 12px; text-align: center; min-width: 70px;", color)}>
                                                            <div style="font-size: 10px; text-transform: uppercase; color: #64748b; margin-bottom: 2px;">{"Pace Chaser"}</div>
                                                            <div style={format!("font-size: 18px; font-weight: {}; color: {};", weight, color)}>{ aptitude_level(val) }</div>
                                                        </div>
                                                    }
                                                }) }
                                                { v.aptitude_late_surger.map(|val| {
                                                    let (color, weight) = aptitude_style(val);
                                                    html! {
                                                        <div style={format!("background: #1e293b; border-top: 3px solid {}; border-radius: 6px; padding: 6px 12px; text-align: center; min-width: 70px;", color)}>
                                                            <div style="font-size: 10px; text-transform: uppercase; color: #64748b; margin-bottom: 2px;">{"Late Surger"}</div>
                                                            <div style={format!("font-size: 18px; font-weight: {}; color: {};", weight, color)}>{ aptitude_level(val) }</div>
                                                        </div>
                                                    }
                                                }) }
                                                { v.aptitude_end_closer.map(|val| {
                                                    let (color, weight) = aptitude_style(val);
                                                    html! {
                                                        <div style={format!("background: #1e293b; border-top: 3px solid {}; border-radius: 6px; padding: 6px 12px; text-align: center; min-width: 70px;", color)}>
                                                            <div style="font-size: 10px; text-transform: uppercase; color: #64748b; margin-bottom: 2px;">{"End Closer"}</div>
                                                            <div style={format!("font-size: 18px; font-weight: {}; color: {};", weight, color)}>{ aptitude_level(val) }</div>
                                                        </div>
                                                    }
                                                }) }
                                            </div>
                                        </div>
                                    </> }
                                } else { html! {} } }
                            </div>
                        },
                        Tab::Sparks => html! {
                            <div class={DetailTabStyle::CLASS_NAME}>
                                if props.loading {
                                    <p>{"Loading..."}</p>
                                } else if props.sparks.is_empty() {
                                    <p>{"No spark data."}</p>
                                } else {
                                    <div class={SparkDetailListStyle::CLASS_NAME}>
                                        { for spark_groups.iter().filter(|g| !g.is_empty()).map(|group| {
                                            html! {
                                                <div class={SparkColorRowStyle::CLASS_NAME}>
                                                    { for group.iter().map(|s| html! { <SparkItem spark={s.clone()} /> }) }
                                                </div>
                                            }
                                        })}
                                    </div>
                                }
                            </div>
                        },
                        Tab::Parents => html! {
                            <div class={DetailTabStyle::CLASS_NAME}>
                                if props.loading {
                                    <p>{"Loading..."}</p>
                                } else if props.parents.is_empty() {
                                    <p>{"No parent data."}</p>
                                } else {
                                    <div class={ParentListStyle::CLASS_NAME}>
                                        { for props.parents.iter().map(|p| {
                                            let pname = p.trainee_name.as_deref().unwrap_or("Unknown").to_string();
                                            let onclick = on_parent_click.clone();
                                            let pclone = p.clone();
                                            let blue_sparks: Vec<&SparkGroupRow> = p.blue_sparks.iter().filter(|s| s.spark_type == 1).collect();
                                            let has_blue = !blue_sparks.is_empty();
                                            let phash = p.hash;
                                            let parent_hash_copied = parent_hash_copied.clone();
                                            let is_copied = *parent_hash_copied == Some(phash);
                                            let parent_owner_copied = parent_owner_copied.clone();
                                            html! {
                                                <div class={ParentCardStyle::CLASS_NAME} onclick={Callback::from(move |_| onclick.emit(pclone.clone()))}>
                                                    <div class={CardHeaderStyle::CLASS_NAME}>
                                                        <span class={CardNameStyle::CLASS_NAME}>{ &pname }</span>
                                                        <span class={classes!(CardRankStyle::CLASS_NAME, (p.owned == 0).then_some("card-borrowed"))}><RankBadge rank={UmaRank::from_raw(p.rank as u16)} /></span>
                                                    </div>
                                                    <div class={CardMetaStyle::CLASS_NAME}>
                                                        <span class={ParentRarityStyle::CLASS_NAME}>{ format!("Rarity {}", p.rarity) }</span>
                                                        { p.talent_level.map(|tl| html! { <span class={ParentTalentStyle::CLASS_NAME}>{ format!("Talent Lv.{}", tl) }</span> }) }
                                                        { if p.owned == 0 {
                                                            if let Some(owner) = p.owner_id {
                                                                html! { <span class={classes!(OwnerIdBadgeStyle::CLASS_NAME, (*parent_owner_copied).then_some("owner-id-copied"))} title="Click to copy owner ID"
                                                                    onclick={Callback::from(move |e: yew::MouseEvent| {
                                                                        e.stop_propagation();
                                                                        let text = owner.to_string();
                                                                        let parent_owner_copied = parent_owner_copied.clone();
                                                                        wasm_bindgen_futures::spawn_local(async move {
                                                                            if let Some(window) = web_sys::window() {
                                                                                let _ = window.navigator().clipboard().write_text(&text);
                                                                            }
                                                                            parent_owner_copied.set(true);
                                                                            gloo_timers::future::TimeoutFuture::new(500).await;
                                                                            parent_owner_copied.set(false);
                                                                        });
                                                                    })}
                                                                ><span class={OwnerIdPrefixStyle::CLASS_NAME}>{"Owner"}</span>{ owner }</span> }
                                                            } else { html! {} }
                                                        } else { html! {} } }
                                                    </div>
                                                    <div class={CardStatsRowStyle::CLASS_NAME}>
                                                        <span class={StatLabelStyle::CLASS_NAME}>{"Sparks:"}</span>
                                                        <span class={StatValueStyle::CLASS_NAME}>{ p.spark_count }</span>
                                                    </div>
                                                    <div class={CardStatsRowStyle::CLASS_NAME}>
                                                        <span class={StatLabelStyle::CLASS_NAME}>{"Wins:"}</span>
                                                        <span class={StatValueStyle::CLASS_NAME}>{ p.major_wins_count }</span>
                                                    </div>
                                                    if has_blue {
                                                        <div class={CardSparksStyle::CLASS_NAME}>
                                                            { for blue_sparks.iter().map(|s| html! { <SparkItem spark={(*s).clone()} /> }) }
                                                        </div>
                                                    }
                                                    <div class={CardFooterStyle::CLASS_NAME} onclick={Callback::from(move |e: yew::MouseEvent| e.stop_propagation())}>
                                                        { {
                                                            let hash_str = format!("{:016x}", phash as u64);
                                                            let parent_hash_copied = parent_hash_copied.clone();
                                                            let hash_display = hash_str.clone();
                                                            html! {
                                                                 <span class={classes!(CardHashStyle::CLASS_NAME, is_copied.then_some("hash-copied"))} title="Click to copy parent hash"
                                                                     onclick={Callback::from(move |e: yew::MouseEvent| {
                                                                         e.stop_propagation();
                                                                         let text = hash_display.clone();
                                                                         let parent_hash_copied = parent_hash_copied.clone();
                                                                         wasm_bindgen_futures::spawn_local(async move {
                                                                             if let Some(window) = web_sys::window() {
                                                                                 let _ = window.navigator().clipboard().write_text(&text);
                                                                             }
                                                                             parent_hash_copied.set(Some(phash));
                                                                             gloo_timers::future::TimeoutFuture::new(500).await;
                                                                             parent_hash_copied.set(None);
                                                                         });
                                                                     })}>
                                                                     { "PRT " }{ hash_str }
                                                                 </span>
                                                            }
                                                        } }
                                                    </div>
                                                </div>
                                            }
                                        }) }
                                    </div>
                                }
                            </div>
                        },
                        Tab::Wins => html! {
                            <div class={DetailTabStyle::CLASS_NAME}>
                                if props.loading {
                                    <p>{"Loading..."}</p>
                                } else {
                                    <WinsList wins={sorted_wins} />
                                }
                            </div>
                        },
                        Tab::Skills => {
                            let on_skill_click = {
                                let selected_skill = selected_skill.clone();
                                Callback::from(move |(id, lvl): (i64, i64)| {
                                    selected_skill.set(Some((id, lvl)));
                                })
                            };
                            render_skills_tab(&props.skills, on_skill_click)
                        },
                        Tab::SupportCards => {
                            html! {
                                <div class={DetailTabStyle::CLASS_NAME}>
                                    if props.support_cards.is_empty() {
                                        <p>{"No support card data."}</p>
                                    } else {
                                        <div class={SupportCardListStyle::CLASS_NAME}>
                                            { for props.support_cards.iter().map(|sc| {
                                                let rarity_class = match sc.rarity {
                                                    1 => "rarity-r",
                                                    2 => "rarity-sr",
                                                    3 => "rarity-ssr",
                                                    _ => "rarity-unknown",
                                                };
                                                let rarity_label = match sc.rarity {
                                                    1 => "R", 2 => "SR", 3 => "SSR", _ => "?"
                                                };
                                                let type_class = match sc.card_type {
                                                    1 => "type-speed",
                                                    2 => "type-stamina",
                                                    3 => "type-power",
                                                    4 => "type-guts",
                                                    5 => "type-wit",
                                                    6 => "type-pal",
                                                    7 => "type-group",
                                                    _ => "type-unknown",
                                                };
                                                let type_label = match sc.card_type {
                                                    1 => "Speed", 2 => "Stamina", 3 => "Power",
                                                    4 => "Guts", 5 => "Wit", 6 => "Pal",
                                                    7 => "Group", _ => "?"
                                                };
                                                let lb = sc.limit_break_count.min(4);
                                                let is_mlb = lb >= 4;
                                                let is_borrow = sc.position == 6;
                                                let (variant, character_name) = parse_card_name(&sc.name);
                                                html! {
                                                    <div class={classes!(SupportCardRowStyle::CLASS_NAME, is_borrow.then_some("borrow-row"))}>
                                                        <span class={classes!(SupportCardLbStyle::CLASS_NAME, is_mlb.then_some("mlb"))}>
                                                            { for (0..4).map(|idx| {
                                                                let on = idx < lb;
                                                                html! { <span class={classes!("diamond", on.then_some("on"))}></span> }
                                                            }) }
                                                        </span>
                                                        <span class={classes!(SupportCardTypeStyle::CLASS_NAME, type_class)}>
                                                            <span class="type-text">{ type_label }</span>
                                                        </span>
                                                        <span class={classes!(SupportCardRarityStyle::CLASS_NAME, rarity_class)}>
                                                            <span class="rarity-text">{ rarity_label }</span>
                                                        </span>
                                                        <div>
                                                            {if let Some(v) = &variant {
                                                                html! { <div class={SupportCardVariantStyle::CLASS_NAME}>{v}</div> }
                                                            } else { html! {} }}
                                                            <span class={SupportCardNameStyle::CLASS_NAME}>{ character_name }</span>
                                                        </div>
                                                    </div>
                                                }
                                            }) }
                                        </div>
                                    }
                                </div>
                            }
                        },
                        Tab::Tags => {
                            if props.api_mode {
                                html! {}
                            } else {
                                html! {
                                    <div class={DetailTabStyle::CLASS_NAME}>
                                        if !(*veteran_tags).is_empty() {
                                            <div class={TagPillListStyle::CLASS_NAME}>
                                                { for (*veteran_tags).iter().map(|tag| {
                                                    let tag_id = tag.id;
                                                    let on_remove_tag = on_remove_tag.clone();
                                                    html! {
                                                        <span class={TagPillStyle::CLASS_NAME}>
                                                            { &tag.tag_value }
                                                            <button class={TagPillRemoveStyle::CLASS_NAME}
                                                                onclick={Callback::from(move |e: yew::MouseEvent| {
                                                                    e.stop_propagation();
                                                                    on_remove_tag.emit(tag_id);
                                                                })}>
                                                                {"×"}
                                                            </button>
                                                        </span>
                                                    }
                                                }) }
                                            </div>
                                        } else {
                                            <p style="color: #64748b; font-size: 13px;">{"No tags yet."}</p>
                                        }
                                        <button class={TabBtnStyle::CLASS_NAME}
                                            onclick={on_open_tag_modal}>
                                            {"+ Manage Tags"}
                                        </button>
                                    </div>
                                }
                            }
                        },
                    } }
                </div>
            </div>

            { render_parent_detail(
                &*selected_parent,
                &*parent_sparks,
                &*parent_wins,
                *parent_loading,
                &close_parent_detail,
            ) }

            { if !props.api_mode {
                html! { <TagModal
                    show={*tag_modal_open}
                    title={"Manage Tags".to_string()}
                    current_tags={(*veteran_tags).clone()}
                    search_results={(*tag_search_results).clone()}
                    on_search={on_tag_search}
                    on_save={on_tag_save}
                    on_close={on_close_tag_modal}
                /> }
            } else { html! {} } }

            { if let Some((skill_id, level)) = *selected_skill {
                let on_close = {
                    let selected_skill = selected_skill.clone();
                    Callback::from(move |_| selected_skill.set(None))
                };
                html! {
                    <SkillDetailModal
                        skill_id={skill_id}
                        level={level}
                        on_close={on_close}
                    />
                }
            } else {
                html! {}
            } }
        </div>
    }
}
