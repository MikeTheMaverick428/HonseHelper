use crate::styles::{
    race_dump_detail::*,
    veteran_card::{OwnerIdBadgeStyle, OwnerIdPrefixStyle},
    Style,
};
use crate::tauri_bridge::invoke_tauri_command;
use crate::veteran_browser::components::detail_modal::DetailModal;
use gloo_timers::future::TimeoutFuture;
use serde_json::json;
use shared::race_dump_types::RaceDumpParticipant;
use shared::veteran_browser::{
    FilterOptions, MajorWinRow, ParentRow, SparkGroupRow, VeteranRow, VeteranSkillRow,
    VeteranSupportCardRow,
};
use std::collections::{HashMap, HashSet};
use yew::prelude::*;

fn finish_label(v: Option<i64>) -> String {
    match v {
        Some(0) => "1st".into(),
        Some(1) => "2nd".into(),
        Some(2) => "3rd".into(),
        Some(n) => format!("{}th", n + 1),
        None => "DNF".into(),
    }
}

fn running_style_label(v: Option<i64>) -> &'static str {
    match v.unwrap_or(0) {
        1 => "Front",
        2 => "Pace Chaser",
        3 => "Late Surger",
        4 => "End Closer",
        5 => "Run-Away",
        _ => "—",
    }
}

fn format_rank(score: i64) -> String {
    if score >= 100_000_000 {
        format!("UG+{:.1}", (score as f64 - 100_000_000.0) / 10_000_000.0)
    } else if score >= 50_000_000 {
        format!("UF+{:.1}", (score as f64 - 50_000_000.0) / 10_000_000.0)
    } else if score >= 25_000_000 {
        format!("UE+{:.1}", (score as f64 - 25_000_000.0) / 10_000_000.0)
    } else if score >= 12_000_000 {
        format!("UD+{:.1}", (score as f64 - 12_000_000.0) / 10_000_000.0)
    } else if score >= 6_000_000 {
        format!("UC+{:.1}", (score as f64 - 6_000_000.0) / 10_000_000.0)
    } else if score >= 3_000_000 {
        format!("UB+{:.1}", (score as f64 - 3_000_000.0) / 10_000_000.0)
    } else if score >= 1_500_000 {
        format!("UA+{:.1}", (score as f64 - 1_500_000.0) / 10_000_000.0)
    } else {
        format!("{}", score)
    }
}

fn format_time(secs: Option<f64>) -> String {
    match secs {
        Some(t) => {
            let min = (t / 60.0).floor();
            let sec = t - min * 60.0;
            format!("{:.0}:{:06.3}", min, sec)
        }
        None => "—".into(),
    }
}

fn finish_class(v: Option<i64>) -> &'static str {
    match v {
        Some(0) => PartFinish1stStyle::CLASS_NAME,
        Some(1) => PartFinish2ndStyle::CLASS_NAME,
        Some(2) => PartFinish3rdStyle::CLASS_NAME,
        _ => PartFinishOtherStyle::CLASS_NAME,
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ParticipantsTabProps {
    pub participants: Vec<RaceDumpParticipant>,
}

#[function_component]
pub fn ParticipantsTab(props: &ParticipantsTabProps) -> Html {
    let veteran_map = use_state(|| HashMap::new() as HashMap<i64, VeteranRow>);
    let loading = use_state(|| true);
    let scenarios = use_state(|| Vec::new() as Vec<(i64, String)>);

    let detail_veteran = use_state(|| None::<VeteranRow>);
    let detail_sparks = use_state(Vec::<SparkGroupRow>::new);
    let detail_wins = use_state(Vec::<MajorWinRow>::new);
    let detail_parents = use_state(Vec::<ParentRow>::new);
    let detail_skills = use_state(Vec::<VeteranSkillRow>::new);
    let detail_support_cards = use_state(Vec::<VeteranSupportCardRow>::new);
    let detail_loading = use_state(|| false);
    let copied_horse = use_state(|| HashSet::<i64>::new());

    {
        let participants = props.participants.clone();
        let veteran_map = veteran_map.clone();
        let loading = loading.clone();
        let scenarios = scenarios.clone();
        use_effect_with((), move |_| {
            let veteran_map = veteran_map.clone();
            let loading = loading.clone();
            let scenarios = scenarios.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(result) = invoke_tauri_command("get_filter_options", json!({})).await {
                    if let Ok(opts) = serde_json::from_value::<FilterOptions>(result) {
                        scenarios.set(opts.scenarios);
                    }
                }

                let hashes: Vec<i64> = participants.iter().filter_map(|p| p.veteran_hash).collect();
                let mut map = HashMap::new();
                for hash in hashes {
                    if let Ok(result) = invoke_tauri_command(
                        "get_veteran_detail",
                        json!({"hash": hash.to_string()}),
                    )
                    .await
                    {
                        if let Ok(Some(v)) = serde_json::from_value::<Option<VeteranRow>>(result) {
                            map.insert(hash, v);
                        }
                    }
                }
                veteran_map.set(map);
                loading.set(false);
            });
            || {}
        });
    }

    let open_detail = {
        let detail_veteran = detail_veteran.clone();
        let detail_sparks = detail_sparks.clone();
        let detail_wins = detail_wins.clone();
        let detail_parents = detail_parents.clone();
        let detail_skills = detail_skills.clone();
        let detail_support_cards = detail_support_cards.clone();
        let detail_loading = detail_loading.clone();
        Callback::from(move |v: VeteranRow| {
            let hash = v.hash;
            let detail_veteran = detail_veteran.clone();
            let detail_sparks = detail_sparks.clone();
            let detail_wins = detail_wins.clone();
            let detail_parents = detail_parents.clone();
            let detail_skills = detail_skills.clone();
            let detail_support_cards = detail_support_cards.clone();
            let detail_loading = detail_loading.clone();
            detail_veteran.set(Some(v));
            detail_support_cards.set(Vec::new());
            detail_loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let h1 = hash.to_string();
                let h2 = hash.to_string();
                let h3 = hash.to_string();
                let h4 = hash.to_string();
                let h5 = hash.to_string();
                let sparks_fut = invoke_tauri_command("get_veteran_sparks", json!({"hash": h1}));
                let wins_fut = invoke_tauri_command("get_veteran_wins", json!({"hash": h2}));
                let parents_fut = invoke_tauri_command("get_veteran_parents", json!({"hash": h3}));
                let skills_fut = invoke_tauri_command("get_veteran_skills", json!({"hash": h4}));
                let sc_fut = invoke_tauri_command("get_veteran_support_cards", json!({"hash": h5}));

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
        let detail_veteran = detail_veteran.clone();
        Callback::from(move |_| detail_veteran.set(None))
    };

    let mut sorted: Vec<&RaceDumpParticipant> = props.participants.iter().collect();
    sorted.sort_by(|a, b| {
        a.finish_order
            .unwrap_or(999)
            .cmp(&b.finish_order.unwrap_or(999))
    });

    html! {
        <div class={PartTabStyle::CLASS_NAME}>
            {sorted.iter().map(|p| {
                let veteran = p.veteran_hash.and_then(|h| veteran_map.get(&h));
                let is_player = p.is_player == 1;
                let finish = finish_label(p.finish_order);
                let f_class = finish_class(p.finish_order);
                let rs = running_style_label(p.running_style);
                let ft = format_time(p.finish_time);

                let stat_sp = p.speed.map(|v| v.to_string()).unwrap_or_else(|| "—".into());
                let stat_st = p.stamina.map(|v| v.to_string()).unwrap_or_else(|| "—".into());
                let stat_pw = p.pow.map(|v| v.to_string()).unwrap_or_else(|| "—".into());
                let stat_gu = p.guts.map(|v| v.to_string()).unwrap_or_else(|| "—".into());
                let stat_wz = p.wiz.map(|v| v.to_string()).unwrap_or_else(|| "—".into());

                if let Some(v) = veteran {
                    let vrow = v.clone();
                    let on_details = {
                        let open_detail = open_detail.clone();
                        Callback::from(move |_| open_detail.emit(vrow.clone()))
                    };
                    let scenario_name = v.scenario
                        .and_then(|sc_id| scenarios.iter().find(|(id, _)| *id == sc_id).map(|(_, n)| n.as_str()))
                        .unwrap_or("?");
                    let vstat_sp = v.stat_speed.map(|v| v.to_string()).unwrap_or_else(|| "—".into());
                    let vstat_st = v.stat_stamina.map(|v| v.to_string()).unwrap_or_else(|| "—".into());
                    let vstat_pw = v.stat_power.map(|v| v.to_string()).unwrap_or_else(|| "—".into());
                    let vstat_gu = v.stat_guts.map(|v| v.to_string()).unwrap_or_else(|| "—".into());
                    let vstat_wz = v.stat_wit.map(|v| v.to_string()).unwrap_or_else(|| "—".into());

                    html! {
                        <div class={PartRowStyle::CLASS_NAME}>
                            <div class={classes!(PartFinishStyle::CLASS_NAME, f_class)}>
                                {finish}
                            </div>
                            <div class={PartInfoSectionStyle::CLASS_NAME}>
                                <div class={PartNameStyle::CLASS_NAME}>
                                    {v.trainee_name.as_deref().unwrap_or("?")}
                                </div>
                                <div class={PartMetaStyle::CLASS_NAME}>
                                    {if is_player { html! { <span class={PartPlayerBadgeStyle::CLASS_NAME}>{"★ Player"}</span> } } else { html! {} } }
                                    {if !v.owned && p.viewer_id.filter(|&vid| vid > 0).is_some() {
                                        let viewer_id = p.viewer_id.unwrap();
                                        let ch = copied_horse.clone();
                                        let hi = p.horse_index;
                                        let is_copied = copied_horse.contains(&hi);
                                        html! {
                                            <span class={classes!(OwnerIdBadgeStyle::CLASS_NAME, is_copied.then_some("owner-id-copied"))}
                                                title="Click to copy trainer ID"
                                                onclick={Callback::from(move |e: MouseEvent| {
                                                    e.stop_propagation();
                                                    let text = viewer_id.to_string();
                                                    let ch = ch.clone();
                                                    let hi = hi;
                                                    wasm_bindgen_futures::spawn_local(async move {
                                                        if let Some(win) = web_sys::window() {
                                                            let _ = win.navigator().clipboard().write_text(&text);
                                                        }
                                                        ch.set(HashSet::from([hi]));
                                                        TimeoutFuture::new(500).await;
                                                        ch.set(HashSet::new());
                                                    });
                                                })}
                                            >
                                                <span class={OwnerIdPrefixStyle::CLASS_NAME}>{"ID"}</span>{ viewer_id }
                                            </span>
                                        }
                                    } else { html! {} } }
                                    {if is_player { html! { <span class={if v.active { PartActiveBadgeStyle::CLASS_NAME } else { PartPastBadgeStyle::CLASS_NAME }}>
                                        {if v.active { "Active" } else { "Past" }}
                                    </span> } } else { html! {} } }
                                    <span class={PartRankScoreStyle::CLASS_NAME}>{format_rank(v.rank_score)}</span>
                                    <span class={PartScenarioBadgeStyle::CLASS_NAME}>{scenario_name}</span>
                                </div>
                                <div class={PartStatsRowStyle::CLASS_NAME}>
                                    <span class={PartStatStyle::CLASS_NAME}>{format!("Spd {}", vstat_sp)}</span>
                                    <span class={PartStatStyle::CLASS_NAME}>{format!("Sta {}", vstat_st)}</span>
                                    <span class={PartStatStyle::CLASS_NAME}>{format!("Pow {}", vstat_pw)}</span>
                                    <span class={PartStatStyle::CLASS_NAME}>{format!("Gut {}", vstat_gu)}</span>
                                    <span class={PartStatStyle::CLASS_NAME}>{format!("Wiz {}", vstat_wz)}</span>
                                </div>
                            </div>
                            <div class={PartResultSectionStyle::CLASS_NAME}>
                                <div class={PartTimeStyle::CLASS_NAME}>{ft}</div>
                                <div class={PartRunningStyleStyle::CLASS_NAME}>{rs}</div>
                                <button class={PartDetailBtnStyle::CLASS_NAME} onclick={on_details}>
                                    {"Details"}
                                </button>
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <div class={PartRowStyle::CLASS_NAME}>
                            <div class={classes!(PartFinishStyle::CLASS_NAME, f_class)}>
                                {finish}
                            </div>
                            <div class={PartInfoSectionStyle::CLASS_NAME}>
                                <div class={PartNameStyle::CLASS_NAME}>
                                    {p.chara_name.as_deref().unwrap_or("???")}
                                    if is_player { <span class={PartPlayerBadgeStyle::CLASS_NAME}>{" ★ Player"}</span> }
                                </div>
                                <div class={PartMetaStyle::CLASS_NAME}>
                                    <span class={PartNpcBadgeStyle::CLASS_NAME}>{"NPC"}</span>
                                </div>
                                <div class={PartStatsRowStyle::CLASS_NAME}>
                                    <span class={PartStatStyle::CLASS_NAME}>{format!("Spd {}", stat_sp)}</span>
                                    <span class={PartStatStyle::CLASS_NAME}>{format!("Sta {}", stat_st)}</span>
                                    <span class={PartStatStyle::CLASS_NAME}>{format!("Pow {}", stat_pw)}</span>
                                    <span class={PartStatStyle::CLASS_NAME}>{format!("Gut {}", stat_gu)}</span>
                                    <span class={PartStatStyle::CLASS_NAME}>{format!("Wiz {}", stat_wz)}</span>
                                </div>
                            </div>
                            <div class={PartResultSectionStyle::CLASS_NAME}>
                                <div class={PartTimeStyle::CLASS_NAME}>{ft}</div>
                                <div class={PartRunningStyleStyle::CLASS_NAME}>{rs}</div>
                            </div>
                        </div>
                    }
                }
            }).collect::<Html>()}

            if let Some(ref v) = *detail_veteran {
                <DetailModal
                    veteran={Some(v.clone())}
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
        </div>
    }
}
