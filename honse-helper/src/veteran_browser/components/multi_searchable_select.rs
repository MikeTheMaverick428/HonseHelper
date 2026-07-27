use crate::styles::Style as _;
use crate::styles::filter_panel::FilterInputStyle;
use crate::veteran_browser::components::searchable_select::SelectOption;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct MultiSearchableSelectProps<T: Clone + PartialEq + 'static> {
    pub options: Vec<SelectOption<T>>,
    pub on_select: Callback<T>,
    pub on_remove: Callback<T>,
    pub selected: Vec<T>,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub on_input: Option<Callback<String>>,
}

#[function_component]
pub fn MultiSearchableSelect<T: Clone + PartialEq + 'static>(
    props: &MultiSearchableSelectProps<T>,
) -> Html {
    let internal_query = use_state(|| String::new());
    let is_open = use_state(|| false);
    let input_ref = use_node_ref();

    let search_query = props.value.as_ref().unwrap_or(&*internal_query);

    let filtered_options = use_memo(
        (search_query.clone(), props.options.len(), props.selected.len()),
        |_| {
            let query_lower = search_query.to_lowercase();
            props
                .options
                .iter()
                .filter(|opt| !props.selected.iter().any(|s| s == &opt.value))
                .filter(|opt| opt.label.to_lowercase().contains(&query_lower))
                .cloned()
                .collect::<Vec<_>>()
        },
    );

    let on_input = {
        let internal_query = internal_query.clone();
        let is_open = is_open.clone();
        let external_on_input = props.on_input.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                let value = input.value();
                internal_query.set(value.clone());
                if let Some(callback) = &external_on_input {
                    callback.emit(value);
                }
                is_open.set(true);
            }
        })
    };

    let on_focus = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(true);
        })
    };

    let on_blur = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            let is_open = is_open.clone();
            gloo_timers::callback::Timeout::new(200, move || {
                is_open.set(false);
            })
            .forget();
        })
    };

    let on_option_click = {
        let on_select = props.on_select.clone();
        let is_open = is_open.clone();
        let external_on_input = props.on_input.clone();
        let internal_query = internal_query.clone();
        move |option: SelectOption<T>| {
            let on_select = on_select.clone();
            let is_open = is_open.clone();
            let external_on_input = external_on_input.clone();
            let internal_query = internal_query.clone();
            Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                on_select.emit(option.value.clone());
                internal_query.set(String::new());
                if let Some(callback) = &external_on_input {
                    callback.emit(String::new());
                }
                is_open.set(false);
            })
        }
    };

    let selected_options: Vec<SelectOption<T>> = props
        .selected
        .iter()
        .filter_map(|val| props.options.iter().find(|opt| &opt.value == val).cloned())
        .collect();

    let placeholder = if props.placeholder.is_empty() {
        "Search...".to_string()
    } else {
        props.placeholder.clone()
    };

    html! {
        <div style="width: 100%;">
            if !selected_options.is_empty() {
                <div style="
                    display: flex;
                    flex-wrap: wrap;
                    gap: 4px;
                    margin-bottom: 6px;
                ">
                    {for selected_options.iter().map(|opt| {
                        let value = opt.value.clone();
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
                                {&opt.label}
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
            <div style="position: relative; width: 100%;">
                <input
                    ref={input_ref}
                    type="text"
                    class={FilterInputStyle::CLASS_NAME}
                    style="width: 100%;"
                    value={search_query.clone()}
                    placeholder={placeholder}
                    oninput={on_input}
                    onfocus={on_focus}
                    onblur={on_blur}
                />
                if *is_open && !filtered_options.is_empty() {
                    <div style="
                        position: absolute;
                        top: 100%;
                        left: 0;
                        right: 0;
                        max-height: 200px;
                        overflow-y: auto;
                        background: #1e293b;
                        border: 1px solid #334155;
                        border-radius: 4px;
                        box-shadow: 0 4px 12px rgba(0,0,0,0.4);
                        z-index: 1000;
                        margin-top: 4px;
                    ">
                        {filtered_options.iter().map(|option| {
                            let onclick = on_option_click(option.clone());
                            html! {
                                <div
                                    key={option.label.clone()}
                                    style="
                                        padding: 8px 12px;
                                        cursor: pointer;
                                        border-bottom: 1px solid #334155;
                                        color: #f8fafc;
                                        font-size: 13px;
                                    "
                                    onmouseover={Callback::from(move |e: MouseEvent| {
                                        if let Some(target) = e.target_dyn_into::<web_sys::HtmlElement>() {
                                            let _ = target.style().set_property("background-color", "#334155");
                                        }
                                    })}
                                    onmouseout={Callback::from(move |e: MouseEvent| {
                                        if let Some(target) = e.target_dyn_into::<web_sys::HtmlElement>() {
                                            let _ = target.style().set_property("background-color", "#1e293b");
                                        }
                                    })}
                                    onmousedown={onclick}
                                >
                                    {&option.label}
                                </div>
                            }
                        }).collect::<Html>()}
                    </div>
                }
            </div>
        </div>
    }
}
