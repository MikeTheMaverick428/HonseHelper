use crate::components::date_time_selector::DateTimeRangeSelector;
use crate::styles::{
    filter_panel::{
        FilterActionsStyle, FilterChipRemoveStyle, FilterChipStyle, FilterChipTextStyle,
        FilterEmptyHintStyle, FilterInputStyle, FilterPanelStyle, FilterRangeStyle,
        FilterSectionStyle, FilterTitleStyle,
    },
    legacy_planner::SecondaryBtnStyle,
    Style,
};
use crate::veteran_browser::components::searchable_select::{SearchableSelect, SelectOption};
use shared::race_dump_types::RaceDumpFilter;
use shared::{DateTimeRange, RaceDumpFilterOptions};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RaceFilterPanelProps {
    pub filters: Vec<RaceDumpFilter>,
    pub on_change: Callback<Vec<RaceDumpFilter>>,
    pub options: Option<RaceDumpFilterOptions>,
}

#[derive(Clone, PartialEq)]
enum AddingType {
    None,
    RaceType,
    DistanceMeters,
    Distance,
    GroundType,
    Season,
    Weather,
    GroundCondition,
    Character,
    Trainee,
    VeteranHash,
    HasTag,
    CaptureDate,
}

fn filter_label(f: &RaceDumpFilter, options: Option<&RaceDumpFilterOptions>) -> String {
    match f {
        RaceDumpFilter::RaceType(v) => format!("Race Type: {}", v.label()),
        RaceDumpFilter::DistanceMeters { min, max } => {
            let mut s = "Distance (m)".to_string();
            if let Some(m) = min {
                s += &format!(" >= {}", m);
            }
            if let Some(m) = max {
                s += &format!(" <= {}", m);
            }
            s
        }
        RaceDumpFilter::Distance(d) => format!("Distance: {:?}", d),
        RaceDumpFilter::GroundType(v) => format!("Ground: {}", v.label()),
        RaceDumpFilter::Season(v) => format!("Season: {}", v.label()),
        RaceDumpFilter::Weather(v) => format!("Weather: {}", v.label()),
        RaceDumpFilter::GroundCondition(v) => format!("Condition: {}", ground_cond_label(v)),
        RaceDumpFilter::Character(id) => options
            .and_then(|o| o.characters.iter().find(|(i, _)| i == id))
            .map(|(_, n)| format!("Character: {}", n))
            .unwrap_or_else(|| format!("Character: #{}", id)),
        RaceDumpFilter::Trainee(id) => options
            .and_then(|o| o.trainees.iter().find(|(i, _)| i == id))
            .map(|(_, n)| format!("Trainee: {}", n))
            .unwrap_or_else(|| format!("Trainee: #{}", id)),
        RaceDumpFilter::VeteranHash(h) => format!("Hash: {:016x}", h),
        RaceDumpFilter::HasTag(s) => format!("Tag: {}", s),
        RaceDumpFilter::CaptureDate(r) => {
            let mut s = "Date".to_string();
            if let Some(a) = &r.after {
                s += &format!(" >= {}", a);
            }
            if let Some(b) = &r.before {
                s += &format!(" <= {}", b);
            }
            s
        }
    }
}

fn ground_cond_label(v: &shared::models::GroundCondition) -> &'static str {
    match v {
        shared::models::GroundCondition::Firm => "Firm",
        shared::models::GroundCondition::Good => "Good",
        shared::models::GroundCondition::Soft => "Soft",
        shared::models::GroundCondition::Heavy => "Heavy",
    }
}

fn adding_type_to_key(t: &AddingType) -> Option<String> {
    match t {
        AddingType::RaceType => Some("race_type"),
        AddingType::DistanceMeters => Some("distance_meters"),
        AddingType::Distance => Some("distance"),
        AddingType::GroundType => Some("ground"),
        AddingType::Season => Some("season"),
        AddingType::Weather => Some("weather"),
        AddingType::GroundCondition => Some("condition"),
        AddingType::Character => Some("character"),
        AddingType::Trainee => Some("trainee"),
        AddingType::VeteranHash => Some("veteran_hash"),
        AddingType::HasTag => Some("tag"),
        AddingType::CaptureDate => Some("date"),
        AddingType::None => None,
    }
    .map(String::from)
}

fn filter_to_adding_type(f: &RaceDumpFilter) -> Option<(&'static str, AddingType)> {
    match f {
        RaceDumpFilter::RaceType(_) => Some(("race_type", AddingType::RaceType)),
        RaceDumpFilter::DistanceMeters { .. } => {
            Some(("distance_meters", AddingType::DistanceMeters))
        }
        RaceDumpFilter::Distance(_) => Some(("distance", AddingType::Distance)),
        RaceDumpFilter::GroundType(_) => Some(("ground", AddingType::GroundType)),
        RaceDumpFilter::Season(_) => Some(("season", AddingType::Season)),
        RaceDumpFilter::Weather(_) => Some(("weather", AddingType::Weather)),
        RaceDumpFilter::GroundCondition(_) => Some(("condition", AddingType::GroundCondition)),
        RaceDumpFilter::Character(_) => Some(("character", AddingType::Character)),
        RaceDumpFilter::Trainee(_) => Some(("trainee", AddingType::Trainee)),
        RaceDumpFilter::VeteranHash(_) => Some(("veteran_hash", AddingType::VeteranHash)),
        RaceDumpFilter::HasTag(_) => Some(("tag", AddingType::HasTag)),
        RaceDumpFilter::CaptureDate(_) => Some(("date", AddingType::CaptureDate)),
    }
}

#[function_component]
pub fn RaceFilterPanel(props: &RaceFilterPanelProps) -> Html {
    let adding = use_state(|| AddingType::None);
    let pending = use_state(|| None::<RaceDumpFilter>);
    let editing_idx: UseStateHandle<Option<usize>> = use_state(|| None);
    let edit_nonce: UseStateHandle<u32> = use_state(|| 0);
    let options = props.options.clone();

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
        let edit_nonce = edit_nonce.clone();
        Callback::from(move |idx: usize| {
            let Some(f) = filters.get(idx) else { return };
            let Some((_, t)) = filter_to_adding_type(f) else { return };
            adding.set(t);
            pending.set(Some(f.clone()));
            editing_idx.set(Some(idx));
            edit_nonce.set(*edit_nonce + 1);
        })
    };

    let on_pending = {
        let pending = pending.clone();
        Callback::from(move |f: Option<RaceDumpFilter>| {
            pending.set(f);
        })
    };

    let on_type_select = {
        let adding = adding.clone();
        let pending = pending.clone();
        Callback::from(move |t: AddingType| {
            adding.set(t);
            pending.set(None);
        })
    };

    let on_add_clicked = {
        let adding = adding.clone();
        let pending = pending.clone();
        let on_change = props.on_change.clone();
        let filters = props.filters.clone();
        let editing_idx = editing_idx.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(f) = (*pending).clone() {
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

    html! {
        <div class={FilterPanelStyle::CLASS_NAME}>
            <div class={FilterTitleStyle::CLASS_NAME}>{"Filters"}</div>

            if props.filters.is_empty() {
                <div class={FilterEmptyHintStyle::CLASS_NAME}>{"No filters active."}</div>
            }

            {props.filters.iter().enumerate().map(|(idx, f)| {
                let label = filter_label(f, options.as_ref());
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
                if (*editing_idx).is_some() {
                    <div style="color:#f59e0b;font-size:12px;margin-bottom:6px;">
                        {"Editing existing filter — click Save to apply changes"}
                    </div>
                }
                <FilterTypePicker
                    selected={adding_type_to_key(&(*adding))}
                    on_select={on_type_select}
                    disabled={(*editing_idx).is_some()}
                />
            </div>

            if *adding != AddingType::None {
                <FilterEditor
                    key={match *editing_idx {
                        Some(i) => format!("edit-{}-{}", i, *edit_nonce),
                        None => "edit-none".to_string(),
                    }}
                    current={(*adding).clone()}
                    options={options}
                    initial={(*pending).clone()}
                    on_pending={on_pending.clone()}
                />
                <div class={FilterActionsStyle::CLASS_NAME} style="margin-top:8px;">
                    <button
                        disabled={pending.is_none()}
                        onclick={on_add_clicked}
                    >
                        { if (*editing_idx).is_some() { "Save" } else { "Add" } }
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
            value: "race_type".to_string(),
            label: "Race Type".into(),
        },
        SelectOption {
            value: "distance_meters".to_string(),
            label: "Distance (meters)".into(),
        },
        SelectOption {
            value: "distance".to_string(),
            label: "Distance (category)".into(),
        },
        SelectOption {
            value: "ground".to_string(),
            label: "Ground".into(),
        },
        SelectOption {
            value: "season".to_string(),
            label: "Season".into(),
        },
        SelectOption {
            value: "weather".to_string(),
            label: "Weather".into(),
        },
        SelectOption {
            value: "condition".to_string(),
            label: "Ground Condition".into(),
        },
        SelectOption {
            value: "character".to_string(),
            label: "Character".into(),
        },
        SelectOption {
            value: "trainee".to_string(),
            label: "Trainee".into(),
        },
        SelectOption {
            value: "veteran_hash".to_string(),
            label: "Veteran Hash".into(),
        },
        SelectOption {
            value: "tag".to_string(),
            label: "Tag".into(),
        },
        SelectOption {
            value: "date".to_string(),
            label: "Capture Date".into(),
        },
    ];

    let on_select = {
        let cb = props.on_select.clone();
        Callback::from(move |v: String| {
            let t = match v.as_str() {
                "race_type" => AddingType::RaceType,
                "distance_meters" => AddingType::DistanceMeters,
                "distance" => AddingType::Distance,
                "ground" => AddingType::GroundType,
                "season" => AddingType::Season,
                "weather" => AddingType::Weather,
                "condition" => AddingType::GroundCondition,
                "character" => AddingType::Character,
                "trainee" => AddingType::Trainee,
                "veteran_hash" => AddingType::VeteranHash,
                "tag" => AddingType::HasTag,
                "date" => AddingType::CaptureDate,
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

// ── Individual filter input widgets ─────────────────────────────────

#[derive(Properties, Clone, PartialEq)]
struct RaceTypeFilterProps {
    on_pending: Callback<Option<RaceDumpFilter>>,
    initial: Option<RaceDumpFilter>,
}

#[function_component]
fn RaceTypeFilter(props: &RaceTypeFilterProps) -> Html {
    let options = vec![
        SelectOption {
            value: shared::race_dump_types::RaceType::Single,
            label: "Single".into(),
        },
        SelectOption {
            value: shared::race_dump_types::RaceType::TeamStadium,
            label: "TeamStadium".into(),
        },
    ];
    let selected = props
        .initial
        .as_ref()
        .and_then(|f| match f {
            RaceDumpFilter::RaceType(v) => Some(*v),
            _ => None,
        });
    html! {
        <div style="margin-top: 8px;">
            <SearchableSelect<shared::race_dump_types::RaceType>
                options={options}
                selected={selected}
                on_select={{
                    let cb = props.on_pending.clone();
                    Callback::from(move |v| cb.emit(Some(RaceDumpFilter::RaceType(v))))
                }}
                placeholder={"Select race type…".to_string()}/>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct DistanceMetersFilterProps {
    on_pending: Callback<Option<RaceDumpFilter>>,
    initial: Option<RaceDumpFilter>,
}

#[function_component]
fn DistanceMetersFilter(props: &DistanceMetersFilterProps) -> Html {
    let (min0, max0) = match &props.initial {
        Some(RaceDumpFilter::DistanceMeters { min, max }) => (
            min.map(|m| m.to_string()).unwrap_or_default(),
            max.map(|m| m.to_string()).unwrap_or_default(),
        ),
        _ => (String::new(), String::new()),
    };
    let min_val = use_state(|| min0);
    let max_val = use_state(|| max0);

    let emit_pending = {
        let min_val = min_val.clone();
        let max_val = max_val.clone();
        let cb = props.on_pending.clone();
        move || {
            let min = min_val.parse::<i64>().ok();
            let max = max_val.parse::<i64>().ok();
            if min.is_some() || max.is_some() {
                cb.emit(Some(RaceDumpFilter::DistanceMeters { min, max }));
            } else {
                cb.emit(None);
            }
        }
    };

    html! {
        <div style="margin-top: 8px;">
            <div class={FilterRangeStyle::CLASS_NAME}>
                <input class={FilterInputStyle::CLASS_NAME} type="number" placeholder="min m"
                    value={(*min_val).clone()}
                    oninput={let min_val = min_val.clone(); let emit = emit_pending.clone(); Callback::from(move |e: InputEvent| {
                        min_val.set(e.target_unchecked_into::<HtmlInputElement>().value());
                        emit();
                    })} />
                <span style="color:#64748b;">{"–"}</span>
                <input class={FilterInputStyle::CLASS_NAME} type="number" placeholder="max m"
                    value={(*max_val).clone()}
                    oninput={let max_val = max_val.clone(); let emit = emit_pending.clone(); Callback::from(move |e: InputEvent| {
                        max_val.set(e.target_unchecked_into::<HtmlInputElement>().value());
                        emit();
                    })} />
            </div>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct DistanceFilterProps {
    on_pending: Callback<Option<RaceDumpFilter>>,
    initial: Option<RaceDumpFilter>,
}

#[function_component]
fn DistanceFilter(props: &DistanceFilterProps) -> Html {
    let options = vec![
        SelectOption {
            value: shared::models::RaceDistance::Sprint,
            label: "Sprint (≤1200m)".into(),
        },
        SelectOption {
            value: shared::models::RaceDistance::Mile,
            label: "Mile (1201-2000m)".into(),
        },
        SelectOption {
            value: shared::models::RaceDistance::Medium,
            label: "Medium (2001-2500m)".into(),
        },
        SelectOption {
            value: shared::models::RaceDistance::Long,
            label: "Long (>2500m)".into(),
        },
    ];
    let selected = props
        .initial
        .as_ref()
        .and_then(|f| match f {
            RaceDumpFilter::Distance(v) => Some(*v),
            _ => None,
        });
    html! {
        <div style="margin-top: 8px;">
            <SearchableSelect<shared::models::RaceDistance>
                options={options}
                selected={selected}
                on_select={{
                    let cb = props.on_pending.clone();
                    Callback::from(move |v| cb.emit(Some(RaceDumpFilter::Distance(v))))
                }}
                placeholder={"Select distance…".to_string()}/>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct VeteranHashFilterProps {
    on_pending: Callback<Option<RaceDumpFilter>>,
    initial: Option<RaceDumpFilter>,
}

#[function_component]
fn VeteranHashFilter(props: &VeteranHashFilterProps) -> Html {
    let hash0 = match &props.initial {
        Some(RaceDumpFilter::VeteranHash(h)) => format!("{:x}", *h as u64),
        _ => String::new(),
    };
    let hash_val = use_state(|| hash0);
    let emit_pending = {
        let hash_val = hash_val.clone();
        let cb = props.on_pending.clone();
        move || {
            let v = (*hash_val).clone();
            if v.is_empty() {
                cb.emit(None);
            } else {
                match u64::from_str_radix(&v, 16) {
                    Ok(h) => cb.emit(Some(RaceDumpFilter::VeteranHash(h as i64))),
                    Err(_) => cb.emit(None),
                }
            }
        }
    };
    html! {
        <div style="margin-top: 8px;">
            <div class={FilterSectionStyle::CLASS_NAME}>
                <label>{"Veteran Hash (hex)"}</label>
                <input class={FilterInputStyle::CLASS_NAME} type="text" placeholder="e.g. 0a1b2c3d4e5f6a7b"
                    value={(*hash_val).clone()}
                    oninput={let hash_val = hash_val.clone(); let emit = emit_pending.clone(); Callback::from(move |e: InputEvent| {
                        hash_val.set(e.target_unchecked_into::<HtmlInputElement>().value());
                        emit();
                    })} />
            </div>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct GroundTypeFilterProps {
    on_pending: Callback<Option<RaceDumpFilter>>,
    initial: Option<RaceDumpFilter>,
}

#[function_component]
fn GroundTypeFilter(props: &GroundTypeFilterProps) -> Html {
    let options = vec![
        SelectOption {
            value: shared::race_dump_types::GroundType::Turf,
            label: "Turf".into(),
        },
        SelectOption {
            value: shared::race_dump_types::GroundType::Dirt,
            label: "Dirt".into(),
        },
    ];
    let selected = props
        .initial
        .as_ref()
        .and_then(|f| match f {
            RaceDumpFilter::GroundType(v) => Some(*v),
            _ => None,
        });
    html! {
        <div style="margin-top: 8px;">
            <SearchableSelect<shared::race_dump_types::GroundType>
                options={options}
                selected={selected}
                on_select={{
                    let cb = props.on_pending.clone();
                    Callback::from(move |v| cb.emit(Some(RaceDumpFilter::GroundType(v))))
                }}
                placeholder={"Select ground…".to_string()}/>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct SeasonFilterProps {
    on_pending: Callback<Option<RaceDumpFilter>>,
    initial: Option<RaceDumpFilter>,
}

#[function_component]
fn SeasonFilter(props: &SeasonFilterProps) -> Html {
    let options = vec![
        SelectOption {
            value: shared::race_dump_types::Season::Spring,
            label: "Spring".into(),
        },
        SelectOption {
            value: shared::race_dump_types::Season::Summer,
            label: "Summer".into(),
        },
        SelectOption {
            value: shared::race_dump_types::Season::Fall,
            label: "Fall".into(),
        },
        SelectOption {
            value: shared::race_dump_types::Season::Winter,
            label: "Winter".into(),
        },
        SelectOption {
            value: shared::race_dump_types::Season::CherryBlossom,
            label: "Cherry Blossom".into(),
        },
    ];
    let selected = props
        .initial
        .as_ref()
        .and_then(|f| match f {
            RaceDumpFilter::Season(v) => Some(*v),
            _ => None,
        });
    html! {
        <div style="margin-top: 8px;">
            <SearchableSelect<shared::race_dump_types::Season>
                options={options}
                selected={selected}
                on_select={{
                    let cb = props.on_pending.clone();
                    Callback::from(move |v| cb.emit(Some(RaceDumpFilter::Season(v))))
                }}
                placeholder={"Select season…".to_string()}/>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct WeatherFilterProps {
    on_pending: Callback<Option<RaceDumpFilter>>,
    initial: Option<RaceDumpFilter>,
}

#[function_component]
fn WeatherFilter(props: &WeatherFilterProps) -> Html {
    let options = vec![
        SelectOption {
            value: shared::race_dump_types::Weather::Sunny,
            label: "Sunny".into(),
        },
        SelectOption {
            value: shared::race_dump_types::Weather::Rainy,
            label: "Rainy".into(),
        },
        SelectOption {
            value: shared::race_dump_types::Weather::Snow,
            label: "Snow".into(),
        },
        SelectOption {
            value: shared::race_dump_types::Weather::Cloudy,
            label: "Cloudy".into(),
        },
        SelectOption {
            value: shared::race_dump_types::Weather::Star,
            label: "Star".into(),
        },
        SelectOption {
            value: shared::race_dump_types::Weather::Firework,
            label: "Firework".into(),
        },
    ];
    let selected = props
        .initial
        .as_ref()
        .and_then(|f| match f {
            RaceDumpFilter::Weather(v) => Some(*v),
            _ => None,
        });
    html! {
        <div style="margin-top: 8px;">
            <SearchableSelect<shared::race_dump_types::Weather>
                options={options}
                selected={selected}
                on_select={{
                    let cb = props.on_pending.clone();
                    Callback::from(move |v| cb.emit(Some(RaceDumpFilter::Weather(v))))
                }}
                placeholder={"Select weather…".to_string()}/>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct GroundConditionFilterProps {
    on_pending: Callback<Option<RaceDumpFilter>>,
    initial: Option<RaceDumpFilter>,
}

#[function_component]
fn GroundConditionFilter(props: &GroundConditionFilterProps) -> Html {
    let options = vec![
        SelectOption {
            value: shared::models::GroundCondition::Firm,
            label: "Firm".into(),
        },
        SelectOption {
            value: shared::models::GroundCondition::Good,
            label: "Good".into(),
        },
        SelectOption {
            value: shared::models::GroundCondition::Soft,
            label: "Soft".into(),
        },
        SelectOption {
            value: shared::models::GroundCondition::Heavy,
            label: "Heavy".into(),
        },
    ];
    let selected = props
        .initial
        .as_ref()
        .and_then(|f| match f {
            RaceDumpFilter::GroundCondition(v) => Some(*v),
            _ => None,
        });
    html! {
        <div style="margin-top: 8px;">
            <SearchableSelect<shared::models::GroundCondition>
                options={options}
                selected={selected}
                on_select={{
                    let cb = props.on_pending.clone();
                    Callback::from(move |v| cb.emit(Some(RaceDumpFilter::GroundCondition(v))))
                }}
                placeholder={"Select condition…".to_string()}/>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct CaptureDateFilterProps {
    on_pending: Callback<Option<RaceDumpFilter>>,
    initial: Option<RaceDumpFilter>,
}

#[function_component]
fn CaptureDateFilter(props: &CaptureDateFilterProps) -> Html {
    let range0 = props
        .initial
        .as_ref()
        .and_then(|f| match f {
            RaceDumpFilter::CaptureDate(r) => Some(r.clone()),
            _ => None,
        })
        .unwrap_or_default();
    let range = use_state(|| range0);

    let on_change = {
        let range = range.clone();
        let cb = props.on_pending.clone();
        Callback::from(move |r: DateTimeRange| {
            range.set(r.clone());
            if r.is_empty() {
                cb.emit(None);
            } else {
                cb.emit(Some(RaceDumpFilter::CaptureDate(r)));
            }
        })
    };

    html! {
        <DateTimeRangeSelector
            value={(*range).clone()}
            on_change={on_change}
            show_time={true}
        />
    }
}

// ── Character / Trainee (shared) ───────────────────────────────────

#[derive(Properties, Clone, PartialEq)]
struct CharOrTraineeFilterProps {
    is_char: bool,
    options: Option<RaceDumpFilterOptions>,
    on_pending: Callback<Option<RaceDumpFilter>>,
    initial: Option<RaceDumpFilter>,
}

#[function_component]
fn CharOrTraineeFilter(props: &CharOrTraineeFilterProps) -> Html {
    let items: Vec<SelectOption<i64>> = props
        .options
        .as_ref()
        .map(|o| {
            let src = if props.is_char {
                &o.characters
            } else {
                &o.trainees
            };
            src.iter()
                .map(|(id, n)| SelectOption {
                    value: *id,
                    label: n.clone(),
                })
                .collect()
        })
        .unwrap_or_default();
    let is_char = props.is_char;
    let cb = props.on_pending.clone();
    let selected = props
        .initial
        .as_ref()
        .and_then(|f| match (is_char, f) {
            (true, RaceDumpFilter::Character(id)) => Some(*id),
            (false, RaceDumpFilter::Trainee(id)) => Some(*id),
            _ => None,
        });
    html! {
        <div style="margin-top: 8px;">
            <SearchableSelect<i64>
                options={items}
                selected={selected}
                on_select={Callback::from(move |id: i64| {
                    cb.emit(Some(if is_char { RaceDumpFilter::Character(id) } else { RaceDumpFilter::Trainee(id) }));
                })}
                placeholder={if is_char { "Search character…".to_string() } else { "Search trainee…".to_string() }}/>
        </div>
    }
}

// ── Tag filter ──────────────────────────────────────────────────────

#[derive(Properties, Clone, PartialEq)]
struct TagFilterProps {
    options: Option<RaceDumpFilterOptions>,
    on_pending: Callback<Option<RaceDumpFilter>>,
    initial: Option<RaceDumpFilter>,
}

#[function_component]
fn TagFilter(props: &TagFilterProps) -> Html {
    let tags: Vec<SelectOption<String>> = props
        .options
        .as_ref()
        .map(|o| {
            o.tags
                .iter()
                .map(|t| SelectOption {
                    value: t.clone(),
                    label: t.clone(),
                })
                .collect()
        })
        .unwrap_or_default();
    let cb = props.on_pending.clone();
    let selected = props
        .initial
        .as_ref()
        .and_then(|f| match f {
            RaceDumpFilter::HasTag(v) => Some(v.clone()),
            _ => None,
        });
    html! {
        <div style="margin-top: 8px;">
            <SearchableSelect<String>
                options={tags}
                selected={selected}
                on_select={Callback::from(move |v: String| cb.emit(Some(RaceDumpFilter::HasTag(v))))}
                placeholder={"Search tag…".to_string()}/>
        </div>
    }
}

// ── FilterEditor (dispatches based on current AddingType) ──────────

#[derive(Properties, Clone, PartialEq)]
struct FilterEditorProps {
    current: AddingType,
    options: Option<RaceDumpFilterOptions>,
    initial: Option<RaceDumpFilter>,
    on_pending: Callback<Option<RaceDumpFilter>>,
}

#[function_component]
fn FilterEditor(props: &FilterEditorProps) -> Html {
    let initial = props.initial.clone();
    match props.current {
        AddingType::RaceType => html! {
            <RaceTypeFilter initial={initial.clone()} on_pending={props.on_pending.clone()}/>
        },
        AddingType::DistanceMeters => html! {
            <DistanceMetersFilter initial={initial.clone()} on_pending={props.on_pending.clone()}/>
        },
        AddingType::Distance => html! {
            <DistanceFilter initial={initial.clone()} on_pending={props.on_pending.clone()}/>
        },
        AddingType::GroundType => html! {
            <GroundTypeFilter initial={initial.clone()} on_pending={props.on_pending.clone()}/>
        },
        AddingType::Season => html! {
            <SeasonFilter initial={initial.clone()} on_pending={props.on_pending.clone()}/>
        },
        AddingType::Weather => html! {
            <WeatherFilter initial={initial.clone()} on_pending={props.on_pending.clone()}/>
        },
        AddingType::GroundCondition => html! {
            <GroundConditionFilter initial={initial.clone()} on_pending={props.on_pending.clone()}/>
        },
        AddingType::Character => html! {
            <CharOrTraineeFilter is_char={true} options={props.options.clone()} initial={initial.clone()} on_pending={props.on_pending.clone()}/>
        },
        AddingType::Trainee => html! {
            <CharOrTraineeFilter is_char={false} options={props.options.clone()} initial={initial.clone()} on_pending={props.on_pending.clone()}/>
        },
        AddingType::VeteranHash => html! {
            <VeteranHashFilter initial={initial.clone()} on_pending={props.on_pending.clone()}/>
        },
        AddingType::HasTag => html! {
            <TagFilter options={props.options.clone()} initial={initial.clone()} on_pending={props.on_pending.clone()}/>
        },
        AddingType::CaptureDate => html! {
            <CaptureDateFilter initial={initial.clone()} on_pending={props.on_pending.clone()}/>
        },
        AddingType::None => html! {},
    }
}
