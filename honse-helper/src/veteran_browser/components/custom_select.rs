use crate::styles::{custom_select::*, Style};
use yew::prelude::*;

use super::searchable_select::SelectOption;

#[derive(Properties, Clone, PartialEq)]
pub struct CustomSelectProps {
    pub options: Vec<SelectOption<String>>,
    pub on_change: Callback<String>,
    #[prop_or_default]
    pub selected: Option<String>,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub class: String,
}

#[function_component]
pub fn CustomSelect(props: &CustomSelectProps) -> Html {
    let is_open = use_state(|| false);

    let toggle = {
        let is_open = is_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_open.set(!*is_open);
        })
    };

    let on_blur = {
        let is_open = is_open.clone();
        Callback::from(move |_: FocusEvent| {
            let is_open = is_open.clone();
            gloo_timers::callback::Timeout::new(200, move || {
                is_open.set(false);
            })
            .forget();
        })
    };

    let selected_label = props
        .selected
        .as_ref()
        .and_then(|s| {
            props
                .options
                .iter()
                .find(|o| &o.value == s)
                .map(|o| o.label.clone())
        })
        .unwrap_or_else(|| {
            if props.placeholder.is_empty() {
                String::new()
            } else {
                props.placeholder.clone()
            }
        });

    let on_option_click = {
        let on_change = props.on_change.clone();
        let is_open = is_open.clone();
        move |val: String| {
            let on_change = on_change.clone();
            let is_open = is_open.clone();
            Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                on_change.emit(val.clone());
                is_open.set(false);
            })
        }
    };

    html! {
        <div class={format!("{} {}", CustomSelectRootStyle::CLASS_NAME, props.class)} tabindex="0" onblur={on_blur}>
            <div class={CustomSelectTriggerStyle::CLASS_NAME} onclick={toggle}>
                <span class={if props.selected.is_some() { CustomSelectValueStyle::CLASS_NAME } else { CustomSelectPlaceholderStyle::CLASS_NAME }}>
                    {selected_label}
                </span>
                <span class={CustomSelectArrowStyle::CLASS_NAME}>{"▾"}</span>
            </div>
            if *is_open {
                <div class={CustomSelectDropdownStyle::CLASS_NAME}>
                    { for props.options.iter().map(|option| {
                        let selected = props.selected.as_ref().map_or(false, |s| s == &option.value);
                        let onclick = on_option_click(option.value.clone());
                        let class = if selected { format!("{} {}", CustomSelectOptionStyle::CLASS_NAME, CustomSelectOptionSelectedStyle::CLASS_NAME) } else { CustomSelectOptionStyle::CLASS_NAME.to_string() };
                        html! {
                            <div class={class} onclick={onclick}>
                                { &option.label }
                            </div>
                        }
                    })}
                </div>
            }
        </div>
    }
}
