use crate::styles::{
    Style,
    veteran_card::{CardHashStyle, OwnerIdBadgeStyle, OwnerIdPrefixStyle},
};
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

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