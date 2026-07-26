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
use shared::RaceDumpFilterOptions;
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
        RaceDumpFilter::CaptureDate { after, before } => {
            let mut s = "Date".to_string();
            if let Some(a) = after {
                s += &format!(" >= {}", a);
            }
            if let Some(b) = before {
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

#[function_component]
pub fn RaceFilterPanel(props: &RaceFilterPanelProps) -> Html {
    let adding = use_state(|| AddingType::None);
    let pending = use_state(|| None::<RaceDumpFilter>);
    let options = props.options.clone();

    let remove_filter = {
        let filters = props.filters.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |idx: usize| {
            let mut nf = filters.clone();
            nf.remove(idx);
            on_change.emit(nf);
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
        Callback::from(move |_: MouseEvent| {
            if let Some(f) = (*pending).clone() {
                let mut nf = filters.clone();
                nf.push(f);
                on_change.emit(nf);
            }
            adding.set(AddingType::None);
            pending.set(None);
        })
    };

    let on_cancel = {
        let adding = adding.clone();
        let pending = pending.clone();
        Callback::from(move |_: MouseEvent| {
            adding.set(AddingType::None);
            pending.set(None);
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
                let onclick = Callback::from(move |_| remove.emit(idx));
                html! {
                    <div class={FilterChipStyle::CLASS_NAME}>
                        <span class={FilterChipTextStyle::CLASS_NAME}>{label}</span>
                        <button class={FilterChipRemoveStyle::CLASS_NAME} onclick={onclick}>{"✕"}</button>
                    </div>
                }
            }).collect::<Html>()}

            <div style="margin-top: 12px; border-top: 1px solid #1f2937; padding-top: 12px;">
                <FilterTypePicker
                    selected={adding_type_to_key(&(*adding))}
                    on_select={on_type_select}
                />
            </div>

            if *adding != AddingType::None {
                <FilterEditor
                    current={(*adding).clone()}
                    options={options}
                    on_pending={on_pending.clone()}
                />
                <div class={FilterActionsStyle::CLASS_NAME} style="margin-top:8px;">
                    <button
                        disabled={pending.is_none()}
                        onclick={on_add_clicked}
                    >
                        {"Add"}
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
        />
    }
}

// ── Individual filter input widgets ─────────────────────────────────

#[derive(Properties, Clone, PartialEq)]
struct RaceTypeFilterProps {
    on_pending: Callback<Option<RaceDumpFilter>>,
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
    html! {
        <div style="margin-top: 8px;">
            <SearchableSelect<shared::race_dump_types::RaceType>
                options={options}
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
}

#[function_component]
fn DistanceMetersFilter(props: &DistanceMetersFilterProps) -> Html {
    let min_val = use_state(String::new);
    let max_val = use_state(String::new);

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
    html! {
        <div style="margin-top: 8px;">
            <SearchableSelect<shared::models::RaceDistance>
                options={options}
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
}

#[function_component]
fn VeteranHashFilter(props: &VeteranHashFilterProps) -> Html {
    let hash_val = use_state(String::new);
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
    html! {
        <div style="margin-top: 8px;">
            <SearchableSelect<shared::race_dump_types::GroundType>
                options={options}
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
    html! {
        <div style="margin-top: 8px;">
            <SearchableSelect<shared::race_dump_types::Season>
                options={options}
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
    html! {
        <div style="margin-top: 8px;">
            <SearchableSelect<shared::race_dump_types::Weather>
                options={options}
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
    html! {
        <div style="margin-top: 8px;">
            <SearchableSelect<shared::models::GroundCondition>
                options={options}
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
}

#[function_component]
fn CaptureDateFilter(props: &CaptureDateFilterProps) -> Html {
    let after_val = use_state(String::new);
    let before_val = use_state(String::new);

    let emit_pending = {
        let after_val = after_val.clone();
        let before_val = before_val.clone();
        let cb = props.on_pending.clone();
        move || {
            let a = if after_val.is_empty() {
                None
            } else {
                Some((*after_val).clone())
            };
            let b = if before_val.is_empty() {
                None
            } else {
                Some((*before_val).clone())
            };
            if a.is_some() || b.is_some() {
                cb.emit(Some(RaceDumpFilter::CaptureDate {
                    after: a,
                    before: b,
                }));
            } else {
                cb.emit(None);
            }
        }
    };

    html! {
        <div style="margin-top: 8px;">
            <div class={FilterRangeStyle::CLASS_NAME}>
                <input class={FilterInputStyle::CLASS_NAME} type="text" placeholder="After (e.g. 2025-01-01)"
                    value={(*after_val).clone()}
                    oninput={let after_val = after_val.clone(); let emit = emit_pending.clone(); Callback::from(move |e: InputEvent| {
                        after_val.set(e.target_unchecked_into::<HtmlInputElement>().value());
                        emit();
                    })} />
                <span style="color:#64748b;">{"–"}</span>
                <input class={FilterInputStyle::CLASS_NAME} type="text" placeholder="Before"
                    value={(*before_val).clone()}
                    oninput={let before_val = before_val.clone(); let emit = emit_pending.clone(); Callback::from(move |e: InputEvent| {
                        before_val.set(e.target_unchecked_into::<HtmlInputElement>().value());
                        emit();
                    })} />
            </div>
        </div>
    }
}

// ── Character / Trainee (shared) ───────────────────────────────────

#[derive(Properties, Clone, PartialEq)]
struct CharOrTraineeFilterProps {
    is_char: bool,
    options: Option<RaceDumpFilterOptions>,
    on_pending: Callback<Option<RaceDumpFilter>>,
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
    html! {
        <div style="margin-top: 8px;">
            <SearchableSelect<i64>
                options={items}
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
    html! {
        <div style="margin-top: 8px;">
            <SearchableSelect<String>
                options={tags}
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
    on_pending: Callback<Option<RaceDumpFilter>>,
}

#[function_component]
fn FilterEditor(props: &FilterEditorProps) -> Html {
    match props.current {
        AddingType::RaceType => html! {
            <RaceTypeFilter on_pending={props.on_pending.clone()}/>
        },
        AddingType::DistanceMeters => html! {
            <DistanceMetersFilter on_pending={props.on_pending.clone()}/>
        },
        AddingType::Distance => html! {
            <DistanceFilter on_pending={props.on_pending.clone()}/>
        },
        AddingType::GroundType => html! {
            <GroundTypeFilter on_pending={props.on_pending.clone()}/>
        },
        AddingType::Season => html! {
            <SeasonFilter on_pending={props.on_pending.clone()}/>
        },
        AddingType::Weather => html! {
            <WeatherFilter on_pending={props.on_pending.clone()}/>
        },
        AddingType::GroundCondition => html! {
            <GroundConditionFilter on_pending={props.on_pending.clone()}/>
        },
        AddingType::Character => html! {
            <CharOrTraineeFilter is_char={true} options={props.options.clone()} on_pending={props.on_pending.clone()}/>
        },
        AddingType::Trainee => html! {
            <CharOrTraineeFilter is_char={false} options={props.options.clone()} on_pending={props.on_pending.clone()}/>
        },
        AddingType::VeteranHash => html! {
            <VeteranHashFilter on_pending={props.on_pending.clone()}/>
        },
        AddingType::HasTag => html! {
            <TagFilter options={props.options.clone()} on_pending={props.on_pending.clone()}/>
        },
        AddingType::CaptureDate => html! {
            <CaptureDateFilter on_pending={props.on_pending.clone()}/>
        },
        AddingType::None => html! {},
    }
}
