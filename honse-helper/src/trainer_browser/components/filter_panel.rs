use crate::styles::{
    filter_panel::*,
    legacy_planner::SecondaryBtnStyle,
    trainer_browser::TrainerIdListStyle,
    worker_status::{ToggleCheckboxStyle, ToggleLabelStyle},
    Style,
};
use shared::trainer_browser::{TrainerFilter, TrainerFilterOptions};
use yew::prelude::*;

use crate::veteran_browser::components::custom_select::CustomSelect;
use crate::veteran_browser::components::searchable_select::{SearchableSelect, SelectOption};

fn id_name_options(items: &[(i64, String)]) -> Vec<SelectOption<i64>> {
    items
        .iter()
        .map(|(id, name)| SelectOption {
            value: *id,
            label: name.clone(),
        })
        .collect()
}

fn checkbox_input(checked: bool, label: &str, on_change: Callback<bool>) -> Html {
    let onchange = Callback::from(move |e: web_sys::Event| {
        on_change.emit(e.target_unchecked_into::<web_sys::HtmlInputElement>().checked());
    });
    html! {
        <label class={ToggleLabelStyle::CLASS_NAME} style="display:flex;align-items:center;gap:6px;margin-top:4px;cursor:pointer;">
            <input type="checkbox" class={ToggleCheckboxStyle::CLASS_NAME} checked={checked} onchange={onchange} />
            {label}
        </label>
    }
}

#[derive(Properties, PartialEq)]
pub struct TrainerFilterPanelProps {
    pub filters: Vec<TrainerFilter>,
    pub on_change: Callback<Vec<TrainerFilter>>,
    pub options: TrainerFilterOptions,
}

enum AddingType {
    None,
    Name,
    Following,
    VeteranTrainee,
    VeteranRank,
    ScType,
    ScRarity,
    ScLimitBreak,
    ScCharacter,
}

fn filter_description(f: &TrainerFilter, options: &TrainerFilterOptions) -> String {
    match f {
        TrainerFilter::NameSearch { query } => format!("Name: \"{}\"", query),
        TrainerFilter::Following { is_following } => {
            if *is_following {
                "Following only".to_string()
            } else {
                "Not following".to_string()
            }
        }
        TrainerFilter::VeteranTrainee { ids, negate } => {
            let names: Vec<String> = ids
                .iter()
                .map(|id| {
                    options
                        .characters
                        .iter()
                        .find(|(cid, _)| cid == id)
                        .map(|(_, l)| l.clone())
                        .unwrap_or_else(|| id.to_string())
                })
                .collect();
            let prefix = if *negate { "Not chara: " } else { "Chara: " };
            format!("{}{}", prefix, names.join(", "))
        }
        TrainerFilter::VeteranRank { min } => format!("Rank >= {}", min),
        TrainerFilter::ScType { card_types } => {
            let labels: Vec<String> = card_types
                .iter()
                .map(|id| {
                    options
                        .card_types
                        .iter()
                        .find(|(ct, _)| ct == id)
                        .map(|(_, l)| l.clone())
                        .unwrap_or_else(|| id.to_string())
                })
                .collect();
            format!("SC Type: {}", labels.join(", "))
        }
        TrainerFilter::ScRarity { rarities } => {
            let labels: Vec<String> = rarities
                .iter()
                .map(|id| {
                    options
                        .rarities
                        .iter()
                        .find(|(r, _)| r == id)
                        .map(|(_, l)| l.clone())
                        .unwrap_or_else(|| id.to_string())
                })
                .collect();
            format!("SC Rarity: {}", labels.join(", "))
        }
        TrainerFilter::ScLimitBreak { min, max } => format!("SC LB: {}–{}", min, max),
        TrainerFilter::ScCharacter { character_ids } => {
            let names: Vec<String> = character_ids
                .iter()
                .map(|id| {
                    options
                        .characters
                        .iter()
                        .find(|(cid, _)| cid == id)
                        .map(|(_, l)| l.clone())
                        .unwrap_or_else(|| id.to_string())
                })
                .collect();
            format!("SC Chara: {}", names.join(", "))
        }
    }
}

fn filter_to_adding_type(f: &TrainerFilter) -> Option<(&'static str, AddingType)> {
    match f {
        TrainerFilter::NameSearch { .. } => Some(("name", AddingType::Name)),
        TrainerFilter::Following { .. } => Some(("following", AddingType::Following)),
        TrainerFilter::VeteranTrainee { .. } => Some(("veteran_trainee", AddingType::VeteranTrainee)),
        TrainerFilter::VeteranRank { .. } => Some(("veteran_rank", AddingType::VeteranRank)),
        TrainerFilter::ScType { .. } => Some(("sc_type", AddingType::ScType)),
        TrainerFilter::ScRarity { .. } => Some(("sc_rarity", AddingType::ScRarity)),
        TrainerFilter::ScLimitBreak { .. } => Some(("sc_limit_break", AddingType::ScLimitBreak)),
        TrainerFilter::ScCharacter { .. } => Some(("sc_character", AddingType::ScCharacter)),
    }
}

#[function_component]
pub fn TrainerFilterPanel(props: &TrainerFilterPanelProps) -> Html {
    let adding = use_state(|| AddingType::None);
    let add_name = use_state(String::new);
    let add_following: UseStateHandle<bool> = use_state(|| true);
    let add_veteran_trainee_ids: UseStateHandle<Vec<i64>> = use_state(Vec::new);
    let add_veteran_trainee_negate: UseStateHandle<bool> = use_state(|| false);
    let add_veteran_rank = use_state(|| String::new());
    let add_sc_type_ids: UseStateHandle<Vec<i64>> = use_state(Vec::new);
    let add_sc_rarity_ids: UseStateHandle<Vec<i64>> = use_state(Vec::new);
    let add_sc_lb_min = use_state(|| String::new());
    let add_sc_lb_max = use_state(|| String::new());
    let add_sc_character_ids: UseStateHandle<Vec<i64>> = use_state(Vec::new);
    let add_filter_type: UseStateHandle<String> = use_state(String::new);
    let editing_idx: UseStateHandle<Option<usize>> = use_state(|| None);

    let on_change = props.on_change.clone();

    let reset_all_inputs = {
        let add_name = add_name.clone();
        let add_following = add_following.clone();
        let add_veteran_trainee_ids = add_veteran_trainee_ids.clone();
        let add_veteran_trainee_negate = add_veteran_trainee_negate.clone();
        let add_veteran_rank = add_veteran_rank.clone();
        let add_sc_type_ids = add_sc_type_ids.clone();
        let add_sc_rarity_ids = add_sc_rarity_ids.clone();
        let add_sc_lb_min = add_sc_lb_min.clone();
        let add_sc_lb_max = add_sc_lb_max.clone();
        let add_sc_character_ids = add_sc_character_ids.clone();
        Callback::from(move |_| {
            add_name.set(String::new());
            add_following.set(true);
            add_veteran_trainee_ids.set(Vec::new());
            add_veteran_trainee_negate.set(false);
            add_veteran_rank.set(String::new());
            add_sc_type_ids.set(Vec::new());
            add_sc_rarity_ids.set(Vec::new());
            add_sc_lb_min.set(String::new());
            add_sc_lb_max.set(String::new());
            add_sc_character_ids.set(Vec::new());
        })
    };

    let open_edit = {
        let filters = props.filters.clone();
        let adding = adding.clone();
        let add_filter_type = add_filter_type.clone();
        let editing_idx = editing_idx.clone();
        let reset_all_inputs = reset_all_inputs.clone();
        let add_name = add_name.clone();
        let add_following = add_following.clone();
        let add_veteran_trainee_ids = add_veteran_trainee_ids.clone();
        let add_veteran_trainee_negate = add_veteran_trainee_negate.clone();
        let add_veteran_rank = add_veteran_rank.clone();
        let add_sc_type_ids = add_sc_type_ids.clone();
        let add_sc_rarity_ids = add_sc_rarity_ids.clone();
        let add_sc_lb_min = add_sc_lb_min.clone();
        let add_sc_lb_max = add_sc_lb_max.clone();
        let add_sc_character_ids = add_sc_character_ids.clone();
        Callback::from(move |idx: usize| {
            let Some(f) = filters.get(idx) else { return };
            let Some((value, t)) = filter_to_adding_type(f) else { return };
            reset_all_inputs.emit(());
            adding.set(t);
            add_filter_type.set(value.to_string());
            editing_idx.set(Some(idx));
            match f {
                TrainerFilter::NameSearch { query } => add_name.set(query.clone()),
                TrainerFilter::Following { is_following } => add_following.set(*is_following),
                TrainerFilter::VeteranTrainee { ids, negate } => {
                    add_veteran_trainee_ids.set(ids.clone());
                    add_veteran_trainee_negate.set(*negate);
                }
                TrainerFilter::VeteranRank { min } => add_veteran_rank.set(min.to_string()),
                TrainerFilter::ScType { card_types } => add_sc_type_ids.set(card_types.clone()),
                TrainerFilter::ScRarity { rarities } => add_sc_rarity_ids.set(rarities.clone()),
                TrainerFilter::ScLimitBreak { min, max } => {
                    add_sc_lb_min.set(min.to_string());
                    add_sc_lb_max.set(max.to_string());
                }
                TrainerFilter::ScCharacter { character_ids } => {
                    add_sc_character_ids.set(character_ids.clone());
                }
            }
        })
    };

    let add_filter = {
        let on_change = on_change.clone();
        let adding = adding.clone();
        let filters = props.filters.clone();
        let add_name = add_name.clone();
        let add_following = add_following.clone();
        let add_veteran_trainee_ids = add_veteran_trainee_ids.clone();
        let add_veteran_trainee_negate = add_veteran_trainee_negate.clone();
        let add_veteran_rank = add_veteran_rank.clone();
        let add_sc_type_ids = add_sc_type_ids.clone();
        let add_sc_rarity_ids = add_sc_rarity_ids.clone();
        let add_sc_lb_min = add_sc_lb_min.clone();
        let add_sc_lb_max = add_sc_lb_max.clone();
        let add_sc_character_ids = add_sc_character_ids.clone();
        let add_filter_type = add_filter_type.clone();
        let editing_idx = editing_idx.clone();
        let reset_all_inputs = reset_all_inputs.clone();
        Callback::from(move |_| {
            let new_filter = match &*adding {
                AddingType::Name => {
                    let text = (*add_name).clone();
                    if text.is_empty() {
                        None
                    } else {
                        Some(TrainerFilter::NameSearch { query: text })
                    }
                }
                AddingType::Following => {
                    Some(TrainerFilter::Following { is_following: *add_following })
                }
                AddingType::VeteranTrainee => {
                    let ids = (*add_veteran_trainee_ids).clone();
                    if ids.is_empty() {
                        None
                    } else {
                        Some(TrainerFilter::VeteranTrainee {
                            ids,
                            negate: *add_veteran_trainee_negate,
                        })
                    }
                }
                AddingType::VeteranRank => match (*add_veteran_rank).parse::<i64>() {
                    Ok(min) => Some(TrainerFilter::VeteranRank { min }),
                    Err(_) => None,
                },
                AddingType::ScType => {
                    let card_types = (*add_sc_type_ids).clone();
                    if card_types.is_empty() {
                        None
                    } else {
                        Some(TrainerFilter::ScType { card_types })
                    }
                }
                AddingType::ScRarity => {
                    let rarities = (*add_sc_rarity_ids).clone();
                    if rarities.is_empty() {
                        None
                    } else {
                        Some(TrainerFilter::ScRarity { rarities })
                    }
                }
                AddingType::ScLimitBreak => {
                    let min = (*add_sc_lb_min).parse::<i64>().ok().unwrap_or(0);
                    let max = (*add_sc_lb_max).parse::<i64>().ok().unwrap_or(4);
                    Some(TrainerFilter::ScLimitBreak { min, max })
                }
                AddingType::ScCharacter => {
                    let character_ids = (*add_sc_character_ids).clone();
                    if character_ids.is_empty() {
                        None
                    } else {
                        Some(TrainerFilter::ScCharacter { character_ids })
                    }
                }
                AddingType::None => None,
            };
            if let Some(f) = new_filter {
                let mut updated = filters.clone();
                match *editing_idx {
                    Some(i) if i < updated.len() => {
                        updated[i] = f;
                    }
                    _ => {
                        updated.push(f);
                    }
                }
                on_change.emit(updated);
            }
            adding.set(AddingType::None);
            add_filter_type.set(String::new());
            editing_idx.set(None);
            reset_all_inputs.emit(());
        })
    };

    let cancel_adding = {
        let adding = adding.clone();
        let add_filter_type = add_filter_type.clone();
        let editing_idx = editing_idx.clone();
        let reset_all_inputs = reset_all_inputs.clone();
        Callback::from(move |_| {
            adding.set(AddingType::None);
            add_filter_type.set(String::new());
            editing_idx.set(None);
            reset_all_inputs.emit(());
        })
    };

    let remove_filter = {
        let on_change = on_change.clone();
        let filters = props.filters.clone();
        let editing_idx = editing_idx.clone();
        Callback::from(move |idx: usize| {
            let mut updated = filters.clone();
            updated.remove(idx);
            on_change.emit(updated);
            match *editing_idx {
                Some(e) if e == idx => editing_idx.set(None),
                Some(e) if e > idx => editing_idx.set(Some(e - 1)),
                _ => {}
            }
        })
    };

    // ── Add-inputs UI ──────────────────────────────────────────────

    let build_add_inputs = |add_name: &UseStateHandle<String>,
                            add_following: &UseStateHandle<bool>,
                            add_veteran_trainee_ids: &UseStateHandle<Vec<i64>>,
                            add_veteran_trainee_negate: &UseStateHandle<bool>,
                            add_veteran_rank: &UseStateHandle<String>,
                            add_sc_type_ids: &UseStateHandle<Vec<i64>>,
                            add_sc_rarity_ids: &UseStateHandle<Vec<i64>>,
                            add_sc_lb_min: &UseStateHandle<String>,
                            add_sc_lb_max: &UseStateHandle<String>,
                            add_sc_character_ids: &UseStateHandle<Vec<i64>>,
                            options: &TrainerFilterOptions| match &*adding {
        AddingType::None => None,
        AddingType::Name => Some(html! {
            <div class={FilterSectionStyle::CLASS_NAME}>
                <label>{"Name"}</label>
                <input type="text" class={FilterInputStyle::CLASS_NAME} placeholder="Search name..."
                    value={(**add_name).clone()}
                    oninput={let v=add_name.clone(); Callback::from(move|e:InputEvent| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
            </div>
        }),
        AddingType::Following => {
            let opts: Vec<SelectOption<String>> = vec![
                SelectOption { value: "true".to_string(), label: "Following only".to_string() },
                SelectOption { value: "false".to_string(), label: "Added manually".to_string() },
            ];
            let selected = if **add_following { "true".to_string() } else { "false".to_string() };
            Some(html! {
                <div class={FilterSectionStyle::CLASS_NAME}>
                    <label>{"Following"}</label>
                    <CustomSelect
                        options={opts}
                        selected={Some(selected)}
                        on_change={let v = add_following.clone(); Callback::from(move |val: String| v.set(val == "true"))}
                        placeholder={"Select..."}
                    />
                </div>
            })
        }
        AddingType::VeteranTrainee => {
            let opts = id_name_options(&options.characters);
            let on_select = {
                let v = add_veteran_trainee_ids.clone();
                Callback::from(move |id: i64| {
                    let mut list = (*v).clone();
                    if !list.contains(&id) {
                        list.push(id);
                    }
                    v.set(list);
                })
            };
            Some(html! {
                <>
                    <div class={FilterSectionStyle::CLASS_NAME}>
                        <label>{"Borrow Veteran Chara"}</label>
                        <SearchableSelect<i64>
                            options={opts}
                            on_select={on_select}
                            selected={None}
                            placeholder={"Add chara..."}
                        />
                        <div class={TrainerIdListStyle::CLASS_NAME}>
                            {for (*add_veteran_trainee_ids).iter().map(|id| {
                                let id = *id;
                                let label = options.characters.iter().find(|(cid, _)| *cid == id).map(|(_, l)| l.clone()).unwrap_or_else(|| id.to_string());
                                let onclick = {
                                    let v = add_veteran_trainee_ids.clone();
                                    Callback::from(move |_: MouseEvent| {
                                        let mut list = (*v).clone();
                                        list.retain(|x| *x != id);
                                        v.set(list);
                                    })
                                };
                                html! { <span class={FilterChipStyle::CLASS_NAME}>{label}<button class={FilterChipRemoveStyle::CLASS_NAME} onclick={onclick}>{"\u{00D7}"}</button></span> }
                            })}
                        </div>
                    </div>
                    <div class={FilterSectionStyle::CLASS_NAME}>
                        {checkbox_input(**add_veteran_trainee_negate, "Exclude these charas", {
                            let v = add_veteran_trainee_negate.clone();
                            Callback::from(move |c| v.set(c))
                        })}
                    </div>
                </>
            })
        }
        AddingType::VeteranRank => Some(html! {
            <div class={FilterSectionStyle::CLASS_NAME}>
                <label>{"Min Borrow Veteran Rank"}</label>
                <input type="number" class={FilterInputStyle::CLASS_NAME} placeholder="0"
                    value={(**add_veteran_rank).clone()}
                    oninput={let v = add_veteran_rank.clone(); Callback::from(move |e: InputEvent| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
            </div>
        }),
        AddingType::ScType => {
            let opts = id_name_options(&options.card_types);
            let on_select = {
                let v = add_sc_type_ids.clone();
                Callback::from(move |id: i64| {
                    let mut list = (*v).clone();
                    if !list.contains(&id) {
                        list.push(id);
                    }
                    v.set(list);
                })
            };
            Some(html! {
                <div class={FilterSectionStyle::CLASS_NAME}>
                    <label>{"Support Card Type"}</label>
                    <SearchableSelect<i64>
                        options={opts}
                        on_select={on_select}
                        selected={None}
                        placeholder={"Add type..."}
                    />
                    <div class={TrainerIdListStyle::CLASS_NAME}>
                        {for (*add_sc_type_ids).iter().map(|id| {
                                let id = *id;
                                let label = options.card_types.iter().find(|(ct, _)| *ct == id).map(|(_, l)| l.clone()).unwrap_or_else(|| id.to_string());
                                let onclick = {
                                    let v = add_sc_type_ids.clone();
                                    Callback::from(move |_: MouseEvent| {
                                        let mut list = (*v).clone();
                                        list.retain(|x| *x != id);
                                    v.set(list);
                                })
                            };
                            html! { <span class={FilterChipStyle::CLASS_NAME}>{label}<button class={FilterChipRemoveStyle::CLASS_NAME} onclick={onclick}>{"\u{00D7}"}</button></span> }
                        })}
                    </div>
                </div>
            })
        }
        AddingType::ScRarity => {
            let opts = id_name_options(&options.rarities);
            let on_select = {
                let v = add_sc_rarity_ids.clone();
                Callback::from(move |id: i64| {
                    let mut list = (*v).clone();
                    if !list.contains(&id) {
                        list.push(id);
                    }
                    v.set(list);
                })
            };
            Some(html! {
                <div class={FilterSectionStyle::CLASS_NAME}>
                    <label>{"Support Card Rarity"}</label>
                    <SearchableSelect<i64>
                        options={opts}
                        on_select={on_select}
                        selected={None}
                        placeholder={"Add rarity..."}
                    />
                    <div class={TrainerIdListStyle::CLASS_NAME}>
                        {for (*add_sc_rarity_ids).iter().map(|id| {
                                let id = *id;
                                let label = options.rarities.iter().find(|(r, _)| *r == id).map(|(_, l)| l.clone()).unwrap_or_else(|| id.to_string());
                                let onclick = {
                                    let v = add_sc_rarity_ids.clone();
                                    Callback::from(move |_: MouseEvent| {
                                        let mut list = (*v).clone();
                                        list.retain(|x| *x != id);
                                    v.set(list);
                                })
                            };
                            html! { <span class={FilterChipStyle::CLASS_NAME}>{label}<button class={FilterChipRemoveStyle::CLASS_NAME} onclick={onclick}>{"\u{00D7}"}</button></span> }
                        })}
                    </div>
                </div>
            })
        }
        AddingType::ScLimitBreak => Some(html! {
            <>
                <div class={FilterSectionStyle::CLASS_NAME}>
                    <label>{"Min LB"}</label>
                    <input type="number" class={FilterInputStyle::CLASS_NAME} placeholder="0"
                        min="0" max="4"
                        value={(**add_sc_lb_min).clone()}
                        oninput={let v = add_sc_lb_min.clone(); Callback::from(move |e: InputEvent| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                </div>
                <div class={FilterSectionStyle::CLASS_NAME}>
                    <label>{"Max LB"}</label>
                    <input type="number" class={FilterInputStyle::CLASS_NAME} placeholder="4"
                        min="0" max="4"
                        value={(**add_sc_lb_max).clone()}
                        oninput={let v = add_sc_lb_max.clone(); Callback::from(move |e: InputEvent| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                </div>
            </>
        }),
        AddingType::ScCharacter => {
            let opts = id_name_options(&options.characters);
            let on_select = {
                let v = add_sc_character_ids.clone();
                Callback::from(move |id: i64| {
                    let mut list = (*v).clone();
                    if !list.contains(&id) {
                        list.push(id);
                    }
                    v.set(list);
                })
            };
            Some(html! {
                <div class={FilterSectionStyle::CLASS_NAME}>
                    <label>{"Support Card Character"}</label>
                    <SearchableSelect<i64>
                        options={opts}
                        on_select={on_select}
                        selected={None}
                        placeholder={"Add chara..."}
                    />
                    <div class={TrainerIdListStyle::CLASS_NAME}>
                        {for (*add_sc_character_ids).iter().map(|id| {
                                let id = *id;
                                let label = options.characters.iter().find(|(cid, _)| *cid == id).map(|(_, l)| l.clone()).unwrap_or_else(|| id.to_string());
                                let onclick = {
                                    let v = add_sc_character_ids.clone();
                                    Callback::from(move |_: MouseEvent| {
                                        let mut list = (*v).clone();
                                        list.retain(|x| *x != id);
                                    v.set(list);
                                })
                            };
                            html! { <span class={FilterChipStyle::CLASS_NAME}>{label}<button class={FilterChipRemoveStyle::CLASS_NAME} onclick={onclick}>{"\u{00D7}"}</button></span> }
                        })}
                    </div>
                </div>
            })
        }
    };

    let add_ui = build_add_inputs(
        &add_name,
        &add_following,
        &add_veteran_trainee_ids,
        &add_veteran_trainee_negate,
        &add_veteran_rank,
        &add_sc_type_ids,
        &add_sc_rarity_ids,
        &add_sc_lb_min,
        &add_sc_lb_max,
        &add_sc_character_ids,
        &props.options,
    );
    let add_ui = match add_ui {
        Some(inputs) => {
            let can_add = match &*adding {
                AddingType::Name => !add_name.is_empty(),
                AddingType::Following => true,
                AddingType::VeteranTrainee => !add_veteran_trainee_ids.is_empty(),
                AddingType::VeteranRank => add_veteran_rank.parse::<i64>().is_ok(),
                AddingType::ScType => !add_sc_type_ids.is_empty(),
                AddingType::ScRarity => !add_sc_rarity_ids.is_empty(),
                AddingType::ScLimitBreak => true,
                AddingType::ScCharacter => !add_sc_character_ids.is_empty(),
                AddingType::None => false,
            };
            html! {
                <div style="margin-top:8px;">
                    {inputs}
                    <div class={FilterActionsStyle::CLASS_NAME} style="margin-top:8px;">
                        <button disabled={!can_add} onclick={add_filter}>{(if (*editing_idx).is_some() { "Save" } else { "Add" })}</button>
                        <button class={SecondaryBtnStyle::CLASS_NAME} onclick={cancel_adding}>{"Cancel"}</button>
                    </div>
                </div>
            }
        }
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

    html! {
        <div class={FilterPanelStyle::CLASS_NAME}>
            <div class={FilterTitleStyle::CLASS_NAME}>{"Active Filters"}</div>

            if props.filters.is_empty() {
                <div class={FilterEmptyHintStyle::CLASS_NAME}>
                    {"No filters — showing all trainers"}
                </div>
            } else {
                <div style="margin-bottom:12px;">
                    {for props.filters.iter().enumerate().map(|(i, f)| {
                        let desc = filter_description(f, &props.options);
                        let remove = {
                            let remove_filter = remove_filter.clone();
                            Callback::from(move |_| remove_filter.emit(i))
                        };
                        let edit = {
                            let open_edit = open_edit.clone();
                            Callback::from(move |_| open_edit.emit(i))
                        };
                        let is_editing = matches!(*editing_idx, Some(e) if e == i);
                        let pill_style = if is_editing {
                            "border:1px solid #f59e0b;".to_string()
                        } else {
                            "border:1px solid #334155;".to_string()
                        };
                        html! {
                            <div key={i} class={FilterChipStyle::CLASS_NAME} style={pill_style}>
                                <button type="button" onclick={edit} class={FilterChipTextStyle::CLASS_NAME} style="text-align:left;background:none;border:none;cursor:pointer;padding:0;font-size:12px;color:#e2e8f0;">
                                    {desc}
                                </button>
                                <button onclick={remove} class={FilterChipRemoveStyle::CLASS_NAME}>{"\u{00D7}"}</button>
                            </div>
                        }
                    })}
                </div>
            }

            <div class={FilterTitleStyle::CLASS_NAME}>{"Add Filter"}</div>
            if (*editing_idx).is_some() {
                <div style="color:#f59e0b;font-size:12px;margin-bottom:6px;">
                    {"Editing existing filter — click Save to apply changes"}
                </div>
            }
            <div class={FilterSectionStyle::CLASS_NAME}>
                <SearchableSelect<String>
                    options={
                        vec![
                            SelectOption { value: "name".to_string(), label: "Name".to_string() },
                            SelectOption { value: "following".to_string(), label: "Following".to_string() },
                            SelectOption { value: "veteran_trainee".to_string(), label: "Borrow Veteran Chara".to_string() },
                            SelectOption { value: "veteran_rank".to_string(), label: "Borrow Veteran Rank".to_string() },
                            SelectOption { value: "sc_type".to_string(), label: "SC Type".to_string() },
                            SelectOption { value: "sc_rarity".to_string(), label: "SC Rarity".to_string() },
                            SelectOption { value: "sc_limit_break".to_string(), label: "SC Limit Break".to_string() },
                            SelectOption { value: "sc_character".to_string(), label: "SC Character".to_string() },
                        ]
                    }
                    selected={filter_type_selected}
                    on_select={let a = adding.clone(); let ft = add_filter_type.clone(); Callback::from(move |val: String| {
                        ft.set(val.clone());
                        a.set(match val.as_str() {
                            "name" => AddingType::Name,
                            "following" => AddingType::Following,
                            "veteran_trainee" => AddingType::VeteranTrainee,
                            "veteran_rank" => AddingType::VeteranRank,
                            "sc_type" => AddingType::ScType,
                            "sc_rarity" => AddingType::ScRarity,
                            "sc_limit_break" => AddingType::ScLimitBreak,
                            "sc_character" => AddingType::ScCharacter,
                            _ => AddingType::None,
                        });
                    })}
                    placeholder={"Select type..."}
                    disabled={(*editing_idx).is_some()}
                />
            </div>

            {add_ui}
        </div>
    }
}
