use crate::styles::{
    filter_panel::{
        FilterActionsStyle, FilterChipRemoveStyle, FilterChipStyle, FilterChipTextStyle,
        FilterEmptyHintStyle, FilterInputStyle, FilterPanelStyle, FilterSectionStyle,
        FilterTitleStyle,
    },
    legacy_planner::SecondaryBtnStyle,
    worker_status::{ToggleCheckboxStyle, ToggleLabelStyle},
    Style,
};
use crate::veteran_browser::components::custom_select::CustomSelect;
use crate::veteran_browser::components::searchable_select::{SearchableSelect, SelectOption};
use shared::filters::{AptitudeType, StatType};
use shared::trainee_browser::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;

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

const LEVEL_OPTIONS: &[(&str, i64)] = &[
    ("S (8)", 8),
    ("A (7)", 7),
    ("B (6)", 6),
    ("C (5)", 5),
    ("D (4)", 4),
    ("E (3)", 3),
    ("F (2)", 2),
    ("G (1)", 1),
];

#[derive(Properties, PartialEq)]
pub struct TrFilterPanelProps {
    pub filters: Vec<TraineeFilter>,
    pub on_change: Callback<Vec<TraineeFilter>>,
    pub options: Option<TraineeFilterOptions>,
}

#[derive(Clone, PartialEq)]
enum AddingType {
    None,
    Ownership,
    GrowthBonus,
    MinAptitude,
    MaxAAptitudes,
    Character,
    HasSkill,
}

fn filter_label(f: &TraineeFilter, options: Option<&TraineeFilterOptions>) -> String {
    match f {
        TraineeFilter::Owned { owned } => {
            if *owned {
                "Owned".to_string()
            } else {
                "Not owned".to_string()
            }
        }
        TraineeFilter::GrowthBonus { stat, min_value } => {
            let mn = min_value.map(|v| format!(" >= {}", v)).unwrap_or_default();
            format!("Growth: {}{}", stat.label(), mn)
        }
        TraineeFilter::MinAptitude {
            category,
            min_level,
        } => {
            format!(
                "{} >= {}",
                category.label(),
                shared::models::AptitudeLevel::from_raw(*min_level).to_string()
            )
        }
        TraineeFilter::MaxAAptitudes { max_count } => {
            format!("<= {} A aptitudes", max_count)
        }
        TraineeFilter::Character { character_id } => {
            if let Some(opts) = options {
                if let Some((_, name)) = opts.characters.iter().find(|(id, _)| id == character_id) {
                    return format!("Character: {}", name);
                }
            }
            format!("Character: #{}", character_id)
        }
        TraineeFilter::HasSkill { group_id, exact_skill_id, sources } => {
            let lookup = exact_skill_id.map(|id| -id).unwrap_or(*group_id);
            let skill_label = options
                .and_then(|opts| opts.skills.iter().find(|(id, _)| *id == lookup))
                .map(|(_, l)| l.as_str())
                .unwrap_or("?");
            let mut parts = Vec::new();
            if sources.innate { parts.push("I"); }
            if sources.event { parts.push("E"); }
            if sources.secret { parts.push("S"); }
            format!("Skill: {} [{}]", skill_label, parts.join(","))
        }
    }
}

fn adding_type_to_key(t: &AddingType) -> Option<String> {
    match t {
        AddingType::Ownership => Some("ownership"),
        AddingType::GrowthBonus => Some("growth"),
        AddingType::MinAptitude => Some("min_apt"),
        AddingType::MaxAAptitudes => Some("max_a"),
        AddingType::Character => Some("character"),
        AddingType::HasSkill => Some("has_skill"),
        AddingType::None => None,
    }
    .map(String::from)
}

fn filter_to_adding_type(f: &TraineeFilter) -> Option<(&'static str, AddingType)> {
    match f {
        TraineeFilter::Owned { .. } => Some(("ownership", AddingType::Ownership)),
        TraineeFilter::GrowthBonus { .. } => Some(("growth", AddingType::GrowthBonus)),
        TraineeFilter::MinAptitude { .. } => Some(("min_apt", AddingType::MinAptitude)),
        TraineeFilter::MaxAAptitudes { .. } => Some(("max_a", AddingType::MaxAAptitudes)),
        TraineeFilter::Character { .. } => Some(("character", AddingType::Character)),
        TraineeFilter::HasSkill { .. } => Some(("has_skill", AddingType::HasSkill)),
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

#[function_component]
pub fn TrFilterPanel(props: &TrFilterPanelProps) -> Html {
    let adding = use_state(|| AddingType::None);
    let pending = use_state(|| None::<TraineeFilter>);

    let gb_stat = use_state(|| "speed".to_string());
    let gb_min = use_state(String::new);

    let ma_cat = use_state(|| "turf".to_string());
    let ma_level = use_state(|| 7i64);

    let max_a_count = use_state(String::new);

    let own_val = use_state(|| true);

    let char_id = use_state(|| None::<i64>);

    let add_skill_id = use_state(|| None::<i64>);
    let add_skill_innate = use_state(|| true);
    let add_skill_event = use_state(|| true);
    let add_skill_secret = use_state(|| true);

    let editing_idx: UseStateHandle<Option<usize>> = use_state(|| None);

    let make_gb_pending = {
        let gb_stat = gb_stat.clone();
        let gb_min = gb_min.clone();
        move || {
            let stat = StatType::from_str(&gb_stat).unwrap_or(StatType::Speed);
            let min = (*gb_min).parse::<i64>().ok();
            TraineeFilter::GrowthBonus {
                stat,
                min_value: min,
            }
        }
    };

    let make_ma_pending = {
        let ma_cat = ma_cat.clone();
        let ma_level = ma_level.clone();
        move || TraineeFilter::MinAptitude {
            category: AptitudeType::from_str(&ma_cat).unwrap_or(AptitudeType::Turf),
            min_level: *ma_level,
        }
    };

    let remove_filter = {
        let filters = props.filters.clone();
        let on_change = props.on_change.clone();
        let editing_idx = editing_idx.clone();
        Callback::from(move |idx: usize| {
            let mut nf = filters.clone();
            nf.remove(idx);
            on_change.emit(nf);
            match *editing_idx {
                Some(e) if e == idx => editing_idx.set(None),
                Some(e) if e > idx => editing_idx.set(Some(e - 1)),
                _ => {}
            }
        })
    };

    let open_edit = {
        let filters = props.filters.clone();
        let adding = adding.clone();
        let pending = pending.clone();
        let editing_idx = editing_idx.clone();
        let gb_stat = gb_stat.clone();
        let gb_min = gb_min.clone();
        let ma_cat = ma_cat.clone();
        let ma_level = ma_level.clone();
        let max_a_count = max_a_count.clone();
        let own_val = own_val.clone();
        let char_id = char_id.clone();
        let add_skill_id = add_skill_id.clone();
        let add_skill_innate = add_skill_innate.clone();
        let add_skill_event = add_skill_event.clone();
        let add_skill_secret = add_skill_secret.clone();
        Callback::from(move |idx: usize| {
            let Some(f) = filters.get(idx) else { return };
            let Some((_, t)) = filter_to_adding_type(f) else { return };
            gb_stat.set("speed".to_string());
            gb_min.set(String::new());
            ma_cat.set("turf".to_string());
            ma_level.set(7);
            max_a_count.set(String::new());
            own_val.set(true);
            char_id.set(None);
            add_skill_id.set(None);
            add_skill_innate.set(true);
            add_skill_event.set(true);
            add_skill_secret.set(true);
            adding.set(t);
            pending.set(Some(f.clone()));
            editing_idx.set(Some(idx));
            match f {
                TraineeFilter::Owned { owned } => own_val.set(*owned),
                TraineeFilter::GrowthBonus { stat, min_value } => {
                    gb_stat.set(stat.value().to_string());
                    gb_min.set(min_value.map(|v| v.to_string()).unwrap_or_default());
                }
                TraineeFilter::MinAptitude {
                    category,
                    min_level,
                } => {
                    ma_cat.set(category.value().to_string());
                    ma_level.set(*min_level);
                }
                TraineeFilter::MaxAAptitudes { max_count } => {
                    max_a_count.set(max_count.to_string());
                }
                TraineeFilter::Character { character_id } => char_id.set(Some(*character_id)),
                TraineeFilter::HasSkill {
                    group_id,
                    exact_skill_id,
                    sources,
                } => {
                    let val = exact_skill_id.map(|id| -id).unwrap_or(*group_id);
                    add_skill_id.set(Some(val));
                    add_skill_innate.set(sources.innate);
                    add_skill_event.set(sources.event);
                    add_skill_secret.set(sources.secret);
                }
            }
        })
    };

    let on_type_select = {
        let adding = adding.clone();
        let pending = pending.clone();
        let make_gb = make_gb_pending.clone();
        let make_ma = make_ma_pending.clone();
        let own_val = own_val.clone();
        let char_id = char_id.clone();
        let add_skill_id = add_skill_id.clone();
        let add_skill_innate = add_skill_innate.clone();
        let add_skill_event = add_skill_event.clone();
        let add_skill_secret = add_skill_secret.clone();
        Callback::from(move |t: AddingType| {
            adding.set(t.clone());
            match t {
                AddingType::Ownership => {
                    pending.set(Some(TraineeFilter::Owned { owned: *own_val }))
                }
                AddingType::GrowthBonus => pending.set(Some(make_gb())),
                AddingType::MinAptitude => pending.set(Some(make_ma())),
                AddingType::MaxAAptitudes => pending.set(None),
                AddingType::Character => {
                    char_id.set(None);
                    pending.set(None);
                }
                AddingType::HasSkill => {
                    add_skill_id.set(None);
                    add_skill_innate.set(true);
                    add_skill_event.set(true);
                    add_skill_secret.set(true);
                    pending.set(None);
                }
                AddingType::None => pending.set(None),
            }
        })
    };

    let on_add_clicked = {
        let adding = adding.clone();
        let on_change = props.on_change.clone();
        let filters = props.filters.clone();
        let pending = pending.clone();
        let editing_idx = editing_idx.clone();
        let add_skill_id = add_skill_id.clone();
        let add_skill_innate = add_skill_innate.clone();
        let add_skill_event = add_skill_event.clone();
        let add_skill_secret = add_skill_secret.clone();
        let max_a_count = max_a_count.clone();
        let char_id = char_id.clone();
        let own_val = own_val.clone();
        Callback::from(move |_: MouseEvent| {
            let new_filter = match &*adding {
                AddingType::HasSkill => {
                    (*add_skill_id).map(|val| {
                        let (group_id, exact_skill_id) = if val < 0 {
                            (0, Some(-val))
                        } else {
                            (val, None)
                        };
                        TraineeFilter::HasSkill {
                            group_id,
                            exact_skill_id,
                            sources: TraineeSkillSources {
                                innate: *add_skill_innate,
                                event: *add_skill_event,
                                secret: *add_skill_secret,
                            },
                        }
                    })
                }
                _ => {
                    let pending_val = match &*adding {
                        AddingType::Ownership => {
                            Some(TraineeFilter::Owned { owned: *own_val })
                        }
                        AddingType::GrowthBonus => Some(make_gb_pending()),
                        AddingType::MinAptitude => Some(make_ma_pending()),
                        AddingType::MaxAAptitudes => Some(TraineeFilter::MaxAAptitudes {
                            max_count: (*max_a_count).parse::<i64>().unwrap_or(0),
                        }),
                        AddingType::Character => {
                            (*char_id).map(|id| TraineeFilter::Character { character_id: id })
                        }
                        _ => None,
                    };
                    pending_val
                }
            };
            if let Some(f) = new_filter {
                let mut nf = filters.clone();
                match *editing_idx {
                    Some(i) if i < nf.len() => {
                        nf[i] = f;
                    }
                    _ => {
                        nf.push(f);
                    }
                }
                on_change.emit(nf);
            }
            adding.set(AddingType::None);
            pending.set(None);
            editing_idx.set(None);
        })
    };

    let on_cancel = {
        let adding = adding.clone();
        let pending = pending.clone();
        let editing_idx = editing_idx.clone();
        Callback::from(move |_: MouseEvent| {
            adding.set(AddingType::None);
            pending.set(None);
            editing_idx.set(None);
        })
    };

    let editor = match &*adding {
        AddingType::None => html! {},
        AddingType::Ownership => {
            let opts: Vec<SelectOption<String>> = vec![
                SelectOption {
                    value: "true".to_string(),
                    label: "Owned".to_string(),
                },
                SelectOption {
                    value: "false".to_string(),
                    label: "Not owned".to_string(),
                },
            ];
            let selected = if *own_val {
                "true".to_string()
            } else {
                "false".to_string()
            };
            html! {
                <div class={FilterSectionStyle::CLASS_NAME}>
                    <label>{"Status"}</label>
                    <CustomSelect
                        options={opts}
                        selected={Some(selected)}
                        on_change={
                            let own_val = own_val.clone();
                            let pending = pending.clone();
                            Callback::from(move |v: String| {
                                let owned = v == "true";
                                own_val.set(owned);
                                pending.set(Some(TraineeFilter::Owned { owned }));
                            })
                        }
                        placeholder={"Select..."}
                    />
                </div>
            }
        }
        AddingType::GrowthBonus => {
            let stat_opts: Vec<SelectOption<String>> = StatType::all()
                .iter()
                .map(|s| SelectOption {
                    value: s.value().to_string(),
                    label: s.label().to_string(),
                })
                .collect();
            html! {
                <>
                    <div class={FilterSectionStyle::CLASS_NAME}>
                        <label>{"Stat"}</label>
                        <CustomSelect
                            options={stat_opts}
                            selected={Some((*gb_stat).clone())}
                            on_change={
                                let gb_stat = gb_stat.clone();
                                let gb_min = gb_min.clone();
                                let pending = pending.clone();
                                Callback::from(move |v: String| {
                                    gb_stat.set(v.clone());
                                    let min = (*gb_min).parse::<i64>().ok();
                                    let stat = StatType::from_str(&v).unwrap_or(StatType::Speed);
                                    pending.set(Some(TraineeFilter::GrowthBonus { stat, min_value: min }));
                                })
                            }
                            placeholder={"Select stat..."}
                        />
                    </div>
                    <div class={FilterSectionStyle::CLASS_NAME}>
                        <label>{"Min Growth (optional)"}</label>
                        <input type="number" class={FilterInputStyle::CLASS_NAME}
                            placeholder="e.g. 10"
                            value={(*gb_min).clone()}
                            oninput={
                                let gb_min = gb_min.clone();
                                let gb_stat = gb_stat.clone();
                                let pending = pending.clone();
                                Callback::from(move |e: InputEvent| {
                                    let val = e.target_unchecked_into::<HtmlInputElement>().value();
                                    gb_min.set(val.clone());
                                    let stat = StatType::from_str(&gb_stat).unwrap_or(StatType::Speed);
                                    let min = val.parse::<i64>().ok();
                                    pending.set(Some(TraineeFilter::GrowthBonus { stat, min_value: min }));
                                })
                            }
                        />
                    </div>
                </>
            }
        }
        AddingType::MinAptitude => {
            let cat_opts: Vec<SelectOption<String>> = AptitudeType::all()
                .iter()
                .map(|a| SelectOption {
                    value: a.value().to_string(),
                    label: a.label().to_string(),
                })
                .collect();
            let level_opts: Vec<SelectOption<String>> = LEVEL_OPTIONS
                .iter()
                .map(|(l, v)| SelectOption {
                    value: v.to_string(),
                    label: l.to_string(),
                })
                .collect();
            html! {
                <>
                    <div class={FilterSectionStyle::CLASS_NAME}>
                        <label>{"Category"}</label>
                        <CustomSelect
                            options={cat_opts}
                            selected={Some((*ma_cat).clone())}
                            on_change={
                                let ma_cat = ma_cat.clone();
                                let ma_level = ma_level.clone();
                                let pending = pending.clone();
                                Callback::from(move |v: String| {
                                    ma_cat.set(v.clone());
                                    let category = AptitudeType::from_str(&v).unwrap_or(AptitudeType::Turf);
                                    pending.set(Some(TraineeFilter::MinAptitude {
                                        category,
                                        min_level: *ma_level,
                                    }));
                                })
                            }
                            placeholder={"Select category..."}
                        />
                    </div>
                    <div class={FilterSectionStyle::CLASS_NAME}>
                        <label>{"Min Level"}</label>
                        <CustomSelect
                            options={level_opts}
                            selected={Some((*ma_level).to_string())}
                            on_change={
                                let ma_level = ma_level.clone();
                                let ma_cat = ma_cat.clone();
                                let pending = pending.clone();
                                Callback::from(move |v: String| {
                                    if let Ok(lvl) = v.parse::<i64>() {
                                        ma_level.set(lvl);
                                        let category = AptitudeType::from_str(&ma_cat).unwrap_or(AptitudeType::Turf);
                                        pending.set(Some(TraineeFilter::MinAptitude {
                                            category,
                                            min_level: lvl,
                                        }));
                                    }
                                })
                            }
                            placeholder={"Select level..."}
                        />
                    </div>
                </>
            }
        }
        AddingType::MaxAAptitudes => {
            html! {
                <div class={FilterSectionStyle::CLASS_NAME}>
                    <label>{"Max A Aptitudes"}</label>
                    <input type="number" class={FilterInputStyle::CLASS_NAME}
                        placeholder="e.g. 3"
                        value={(*max_a_count).clone()}
                        oninput={
                            let max_a_count = max_a_count.clone();
                            let pending = pending.clone();
                            Callback::from(move |e: InputEvent| {
                                let val = e.target_unchecked_into::<HtmlInputElement>().value();
                                max_a_count.set(val.clone());
                                if let Ok(n) = val.parse::<i64>() {
                                    pending.set(Some(TraineeFilter::MaxAAptitudes { max_count: n }));
                                } else {
                                    pending.set(None);
                                }
                            })
                        }
                    />
                </div>
            }
        }
        AddingType::Character => {
            let opts = props
                .options
                .as_ref()
                .map(|o| id_name_options(&o.characters))
                .unwrap_or_default();
            html! {
                <div class={FilterSectionStyle::CLASS_NAME}>
                    <label>{"Character"}</label>
                    <SearchableSelect<i64>
                        options={opts}
                        on_select={
                            let char_id = char_id.clone();
                            let pending = pending.clone();
                            Callback::from(move |id: i64| {
                                char_id.set(Some(id));
                                pending.set(Some(TraineeFilter::Character { character_id: id }));
                            })
                        }
                        selected={*char_id}
                        placeholder={"Search character..."}
                    />
                </div>
            }
        }
        AddingType::HasSkill => {
            let skill_opts = props
                .options
                .as_ref()
                .map(|o| id_name_options(&o.skills))
                .unwrap_or_default();
            let make_pending = {
                let add_skill_id = add_skill_id.clone();
                let add_skill_innate = add_skill_innate.clone();
                let add_skill_event = add_skill_event.clone();
                let add_skill_secret = add_skill_secret.clone();
                move || {
                    add_skill_id.map(|val| {
                        let (group_id, exact_skill_id) = if val < 0 {
                            (0, Some(-val))
                        } else {
                            (val, None)
                        };
                        TraineeFilter::HasSkill {
                            group_id,
                            exact_skill_id,
                            sources: TraineeSkillSources {
                                innate: *add_skill_innate,
                                event: *add_skill_event,
                                secret: *add_skill_secret,
                            },
                        }
                    })
                }
            };
            html! {
                <>
                    <div class={FilterSectionStyle::CLASS_NAME}>
                        <label>{"Skill"}</label>
                        <SearchableSelect<i64>
                            options={skill_opts}
                            on_select={
                                let v = add_skill_id.clone();
                                Callback::from(move |id: i64| {
                                    v.set(Some(id));
                                })
                            }
                            selected={*add_skill_id}
                            placeholder={"Search skill..."}
                        />
                    </div>
                    <div class={FilterSectionStyle::CLASS_NAME} style="margin-top:8px;">
                        <label>{"Sources"}</label>
                        {checkbox_input(*add_skill_innate, "Innate", {
                            let v = add_skill_innate.clone();
                            let pending = pending.clone();
                            let make = make_pending.clone();
                            Callback::from(move |c| { v.set(c); pending.set(make()); })
                        })}
                        {checkbox_input(*add_skill_event, "Event", {
                            let v = add_skill_event.clone();
                            let pending = pending.clone();
                            let make = make_pending.clone();
                            Callback::from(move |c| { v.set(c); pending.set(make()); })
                        })}
                        {checkbox_input(*add_skill_secret, "Secret", {
                            let v = add_skill_secret.clone();
                            let pending = pending.clone();
                            let make = make_pending.clone();
                            Callback::from(move |c| { v.set(c); pending.set(make()); })
                        })}
                    </div>
                </>
            }
        }
    };

    html! {
        <div class={FilterPanelStyle::CLASS_NAME}>
            <div class={FilterTitleStyle::CLASS_NAME}>{"Filters"}</div>

            if props.filters.is_empty() {
                <div class={FilterEmptyHintStyle::CLASS_NAME}>{"No filters active."}</div>
            }

            {props.filters.iter().enumerate().map(|(idx, f)| {
                let label = filter_label(f, props.options.as_ref());
                let remove = remove_filter.clone();
                let remove_click = Callback::from(move |_| remove.emit(idx));
                let edit = {
                    let open_edit = open_edit.clone();
                    Callback::from(move |_| open_edit.emit(idx))
                };
                let is_editing = matches!(*editing_idx, Some(e) if e == idx);
                let pill_style = if is_editing {
                    "border:1px solid #f59e0b;".to_string()
                } else {
                    "border:1px solid #334155;".to_string()
                };
                html! {
                    <div class={FilterChipStyle::CLASS_NAME} style={pill_style}>
                        <button type="button" onclick={edit} class={FilterChipTextStyle::CLASS_NAME} style="text-align:left;background:none;border:none;cursor:pointer;padding:0;font-size:12px;">
                            {label}
                        </button>
                        <button class={FilterChipRemoveStyle::CLASS_NAME} onclick={remove_click}>{"✕"}</button>
                    </div>
                }
            }).collect::<Html>()}

            <div style="margin-top: 12px; border-top: 1px solid #1f2937; padding-top: 12px;">
                {if (*editing_idx).is_some() {
                    html! { <div style="color:#f59e0b;font-size:12px;margin-bottom:6px;">{"Editing existing filter — click Save to apply changes"}</div> }
                } else { html! {} }}
                <FilterTypePicker
                    selected={adding_type_to_key(&(*adding))}
                    on_select={on_type_select}
                    disabled={(*editing_idx).is_some()}
                />
            </div>

            if *adding != AddingType::None {
                {editor}
                <div class={FilterActionsStyle::CLASS_NAME} style="margin-top:8px;">
                    <button
                        disabled={match &*adding {
                            AddingType::HasSkill => add_skill_id.is_none(),
                            _ => pending.is_none(),
                        }}
                        onclick={on_add_clicked}
                    >
                        {if (*editing_idx).is_some() { "Save" } else { "Add" }}
                    </button>
                    <button class={SecondaryBtnStyle::CLASS_NAME} onclick={on_cancel}>
                        {"Cancel"}
                    </button>
                </div>
            }
        </div>
    }
}

// ── Filter type picker ──────────────────────────────────────────────

#[derive(Properties, Clone, PartialEq)]
struct FilterTypePickerProps {
    selected: Option<String>,
    on_select: Callback<AddingType>,
    #[prop_or_default]
    disabled: bool,
}

#[function_component]
fn FilterTypePicker(props: &FilterTypePickerProps) -> Html {
    let options = vec![
        SelectOption {
            value: "ownership".to_string(),
            label: "Owned status".into(),
        },
        SelectOption {
            value: "growth".to_string(),
            label: "Growth bonus".into(),
        },
        SelectOption {
            value: "min_apt".to_string(),
            label: "Min aptitude".into(),
        },
        SelectOption {
            value: "max_a".to_string(),
            label: "Max A aptitudes".into(),
        },
        SelectOption {
            value: "character".to_string(),
            label: "Character".into(),
        },
        SelectOption {
            value: "has_skill".to_string(),
            label: "Has Skill".into(),
        },
    ];

    let on_select = {
        let cb = props.on_select.clone();
        Callback::from(move |v: String| {
            let t = match v.as_str() {
                "ownership" => AddingType::Ownership,
                "growth" => AddingType::GrowthBonus,
                "min_apt" => AddingType::MinAptitude,
                "max_a" => AddingType::MaxAAptitudes,
                "character" => AddingType::Character,
                "has_skill" => AddingType::HasSkill,
                _ => AddingType::None,
            };
            cb.emit(t);
        })
    };

    html! {
        <SearchableSelect<String>
            options={options}
            selected={props.selected.clone()}
            on_select={on_select}
            placeholder={"Add Filter…".to_string()}
            disabled={props.disabled}
        />
    }
}
