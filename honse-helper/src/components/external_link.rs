use crate::tauri_bridge::invoke_tauri_command;
use serde_json::json;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ExternalLinkProps {
    pub url: String,
    pub label: String,
    #[prop_or_default]
    pub title: Option<String>,
}

#[function_component(ExternalLink)]
pub fn external_link(props: &ExternalLinkProps) -> Html {
    let on_click = {
        let url = props.url.clone();
        Callback::from(move |event: yew::MouseEvent| {
            event.prevent_default();
            let url = url.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke_tauri_command("open_url", json!({ "url": url })).await;
            });
        })
    };

    html! {
        <a
            href={props.url.clone()}
            title={props.title.clone().unwrap_or_else(|| props.url.clone())}
            onclick={on_click}
        >
            { &props.label }
        </a>
    }
}
