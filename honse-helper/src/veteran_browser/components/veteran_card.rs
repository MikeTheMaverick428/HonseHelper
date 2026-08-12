use crate::components::copyable::{HashBadge, OwnerBadge};
use crate::components::delete_button::DeleteButton;
use crate::components::{Badge, parse_variant_name};
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

fn icon_label(icon_type: i64) -> String {
    FavouriteIcon::try_from(icon_type as i16)
        .map(|icon| icon.label().to_string())
        .unwrap_or_else(|_| format!("Icon {}", icon_type))
}

#[function_component]
pub fn VeteranCard(props: &VeteranCardProps) -> Html {
    let v = &props.veteran;
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

    let (variant, character_name) = v.trainee_name.as_deref().map(parse_variant_name).unwrap_or((None, "Unknown"));

    // Helper formatted values
    let scenario_name = v.scenario
        .and_then(|sc| props.scenarios.iter().find(|(id, _)| *id == sc))
        .map(|(_, n)| n.as_str())
        .unwrap_or("?");

    let formatted_date = if v.created_at.len() >= 10 { &v.created_at[..10] } else { &v.created_at };

    // Action button selection logic
    let action_button = on_select.map(|cb| html! {
        <button class={SelectBtnStyle::CLASS_NAME} onclick={cb}>{"Select"}</button>
    }).or_else(|| on_save.map(|cb| html! {
        <button class={SelectBtnStyle::CLASS_NAME} onclick={cb}>{"Save"}</button>
    }));

    html! {
        <div class={VeteranCardRootStyle::CLASS_NAME} onclick={onclick}>
            // --- HEADER ---
            <div class={CardHeaderStyle::CLASS_NAME}>
                <div style="display:flex; flex-direction:column;">
                    { for variant.as_ref().map(|v| html! { <span class={VeteranVariantStyle::CLASS_NAME}>{v}</span> }) }
                    <span class={CardNameStyle::CLASS_NAME}>{ character_name }</span>
                </div>
                <span class={classes!(CardRankStyle::CLASS_NAME, (!v.owned).then_some(CardBorrowedStyle::CLASS_NAME))}>
                    <RankBadge rank={UmaRank::from_raw(v.rank as u16)} />
                    <span class={RankScoreStyle::CLASS_NAME}>{ format_rank(v.rank_score) }</span>
                </span>
            </div>

            // --- META ZONE ---
            <div class={CardMetaStyle::CLASS_NAME}>
                <span class={CardScenarioStyle::CLASS_NAME}>{ scenario_name }</span>
                <span class={CardDateStyle::CLASS_NAME}>{ formatted_date }</span>

                { if !v.owned {
                    v.owner_id.map(|owner_id| {
                        let owner_id = owner_id as u64; 
                        html! { <OwnerBadge {owner_id} /> }
                    })
                } else {
                    None
                }}
            </div>

            // --- STATS BLOCK (Stacked Stat Rows + Right Badges) ---
            <div class={CardStatsBlockStyle::CLASS_NAME}>
                // Left Column: Stacking individual stat rows one below another
                <div class={CardStatsListStyle::CLASS_NAME}>
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
                </div>

                // Right Column: Badges
                <div class={CardBadgesGroupStyle::CLASS_NAME}>
                    { if v.from_followed_trainer {
                        html! { <Badge label="Followed" variant_class="badge-followed" /> }
                    } else { html! {} } }

                    { if v.nickname_id == Some(INDEPENDENT_LEARNER_NICKNAME) {
                        html! { <Badge label="Indep. Training" variant_class="badge-indep-training" /> }
                    } else { html! {} } }
                </div>
            </div>

            // --- AFFINITY ---
            { for props.affinity.as_ref().map(|aff| html! {
                <div class={CardAffinityStyle::CLASS_NAME}>
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
                </div>
            }) }

            // --- SPARKS LIST ---
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

            // --- TAGS ---
            { if !props.tags.is_empty() {
                let remaining = props.tags.len().saturating_sub(3);
                html! {
                    <div class={CardTagsStyle::CLASS_NAME}>
                        { for props.tags.iter().take(3).map(|t| html! {
                            <span class={CardTagPillStyle::CLASS_NAME}>{ &t.tag_value }</span>
                        })}
                        { if remaining > 0 {
                            html! { <span class={CardTagMoreStyle::CLASS_NAME}>{ format!("+{}", remaining) }</span> }
                        } else { html! {} } }
                    </div>
                }
            } else { html! {} } }

            // --- FOOTER ---
            <div class={CardFooterStyle::CLASS_NAME}>
                <div class={CardFooterLeftStyle::CLASS_NAME}>
                    <HashBadge label="VET" hash={v.hash as u64} title="Click to copy trained chara hash" />
                    { for v.min_hash.map(|hash| html! {
                        <HashBadge label="PRT" hash={hash as u64} title="Click to copy parent identity hash" />
                    }) }
                </div>
                <span class={CardFooterRightStyle::CLASS_NAME}>
                    { for on_delete.map(|cb| html! { <DeleteButton onclick={cb} title="Remove this veteran" /> }) }
                    { for v.favorite_icon_type.map(|icon| html! {
                        <span class={CardFavIconStyle::CLASS_NAME} title="Favourite">{ icon_label(icon) }</span>
                    }) }
                    { for v.favorite_memo.as_ref().filter(|m| !m.is_empty()).map(|memo| html! {
                        <span class={CardFavMemoStyle::CLASS_NAME} title="Memo">{ memo }</span>
                    }) }
                </span>
            </div>

            { action_button.unwrap_or_default() }
        </div>
    }
}
