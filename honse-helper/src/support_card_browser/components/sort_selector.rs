use crate::styles::{sort_selector::*, Style};
use crate::veteran_browser::components::custom_select::CustomSelect;
use crate::veteran_browser::components::searchable_select::SelectOption;
use shared::support_card_browser::SupportCardSortConfig;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ScSortSelectorProps {
    pub sort: SupportCardSortConfig,
    pub on_change: Callback<SupportCardSortConfig>,
}

#[function_component]
pub fn ScSortSelector(props: &ScSortSelectorProps) -> Html {
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
            value: "Name".to_string(),
            label: "Name".to_string(),
        },
        SelectOption {
            value: "Rarity".to_string(),
            label: "Rarity".to_string(),
        },
        SelectOption {
            value: "CardType".to_string(),
            label: "Card Type".to_string(),
        },
        SelectOption {
            value: "Level".to_string(),
            label: "Level".to_string(),
        },
    ];

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
