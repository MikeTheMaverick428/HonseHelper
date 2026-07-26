use crate::styles::{
    filter_panel::*,
    legacy_planner::SecondaryBtnStyle,
    worker_status::{ToggleCheckboxStyle, ToggleLabelStyle},
    Style,
};
use shared::support_card_browser::{SupportCardFilter, SupportCardFilterOptions, SupportCardSkillSources};
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

fn checkbox_input(
    checked: bool,
    label: &str,
    on_change: Callback<bool>,
) -> Html {
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
pub struct ScFilterPanelProps {
    pub filters: Vec<SupportCardFilter>,
    pub on_change: Callback<Vec<SupportCardFilter>>,
    pub options: SupportCardFilterOptions,
}

enum AddingType {
    None,
    Name,
    Rarity,
    CardType,
    LimitBreak,
    HasEffect,
    Character,
    HasSkill,
}

fn filter_description(f: &SupportCardFilter, options: &SupportCardFilterOptions) -> String {
    match f {
        SupportCardFilter::NameSearch { search_text } => format!("Name: \"{}\"", search_text),
        SupportCardFilter::Rarity { rarity } => {
            let label = options
                .rarities
                .iter()
                .find(|(id, _)| id == rarity)
                .map(|(_, l)| l.as_str())
                .unwrap_or("?");
            format!("Rarity: {}", label)
        }
        SupportCardFilter::CardType { card_type } => {
            let label = options
                .card_types
                .iter()
                .find(|(id, _)| id == card_type)
                .map(|(_, l)| l.as_str())
                .unwrap_or("?");
            format!("Type: {}", label)
        }
        SupportCardFilter::LimitBreak { min, max } => format!("LB: {}–{}", min, max),
        SupportCardFilter::HasEffect { effect_type } => {
            let label = options
                .effect_types
                .iter()
                .find(|(id, _)| id == effect_type)
                .map(|(_, l)| l.as_str())
                .unwrap_or("?");
            format!("Effect: {}", label)
        }
        SupportCardFilter::Character { character_id } => {
            let label = options
                .characters
                .iter()
                .find(|(id, _)| id == character_id)
                .map(|(_, l)| l.as_str())
                .unwrap_or("?");
            format!("Character: {}", label)
        }
        SupportCardFilter::HasSkill { group_id, exact_skill_id, sources } => {
            let lookup = exact_skill_id.map(|id| -id).unwrap_or(*group_id);
            let skill_label = options
                .skills
                .iter()
                .find(|(id, _)| *id == lookup)
                .map(|(_, l)| l.as_str())
                .unwrap_or("?");
            let mut parts = Vec::new();
            if sources.hint { parts.push("H"); }
            if sources.chain_event { parts.push("CE"); }
            if sources.random_event { parts.push("RE"); }
            format!("Skill: {} [{}]", skill_label, parts.join(","))
        }
    }
}

fn build_add_inputs(
    adding: &AddingType,
    add_name: &UseStateHandle<String>,
    add_rarity: &UseStateHandle<String>,
    add_card_type: &UseStateHandle<String>,
    add_lb_min: &UseStateHandle<String>,
    add_lb_max: &UseStateHandle<String>,
    add_effect_type: &UseStateHandle<Option<i64>>,
    add_character_id: &UseStateHandle<Option<i64>>,
    add_skill_id: &UseStateHandle<Option<i64>>,
    add_skill_hint: &UseStateHandle<bool>,
    add_skill_ce: &UseStateHandle<bool>,
    add_skill_re: &UseStateHandle<bool>,
    options: &SupportCardFilterOptions,
) -> Option<Html> {
    match adding {
        AddingType::None => None,
        AddingType::Name => Some(html! {
            <div class={FilterSectionStyle::CLASS_NAME}>
                <label>{"Name"}</label>
                <input type="text" class={FilterInputStyle::CLASS_NAME} placeholder="Search name..."
                    value={(**add_name).clone()}
                    oninput={let v=add_name.clone(); Callback::from(move|e:InputEvent| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
            </div>
        }),
        AddingType::Rarity => {
            let opts: Vec<SelectOption<String>> = options
                .rarities
                .iter()
                .map(|(id, label)| SelectOption {
                    value: id.to_string(),
                    label: label.clone(),
                })
                .collect();
            let selected = {
                let v = add_rarity.to_string();
                if v.is_empty() {
                    None
                } else {
                    Some(v)
                }
            };
            Some(html! {
                <div class={FilterSectionStyle::CLASS_NAME}>
                    <label>{"Rarity"}</label>
                    <CustomSelect
                        options={opts}
                        selected={selected}
                        on_change={let v = add_rarity.clone(); Callback::from(move |val: String| v.set(val))}
                        placeholder={"Select rarity..."}
                    />
                </div>
            })
        }
        AddingType::CardType => {
            let opts: Vec<SelectOption<String>> = options
                .card_types
                .iter()
                .map(|(id, label)| SelectOption {
                    value: id.to_string(),
                    label: label.clone(),
                })
                .collect();
            let selected = {
                let v = add_card_type.to_string();
                if v.is_empty() {
                    None
                } else {
                    Some(v)
                }
            };
            Some(html! {
                <div class={FilterSectionStyle::CLASS_NAME}>
                    <label>{"Card Type"}</label>
                    <CustomSelect
                        options={opts}
                        selected={selected}
                        on_change={let v = add_card_type.clone(); Callback::from(move |val: String| v.set(val))}
                        placeholder={"Select type..."}
                    />
                </div>
            })
        }
        AddingType::LimitBreak => Some(html! {
            <>
                <div class={FilterSectionStyle::CLASS_NAME}>
                    <label>{"Min LB"}</label>
                    <input type="number" class={FilterInputStyle::CLASS_NAME} placeholder="0"
                        min="0" max="4"
                        value={(**add_lb_min).clone()}
                        oninput={let v = add_lb_min.clone(); Callback::from(move |e: InputEvent| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                </div>
                <div class={FilterSectionStyle::CLASS_NAME}>
                    <label>{"Max LB"}</label>
                    <input type="number" class={FilterInputStyle::CLASS_NAME} placeholder="4"
                        min="0" max="4"
                        value={(**add_lb_max).clone()}
                        oninput={let v = add_lb_max.clone(); Callback::from(move |e: InputEvent| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                </div>
            </>
        }),
        AddingType::HasEffect => {
            let opts = id_name_options(&options.effect_types);
            let on_select = {
                let v = add_effect_type.clone();
                Callback::from(move |id: i64| v.set(Some(id)))
            };
            Some(html! {
                <div class={FilterSectionStyle::CLASS_NAME}>
                    <label>{"Effect"}</label>
                    <SearchableSelect<i64>
                        options={opts}
                        on_select={on_select}
                        selected={**add_effect_type}
                        placeholder={"Search effect..."}
                    />
                </div>
            })
        }
        AddingType::Character => {
            let opts = id_name_options(&options.characters);
            let on_select = {
                let v = add_character_id.clone();
                Callback::from(move |id: i64| v.set(Some(id)))
            };
            Some(html! {
                <div class={FilterSectionStyle::CLASS_NAME}>
                    <label>{"Character"}</label>
                    <SearchableSelect<i64>
                        options={opts}
                        on_select={on_select}
                        selected={**add_character_id}
                        placeholder={"Search character..."}
                    />
                </div>
            })
        }
        AddingType::HasSkill => {
            let opts = id_name_options(&options.skills);
            let on_select = {
                let v = add_skill_id.clone();
                Callback::from(move |id: i64| v.set(Some(id)))
            };
            Some(html! {
                <>
                    <div class={FilterSectionStyle::CLASS_NAME}>
                        <label>{"Skill"}</label>
                        <SearchableSelect<i64>
                            options={opts}
                            on_select={on_select}
                            selected={**add_skill_id}
                            placeholder={"Search skill..."}
                        />
                    </div>
                    <div class={FilterSectionStyle::CLASS_NAME} style="margin-top:8px;">
                        <label>{"Sources"}</label>
                        {checkbox_input(**add_skill_hint, "Hint", {
                            let v = add_skill_hint.clone();
                            Callback::from(move |c| v.set(c))
                        })}
                        {checkbox_input(**add_skill_ce, "Chain Event", {
                            let v = add_skill_ce.clone();
                            Callback::from(move |c| v.set(c))
                        })}
                        {checkbox_input(**add_skill_re, "Random Event", {
                            let v = add_skill_re.clone();
                            Callback::from(move |c| v.set(c))
                        })}
                    </div>
                </>
            })
        }
    }
}

#[function_component]
pub fn ScFilterPanel(props: &ScFilterPanelProps) -> Html {
    let adding = use_state(|| AddingType::None);
    let add_name = use_state(String::new);
    let add_rarity = use_state(String::new);
    let add_card_type = use_state(String::new);
    let add_lb_min = use_state(|| String::new());
    let add_lb_max = use_state(|| String::new());
    let add_effect_type: UseStateHandle<Option<i64>> = use_state(|| None);
    let add_character_id: UseStateHandle<Option<i64>> = use_state(|| None);
    let add_skill_id: UseStateHandle<Option<i64>> = use_state(|| None);
    let add_skill_hint: UseStateHandle<bool> = use_state(|| true);
    let add_skill_ce: UseStateHandle<bool> = use_state(|| true);
    let add_skill_re: UseStateHandle<bool> = use_state(|| true);
    let add_filter_type: UseStateHandle<String> = use_state(String::new);

    let on_change = props.on_change.clone();

    let reset_all_inputs = {
        let add_name = add_name.clone();
        let add_rarity = add_rarity.clone();
        let add_card_type = add_card_type.clone();
        let add_lb_min = add_lb_min.clone();
        let add_lb_max = add_lb_max.clone();
        let add_effect_type = add_effect_type.clone();
        let add_character_id = add_character_id.clone();
        let add_skill_id = add_skill_id.clone();
        let add_skill_hint = add_skill_hint.clone();
        let add_skill_ce = add_skill_ce.clone();
        let add_skill_re = add_skill_re.clone();
        Callback::from(move |_| {
            add_name.set(String::new());
            add_rarity.set(String::new());
            add_card_type.set(String::new());
            add_lb_min.set(String::new());
            add_lb_max.set(String::new());
            add_effect_type.set(None);
            add_character_id.set(None);
            add_skill_id.set(None);
            add_skill_hint.set(true);
            add_skill_ce.set(true);
            add_skill_re.set(true);
        })
    };

    let add_filter = {
        let on_change = on_change.clone();
        let adding = adding.clone();
        let filters = props.filters.clone();
        let add_name = add_name.clone();
        let add_rarity = add_rarity.clone();
        let add_card_type = add_card_type.clone();
        let add_lb_min = add_lb_min.clone();
        let add_lb_max = add_lb_max.clone();
        let add_effect_type = add_effect_type.clone();
        let add_character_id = add_character_id.clone();
        let add_skill_id = add_skill_id.clone();
        let add_skill_hint = add_skill_hint.clone();
        let add_skill_ce = add_skill_ce.clone();
        let add_skill_re = add_skill_re.clone();
        let add_filter_type = add_filter_type.clone();
        let reset_all_inputs = reset_all_inputs.clone();
        Callback::from(move |_| {
            let new_filter = match &*adding {
                AddingType::Name => {
                    let text = (*add_name).clone();
                    if text.is_empty() {
                        None
                    } else {
                        Some(SupportCardFilter::NameSearch { search_text: text })
                    }
                }
                AddingType::Rarity => (*add_rarity)
                    .parse::<i64>()
                    .ok()
                    .map(|r| SupportCardFilter::Rarity { rarity: r }),
                AddingType::CardType => (*add_card_type)
                    .parse::<i64>()
                    .ok()
                    .map(|ct| SupportCardFilter::CardType { card_type: ct }),
                AddingType::LimitBreak => {
                    let min = (*add_lb_min).parse::<i64>().ok().unwrap_or(0);
                    let max = (*add_lb_max).parse::<i64>().ok().unwrap_or(4);
                    Some(SupportCardFilter::LimitBreak { min, max })
                }
                AddingType::HasEffect => {
                    add_effect_type.map(|et| SupportCardFilter::HasEffect { effect_type: et })
                }
                AddingType::Character => {
                    add_character_id.map(|id| SupportCardFilter::Character { character_id: id })
                }
                AddingType::HasSkill => {
                    add_skill_id.map(|val| {
                        let (group_id, exact_skill_id) = if val < 0 {
                            (0, Some(-val))
                        } else {
                            (val, None)
                        };
                        SupportCardFilter::HasSkill {
                            group_id,
                            exact_skill_id,
                            sources: SupportCardSkillSources {
                                hint: *add_skill_hint,
                                chain_event: *add_skill_ce,
                                random_event: *add_skill_re,
                            },
                        }
                    })
                }
                AddingType::None => None,
            };
            if let Some(f) = new_filter {
                let mut updated = filters.clone();
                updated.push(f);
                on_change.emit(updated);
            }
            adding.set(AddingType::None);
            add_filter_type.set(String::new());
            reset_all_inputs.emit(());
        })
    };

    let cancel_adding = {
        let adding = adding.clone();
        let add_filter_type = add_filter_type.clone();
        let reset_all_inputs = reset_all_inputs.clone();
        Callback::from(move |_| {
            adding.set(AddingType::None);
            add_filter_type.set(String::new());
            reset_all_inputs.emit(());
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
        &add_name,
        &add_rarity,
        &add_card_type,
        &add_lb_min,
        &add_lb_max,
        &add_effect_type,
        &add_character_id,
        &add_skill_id,
        &add_skill_hint,
        &add_skill_ce,
        &add_skill_re,
        &props.options,
    );
    let add_ui = match add_ui {
        Some(inputs) => {
            let can_add = match &*adding {
                AddingType::Name => !add_name.is_empty(),
                AddingType::Rarity => add_rarity.parse::<i64>().is_ok(),
                AddingType::CardType => add_card_type.parse::<i64>().is_ok(),
                AddingType::LimitBreak => true,
                AddingType::HasSkill => add_skill_id.is_some(),
                AddingType::HasEffect => add_effect_type.is_some(),
                AddingType::Character => add_character_id.is_some(),
                AddingType::None => false,
            };
            html! {
                <div style="margin-top:8px;">
                    {inputs}
                    <div class={FilterActionsStyle::CLASS_NAME} style="margin-top:8px;">
                        <button disabled={!can_add} onclick={add_filter}>{"Add"}</button>
                        <button class={SecondaryBtnStyle::CLASS_NAME} onclick={cancel_adding}>{"Cancel"}</button>
                    </div>
                </div>
            }
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

    html! {
        <div class={FilterPanelStyle::CLASS_NAME}>
            <div class={FilterTitleStyle::CLASS_NAME}>{"Active Filters"}</div>

            if props.filters.is_empty() {
                <div style="color:#64748b;font-size:12px;margin-bottom:12px;">
                    {"No filters — showing all support cards"}
                </div>
            } else {
                <div style="margin-bottom:12px;">
                    {for props.filters.iter().enumerate().map(|(i, f)| {
                        let desc = filter_description(f, &props.options);
                        let onclick = {
                            let remove_filter = remove_filter.clone();
                            Callback::from(move |_| remove_filter.emit(i))
                        };
                        html! {
                            <div key={i} class={FilterChipStyle::CLASS_NAME}>
                                <span style="flex:1;color:#e2e8f0;">{desc}</span>
                                <button onclick={onclick} class={FilterChipRemoveStyle::CLASS_NAME}>{"\u{00D7}"}</button>
                            </div>
                        }
                    })}
                </div>
            }

            <div class={FilterTitleStyle::CLASS_NAME}>{"Add Filter"}</div>
            <div class={FilterSectionStyle::CLASS_NAME}>
                <SearchableSelect<String>
                    options={
                        vec![
                            SelectOption { value: "name".to_string(), label: "Name".to_string() },
                            SelectOption { value: "rarity".to_string(), label: "Rarity".to_string() },
                            SelectOption { value: "card_type".to_string(), label: "Card Type".to_string() },
                            SelectOption { value: "limit_break".to_string(), label: "Limit Break".to_string() },
                            SelectOption { value: "has_effect".to_string(), label: "Has Effect".to_string() },
                            SelectOption { value: "character".to_string(), label: "Character".to_string() },
                            SelectOption { value: "has_skill".to_string(), label: "Has Skill".to_string() },
                        ]
                    }
                    selected={filter_type_selected}
                    on_select={let a = adding.clone(); let ft = add_filter_type.clone(); Callback::from(move |val: String| {
                        ft.set(val.clone());
                        a.set(match val.as_str() {
                            "name" => AddingType::Name,
                            "rarity" => AddingType::Rarity,
                            "card_type" => AddingType::CardType,
                            "limit_break" => AddingType::LimitBreak,
                            "has_effect" => AddingType::HasEffect,
                            "character" => AddingType::Character,
                            "has_skill" => AddingType::HasSkill,
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
