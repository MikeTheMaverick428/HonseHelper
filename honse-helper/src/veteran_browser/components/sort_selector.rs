use crate::styles::{sort_selector::*, Style};
use shared::veteran_browser::SortConfig;
use yew::prelude::*;

use super::custom_select::CustomSelect;
use super::searchable_select::SelectOption;

#[derive(Properties, PartialEq)]
pub struct SortSelectorProps {
    pub sort: SortConfig,
    pub on_change: Callback<SortConfig>,
    #[prop_or(false)]
    pub api_mode: bool,
}

#[function_component]
pub fn SortSelector(props: &SortSelectorProps) -> Html {
    let on_key_change = {
        let sort = props.sort.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |val: String| {
            let mut s = sort.clone();
            s.key = val;
            on_change.emit(s);
        })
    };

    let on_dir_change = {
        let sort = props.sort.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |val: String| {
            let mut s = sort.clone();
            s.direction = val;
            on_change.emit(s);
        })
    };

    let mut key_options = vec![
        SelectOption {
            value: "CreatedAt".to_string(),
            label: "Created".to_string(),
        },
        SelectOption {
            value: "Rank".to_string(),
            label: "Rank".to_string(),
        },
        SelectOption {
            value: "WhiteSparkCount".to_string(),
            label: "White Sparks".to_string(),
        },
        SelectOption {
            value: "MajorWinCount".to_string(),
            label: "Major Wins".to_string(),
        },
        SelectOption {
            value: "Affinity".to_string(),
            label: "Affinity".to_string(),
        },
    ];
    if !props.api_mode {
        key_options.push(SelectOption {
            value: "Name".to_string(),
            label: "Name".to_string(),
        });
    }

    html! {
        <div class={SortSelectorStyle::CLASS_NAME}>
            <label>{"Sort:"}</label>
            <CustomSelect
                options={key_options}
                selected={Some(props.sort.key.clone())}
                on_change={on_key_change}
                class={"sort-select"}
            />
            <CustomSelect
                options={vec![
                    SelectOption { value: "Desc".to_string(), label: "Desc".to_string() },
                    SelectOption { value: "Asc".to_string(), label: "Asc".to_string() },
                ]}
                selected={Some(props.sort.direction.clone())}
                on_change={on_dir_change}
                class={"sort-select"}
            />
        </div>
    }
}
