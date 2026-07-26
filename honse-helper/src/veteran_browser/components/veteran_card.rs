use crate::components::delete_button::DeleteButton;
use crate::styles::{
    legacy_planner::{AffinityBaseStyle, AffinityBonusStyle, AffinityPlusStyle},
    tag_modal::CardTagMoreStyle,
    tag_modal::CardTagPillStyle,
    veteran_card::*,
    Style,
};
use shared::legacy_planner::lookup_dtos::AffinityResult;
use shared::models::{INDEPENDENT_LEARNER_NICKNAME, FavouriteIcon, UmaRank};
use shared::veteran_browser::{TagRow, VeteranRow};
use yew::prelude::*;

use super::rank_badge::RankBadge;
use super::spark_item::SparkItem;

#[derive(Properties, PartialEq)]
pub struct VeteranCardProps {
    pub veteran: VeteranRow,
    pub on_click: Callback<()>,
    pub on_select: Option<Callback<String>>,
    pub on_save: Option<Callback<String>>,
    pub on_delete: Option<Callback<String>>,
    pub active_spark_group_ids: Vec<i64>,
    pub scenarios: Vec<(i64, String)>,
    #[prop_or(None)]
    pub affinity: Option<AffinityResult>,
    #[prop_or_default]
    pub tags: Vec<TagRow>,
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

pub fn parse_veteran_name(name: &str) -> (Option<String>, &str) {
    let name = name.trim();
    if let Some(end_bracket) = name.find(']') {
        if name.starts_with('[') && end_bracket > 0 {
            let variant = name[1..end_bracket].trim().to_string();
            let character = name[end_bracket + 1..].trim();
            let variant = if variant.is_empty() { None } else { Some(variant) };
            return (variant, character);
        }
    }
    (None, name)
}

fn icon_label(icon_type: i64) -> String {
    FavouriteIcon::try_from(icon_type as i16)
        .map(|icon| icon.label().to_string())
        .unwrap_or_else(|_| format!("Icon {}", icon_type))
}

#[function_component]
pub fn VeteranCard(props: &VeteranCardProps) -> Html {
    let v = &props.veteran;
    let owner_copied = use_state(|| false);
    let hash_copied = use_state(|| false);
    let min_hash_copied = use_state(|| false);
    let onclick = {
        let cb = props.on_click.clone();
        Callback::from(move |_| cb.emit(()))
    };

    let on_select = props.on_select.clone().map(|cb| {
        let hash = v.hash;
        Callback::from(move |e: yew::MouseEvent| {
            e.stop_propagation();
            let hash_str = format!("{:016x}", hash);
            gloo_console::log!(format!(
                "[VeteranCard] Select clicked, emitting hash: {}",
                hash_str
            ));
            cb.emit(hash_str);
        })
    });

    let on_save = props.on_save.clone().map(|cb| {
        let hash = v.hash;
        Callback::from(move |e: yew::MouseEvent| {
            e.stop_propagation();
            let hash_str = format!("{:016x}", hash);
            gloo_console::log!(format!(
                "[VeteranCard] Save clicked, emitting hash: {}",
                hash_str
            ));
            cb.emit(hash_str);
        })
    });

    let on_delete = props.on_delete.clone().map(|cb| {
        let hash = v.hash;
        Callback::from(move |e: yew::MouseEvent| {
            e.stop_propagation();
            let hash_str = format!("{:016x}", hash);
            gloo_console::log!(format!(
                "[VeteranCard] Delete clicked, emitting hash: {}",
                hash_str
            ));
            cb.emit(hash_str);
        })
    });

    let has_filter = !props.active_spark_group_ids.is_empty();
    let mut display_sparks: Vec<&shared::veteran_browser::SparkGroupRow> = v
        .spark_groups
        .iter()
        .filter(|s| {
            s.spark_type == 1
                || (has_filter && props.active_spark_group_ids.contains(&s.spark_group_id))
        })
        .collect();
    display_sparks.sort_by(|a, b| {
        let type_order = |t: i64| -> i8 {
            match t {
                1 => 0,
                2 => 1,
                3 => 2,
                _ => 3,
            }
        };
        let ta = type_order(a.spark_type);
        let tb = type_order(b.spark_type);
        ta.cmp(&tb).then(b.level_sum.cmp(&a.level_sum))
    });

    let (variant, character_name) = v.trainee_name.as_deref().map(parse_veteran_name).unwrap_or((None, "Unknown"));

    html! {
        <div class={VeteranCardRootStyle::CLASS_NAME} onclick={onclick}>
            <div class={CardHeaderStyle::CLASS_NAME}>
                <div style="display:flex;flex-direction:column;">
                    {if let Some(v) = &variant {
                        html! { <span class={VeteranVariantStyle::CLASS_NAME}>{v}</span> }
                    } else { html! {} }}
                    <span class={CardNameStyle::CLASS_NAME}>{ character_name }</span>
                </div>
                <span class={classes!(CardRankStyle::CLASS_NAME, (!v.owned).then_some(CardBorrowedStyle::CLASS_NAME))}>
                    <RankBadge rank={UmaRank::from_raw(v.rank as u16)} />
                    <span class={RankScoreStyle::CLASS_NAME}>{ format_rank(v.rank_score) }</span>
                </span>
            </div>
            <div class={CardMetaStyle::CLASS_NAME}>
                <span class={CardScenarioStyle::CLASS_NAME}>{ v.scenario.and_then(|sc| props.scenarios.iter().find(|(id, _)| *id == sc)).map(|(_, n)| n.as_str()).unwrap_or("?") }</span>
                <span class={CardDateStyle::CLASS_NAME}>{ if v.created_at.len() >= 10 { &v.created_at[..10] } else { &v.created_at } }</span>
                { if !v.owned {
                    if let Some(owner) = v.owner_id {
                        let owner_copied = owner_copied.clone();
                        html! { <span class={classes!(OwnerIdBadgeStyle::CLASS_NAME, (*owner_copied).then_some("owner-id-copied"))} title="Click to copy owner ID"
                            onclick={Callback::from(move |e: MouseEvent| {
                                e.stop_propagation();
                                let text = owner.to_string();
                                let owner_copied = owner_copied.clone();
                                wasm_bindgen_futures::spawn_local(async move {
                                    if let Some(window) = web_sys::window() {
                                        let _ = window.navigator().clipboard().write_text(&text);
                                    }
                                    owner_copied.set(true);
                                    gloo_timers::future::TimeoutFuture::new(500).await;
                                    owner_copied.set(false);
                                });
                            })}
                        ><span class={OwnerIdPrefixStyle::CLASS_NAME}>{"Owner"}</span>{ owner }</span> }
                    } else { html! {} }
                } else { html! {} } }
                { if v.nickname_id == Some(INDEPENDENT_LEARNER_NICKNAME) {
                    html! { <span class={IndepTrainBadgeStyle::CLASS_NAME}>{"Indep. Training"}</span> }
                } else { html! {} } }
            </div>
            <div class={CardStatsRowStyle::CLASS_NAME}>
                <span class={StatLabelStyle::CLASS_NAME}>{"Sparks:"}</span>
                <span class={StatValueStyle::CLASS_NAME}>{ v.white_spark_count }</span>
                <span class={StatSubStyle::CLASS_NAME}>{" ("}{ v.white_spark_on_veteran_count }{")"}</span>
            </div>
            <div class={CardStatsRowStyle::CLASS_NAME}>
                <span class={StatLabelStyle::CLASS_NAME}>{"Wins:"}</span>
                <span class={StatValueStyle::CLASS_NAME}>{ v.major_wins_count }</span>
                <span class={StatSubStyle::CLASS_NAME}>{" ("}{ v.major_wins_on_veteran_count }{")"}</span>
            </div>
            { if let Some(aff) = props.affinity {
                html! { <div class={CardAffinityStyle::CLASS_NAME}>
                    <span>{"Affinity: "}</span>
                    <span class={AffinityBaseStyle::CLASS_NAME}>{aff.base}</span>
                    { if aff.bonus > 0 {
                        html! {
                            <>
                                <span class={AffinityPlusStyle::CLASS_NAME}>{" + "}</span>
                                <span class={AffinityBonusStyle::CLASS_NAME}>{aff.bonus}</span>
                                <span>{" (Total: "}{aff.total()}{")"}</span>
                            </>
                        }
                    } else { html! {} } }
                </div> }
            } else { html! {} } }
            { if !display_sparks.is_empty() {
                html! {
                    <div class={CardSparksStyle::CLASS_NAME}>
                        { for display_sparks.into_iter().map(|s| {
                            let matched = props.active_spark_group_ids.contains(&s.spark_group_id);
                            html! { <SparkItem spark={s.clone()} highlighted={matched} /> }
                        })}
                    </div>
                }
            } else { html! {} } }
            { if !props.tags.is_empty() {
                let display_tags: Vec<&TagRow> = props.tags.iter().take(3).collect();
                let remaining = props.tags.len().saturating_sub(3);
                html! {
                    <div class={CardTagsStyle::CLASS_NAME}>
                        { for display_tags.iter().map(|t| {
                            html! { <span class={CardTagPillStyle::CLASS_NAME}>{ &t.tag_value }</span> }
                        })}
                        { if remaining > 0 {
                            html! { <span class={CardTagMoreStyle::CLASS_NAME}>{ format!("+{}", remaining) }</span> }
                        } else { html! {} } }
                    </div>
                }
            } else { html! {} } }
            <div class={CardFooterStyle::CLASS_NAME}>
                { {
                    let hash_str = format!("{:016x}", v.hash as u64);
                    let hash_copied = hash_copied.clone();
                    let hash_display = hash_str.clone();
                    html! {
                        <span class={classes!(CardHashStyle::CLASS_NAME, (*hash_copied).then_some("hash-copied"))} title="Click to copy trained chara hash"
                            onclick={Callback::from(move |e: yew::MouseEvent| {
                                e.stop_propagation();
                                let text = hash_display.clone();
                                let hash_copied = hash_copied.clone();
                                wasm_bindgen_futures::spawn_local(async move {
                                    if let Some(window) = web_sys::window() {
                                        let _ = window.navigator().clipboard().write_text(&text);
                                    }
                                    hash_copied.set(true);
                                    gloo_timers::future::TimeoutFuture::new(500).await;
                                    hash_copied.set(false);
                                });
                            })}>
                            { "VET " }{ hash_str }
                        </span>
                    }
                } }
                { if let Some(min_hash) = v.min_hash {
                    let hash_str = format!("{:016x}", min_hash as u64);
                    let min_hash_copied = min_hash_copied.clone();
                    let hash_display = hash_str.clone();
                    html! {
                        <span class={classes!(CardHashStyle::CLASS_NAME, (*min_hash_copied).then_some("hash-copied"))} title="Click to copy parent identity hash"
                            onclick={Callback::from(move |e: yew::MouseEvent| {
                                e.stop_propagation();
                                let text = hash_display.clone();
                                let min_hash_copied = min_hash_copied.clone();
                                wasm_bindgen_futures::spawn_local(async move {
                                    if let Some(window) = web_sys::window() {
                                        let _ = window.navigator().clipboard().write_text(&text);
                                    }
                                    min_hash_copied.set(true);
                                    gloo_timers::future::TimeoutFuture::new(500).await;
                                    min_hash_copied.set(false);
                                });
                            })}>
                            { "PRT " }{ hash_str }
                        </span>
                    }
                } else { html! {} } }
                <span class={CardFooterRightStyle::CLASS_NAME}>
                    { if let Some(cb) = on_delete {
                        html! { <DeleteButton onclick={cb} title="Remove this veteran" /> }
                    } else { html! {} } }
                    { if let Some(icon) = &v.favorite_icon_type {
                        html! { <span class={CardFavIconStyle::CLASS_NAME} title="Favourite">{ icon_label(*icon) }</span> }
                    } else { html! {} } }
                    { if let Some(memo) = &v.favorite_memo {
                        if !memo.is_empty() {
                            html! { <span class={CardFavMemoStyle::CLASS_NAME} title="Memo">{ memo }</span> }
                        } else { html! {} }
                    } else { html! {} } }
                </span>
            </div>
            { if let Some(cb) = on_select {
                html! { <button class={SelectBtnStyle::CLASS_NAME} onclick={cb}>{"Select"}</button> }
            } else if let Some(cb) = on_save {
                html! { <button class={SelectBtnStyle::CLASS_NAME} onclick={cb}>{"Save"}</button> }
            } else { html! {} } }
        </div>
    }
}
