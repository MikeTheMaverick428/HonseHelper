use shared::veteran_browser::TagRow;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::styles::{
    detail_modal::{
        ModalBodyStyle, ModalCloseStyle, ModalContentStyle, ModalHeaderStyle, ModalOverlayStyle,
    },
    tag_modal::{
        TagCreateItemStyle, TagDropdownItemStyle, TagDropdownStyle, TagInputContainerStyle,
        TagInputStyle, TagNoResultsStyle, TagPillListStyle, TagPillRemoveStyle, TagPillStyle,
    },
    Style,
};

#[derive(Properties, PartialEq)]
pub struct TagModalProps {
    pub show: bool,
    pub title: String,
    pub current_tags: Vec<TagRow>,
    pub search_results: Vec<TagRow>,
    pub on_search: Callback<String>,
    pub on_save: Callback<Vec<TagRow>>,
    pub on_close: Callback<()>,
}

#[function_component]
pub fn TagModal(props: &TagModalProps) -> Html {
    let input_value = use_state(String::new);
    let is_open = use_state(|| false);
    let input_ref = use_node_ref();
    let working_tags = use_state(|| props.current_tags.clone());

    {
        let working_tags = working_tags.clone();
        use_effect_with(props.current_tags.clone(), move |new_tags| {
            working_tags.set(new_tags.clone());
            || ()
        });
    }

    let on_overlay_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    let on_input = {
        let input_value = input_value.clone();
        let is_open = is_open.clone();
        let on_search = props.on_search.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                let value = input.value();
                input_value.set(value.clone());
                on_search.emit(value);
                is_open.set(true);
            }
        })
    };

    let on_focus = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(true))
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

    let on_keydown = {
        let input_value = input_value.clone();
        let search_results = props.search_results.clone();
        let working_tags = working_tags.clone();
        let is_open = is_open.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let query = (*input_value).trim().to_string();
                if query.is_empty() {
                    return;
                }
                let query_lower = query.to_lowercase();
                let exists = search_results
                    .iter()
                    .any(|t| t.tag_value.to_lowercase() == query_lower);
                if exists {
                    if let Some(tag) = search_results
                        .iter()
                        .find(|t| t.tag_value.to_lowercase() == query_lower)
                    {
                        let mut tags = (*working_tags).clone();
                        if !tags.iter().any(|t| t.id == tag.id) {
                            tags.push(tag.clone());
                            working_tags.set(tags);
                        }
                    }
                } else {
                    let mut tags = (*working_tags).clone();
                    if !tags
                        .iter()
                        .any(|t| t.tag_value.to_lowercase() == query_lower)
                    {
                        tags.push(TagRow {
                            id: 0,
                            tag_value: query,
                            create_date: String::new(),
                        });
                        working_tags.set(tags);
                    }
                }
                input_value.set(String::new());
                is_open.set(false);
            }
        })
    };

    let current_ids: Vec<i64> = working_tags.iter().map(|t| t.id).collect();

    let filtered_results: Vec<&TagRow> = props
        .search_results
        .iter()
        .filter(|t| !current_ids.contains(&t.id))
        .collect();

    let query = (*input_value).trim().to_string();
    let query_lower = query.to_lowercase();
    let exact_match_exists = query_lower.is_empty()
        || working_tags
            .iter()
            .any(|t| t.tag_value.to_lowercase() == query_lower)
        || props
            .search_results
            .iter()
            .any(|t| t.tag_value.to_lowercase() == query_lower);
    let show_create = !query_lower.is_empty() && !exact_match_exists;

    if !props.show {
        return html! {};
    }

    let on_overlay_click_btn = on_overlay_click.clone();

    let on_save = {
        let working_tags = working_tags.clone();
        let on_save = props.on_save.clone();
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_save.emit((*working_tags).clone());
            on_close.emit(());
        })
    };

    let on_cancel = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    let on_remove_tag = {
        let working_tags = working_tags.clone();
        Callback::from(move |tag_id: i64| {
            let mut tags = (*working_tags).clone();
            tags.retain(|t| t.id != tag_id);
            working_tags.set(tags);
        })
    };

    html! {
        <div class={ModalOverlayStyle::CLASS_NAME} onclick={on_overlay_click}>
            <div class={ModalContentStyle::CLASS_NAME}
                style="width: min(420px, 90vw);"
                onclick={|e: yew::MouseEvent| e.stop_propagation()}>
                <div class={ModalHeaderStyle::CLASS_NAME}>
                    <h2>{ &props.title }</h2>
                    <button class={ModalCloseStyle::CLASS_NAME} onclick={on_overlay_click_btn}>
                        {"\u{00D7}"}
                    </button>
                </div>
                <div class={ModalBodyStyle::CLASS_NAME} style="min-height: 400px;">
                    if !working_tags.is_empty() {
                        <div class={TagPillListStyle::CLASS_NAME}>
                            { for working_tags.iter().map(|tag| {
                                let tag_id = tag.id;
                                let on_remove = on_remove_tag.clone();
                                let onclick = Callback::from(move |_| on_remove.emit(tag_id));
                                html! {
                                    <span class={TagPillStyle::CLASS_NAME}>
                                        { &tag.tag_value }
                                        <button class={TagPillRemoveStyle::CLASS_NAME} onclick={onclick}>
                                            {"\u{00D7}"}
                                        </button>
                                    </span>
                                }
                            }) }
                        </div>
                    }

                    <div class={TagInputContainerStyle::CLASS_NAME}>
                        <input
                            ref={input_ref}
                            class={TagInputStyle::CLASS_NAME}
                            type="text"
                            value={(*input_value).clone()}
                            placeholder="Type tag name..."
                            oninput={on_input}
                            onfocus={on_focus}
                            onblur={on_blur}
                            onkeydown={on_keydown}
                        />
                        if *is_open {
                            <div class={TagDropdownStyle::CLASS_NAME}>
                                { for filtered_results.iter().map(|tag| {
                                    let tag_value = tag.tag_value.clone();
                                    let tag_id = tag.id;
                                    let working_tags = working_tags.clone();
                                    let input_value = input_value.clone();
                                    let is_open = is_open.clone();
                                    let onclick = Callback::from(move |_: MouseEvent| {
                                        let mut tags = (*working_tags).clone();
                                        if !tags.iter().any(|t| t.id == tag_id) {
                                            tags.push(TagRow { id: tag_id, tag_value: tag_value.clone(), create_date: String::new() });
                                            working_tags.set(tags);
                                        }
                                        input_value.set(String::new());
                                        is_open.set(false);
                                    });
                                    html! {
                                        <div class={TagDropdownItemStyle::CLASS_NAME} {onclick}>
                                            { &tag.tag_value }
                                        </div>
                                    }
                                }) }
                                { if filtered_results.is_empty() && !query_lower.is_empty() {
                                    html! {
                                        <div class={TagNoResultsStyle::CLASS_NAME}>
                                            {"No matching tags"}
                                        </div>
                                    }
                                } else { html! {} } }
                                { if show_create {
                                    let create_value = query.clone();
                                    let working_tags = working_tags.clone();
                                    let input_value = input_value.clone();
                                    let is_open = is_open.clone();
                                    let onclick = Callback::from(move |_: MouseEvent| {
                                        let mut tags = (*working_tags).clone();
                                        if !tags.iter().any(|t| t.tag_value.to_lowercase() == create_value.to_lowercase()) {
                                            tags.push(TagRow { id: 0, tag_value: create_value.clone(), create_date: String::new() });
                                            working_tags.set(tags);
                                        }
                                        input_value.set(String::new());
                                        is_open.set(false);
                                    });
                                    html! {
                                        <div class={TagCreateItemStyle::CLASS_NAME} {onclick}>
                                            { format!("+ Create \"{}\"", query) }
                                        </div>
                                    }
                                } else { html! {} } }
                            </div>
                        }
                    </div>
                </div>
                <div style="display: flex; gap: 8px; justify-content: flex-end; padding: 0 20px 20px;">
                    <button
                        style="padding: 8px 16px; border: 1px solid #475569; border-radius: 6px; background: transparent; color: #94a3b8; cursor: pointer; font-size: 13px;"
                        onclick={on_cancel}>
                        {"Cancel"}
                    </button>
                    <button
                        style="padding: 8px 16px; border: none; border-radius: 6px; background: #2563eb; color: #fff; cursor: pointer; font-size: 13px; font-weight: 600;"
                        onclick={on_save}>
                        {"Save"}
                    </button>
                </div>
            </div>
        </div>
    }
}
