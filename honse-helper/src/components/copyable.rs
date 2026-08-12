use crate::styles::{
    Style, shared_components::{
        CopyFeedbackStyle, CopyIconStyle, CopyableButtonStyle, CopyableDisplayStyle,
        CopyableLabelStyle, CopyableValueStyle,
    }, veteran_card::{CardHashStyle, OwnerIdBadgeStyle, OwnerIdPrefixStyle},
};
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CopyableValueProps {
    pub value: String,
    pub label: Option<String>,
    #[prop_or_default]
    pub display_value: Option<String>,
    #[prop_or_default]
    pub hover_value: Option<String>,
}

#[function_component(CopyableValue)]
pub fn copyable_value(props: &CopyableValueProps) -> Html {
    let copied = use_state(|| false);

    let value_to_display = props
        .display_value
        .clone()
        .unwrap_or_else(|| props.value.clone());
    let label = props.label.clone().unwrap_or_default();

    let on_click = {
        let value = props.value.clone();
        let copied = copied.clone();

        Callback::from(move |_| {
            let value = value.clone();
            let copied = copied.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Some(window) = window() {
                    let clipboard = window.navigator().clipboard();
                    let promise = clipboard.write_text(&value);

                    if wasm_bindgen_futures::JsFuture::from(promise).await.is_ok() {
                        copied.set(true);

                        spawn_local(async move {
                            gloo_timers::future::TimeoutFuture::new(2000).await;
                            copied.set(false);
                        });
                        return;
                    }
                }
            });
        })
    };

    html! {
        <div class={CopyableValueStyle::CLASS_NAME}>
            if !label.is_empty() {
                <span class={CopyableLabelStyle::CLASS_NAME}>{ label }</span>
            }
            <button
                class={classes!(CopyableButtonStyle::CLASS_NAME, (*copied).then(|| "copied"))}
                onclick={on_click}
                title={{if let Some(hover_value) = props.hover_value.clone() { hover_value } else { "Click to copy".to_string() }}}
            >
                <span class={CopyableDisplayStyle::CLASS_NAME}>{ value_to_display }</span>
                <span class={CopyIconStyle::CLASS_NAME}>{ "📋" }</span>
                if *copied {
                    <span class={CopyFeedbackStyle::CLASS_NAME}>{ "Copied!" }</span>
                }
            </button>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct HashBadgeProps {
    pub label: AttrValue,
    pub hash: u64,
    #[prop_or_default]
    pub title: AttrValue,
}

#[function_component(HashBadge)]
pub fn hash_badge(props: &HashBadgeProps) -> Html {
    let copied = use_state(|| false);

    let onclick = {
        let copied = copied.clone();
        let hash_str = props.hash.to_string();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();

            let copied = copied.clone();
            let text = hash_str.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Some(window) = web_sys::window() {
                    let _ = window.navigator().clipboard().write_text(&text);
                }
                copied.set(true);
                gloo_timers::future::TimeoutFuture::new(500).await;
                copied.set(false);
            });
        })
    };

    html! {
        <span
            class={classes!(
                CardHashStyle::CLASS_NAME,
                (*copied).then_some("hash-copied")
            )}
            title={props.title.clone()}
            {onclick}
        >
            { &props.label }{ " " }{ format!("...{:04x}", props.hash & 0xFFFF) }
        </span>
    }
}

#[derive(Properties, PartialEq)]
pub struct OwnerBadgeProps {
    pub owner_id: u64, // or your specific type
    #[prop_or("Owner".to_string())]
    pub label: String,
    #[prop_or("Click to copy owner ID".to_string())]
    pub title: String,
}

#[function_component(OwnerBadge)]
pub fn owner_badge(props: &OwnerBadgeProps) -> Html {
    let owner_copied = use_state(|| false);
    let owner_id = props.owner_id;

    let onclick = {
        let owner_copied = owner_copied.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            let text = owner_id.to_string();
            let owner_copied = owner_copied.clone();

            spawn_local(async move {
                if let Some(window) = web_sys::window() {
                    let _ = window.navigator().clipboard().write_text(&text);
                }
                owner_copied.set(true);
                TimeoutFuture::new(500).await;
                owner_copied.set(false);
            });
        })
    };

    let badge_class = classes!(
        OwnerIdBadgeStyle::CLASS_NAME,
        (*owner_copied).then_some("owner-id-copied")
    );

    html! {
        <span class={badge_class} title={props.title.clone()} {onclick}>
            <span class={OwnerIdPrefixStyle::CLASS_NAME}>{&props.label}</span>
            { owner_id }
        </span>
    }
}