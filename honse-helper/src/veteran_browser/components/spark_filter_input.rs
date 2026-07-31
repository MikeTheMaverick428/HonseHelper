use yew::prelude::*;

use crate::styles::{filter_panel::*, Style};

use super::searchable_select::{SearchableSelect, SelectOption};

#[derive(Properties, PartialEq)]
pub struct SparkFilterInputProps {
    pub spark_color: String,
    pub group_options: Vec<(i64, String)>,
    pub group_id: Option<i64>,
    pub on_group_change: Callback<Option<i64>>,
    pub min_stars: String,
    pub on_min_change: Callback<String>,
    pub max_stars: String,
    pub on_max_change: Callback<String>,
    pub on_character: bool,
    pub on_on_character_change: Callback<bool>,
    pub min_uma: String,
    pub on_min_uma_change: Callback<String>,
    #[prop_or(false)]
    pub api_mode: bool,
}

fn to_select_options(items: &[(i64, String)]) -> Vec<SelectOption<i64>> {
    items
        .iter()
        .map(|(id, name)| SelectOption {
            value: *id,
            label: name.clone(),
        })
        .collect()
}

#[function_component]
pub fn SparkFilterInput(props: &SparkFilterInputProps) -> Html {
    let options = to_select_options(&props.group_options);

    let on_select = {
        let on_change = props.on_group_change.clone();
        Callback::from(move |id: i64| on_change.emit(Some(id)))
    };

    let on_min_input = {
        let on_change = props.on_min_change.clone();
        Callback::from(move |e: InputEvent| {
            on_change.emit(
                e.target_unchecked_into::<web_sys::HtmlInputElement>()
                    .value(),
            );
        })
    };
    let on_max_input = {
        let on_change = props.on_max_change.clone();
        Callback::from(move |e: InputEvent| {
            on_change.emit(
                e.target_unchecked_into::<web_sys::HtmlInputElement>()
                    .value(),
            );
        })
    };

    let on_character_check = {
        let on_change = props.on_on_character_change.clone();
        Callback::from(move |e: web_sys::Event| {
            on_change.emit(
                e.target_unchecked_into::<web_sys::HtmlInputElement>()
                    .checked(),
            );
        })
    };

    let on_min_uma_input = {
        let on_change = props.on_min_uma_change.clone();
        Callback::from(move |e: InputEvent| {
            on_change.emit(
                e.target_unchecked_into::<web_sys::HtmlInputElement>()
                    .value(),
            );
        })
    };

    html! {
        <div class={FilterSectionStyle::CLASS_NAME}>
            <label>{ format!("{} Spark", props.spark_color) }</label>
            <SearchableSelect<i64>
                options={options}
                on_select={on_select}
                selected={props.group_id}
                placeholder={"Search spark group..."}
            />
            <div style="display:flex;gap:8px;margin-top:6px;">
                <input type="number" class={FilterInputStyle::CLASS_NAME} placeholder="Min stars"
                    value={props.min_stars.clone()}
                    oninput={on_min_input} />
                <input type="number" class={FilterInputStyle::CLASS_NAME} placeholder="Max stars"
                    value={props.max_stars.clone()}
                    oninput={on_max_input} />
            </div>
            if !props.api_mode {
                <div style="display:flex;gap:8px;margin-top:6px;">
                    <input type="number" class={FilterInputStyle::CLASS_NAME} placeholder="Min UMA (1-3)"
                        value={props.min_uma.clone()}
                        oninput={on_min_uma_input} />
                </div>
            }
            <label style="display:flex;align-items:center;gap:6px;margin-top:6px;cursor:pointer;">
                <input type="checkbox" checked={props.on_character} onchange={on_character_check} />
                {"On Character"}
            </label>
        </div>
    }
}
