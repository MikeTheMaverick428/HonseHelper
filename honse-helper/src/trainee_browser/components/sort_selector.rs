use crate::styles::sort_selector::*;
use crate::styles::Style;
use crate::veteran_browser::components::custom_select::CustomSelect;
use crate::veteran_browser::components::searchable_select::SelectOption;
use shared::trainee_browser::TraineeSortConfig;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TrSortSelectorProps {
    pub sort: TraineeSortConfig,
    pub on_change: Callback<TraineeSortConfig>,
    #[prop_or(false)]
    pub show_affinity: bool,
}

#[function_component]
pub fn TrSortSelector(props: &TrSortSelectorProps) -> Html {
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
            value: "name".to_string(),
            label: "Name".to_string(),
        },
        SelectOption {
            value: "id".to_string(),
            label: "ID".to_string(),
        },
        SelectOption {
            value: "owned".to_string(),
            label: "Owned Rarity".to_string(),
        },
        SelectOption {
            value: "piece_count".to_string(),
            label: "Piece Count".to_string(),
        },
    ];
    if props.show_affinity {
        key_options.push(SelectOption {
            value: "Affinity".to_string(),
            label: "Affinity".to_string(),
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
