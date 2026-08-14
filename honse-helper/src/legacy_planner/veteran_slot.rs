use shared::legacy_planner::{LegacyPlannerSlot, LegacySlotValue};
use shared::models::UmaRank;
use yew::prelude::*;

use crate::{
    components::{
        SelectOption, copyable::{HashBadge, OwnerBadge}, parse_variant_name,
    }, styles::{
        Style, legacy_veteran_slots::{
            LegacyVeteranSlotActionsStyle, LegacyVeteranSlotBodyStyle,
            LegacyVeteranSlotCardClearStyle, LegacyVeteranSlotCardHeaderStyle,
            LegacyVeteranSlotCardTitleStyle, LegacyVeteranSlotCharacterIdStyle,
            LegacyVeteranSlotCharacterNameStyle, LegacyVeteranSlotContainerStyle,
        }, shared_components::HeaderActionButtonStyle, veteran_card::{
            CardBorrowedStyle, CardFavIconStyle, CardRankStyle, RankScoreStyle,
        },
    }, veteran_browser::components::rank_badge::RankBadge,
};

use super::detail_modal::LegacyDetailModal;

#[derive(Properties, Clone, PartialEq)]
pub struct LegacyVeteranSlotProps {
    pub title: String,
    pub slot_type: LegacyPlannerSlot,
    pub selected: Option<LegacySlotValue>,
    #[prop_or_default]
    pub trainee_options: Vec<SelectOption<u64>>,
    #[prop_or_default]
    pub selected_hash: Option<u64>,
    #[prop_or_default]
    pub selected_character_id: Option<i64>,
    #[prop_or_default]
    pub veteran_search_text: String,
    #[prop_or_default]
    pub on_veteran_search_input: Callback<String>,
    #[prop_or_default]
    pub on_select_veteran: Callback<u64>,
    #[prop_or_default]
    pub on_clear: Callback<MouseEvent>,
    #[prop_or_default]
    pub on_select_slot_veteran: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub on_select_slot_veteran_api: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub on_open_char_select: Callback<String>,
    #[prop_or(true)]
    pub can_clear: bool,
    #[prop_or_default]
    pub clear_disabled_title: Option<String>,
}

fn lineage_color(slot: LegacyPlannerSlot) -> &'static str {
    match slot {
        LegacyPlannerSlot::ParentA
        | LegacyPlannerSlot::GrandparentAA
        | LegacyPlannerSlot::GrandparentAB => "#3b82f6",
        LegacyPlannerSlot::ParentB
        | LegacyPlannerSlot::GrandparentBA
        | LegacyPlannerSlot::GrandparentBB => "#8b5cf6",
    }
}

fn format_rank(score: u32) -> String {
    let score = i64::from(score);
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

#[function_component]
pub fn LegacyVeteranSlot(props: &LegacyVeteranSlotProps) -> Html {
    let show_detail = use_state(|| false);

    let open_details = {
        let show_detail = show_detail.clone();
        Callback::from(move |_| show_detail.set(true))
    };

    let close_detail = {
        let show_detail = show_detail.clone();
        Callback::from(move |_| show_detail.set(false))
    };

    let on_open_char = {
        let on_open_char_select = props.on_open_char_select.clone();
        let slot_label = props.title.clone();
        Callback::from(move |_: yew::MouseEvent| {
            on_open_char_select.emit(slot_label.clone());
        })
    };

    let accent = lineage_color(props.slot_type);

    let select_button_label = if props.selected.is_some() {
        "Replace"
    } else {
        "Select Veteran"
    };

    html! {
        <div class={LegacyVeteranSlotContainerStyle::CLASS_NAME} style={format!("border-left: 4px solid {};", accent)}>
            <div class={LegacyVeteranSlotCardHeaderStyle::CLASS_NAME}>
                <span
                    class={LegacyVeteranSlotCardTitleStyle::CLASS_NAME}
                    style={format!("background: {};", accent)}
                >
                    {props.title.clone()}
                </span>
                <button
                    onclick={props.on_clear.clone()}
                    class={LegacyVeteranSlotCardClearStyle::CLASS_NAME}
                    disabled={!props.can_clear}
                    title={props.clear_disabled_title.clone().unwrap_or_default()}
                >
                        {"Clear"}
                </button>
            </div>

            <div class={LegacyVeteranSlotBodyStyle::CLASS_NAME}>
                {
                    if let Some(selected) = &props.selected {
                        match selected {
                            LegacySlotValue::LegacyUma(vet) => {
                                let (variant, character_name) = parse_variant_name(&vet.name);
                                html! {
                                    <>
                                        <div style="display: flex; align-items: center; gap: 10px; flex-wrap: wrap; margin-bottom: 10px;">
                                            <div style="display:flex;flex-direction:column;">
                                                {if let Some(v) = &variant {
                                                    html! { <span style="font-size:11px;color:#94a3b8;">{v}</span> }
                                                } else { html! {} }}
                                                    <span style="color:#f3f4f6;font-weight:600;font-size:14px;">{character_name}</span>
                                                </div>
                                                <HashBadge label="VET" hash={vet.hash} title="Click to copy trained chara hash" />
                                                { for vet.min_hash.map(|h| html! {
                                                    <HashBadge label="PRT" hash={h} title="Click to copy parent identity hash" />
                                                }) }
                                                { if vet.is_borrowed {
                                                    vet.owner_id.map(|oid| html! { <OwnerBadge owner_id={oid} /> })
                                                } else {
                                                    None
                                                }}
                                        </div>
                                        <div style="display:flex;align-items:center;gap:8px;flex-wrap:wrap;margin-bottom:10px;">
                                            <span class={classes!(CardRankStyle::CLASS_NAME, vet.is_borrowed.then_some(CardBorrowedStyle::CLASS_NAME))}>
                                                {
                                                    if let Some(rank) = vet.rank {
                                                        html! { <RankBadge rank={UmaRank::from_raw(rank)} /> }
                                                    } else { html! {} }
                                                }
                                                {
                                                    if let Some(score) = vet.rank_score {
                                                        html! { <span class={RankScoreStyle::CLASS_NAME}>{ format_rank(score) }</span> }
                                                    } else { html! {} }
                                                }
                                            </span>
                                            {
                                                if let Some(icon) = vet.favorite_icon {
                                                    html! { <span class={CardFavIconStyle::CLASS_NAME} title="Favourite">{ icon.label() }</span> }
                                                } else { html! {} }
                                            }
                                        </div>
                                        <div class={LegacyVeteranSlotActionsStyle::CLASS_NAME}>
                                            {
                                                if let Some(on_select_slot_veteran) = &props.on_select_slot_veteran {
                                                    html! {
                                                        <>
                                                            <button
                                                                class={HeaderActionButtonStyle::CLASS_NAME}
                                                                onclick={on_select_slot_veteran.clone()}
                                                            >
                                                                {select_button_label}
                                                            </button>
                                                            {
                                                                if let Some(on_api) = &props.on_select_slot_veteran_api {
                                                                    html! {
                                                                        <button
                                                                            class={HeaderActionButtonStyle::CLASS_NAME}
                                                                            onclick={on_api.clone()}
                                                                        >
                                                                            {"API"}
                                                                        </button>
                                                                    }
                                                                } else {
                                                                    html! {}
                                                                }
                                                            }
                                                        </>
                                                    }
                                                } else {
                                                    html! {}
                                                }
                                            }
                                            <button
                                                class={HeaderActionButtonStyle::CLASS_NAME}
                                                onclick={open_details.clone()}
                                            >
                                                {"Details"}
                                            </button>
                                        </div>
                                    </>
                                }
                            }
                            LegacySlotValue::ParentUma(vet) => {
                                let (variant, character_name) = parse_variant_name(&vet.name);
                                html! {
                                    <>
                                        <div style="display: flex; align-items: center; gap: 10px; flex-wrap: wrap; margin-bottom: 10px;">
                                            <div style="display:flex;flex-direction:column;">
                                                {if let Some(v) = &variant {
                                                    html! { <span style="font-size:11px;color:#94a3b8;">{v}</span> }
                                                } else { html! {} }}
                                                <span style="color:#f3f4f6;font-weight:600;font-size:14px;">{character_name}</span>
                                            </div>
                                            <HashBadge label="PRT" hash={vet.hash} title="Click to copy parent identity hash" />
                                            { if vet.is_borrowed {
                                                vet.owner_id.map(|oid| html! { <OwnerBadge owner_id={oid} /> })
                                            } else {
                                                None
                                            }}
                                        </div>
                                        <div style="display:flex;align-items:center;gap:8px;flex-wrap:wrap;margin-bottom:10px;">
                                            <span class={classes!(CardRankStyle::CLASS_NAME, vet.is_borrowed.then_some(CardBorrowedStyle::CLASS_NAME))}>
                                                {
                                                    if let Some(rank) = vet.rank {
                                                        html! { <RankBadge rank={UmaRank::from_raw(rank)} /> }
                                                    } else { html! {} }
                                                }
                                            </span>
                                        </div>
                                        <div class={LegacyVeteranSlotActionsStyle::CLASS_NAME}>
                                            <button
                                                class={HeaderActionButtonStyle::CLASS_NAME}
                                                onclick={open_details.clone()}
                                            >
                                                {"Details"}
                                            </button>
                                        </div>
                                    </>
                                }
                            }
                            LegacySlotValue::Character(character) => {
                                html! {
                                    <>
                                        <div class={LegacyVeteranSlotCharacterNameStyle::CLASS_NAME}>
                                            {"\u{25C7} "}{character.name.clone()}
                                        </div>
                                        <div class={LegacyVeteranSlotCharacterIdStyle::CLASS_NAME}>
                                            {"Character #"}{character.character_id}
                                        </div>
                                        <div class={LegacyVeteranSlotActionsStyle::CLASS_NAME}>
                                            <button
                                                class={HeaderActionButtonStyle::CLASS_NAME}
                                                onclick={on_open_char.clone()}
                                            >
                                                {"Replace"}
                                            </button>
                                        </div>
                                    </>
                                }
                            }
                        }
                    } else {
                        html! {
                            <>
                                <div class={LegacyVeteranSlotActionsStyle::CLASS_NAME}>
                                    {
                                        if let Some(on_select_slot_veteran) = &props.on_select_slot_veteran {
                                            html! {
                                                <>
                                                    <button
                                                        class={HeaderActionButtonStyle::CLASS_NAME}
                                                        onclick={on_select_slot_veteran.clone()}
                                                    >
                                                        {select_button_label}
                                                    </button>
                                                    {
                                                        if let Some(on_api) = &props.on_select_slot_veteran_api {
                                                            html! {
                                                                <button
                                                                    class={HeaderActionButtonStyle::CLASS_NAME}
                                                                    onclick={on_api.clone()}
                                                                >
                                                                    {"API"}
                                                                </button>
                                                            }
                                                        } else {
                                                            html! {}
                                                        }
                                                    }
                                                </>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                    <button
                                        class={HeaderActionButtonStyle::CLASS_NAME}
                                        onclick={on_open_char.clone()}
                                    >
                                        {"Set Character"}
                                    </button>
                                </div>
                            </>
                        }
                    }
                }
            </div>

            {
                if *show_detail {
                    if let Some(selected) = props.selected.clone() {
                        html! {
                            <LegacyDetailModal selected={selected} on_close={close_detail} />
                        }
                    } else {
                        html! {}
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
