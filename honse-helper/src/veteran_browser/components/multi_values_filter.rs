use crate::styles::Style as _;
use crate::styles::filter_panel::FilterInputStyle;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct MultiValuesFilterProps {
    pub values: Vec<String>,
    pub on_add: Callback<String>,
    pub on_remove: Callback<String>,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or(false)]
    pub digits_only: bool,
}

#[function_component]
pub fn MultiValuesFilter(props: &MultiValuesFilterProps) -> Html {
    let input_value = use_state(String::new);

    let on_keydown = {
        let input_value = input_value.clone();
        let on_add = props.on_add.clone();
        let digits_only = props.digits_only;
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let val = (*input_value).trim().to_string();
                if val.is_empty() {
                    return;
                }
                if digits_only && !val.chars().all(|c| c.is_ascii_digit()) {
                    return;
                }
                on_add.emit(val);
                input_value.set(String::new());
            }
        })
    };

    let on_input = {
        let input_value = input_value.clone();
        let digits_only = props.digits_only;
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                let val = input.value();
                if digits_only {
                    let filtered: String = val.chars().filter(|c| c.is_ascii_digit()).collect();
                    if filtered != val {
                        input.set_value(&filtered);
                    }
                    input_value.set(filtered);
                } else {
                    input_value.set(val);
                }
            }
        })
    };

    let add_current_value = {
        let input_value = input_value.clone();
        let on_add = props.on_add.clone();
        let digits_only = props.digits_only;
        Callback::from(move |_| {
            let val = (*input_value).trim().to_string();
            if val.is_empty() {
                return;
            }
            if digits_only && !val.chars().all(|c| c.is_ascii_digit()) {
                return;
            }
            on_add.emit(val);
            input_value.set(String::new());
        })
    };

    let placeholder = if props.placeholder.is_empty() {
        "Type a value and press Enter...".to_string()
    } else {
        props.placeholder.clone()
    };

    html! {
        <div style="width: 100%;">
            if !props.values.is_empty() {
                <div style="
                    display: flex;
                    flex-wrap: wrap;
                    gap: 4px;
                    margin-bottom: 6px;
                ">
                    {for props.values.iter().map(|val| {
                        let value = val.clone();
                        let on_remove = props.on_remove.clone();
                        let onclick = Callback::from(move |_| on_remove.emit(value.clone()));
                        html! {
                            <span style="
                                display: inline-flex;
                                align-items: center;
                                gap: 4px;
                                padding: 3px 8px;
                                border: 1px solid #475569;
                                border-radius: 999px;
                                background: #1e293b;
                                color: #e2e8f0;
                                font-size: 12px;
                                font-weight: 600;
                                white-space: nowrap;
                            ">
                                {val}
                                <button
                                    style="
                                        display: inline-flex;
                                        align-items: center;
                                        justify-content: center;
                                        width: 16px;
                                        height: 16px;
                                        border: none;
                                        border-radius: 50%;
                                        background: #475569;
                                        color: #0f172a;
                                        cursor: pointer;
                                        font-size: 11px;
                                        font-weight: 700;
                                        line-height: 1;
                                        padding: 0;
                                    "
                                    onclick={onclick}
                                >
                                    {"\u{00D7}"}
                                </button>
                            </span>
                        }
                    })}
                </div>
            }
            <div style="display: flex; gap: 4px; align-items: stretch;">
                <input
                    type="text"
                    class={FilterInputStyle::CLASS_NAME}
                    style="flex: 1;"
                    value={(*input_value).clone()}
                    placeholder={placeholder}
                    oninput={on_input}
                    onkeydown={on_keydown}
                />
                <button
                    style="
                        display: inline-flex;
                        align-items: center;
                        justify-content: center;
                        width: 28px;
                        border: 1px solid #475569;
                        border-radius: 6px;
                        background: #1e293b;
                        color: #e2e8f0;
                        cursor: pointer;
                        font-size: 16px;
                        font-weight: 700;
                        line-height: 1;
                        padding: 0;
                    "
                    onclick={add_current_value}
                >
                    {"+"}
                </button>
            </div>
        </div>
    }
}
