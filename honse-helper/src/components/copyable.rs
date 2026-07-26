use crate::styles::{
    shared_components::{
        CopyFeedbackStyle, CopyIconStyle, CopyableButtonStyle, CopyableDisplayStyle,
        CopyableLabelStyle, CopyableValueStyle,
    },
    Style,
};
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
