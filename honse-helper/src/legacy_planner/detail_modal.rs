use shared::legacy_planner::LegacySlotValue;
use shared::veteran_browser::MajorWinRow;
use yew::prelude::*;

use crate::{
    components::{sparks::SparksList, wins_list::WinsList},
    styles::{
        detail_modal::{
            DetailTabStyle, ModalBodyStyle, ModalCloseStyle, ModalContentStyle, ModalHeaderStyle,
            ModalOverlayStyle, ModalTabsStyle, TabActiveStyle, TabBtnStyle,
        },
        Style,
    },
    tauri_bridge::invoke_tauri_command,
};
use serde_json::json;

#[derive(Properties, PartialEq)]
pub struct LegacyDetailModalProps {
    pub selected: LegacySlotValue,
    pub on_close: Callback<()>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    Sparks,
    MajorWins,
}

#[function_component]
pub fn LegacyDetailModal(props: &LegacyDetailModalProps) -> Html {
    let active_tab = use_state(|| Tab::Sparks);
    let major_wins_data = use_state(Vec::<MajorWinRow>::new);
    let wins_loading = use_state(|| false);

    {
        let selected = props.selected.clone();
        let major_wins_data = major_wins_data.clone();
        let wins_loading = wins_loading.clone();
        use_effect_with((), move |_| {
            wins_loading.set(true);
            match &selected {
                LegacySlotValue::LegacyUma(vet) => {
                    let hash = vet.hash.to_string();
                    let mwd = major_wins_data.clone();
                    let wl = wins_loading.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        match invoke_tauri_command("get_veteran_wins", json!({"hash": hash})).await
                        {
                            Ok(result) => {
                                if let Ok(w) = serde_json::from_value::<Vec<MajorWinRow>>(result) {
                                    mwd.set(w);
                                }
                            }
                            Err(_) => {}
                        }
                        wl.set(false);
                    });
                }
                LegacySlotValue::ParentUma(vet) => {
                    let hash = vet.hash.to_string();
                    let mwd = major_wins_data.clone();
                    let wl = wins_loading.clone();
                    let api_mode = vet.api_mode;
                    wasm_bindgen_futures::spawn_local(async move {
                        let cmd = if api_mode {
                            "get_uma_moe_parent_wins"
                        } else {
                            "get_parent_wins"
                        };
                        match invoke_tauri_command(cmd, json!({"hash": hash})).await {
                            Ok(result) => {
                                if let Ok(w) = serde_json::from_value::<Vec<MajorWinRow>>(result) {
                                    mwd.set(w);
                                }
                            }
                            Err(_) => {}
                        }
                        wl.set(false);
                    });
                }
                LegacySlotValue::Character(_) => {
                    major_wins_data.set(Vec::new());
                    wins_loading.set(false);
                }
            }
            || {}
        });
    }

    let on_tab_click = {
        let active_tab = active_tab.clone();
        Callback::from(move |t: Tab| active_tab.set(t))
    };

    let on_close = {
        let cb = props.on_close.clone();
        Callback::from(move |_| cb.emit(()))
    };

    let selected_name = match &props.selected {
        LegacySlotValue::LegacyUma(vet) => vet.name.clone(),
        LegacySlotValue::ParentUma(vet) => vet.name.clone(),
        LegacySlotValue::Character(character) => character.name.clone(),
    };

    html! {
        <div
            class={ModalOverlayStyle::CLASS_NAME}
            onclick={on_close.clone()}
        >
            <div
                class={ModalContentStyle::CLASS_NAME}
                onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
            >
                <div class={ModalHeaderStyle::CLASS_NAME}>
                    <h2 style="margin: 0;">{selected_name}</h2>
                    <button
                        onclick={on_close.clone()}
                        class={ModalCloseStyle::CLASS_NAME}
                    >
                        {"\u{00D7}"}
                    </button>
                </div>

                <div class={ModalTabsStyle::CLASS_NAME}>
                    {{
                        let is_sparks = *active_tab == Tab::Sparks;
                        html! { <button class={classes!(TabBtnStyle::CLASS_NAME, is_sparks.then_some(TabActiveStyle::CLASS_NAME))}
                            onclick={let t = on_tab_click.clone(); Callback::from(move |_| t.emit(Tab::Sparks))}>
                            {"Sparks"}
                        </button> }
                    }}
                    {{
                        let is_wins = *active_tab == Tab::MajorWins;
                        html! { <button class={classes!(TabBtnStyle::CLASS_NAME, is_wins.then_some(TabActiveStyle::CLASS_NAME))}
                            onclick={let t = on_tab_click.clone(); Callback::from(move |_| t.emit(Tab::MajorWins))}>
                            {"Major Wins"}
                        </button> }
                    }}
                </div>

                <div class={ModalBodyStyle::CLASS_NAME}>
                    { match *active_tab {
                        Tab::Sparks => html! {
                            <div class={DetailTabStyle::CLASS_NAME}>
                                <h3>{"Sparks"}</h3>
                                {
                                    match &props.selected {
                                        LegacySlotValue::LegacyUma(vet) => {
                                            let own_sparks = vet.spark_groups
                                                .iter()
                                                .filter(|sg| sg.trainee_stars_veteran > 0)
                                                .map(|sg| {
                                                    let mut cloned = sg.clone();
                                                    cloned.total_stars = sg.trainee_stars_veteran;
                                                    cloned.trainee_stars_veteran = 0;
                                                    cloned.uma_count = 1;
                                                    cloned
                                                })
                                                .collect::<Vec<_>>();
                                            html! {
                                                <SparksList spark_groups={own_sparks} active_spark_filters={Vec::new()} />
                                            }
                                        },
                                        LegacySlotValue::ParentUma(vet) => html! {
                                            <SparksList spark_groups={vet.spark_groups.clone()} active_spark_filters={Vec::new()} />
                                        },
                                        LegacySlotValue::Character(_) => html! {
                                            <p>{"No spark data."}</p>
                                        },
                                    }
                                }
                            </div>
                        },
                        Tab::MajorWins => html! {
                            <div class={DetailTabStyle::CLASS_NAME}>
                                <h3>{"Major Wins"}</h3>
                                if *wins_loading {
                                    <p>{"Loading..."}</p>
                                } else {
                                    <WinsList wins={(*major_wins_data).clone()} />
                                }
                            </div>
                        },
                    } }
                </div>
            </div>
        </div>
    }
}
