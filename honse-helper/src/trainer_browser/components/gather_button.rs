use crate::{
    components::loading_overlay::LoadingOverlay,
    styles::{gather_veterans::GatherVeteransBtnStyle, Style},
    tauri_bridge::invoke_tauri_command,
};
use serde_json::json;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GatherFollowedTrainersButtonProps {
    #[prop_or(Callback::noop())]
    pub on_complete: Callback<Result<(), String>>,

    #[prop_or("Gather Followed Trainers".to_string())]
    pub label: String,

    #[prop_or("Gathering Followed Trainers...".to_string())]
    pub busy_label: String,
}

#[function_component(GatherFollowedTrainersButton)]
pub fn gather_followed_trainers_button(props: &GatherFollowedTrainersButtonProps) -> Html {
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
                    "gather_followed_trainers",
                    json!({
                        "request": {
                            "command": "get_single_mode_start_friends"
                        },
                        "timeoutMs": 20000
                    }),
                )
                .await;
                match result {
                    Ok(_) => on_complete.emit(Ok(())),
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
                title={"Navigate the game to the Single Mode start screen (Select a trainee) first"}
            >
                { &props.label }
            </button>
            <LoadingOverlay active={*busy} label={props.busy_label.clone()} />
        </>
    }
}
