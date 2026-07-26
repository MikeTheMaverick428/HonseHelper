use crate::styles::{shared_components::LoadingOverlayStyle, Style};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LoadingOverlayProps {
    pub active: bool,

    #[prop_or("Loading...".to_string())]
    pub label: String,
}

#[function_component(LoadingOverlay)]
pub fn loading_overlay(props: &LoadingOverlayProps) -> Html {
    if !props.active {
        return html! {};
    }

    html! {
        <div class={LoadingOverlayStyle::CLASS_NAME}>
            <div class="loading-spinner"></div>
            <div class="loading-label">{ &props.label }</div>
        </div>
    }
}
