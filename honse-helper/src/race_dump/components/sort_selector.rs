use crate::styles::{sort_selector::SortSelectorStyle, Style};
use crate::veteran_browser::components::custom_select::CustomSelect;
use crate::veteran_browser::components::searchable_select::SelectOption;
use shared::veteran_browser::SortConfig;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RaceSortSelectorProps {
    pub sort: SortConfig,
    pub on_change: Callback<SortConfig>,
}

#[function_component]
pub fn RaceSortSelector(props: &RaceSortSelectorProps) -> Html {
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

    let key_options = vec![
        SelectOption {
            value: "id".to_string(),
            label: "ID".to_string(),
        },
        SelectOption {
            value: "capture_time".to_string(),
            label: "Time".to_string(),
        },
        SelectOption {
            value: "participant_count".to_string(),
            label: "Participants".to_string(),
        },
        SelectOption {
            value: "player_count".to_string(),
            label: "Players".to_string(),
        },
        SelectOption {
            value: "distance".to_string(),
            label: "Distance".to_string(),
        },
        SelectOption {
            value: "race_type".to_string(),
            label: "Type".to_string(),
        },
    ];

    html! {
        <div class={SortSelectorStyle::CLASS_NAME}>
            <label>{"Sort:"}</label>
            <CustomSelect
                options={key_options}
                selected={Some(props.sort.key.clone())}
                on_change={on_key_change}
            />
            <CustomSelect
                options={vec![
                    SelectOption { value: "desc".to_string(), label: "Desc".to_string() },
                    SelectOption { value: "asc".to_string(), label: "Asc".to_string() },
                ]}
                selected={Some(props.sort.direction.clone())}
                on_change={on_dir_change}
            />
        </div>
    }
}
