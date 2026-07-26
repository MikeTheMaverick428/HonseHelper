use crate::{
    components::notifications::{use_timed_notification, Notification, NotificationOverlay},
    styles::{
        app::{
            ApiConfigContainerStyle, ApiConfigFormStyle, ApiConfigHeaderStyle,
            ApiConfigStatusStyle, ButtonGroupStyle,
        },
        Style, StyleManager,
    },
    tauri_bridge::invoke_tauri_command,
};
use serde_json::json;
use shared::ApiKeyStatus;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
pub fn ApiConfigWindow() -> Html {
    let api_key = use_state(String::new);
    let status = use_state(|| ApiKeyStatus {
        configured: false,
        status: "loading…".to_string(),
    });
    let busy = use_state(|| false);
    let (state, push, remove) = use_timed_notification(5000);

    {
        let status = status.clone();
        let status_events = status.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(result) = invoke_tauri_command("get_api_key_status", json!({})).await {
                    if let Ok(s) = serde_json::from_value::<ApiKeyStatus>(result) {
                        status.set(s);
                    }
                }
            });

            crate::tauri_bridge::listen_to_event(
                "api-key-status",
                move |payload: serde_json::Value| {
                    if let Ok(s) = serde_json::from_value::<ApiKeyStatus>(payload) {
                        status_events.set(s);
                    }
                },
            );
            || {}
        });
    }

    let on_key_input = {
        let api_key = api_key.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            api_key.set(input.value());
        })
    };

    let on_save = {
        let api_key = api_key.clone();
        let busy = busy.clone();
        let push = push.clone();
        let status = status.clone();
        Callback::from(move |_: yew::MouseEvent| {
            let key = (*api_key).clone();
            if key.trim().is_empty() {
                push(Notification::error("Enter an API key first."));
                return;
            }
            let busy = busy.clone();
            let push = push.clone();
            let status = status.clone();
            wasm_bindgen_futures::spawn_local(async move {
                busy.set(true);
                push(Notification::info("Saving and verifying API key…"));
                match invoke_tauri_command("set_api_key", json!({ "apiKey": key })).await {
                    Ok(result) => {
                        if let Ok(s) = serde_json::from_value::<ApiKeyStatus>(result) {
                            status.set(s.clone());
                            if s.status.starts_with("ok") {
                                push(Notification::success("API key verified!"));
                            } else {
                                push(Notification::error(s.status.clone()));
                            }
                        }
                    }
                    Err(e) => {
                        push(Notification::error(format!("Save failed: {e}")));
                    }
                }
                busy.set(false);
            });
        })
    };

    let stylesheet = StyleManager::render_stylesheet();

    let status_class = if status.configured {
        "configured"
    } else {
        "unconfigured"
    };

    html! {
        <div class={ApiConfigContainerStyle::CLASS_NAME}>
            <div class={ApiConfigHeaderStyle::CLASS_NAME}>
                <h1>{"uma.moe API Key"}</h1>
                <p>{"Set your uma.moe API key. It will be encrypted and stored in the app database."}</p>
            </div>

            {stylesheet}

            <NotificationOverlay notifications={state.0.clone()} on_close={{
                let remove = remove.clone();
                Callback::from(move |id: u32| remove(id))
            }} />

            <div class={ApiConfigFormStyle::CLASS_NAME}>
                <label>
                    {"API Key"}
                    <input
                        type="password"
                        value={(*api_key).clone()}
                        oninput={on_key_input}
                        placeholder="Enter your uma.moe API key"
                        autocomplete="off"
                    />
                </label>

                <div class={classes!(ApiConfigStatusStyle::CLASS_NAME, status_class)}>
                    { &status.status }
                </div>

                <div class={ButtonGroupStyle::CLASS_NAME}>
                    <button onclick={on_save} disabled={*busy}>
                        { if *busy { "Verifying…" } else { "Save & Verify" } }
                    </button>
                </div>
            </div>
        </div>
    }
}
