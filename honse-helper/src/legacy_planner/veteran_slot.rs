use shared::legacy_planner::{LegacyPlannerSlot, LegacySlotValue};
use yew::prelude::*;

use crate::{
    components::SelectOption,
    styles::{
        legacy_veteran_slots::{
            LegacyVeteranSlotActionsStyle, LegacyVeteranSlotBodyStyle,
            LegacyVeteranSlotCardClearStyle, LegacyVeteranSlotCardHeaderStyle,
            LegacyVeteranSlotCardTitleStyle, LegacyVeteranSlotCharacterIdStyle,
            LegacyVeteranSlotCharacterNameStyle, LegacyVeteranSlotContainerStyle,
            LegacyVeteranSlotVeteranNameStyle,
        },
        shared_components::HeaderActionButtonStyle,
        veteran_card::CardHashStyle,
        Style,
    },
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
                                html! {
                                    <>
                                        <div style="display: flex; align-items: baseline; gap: 10px; flex-wrap: wrap; margin-bottom: 10px;">
                                            <span class={LegacyVeteranSlotVeteranNameStyle::CLASS_NAME}>
                                                {vet.name.clone()}
                                            </span>
                                            <span class={CardHashStyle::CLASS_NAME} style="font-size: 11px;" title="Copy hash">
                                                {format!("{:016x}", vet.hash)}
                                            </span>
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
                                html! {
                                    <>
                                        <div style="display: flex; align-items: baseline; gap: 10px; flex-wrap: wrap; margin-bottom: 10px;">
                                            <span class={LegacyVeteranSlotVeteranNameStyle::CLASS_NAME}>
                                                {vet.name.clone()}
                                            </span>
                                            <span style="font-size: 11px; color: #888; font-style: italic;">
                                                {"(inherited)"}
                                            </span>
                                            <span class={CardHashStyle::CLASS_NAME} style="font-size: 11px;" title="Copy hash">
                                                {format!("{:016x}", vet.hash)}
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
