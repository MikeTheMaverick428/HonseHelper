use crate::{
    components::loading_overlay::LoadingOverlay,
    styles::{gather_veterans::GatherVeteransBtnStyle, Style},
    tauri_bridge::invoke_tauri_command,
};
use serde_json::json;
use shared::GatherVeteransResult;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GatherVeteransButtonProps {
    #[prop_or(Callback::noop())]
    pub on_complete: Callback<Result<GatherVeteransResult, String>>,

    #[prop_or("Gather Veterans".to_string())]
    pub label: String,

    #[prop_or("Gathering Veterans...".to_string())]
    pub busy_label: String,
}

#[function_component(GatherVeteransButton)]
pub fn gather_veterans_button(props: &GatherVeteransButtonProps) -> Html {
    let busy = use_state(|| false);

    let onclick = {
        let busy = busy.clone();
        let on_complete = props.on_complete.clone();
        Callback::from(move |_: MouseEvent| {
            let busy = busy.clone();
            let on_complete = on_complete.clone();
            wasm_bindgen_futures::spawn_local(async move {
                busy.set(true);
                let result = invoke_tauri_command(
                    "gather_veterans",
                    json!({
                        "request": {
                            "command": "get_veteran_data"
                        },
                        "timeoutMs": 15000
                    }),
                )
                .await;
                match result {
                    Ok(val) => match serde_json::from_value::<GatherVeteransResult>(val) {
                        Ok(counts) => on_complete.emit(Ok(counts)),
                        Err(e) => on_complete.emit(Err(format!("Failed to parse result: {}", e))),
                    },
                    Err(e) => on_complete.emit(Err(e)),
                }
                busy.set(false);
            });
        })
    };

    html! {
        <>
            <button
                class={GatherVeteransBtnStyle::CLASS_NAME}
                onclick={onclick}
                disabled={*busy}
            >
                { &props.label }
            </button>
            <LoadingOverlay active={*busy} label={props.busy_label.clone()} />
        </>
    }
}
