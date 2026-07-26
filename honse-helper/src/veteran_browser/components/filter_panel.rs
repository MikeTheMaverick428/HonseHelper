use crate::styles::{filter_panel::*, legacy_planner::SecondaryBtnStyle, Style};
use shared::filters::{AptitudeType, Filter};
use shared::veteran_browser::FilterOptions;
use yew::prelude::*;

use super::custom_select::CustomSelect;
use super::searchable_select::{SearchableSelect, SelectOption};
use super::spark_filter_input::SparkFilterInput;

#[derive(Properties, PartialEq)]
pub struct FilterPanelProps {
    pub filters: Vec<Filter>,
    pub on_change: Callback<Vec<Filter>>,
    pub options: Option<FilterOptions>,
    #[prop_or(false)]
    pub api_mode: bool,
}

enum AddingType {
    None,
    Hash,
    ParentHash,
    HasParent,
    Character,
    Scenario,
    Trainee,
    Ranking,
    SparkBlue,
    SparkPink,
    SparkGreen,
    SparkWhite,
    WhiteSparkCount,
    MajorWinsCount,
    G1Wins,
    SpecificMajorWin,
    Aptitude,
    FavouriteMemo,
    FavouriteIcon,
    BorrowStatus,
    Affinity,
    Tag,
    IsIndependentTrainer,
}

fn filter_description(f: &Filter, options: Option<&FilterOptions>) -> String {
    match f {
        Filter::TraineeHash(h) => format!("Hash: {:016x}", h.as_u64()),
        Filter::ParentHash(h) => format!("Parent: {:016x}", h.as_u64()),
        Filter::HasParent(h) => format!("HasParent: {:016x}", h.as_u64()),
        Filter::Character(id) => {
            if let Some(opts) = options {
                if let Some((_, name)) = opts.characters.iter().find(|(i, _)| i == id) {
                    return format!("Character: {}", name);
                }
            }
            format!("Character: #{}", id)
        }
        Filter::Scenario(s) => {
            let name = options
                .and_then(|opts| opts.scenarios.iter().find(|(id, _)| *id == *s as i64))
                .map(|(_, name)| name.clone())
                .unwrap_or_else(|| s.to_string());
            format!("Scenario: {}", name)
        }
        Filter::Trainee(id) => {
            if let Some(opts) = options {
                if let Some((_, name)) = opts.trainees.iter().find(|(i, _)| i == id) {
                    return format!("Trainee: {}", name);
                }
            }
            format!("Trainee: #{}", id)
        }
        Filter::Ranking { min, max } => {
            let mut s = "Rank".to_string();
            if let Some(m) = min {
                s += &format!(" >= {}", m);
            }
            if let Some(m) = max {
                s += &format!(" <= {}", m);
            }
            s
        }
        Filter::Spark(sp) => {
            let mut s = "Spark".to_string();
            if let Some(opts) = options {
                if let Some(name) = opts.spark_group_name(sp.group_id as i64) {
                    s += &format!(": {}", name);
                } else {
                    s += &format!(": #{}", sp.group_id);
                }
            } else {
                s += &format!(": #{}", sp.group_id);
            }
            if let Some(v) = sp.min_stars {
                s += &format!(" stars>={}", v);
            }
            if let Some(v) = sp.max_stars {
                s += &format!(" stars<={}", v);
            }
            if sp.on_trainee {
                s += " (trainee)";
            }
            if let Some(v) = sp.shared_count {
                s += &format!(" shared>={}", v);
            }
            s
        }
        Filter::WhiteSparkCount { min, max } => {
            let mut s = "White sparks".to_string();
            if let Some(m) = min {
                s += &format!(" >= {}", m);
            }
            if let Some(m) = max {
                s += &format!(" <= {}", m);
            }
            s
        }
        Filter::MajorWinsCount { min, both } => {
            let mut s = "Major wins".to_string();
            if let Some(m) = min {
                s += &format!(" >= {}", m);
            }
            if *both {
                s += " (incl. parents)";
            }
            s
        }
        Filter::G1Wins { min, max } => {
            let mut s = "G1 wins".to_string();
            if let Some(m) = min {
                s += &format!(" >= {}", m);
            }
            if let Some(m) = max {
                s += &format!(" <= {}", m);
            }
            s
        }
        Filter::SpecificMajorWin {
            major_win_id,
            shared_with_parent,
        } => {
            let shared = match shared_with_parent {
                Some(true) => " (shared)",
                _ => "",
            };
            format!("Win #{}{}", major_win_id, shared)
        }
        Filter::Aptitude {
            aptitude_type,
            min_level,
        } => {
            format!("{} >= {}", aptitude_type.label(), min_level)
        }
        Filter::HasFavouriteMemo { search_text } => {
            if let Some(t) = search_text {
                format!("Memo contains \"{}\"", t)
            } else {
                "Has memo".to_string()
            }
        }
        Filter::HasFavouriteIcon { icon_type } => {
            if let Some(i) = icon_type {
                format!("Icon type: {}", i)
            } else {
                "Has icon".to_string()
            }
        }
        Filter::BorrowStatus { is_borrowed } => {
            if *is_borrowed {
                "Borrow: Borrowed".to_string()
            } else {
                "Borrow: Owned".to_string()
            }
        }
        Filter::IsIndependentTrainer { is_independent } => {
            if *is_independent {
                "Indep. Training".to_string()
            } else {
                "Not Indep. Training".to_string()
            }
        }
        Filter::Affinity { min } => format!("Affinity >= {}", min),
        Filter::HasTag { tag_value } => format!("Tag: {}", tag_value),
    }
}

fn id_name_options(items: &[(i64, String)]) -> Vec<SelectOption<i64>> {
    items
        .iter()
        .map(|(id, name)| SelectOption {
            value: *id,
            label: name.clone(),
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn build_add_inputs(
    adding: &AddingType,
    add_hash: &UseStateHandle<String>,
    add_parent_hash: &UseStateHandle<String>,
    add_has_parent_hash: &UseStateHandle<String>,
    add_character_id: &UseStateHandle<Option<i64>>,
    add_scenario: &UseStateHandle<String>,
    add_trainee_id: &UseStateHandle<Option<i64>>,
    add_rank_min: &UseStateHandle<String>,
    add_rank_max: &UseStateHandle<String>,
    add_white_min: &UseStateHandle<String>,
    add_white_max: &UseStateHandle<String>,
    add_wins_min: &UseStateHandle<String>,
    add_wins_both: &UseStateHandle<bool>,
    add_win_id: &UseStateHandle<String>,
    add_win_shared: &UseStateHandle<bool>,
    add_apt_field: &UseStateHandle<String>,
    add_apt_level: &UseStateHandle<String>,
    add_memo_text: &UseStateHandle<String>,
    add_icon_type: &UseStateHandle<String>,
    add_borrow: &UseStateHandle<String>,
    add_indep: &UseStateHandle<String>,
    add_g1_min: &UseStateHandle<String>,
    add_g1_max: &UseStateHandle<String>,
    add_affinity_min: &UseStateHandle<String>,
    add_tag_value: &UseStateHandle<String>,
    add_spark_group: &UseStateHandle<Option<i64>>,
    add_spark_min: &UseStateHandle<String>,
    add_spark_max: &UseStateHandle<String>,
    add_spark_on_character: &UseStateHandle<bool>,
    add_spark_min_uma: &UseStateHandle<String>,
    options: &Option<FilterOptions>,
) -> Option<Html> {
    match adding {
        AddingType::None => None,
        _ => {
            let inputs = match adding {
                AddingType::Hash => html! {
                    <div class={FilterSectionStyle::CLASS_NAME}>
                        <label>{"Hash (hex)"}</label>
                        <input type="text" class={FilterInputStyle::CLASS_NAME} placeholder="e.g. 0a1b2c3d4e5f6a7b"
                            value={(**add_hash).clone()}
                            oninput={let v=add_hash.clone(); Callback::from(move|e:InputEvent| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                    </div>
                },
                AddingType::ParentHash => html! {
                    <div class={FilterSectionStyle::CLASS_NAME}>
                        <label>{"Parent Hash (hex)"}</label>
                        <input type="text" class={FilterInputStyle::CLASS_NAME} placeholder="e.g. 0a1b2c3d4e5f6a7b"
                            value={(**add_parent_hash).clone()}
                            oninput={let v=add_parent_hash.clone(); Callback::from(move|e:InputEvent| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                    </div>
                },
                AddingType::HasParent => html! {
                    <div class={FilterSectionStyle::CLASS_NAME}>
                        <label>{"Has Parent Hash (hex)"}</label>
                        <input type="text" class={FilterInputStyle::CLASS_NAME} placeholder="e.g. 0a1b2c3d4e5f6a7b"
                            value={(**add_has_parent_hash).clone()}
                            oninput={let v=add_has_parent_hash.clone(); Callback::from(move|e:InputEvent| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                    </div>
                },
                AddingType::Character => {
                    let opts = match options {
                        Some(o) => id_name_options(&o.characters),
                        None => Vec::new(),
                    };
                    let on_select = {
                        let v = add_character_id.clone();
                        Callback::from(move |id: i64| v.set(Some(id)))
                    };
                    html! {
                        <div class={FilterSectionStyle::CLASS_NAME}>
                            <label>{"Character"}</label>
                            <SearchableSelect<i64>
                                options={opts}
                                on_select={on_select}
                                selected={**add_character_id}
                                placeholder={"Search character..."}
                            />
                        </div>
                    }
                }
                AddingType::Scenario => {
                    let scenario_opts = match options {
                        Some(o) => o
                            .scenarios
                            .iter()
                            .map(|(id, name)| SelectOption {
                                value: id.to_string(),
                                label: name.clone(),
                            })
                            .collect(),
                        None => Vec::new(),
                    };
                    let on_scenario = {
                        let v = add_scenario.clone();
                        Callback::from(move |val: String| v.set(val))
                    };
                    let scenario_selected = {
                        let v = add_scenario.to_string();
                        if v.is_empty() {
                            None
                        } else {
                            Some(v)
                        }
                    };
                    html! {
                        <div class={FilterSectionStyle::CLASS_NAME}>
                            <label>{"Scenario"}</label>
                            <CustomSelect
                                options={scenario_opts}
                                selected={scenario_selected}
                                on_change={on_scenario}
                                placeholder={"Select..."}
                            />
                        </div>
                    }
                }
                AddingType::Trainee => {
                    let opts = match options {
                        Some(o) => id_name_options(&o.trainees),
                        None => Vec::new(),
                    };
                    let on_select = {
                        let v = add_trainee_id.clone();
                        Callback::from(move |id: i64| v.set(Some(id)))
                    };
                    html! {
                        <div class={FilterSectionStyle::CLASS_NAME}>
                            <label>{"Trainee"}</label>
                            <SearchableSelect<i64>
                                options={opts}
                                on_select={on_select}
                                selected={**add_trainee_id}
                                placeholder={"Search trainee..."}
                            />
                        </div>
                    }
                }
                AddingType::Ranking => html! {
                    <div class={FilterSectionStyle::CLASS_NAME}>
                        <label>{"Rank Score Range"}</label>
                        <div class={FilterRangeStyle::CLASS_NAME}>
                            <input type="number" class={FilterInputStyle::CLASS_NAME} placeholder="Min"
                                value={(**add_rank_min).clone()}
                                oninput={let v=add_rank_min.clone(); Callback::from(move|e:InputEvent| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                            <span class={RangeSepStyle::CLASS_NAME}>{"-"}</span>
                            <input type="number" class={FilterInputStyle::CLASS_NAME} placeholder="Max"
                                value={(**add_rank_max).clone()}
                                oninput={let v=add_rank_max.clone(); Callback::from(move|e:InputEvent| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                        </div>
                    </div>
                },
                AddingType::SparkBlue => {
                    let opts = options
                        .as_ref()
                        .map(|o| o.blue_spark_groups.clone())
                        .unwrap_or_default();
                    let on_group = add_spark_group.clone();
                    let on_min = add_spark_min.clone();
                    let on_max = add_spark_max.clone();
                    let on_char = add_spark_on_character.clone();
                    let on_min_uma = add_spark_min_uma.clone();
                    html! { <SparkFilterInput spark_color="Blue" group_options={opts}
                    group_id={**add_spark_group} on_group_change={Callback::from(move |id| on_group.set(id))}
                    min_stars={(**add_spark_min).clone()} on_min_change={Callback::from(move |s| on_min.set(s))}
                    max_stars={(**add_spark_max).clone()} on_max_change={Callback::from(move |s| on_max.set(s))}
                    on_character={**add_spark_on_character} on_on_character_change={Callback::from(move |v| on_char.set(v))}
                    min_uma={(**add_spark_min_uma).clone()} on_min_uma_change={Callback::from(move |s| on_min_uma.set(s))} /> }
                }
                AddingType::SparkPink => {
                    let opts = options
                        .as_ref()
                        .map(|o| o.pink_spark_groups.clone())
                        .unwrap_or_default();
                    let on_group = add_spark_group.clone();
                    let on_min = add_spark_min.clone();
                    let on_max = add_spark_max.clone();
                    let on_char = add_spark_on_character.clone();
                    let on_min_uma = add_spark_min_uma.clone();
                    html! { <SparkFilterInput spark_color="Pink" group_options={opts}
                    group_id={**add_spark_group} on_group_change={Callback::from(move |id| on_group.set(id))}
                    min_stars={(**add_spark_min).clone()} on_min_change={Callback::from(move |s| on_min.set(s))}
                    max_stars={(**add_spark_max).clone()} on_max_change={Callback::from(move |s| on_max.set(s))}
                    on_character={**add_spark_on_character} on_on_character_change={Callback::from(move |v| on_char.set(v))}
                    min_uma={(**add_spark_min_uma).clone()} on_min_uma_change={Callback::from(move |s| on_min_uma.set(s))} /> }
                }
                AddingType::SparkGreen => {
                    let opts = options
                        .as_ref()
                        .map(|o| o.green_spark_groups.clone())
                        .unwrap_or_default();
                    let on_group = add_spark_group.clone();
                    let on_min = add_spark_min.clone();
                    let on_max = add_spark_max.clone();
                    let on_char = add_spark_on_character.clone();
                    let on_min_uma = add_spark_min_uma.clone();
                    html! { <SparkFilterInput spark_color="Green" group_options={opts}
                    group_id={**add_spark_group} on_group_change={Callback::from(move |id| on_group.set(id))}
                    min_stars={(**add_spark_min).clone()} on_min_change={Callback::from(move |s| on_min.set(s))}
                    max_stars={(**add_spark_max).clone()} on_max_change={Callback::from(move |s| on_max.set(s))}
                    on_character={**add_spark_on_character} on_on_character_change={Callback::from(move |v| on_char.set(v))}
                    min_uma={(**add_spark_min_uma).clone()} on_min_uma_change={Callback::from(move |s| on_min_uma.set(s))} /> }
                }
                AddingType::SparkWhite => {
                    let opts = options
                        .as_ref()
                        .map(|o| o.white_spark_groups.clone())
                        .unwrap_or_default();
                    let on_group = add_spark_group.clone();
                    let on_min = add_spark_min.clone();
                    let on_max = add_spark_max.clone();
                    let on_char = add_spark_on_character.clone();
                    let on_min_uma = add_spark_min_uma.clone();
                    html! { <SparkFilterInput spark_color="White" group_options={opts}
                    group_id={**add_spark_group} on_group_change={Callback::from(move |id| on_group.set(id))}
                    min_stars={(**add_spark_min).clone()} on_min_change={Callback::from(move |s| on_min.set(s))}
                    max_stars={(**add_spark_max).clone()} on_max_change={Callback::from(move |s| on_max.set(s))}
                    on_character={**add_spark_on_character} on_on_character_change={Callback::from(move |v| on_char.set(v))}
                    min_uma={(**add_spark_min_uma).clone()} on_min_uma_change={Callback::from(move |s| on_min_uma.set(s))} /> }
                }
                AddingType::WhiteSparkCount => html! {
                    <div class={FilterSectionStyle::CLASS_NAME}>
                        <label>{"White Spark Count"}</label>
                        <div class={FilterRangeStyle::CLASS_NAME}>
                            <input type="number" class={FilterInputStyle::CLASS_NAME} placeholder="Min"
                                value={(**add_white_min).clone()}
                                oninput={let v=add_white_min.clone(); Callback::from(move|e:InputEvent| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                            <span class={RangeSepStyle::CLASS_NAME}>{"-"}</span>
                            <input type="number" class={FilterInputStyle::CLASS_NAME} placeholder="Max"
                                value={(**add_white_max).clone()}
                                oninput={let v=add_white_max.clone(); Callback::from(move|e:InputEvent| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                        </div>
                    </div>
                },
                AddingType::MajorWinsCount => html! {
                    <>
                        <div class={FilterSectionStyle::CLASS_NAME}>
                            <label>{"Major Win Count (min)"}</label>
                            <input type="number" class={FilterInputStyle::CLASS_NAME} placeholder="Min"
                                value={(**add_wins_min).clone()}
                                oninput={let v=add_wins_min.clone(); Callback::from(move|e:InputEvent| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                        </div>
                        <div class={FilterSectionStyle::CLASS_NAME}>
                            <label>
                                <input type="checkbox" checked={**add_wins_both}
                                    onchange={let v=add_wins_both.clone(); Callback::from(move|e:Event| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().checked()))} />
                                {" Include parent wins"}
                            </label>
                        </div>
                    </>
                },
                AddingType::G1Wins => html! {
                    <div class={FilterSectionStyle::CLASS_NAME}>
                        <label>{"G1 Win Count"}</label>
                        <div class={FilterRangeStyle::CLASS_NAME}>
                            <input type="number" class={FilterInputStyle::CLASS_NAME} placeholder="Min"
                                value={(**add_g1_min).clone()}
                                oninput={let v=add_g1_min.clone(); Callback::from(move|e:InputEvent| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                            <span class={RangeSepStyle::CLASS_NAME}>{"-"}</span>
                            <input type="number" class={FilterInputStyle::CLASS_NAME} placeholder="Max"
                                value={(**add_g1_max).clone()}
                                oninput={let v=add_g1_max.clone(); Callback::from(move|e:InputEvent| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                        </div>
                    </div>
                },
                AddingType::SpecificMajorWin => html! {
                    <>
                        <div class={FilterSectionStyle::CLASS_NAME}>
                            <label>{"Win ID"}</label>
                            <input type="number" class={FilterInputStyle::CLASS_NAME} placeholder="win id"
                                value={(**add_win_id).clone()}
                                oninput={let v=add_win_id.clone(); Callback::from(move|e:InputEvent| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                        </div>
                        <div class={FilterSectionStyle::CLASS_NAME}>
                            <label>
                                <input type="checkbox" checked={**add_win_shared}
                                    onchange={let v=add_win_shared.clone(); Callback::from(move|e:Event| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().checked()))} />
                                {" Shared only (count > 1)"}
                            </label>
                        </div>
                    </>
                },
                AddingType::Aptitude => {
                    let field_opts = vec![
                        SelectOption {
                            value: "Turf".to_string(),
                            label: "Turf".to_string(),
                        },
                        SelectOption {
                            value: "Dirt".to_string(),
                            label: "Dirt".to_string(),
                        },
                        SelectOption {
                            value: "Sprint".to_string(),
                            label: "Sprint".to_string(),
                        },
                        SelectOption {
                            value: "Mile".to_string(),
                            label: "Mile".to_string(),
                        },
                        SelectOption {
                            value: "Medium".to_string(),
                            label: "Medium".to_string(),
                        },
                        SelectOption {
                            value: "Long".to_string(),
                            label: "Long".to_string(),
                        },
                        SelectOption {
                            value: "Front".to_string(),
                            label: "Front".to_string(),
                        },
                        SelectOption {
                            value: "PaceChaser".to_string(),
                            label: "Pace Chaser".to_string(),
                        },
                        SelectOption {
                            value: "LateSurger".to_string(),
                            label: "Late Surger".to_string(),
                        },
                        SelectOption {
                            value: "EndCloser".to_string(),
                            label: "End Closer".to_string(),
                        },
                    ];
                    let level_opts = vec![
                        SelectOption {
                            value: "S".to_string(),
                            label: "S".to_string(),
                        },
                        SelectOption {
                            value: "A".to_string(),
                            label: "A".to_string(),
                        },
                        SelectOption {
                            value: "B".to_string(),
                            label: "B".to_string(),
                        },
                        SelectOption {
                            value: "C".to_string(),
                            label: "C".to_string(),
                        },
                        SelectOption {
                            value: "D".to_string(),
                            label: "D".to_string(),
                        },
                        SelectOption {
                            value: "E".to_string(),
                            label: "E".to_string(),
                        },
                        SelectOption {
                            value: "F".to_string(),
                            label: "F".to_string(),
                        },
                        SelectOption {
                            value: "G".to_string(),
                            label: "G".to_string(),
                        },
                    ];
                    let on_apt_field = {
                        let v = add_apt_field.clone();
                        Callback::from(move |val: String| v.set(val))
                    };
                    let on_apt_level = {
                        let v = add_apt_level.clone();
                        Callback::from(move |val: String| v.set(val))
                    };
                    let apt_field_selected = {
                        let v = add_apt_field.to_string();
                        if v.is_empty() {
                            None
                        } else {
                            Some(v)
                        }
                    };
                    let apt_level_selected = {
                        let v = add_apt_level.to_string();
                        if v.is_empty() {
                            None
                        } else {
                            Some(v)
                        }
                    };
                    html! {
                        <div class={FilterSectionStyle::CLASS_NAME}>
                            <label>{"Aptitude"}</label>
                            <div class={FilterInlineStyle::CLASS_NAME}>
                                <CustomSelect
                                    options={field_opts}
                                    selected={apt_field_selected}
                                    on_change={on_apt_field}
                                    placeholder={"Field"}
                                />
                                <CustomSelect
                                    options={level_opts}
                                    selected={apt_level_selected}
                                    on_change={on_apt_level}
                                    placeholder={"Level"}
                                />
                            </div>
                        </div>
                    }
                }
                AddingType::FavouriteMemo => html! {
                    <div class={FilterSectionStyle::CLASS_NAME}>
                        <label>{"Memo text (optional)"}</label>
                        <input type="text" class={FilterInputStyle::CLASS_NAME} placeholder="search text"
                            value={(**add_memo_text).clone()}
                            oninput={let v=add_memo_text.clone(); Callback::from(move|e:InputEvent| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                    </div>
                },
                AddingType::FavouriteIcon => html! {
                    <div class={FilterSectionStyle::CLASS_NAME}>
                        <label>{"Icon type (optional)"}</label>
                        <input type="text" class={FilterInputStyle::CLASS_NAME} placeholder="icon type"
                            value={(**add_icon_type).clone()}
                            oninput={let v=add_icon_type.clone(); Callback::from(move|e:InputEvent| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                    </div>
                },
                AddingType::BorrowStatus => {
                    let borrow_opts = vec![
                        SelectOption {
                            value: "Owned".to_string(),
                            label: "Owned".to_string(),
                        },
                        SelectOption {
                            value: "Borrowed".to_string(),
                            label: "Borrowed".to_string(),
                        },
                    ];
                    let on_borrow = {
                        let v = add_borrow.clone();
                        Callback::from(move |val: String| v.set(val))
                    };
                    let borrow_selected = {
                        let v = add_borrow.to_string();
                        if v.is_empty() {
                            None
                        } else {
                            Some(v)
                        }
                    };
                    html! {
                        <div class={FilterSectionStyle::CLASS_NAME}>
                            <label>{"Borrow Status"}</label>
                            <CustomSelect
                                options={borrow_opts}
                                selected={borrow_selected}
                                on_change={on_borrow}
                                placeholder={"Select..."}
                            />
                        </div>
                    }
                }
                AddingType::IsIndependentTrainer => {
                    let indep_opts = vec![
                        SelectOption {
                            value: "Yes".to_string(),
                            label: "Indep. Training".to_string(),
                        },
                        SelectOption {
                            value: "No".to_string(),
                            label: "Not Indep. Training".to_string(),
                        },
                    ];
                    let on_indep = {
                        let v = add_indep.clone();
                        Callback::from(move |val: String| v.set(val))
                    };
                    let indep_selected = {
                        let v = add_indep.to_string();
                        if v.is_empty() {
                            None
                        } else {
                            Some(v)
                        }
                    };
                    html! {
                        <div class={FilterSectionStyle::CLASS_NAME}>
                            <label>{"Independent Training"}</label>
                            <CustomSelect
                                options={indep_opts}
                                selected={indep_selected}
                                on_change={on_indep}
                                placeholder={"Select..."}
                            />
                        </div>
                    }
                }
                AddingType::Affinity => html! {
                    <div class={FilterSectionStyle::CLASS_NAME}>
                        <label>{"Min Affinity"}</label>
                        <input type="number" class={FilterInputStyle::CLASS_NAME} placeholder="min affinity"
                            value={(**add_affinity_min).clone()}
                            oninput={let v=add_affinity_min.clone(); Callback::from(move|e:InputEvent| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                    </div>
                },
                AddingType::Tag => {
                    let tag_opts = match options {
                        Some(o) => o
                            .tags
                            .iter()
                            .map(|t| SelectOption {
                                value: t.clone(),
                                label: t.clone(),
                            })
                            .collect(),
                        None => Vec::new(),
                    };
                    let on_select = {
                        let v = add_tag_value.clone();
                        Callback::from(move |val: String| v.set(val))
                    };
                    let selected: Option<String> = if add_tag_value.is_empty() {
                        None
                    } else {
                        Some((**add_tag_value).clone())
                    };
                    html! {
                        <div class={FilterSectionStyle::CLASS_NAME}>
                            <label>{"Tag"}</label>
                            <SearchableSelect<String>
                                options={tag_opts}
                                on_select={on_select}
                                selected={selected}
                                placeholder={"Search tag..."}
                            />
                        </div>
                    }
                }
                AddingType::None => unreachable!(),
            };
            Some(inputs)
        }
    }
}

#[function_component]
pub fn FilterPanel(props: &FilterPanelProps) -> Html {
    let adding = use_state(|| AddingType::None);
    let add_hash = use_state(String::new);
    let add_parent_hash = use_state(String::new);
    let add_character_id: UseStateHandle<Option<i64>> = use_state(|| None);
    let add_scenario = use_state(String::new);
    let add_trainee_id: UseStateHandle<Option<i64>> = use_state(|| None);
    let add_rank_min = use_state(String::new);
    let add_rank_max = use_state(String::new);
    let add_white_min = use_state(String::new);
    let add_white_max = use_state(String::new);
    let add_spark_group = use_state(|| None::<i64>);
    let add_spark_min = use_state(String::new);
    let add_spark_max = use_state(String::new);
    let add_wins_min = use_state(String::new);
    let add_wins_both = use_state(|| false);
    let add_win_id = use_state(String::new);
    let add_win_shared = use_state(|| false);
    let add_apt_field = use_state(String::new);
    let add_apt_level = use_state(String::new);
    let add_memo_text = use_state(String::new);
    let add_icon_type = use_state(String::new);
    let add_borrow = use_state(String::new);
    let add_indep = use_state(String::new);
    let add_g1_min = use_state(String::new);
    let add_g1_max = use_state(String::new);
    let add_affinity_min = use_state(String::new);
    let add_tag_value = use_state(String::new);
    let add_filter_type: UseStateHandle<String> = use_state(String::new);
    let add_spark_on_character = use_state(|| false);
    let add_spark_min_uma = use_state(String::new);
    let add_has_parent_hash = use_state(String::new);

    let cancel_adding = {
        let adding = adding.clone();
        let add_character_id = add_character_id.clone();
        let add_trainee_id = add_trainee_id.clone();
        let add_tag_value = add_tag_value.clone();
        let add_filter_type = add_filter_type.clone();
        let add_spark_group = add_spark_group.clone();
        let add_spark_min = add_spark_min.clone();
        let add_spark_max = add_spark_max.clone();
        let add_spark_on_character = add_spark_on_character.clone();
        let add_spark_min_uma = add_spark_min_uma.clone();
        let add_has_parent_hash = add_has_parent_hash.clone();
        Callback::from(move |_| {
            adding.set(AddingType::None);
            add_character_id.set(None);
            add_trainee_id.set(None);
            add_tag_value.set(String::new());
            add_filter_type.set(String::new());
            add_spark_group.set(None);
            add_spark_min.set(String::new());
            add_spark_max.set(String::new());
            add_spark_on_character.set(false);
            add_spark_min_uma.set(String::new());
            add_has_parent_hash.set(String::new());
        })
    };

    let reset_states = {
        let add_character_id = add_character_id.clone();
        let add_trainee_id = add_trainee_id.clone();
        let add_g1_min = add_g1_min.clone();
        let add_g1_max = add_g1_max.clone();
        let add_affinity_min = add_affinity_min.clone();
        let add_tag_value = add_tag_value.clone();
        let add_spark_group = add_spark_group.clone();
        let add_spark_min = add_spark_min.clone();
        let add_spark_max = add_spark_max.clone();
        let add_spark_on_character = add_spark_on_character.clone();
        let add_spark_min_uma = add_spark_min_uma.clone();
        let add_has_parent_hash = add_has_parent_hash.clone();
        move || {
            add_character_id.set(None);
            add_trainee_id.set(None);
            add_g1_min.set(String::new());
            add_g1_max.set(String::new());
            add_affinity_min.set(String::new());
            add_tag_value.set(String::new());
            add_spark_group.set(None);
            add_spark_min.set(String::new());
            add_spark_max.set(String::new());
            add_spark_on_character.set(false);
            add_spark_min_uma.set(String::new());
            add_has_parent_hash.set(String::new());
        }
    };

    let on_change = props.on_change.clone();

    let add_filter = {
        let on_change = on_change.clone();
        let adding = adding.clone();
        let filters = props.filters.clone();
        let add_hash = add_hash.clone();
        let add_character_id = add_character_id.clone();
        let add_scenario = add_scenario.clone();
        let add_trainee_id = add_trainee_id.clone();
        let add_rank_min = add_rank_min.clone();
        let add_rank_max = add_rank_max.clone();
        let add_white_min = add_white_min.clone();
        let add_white_max = add_white_max.clone();
        let add_wins_min = add_wins_min.clone();
        let add_wins_both = add_wins_both.clone();
        let add_win_id = add_win_id.clone();
        let add_win_shared = add_win_shared.clone();
        let add_apt_field = add_apt_field.clone();
        let add_apt_level = add_apt_level.clone();
        let add_memo_text = add_memo_text.clone();
        let add_icon_type = add_icon_type.clone();
        let add_borrow = add_borrow.clone();
        let add_indep = add_indep.clone();
        let add_g1_min = add_g1_min.clone();
        let add_g1_max = add_g1_max.clone();
        let add_affinity_min = add_affinity_min.clone();
        let add_tag_value = add_tag_value.clone();
        let add_filter_type = add_filter_type.clone();
        let add_spark_group = add_spark_group.clone();
        let add_spark_min = add_spark_min.clone();
        let add_spark_max = add_spark_max.clone();
        let add_spark_on_character = add_spark_on_character.clone();
        let add_spark_min_uma = add_spark_min_uma.clone();
        let add_parent_hash = add_parent_hash.clone();
        let add_has_parent_hash = add_has_parent_hash.clone();
        let reset = reset_states.clone();
        Callback::from(move |_| {
            let new_filter = match &*adding {
                AddingType::Hash => u64::from_str_radix(&add_hash, 16)
                    .ok()
                    .map(|h| Filter::TraineeHash(h.into())),
                AddingType::ParentHash => u64::from_str_radix(&add_parent_hash, 16)
                    .ok()
                    .map(|h| Filter::ParentHash(h.into())),
                AddingType::HasParent => u64::from_str_radix(&add_has_parent_hash, 16)
                    .ok()
                    .map(|h| Filter::HasParent(h.into())),
                AddingType::Character => (*add_character_id).map(Filter::Character),
                AddingType::Scenario => {
                    let v = (*add_scenario).clone();
                    if v.is_empty() {
                        None
                    } else {
                        v.parse::<u16>().ok().map(Filter::Scenario)
                    }
                }
                AddingType::Trainee => (*add_trainee_id).map(Filter::Trainee),
                AddingType::Ranking => Some(Filter::Ranking {
                    min: add_rank_min.parse().ok(),
                    max: add_rank_max.parse().ok(),
                }),
                AddingType::WhiteSparkCount => Some(Filter::WhiteSparkCount {
                    min: add_white_min.parse().ok(),
                    max: add_white_max.parse().ok(),
                }),
                AddingType::MajorWinsCount => Some(Filter::MajorWinsCount {
                    min: add_wins_min.parse().ok(),
                    both: *add_wins_both,
                }),
                AddingType::G1Wins => Some(Filter::G1Wins {
                    min: add_g1_min.parse().ok(),
                    max: add_g1_max.parse().ok(),
                }),
                AddingType::SpecificMajorWin => {
                    add_win_id
                        .parse::<i64>()
                        .ok()
                        .map(|id| Filter::SpecificMajorWin {
                            major_win_id: id,
                            shared_with_parent: if *add_win_shared { Some(true) } else { None },
                        })
                }
                AddingType::Aptitude => {
                    let field = (*add_apt_field).clone();
                    let level = (*add_apt_level).clone();
                    if field.is_empty() || level.is_empty() {
                        None
                    } else {
                        AptitudeType::from_str(&field).map(|apt_type| Filter::Aptitude {
                            aptitude_type: apt_type,
                            min_level: match level.as_str() {
                                "S" => 8,
                                "A" => 7,
                                "B" => 6,
                                "C" => 5,
                                "D" => 4,
                                "E" => 3,
                                "F" => 2,
                                "G" => 1,
                                _ => 0,
                            },
                        })
                    }
                }
                AddingType::FavouriteMemo => {
                    let text = (*add_memo_text).clone();
                    Some(Filter::HasFavouriteMemo {
                        search_text: if text.is_empty() { None } else { Some(text) },
                    })
                }
                AddingType::FavouriteIcon => {
                    let icon = (*add_icon_type).clone();
                    Some(Filter::HasFavouriteIcon {
                        icon_type: if icon.is_empty() {
                            None
                        } else {
                            icon.parse::<i16>().ok()
                        },
                    })
                }
                AddingType::BorrowStatus => {
                    let v = (*add_borrow).clone();
                    if v.is_empty() {
                        None
                    } else {
                        Some(Filter::BorrowStatus {
                            is_borrowed: v == "Borrowed",
                        })
                    }
                }
                AddingType::IsIndependentTrainer => {
                    let v = (*add_indep).clone();
                    if v.is_empty() {
                        None
                    } else {
                        Some(Filter::IsIndependentTrainer {
                            is_independent: v == "Yes",
                        })
                    }
                }
                AddingType::Affinity => add_affinity_min
                    .parse::<u32>()
                    .ok()
                    .map(|min| Filter::Affinity { min }),
                AddingType::Tag => {
                    let v = (*add_tag_value).clone();
                    if v.is_empty() {
                        None
                    } else {
                        Some(Filter::HasTag { tag_value: v })
                    }
                }
                AddingType::None => None,
                AddingType::SparkBlue
                | AddingType::SparkPink
                | AddingType::SparkGreen
                | AddingType::SparkWhite => {
                    let spark_type = match &*adding {
                        AddingType::SparkBlue => 1,
                        AddingType::SparkPink => 2,
                        AddingType::SparkGreen => 3,
                        AddingType::SparkWhite => 4,
                        _ => unreachable!(),
                    };
                    add_spark_group.map(|gid| {
                        Filter::Spark(shared::filters::SparkFilter {
                            group_id: gid as i32,
                            min_stars: add_spark_min.parse::<i32>().ok(),
                            max_stars: add_spark_max.parse::<i32>().ok(),
                            on_trainee: *add_spark_on_character,
                            shared_count: add_spark_min_uma.parse::<i8>().ok().filter(|v| *v > 0),
                            spark_type: Some(spark_type),
                        })
                    })
                }
            };
            if let Some(f) = new_filter {
                let mut updated = filters.clone();
                updated.push(f);
                on_change.emit(updated);
            }
            adding.set(AddingType::None);
            add_filter_type.set(String::new());
            reset();
        })
    };

    let remove_filter = {
        let on_change = on_change.clone();
        let filters = props.filters.clone();
        Callback::from(move |idx: usize| {
            let mut updated = filters.clone();
            updated.remove(idx);
            on_change.emit(updated);
        })
    };

    let add_ui = build_add_inputs(
        &adding,
        &add_hash,
        &add_parent_hash,
        &add_has_parent_hash,
        &add_character_id,
        &add_scenario,
        &add_trainee_id,
        &add_rank_min,
        &add_rank_max,
        &add_white_min,
        &add_white_max,
        &add_wins_min,
        &add_wins_both,
        &add_win_id,
        &add_win_shared,
        &add_apt_field,
        &add_apt_level,
        &add_memo_text,
        &add_icon_type,
        &add_borrow,
        &add_indep,
        &add_g1_min,
        &add_g1_max,
        &add_affinity_min,
        &add_tag_value,
        &add_spark_group,
        &add_spark_min,
        &add_spark_max,
        &add_spark_on_character,
        &add_spark_min_uma,
        &props.options,
    );
    let add_ui = match add_ui {
        Some(inputs) => html! {
            <div style="margin-top:8px;">
                {inputs}
                <div class={FilterActionsStyle::CLASS_NAME} style="margin-top:8px;">
                    <button onclick={add_filter}>{"Add"}</button>
                    <button class={SecondaryBtnStyle::CLASS_NAME} onclick={cancel_adding}>{"Cancel"}</button>
                </div>
            </div>
        },
        None => html! {},
    };

    let filter_type_selected = {
        let v = add_filter_type.to_string();
        if v.is_empty() {
            None
        } else {
            Some(v)
        }
    };

    let display_indices: Vec<usize> = if props.api_mode {
        props
            .filters
            .iter()
            .enumerate()
            .filter(|(_, f)| !matches!(f, Filter::HasTag { .. }))
            .map(|(i, _)| i)
            .collect()
    } else {
        (0..props.filters.len()).collect()
    };

    html! {
        <div class={FilterPanelStyle::CLASS_NAME}>
            <div class={FilterTitleStyle::CLASS_NAME}>{"Active Filters"}</div>

            if display_indices.is_empty() {
                <div style="color:#64748b;font-size:12px;margin-bottom:12px;">
                    {"No filters — showing all veterans"}
                </div>
            } else {
                <div style="margin-bottom:12px;">
                    {for display_indices.iter().map(|&i| {
                        let f = &props.filters[i];
                        let desc = filter_description(f, props.options.as_ref());
                        let onclick = {
                            let remove_filter = remove_filter.clone();
                            Callback::from(move |_| remove_filter.emit(i))
                        };
                        html! {
                            <div key={i} class={FilterChipStyle::CLASS_NAME} style="display:flex;align-items:center;gap:6px;background:#1e293b;border-radius:6px;padding:4px 10px;margin-bottom:4px;font-size:12px;">
                                <span style="flex:1;color:#e2e8f0;">{desc}</span>
                                <button onclick={onclick} style="background:none;border:none;color:#94a3b8;cursor:pointer;padding:0;font-size:14px;line-height:1;">{"\u{00D7}"}</button>
                            </div>
                        }
                    })}
                </div>
            }

            <div class={FilterTitleStyle::CLASS_NAME}>{"Add Filter"}</div>
            <div class={FilterSectionStyle::CLASS_NAME}>
                <SearchableSelect<String>
                    options={
                        if props.api_mode {
                            vec![
                                SelectOption { value: "trainee".to_string(), label: "Trainee".to_string() },
                                SelectOption { value: "rank".to_string(), label: "Rank Score".to_string() },
                                SelectOption { value: "spark_blue".to_string(), label: "Blue Spark".to_string() },
                                SelectOption { value: "spark_pink".to_string(), label: "Pink Spark".to_string() },
                                SelectOption { value: "spark_green".to_string(), label: "Green Spark".to_string() },
                                SelectOption { value: "spark_white".to_string(), label: "White Spark".to_string() },
                                SelectOption { value: "white_spark".to_string(), label: "White Spark Count".to_string() },
                                SelectOption { value: "wins".to_string(), label: "Major Win Count".to_string() },
                                SelectOption { value: "affinity".to_string(), label: "Affinity".to_string() },
                            ]
                        } else {
                            vec![
                                SelectOption { value: "hash".to_string(), label: "Hash".to_string() },
                                SelectOption { value: "parent_hash".to_string(), label: "Parent Hash".to_string() },
                                SelectOption { value: "character".to_string(), label: "Character".to_string() },
                                SelectOption { value: "scenario".to_string(), label: "Scenario".to_string() },
                                SelectOption { value: "trainee".to_string(), label: "Trainee".to_string() },
                                SelectOption { value: "rank".to_string(), label: "Rank Score".to_string() },
                                SelectOption { value: "spark_blue".to_string(), label: "Blue Spark".to_string() },
                                SelectOption { value: "spark_pink".to_string(), label: "Pink Spark".to_string() },
                                SelectOption { value: "spark_green".to_string(), label: "Green Spark".to_string() },
                                SelectOption { value: "spark_white".to_string(), label: "White Spark".to_string() },
                                SelectOption { value: "white_spark".to_string(), label: "White Spark Count".to_string() },
                                SelectOption { value: "wins".to_string(), label: "Major Win Count".to_string() },
                                SelectOption { value: "g1_wins".to_string(), label: "G1 Win Count".to_string() },
                                SelectOption { value: "specific_win".to_string(), label: "Specific Major Win".to_string() },
                                SelectOption { value: "apt".to_string(), label: "Aptitude".to_string() },
                                SelectOption { value: "memo".to_string(), label: "Favourite Memo".to_string() },
                                SelectOption { value: "icon".to_string(), label: "Favourite Icon".to_string() },
                                SelectOption { value: "borrow".to_string(), label: "Borrow Status".to_string() },
                                SelectOption { value: "indep".to_string(), label: "Independent Training".to_string() },
                                SelectOption { value: "affinity".to_string(), label: "Affinity".to_string() },
                                SelectOption { value: "tag".to_string(), label: "Tag".to_string() },
                                SelectOption { value: "has_parent".to_string(), label: "Has Parent".to_string() },
                            ]
                        }
                    }
                    selected={filter_type_selected}
                    on_select={let a = adding.clone(); let ft = add_filter_type.clone(); Callback::from(move |val: String| {
                        ft.set(val.clone());
                        a.set(match val.as_str() {
                            "hash" => AddingType::Hash,
                            "parent_hash" => AddingType::ParentHash,
                            "has_parent" => AddingType::HasParent,
                            "character" => AddingType::Character,
                            "scenario" => AddingType::Scenario,
                            "trainee" => AddingType::Trainee,
                            "rank" => AddingType::Ranking,
                            "spark_blue" => AddingType::SparkBlue,
                            "spark_pink" => AddingType::SparkPink,
                            "spark_green" => AddingType::SparkGreen,
                            "spark_white" => AddingType::SparkWhite,
                            "white_spark" => AddingType::WhiteSparkCount,
                            "wins" => AddingType::MajorWinsCount,
                            "g1_wins" => AddingType::G1Wins,
                            "specific_win" => AddingType::SpecificMajorWin,
                            "apt" => AddingType::Aptitude,
                            "memo" => AddingType::FavouriteMemo,
                            "icon" => AddingType::FavouriteIcon,
                            "borrow" => AddingType::BorrowStatus,
                            "indep" => AddingType::IsIndependentTrainer,
                            "affinity" => AddingType::Affinity,
                            "tag" => AddingType::Tag,
                            _ => AddingType::None,
                        });
                    })}
                    placeholder={"Select type..."}
                />
            </div>

            {add_ui}
        </div>
    }
}
