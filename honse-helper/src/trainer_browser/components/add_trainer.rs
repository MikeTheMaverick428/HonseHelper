use crate::{
    styles::{
        detail_modal::{
            ModalBodyStyle, ModalCloseStyle, ModalContentStyle, ModalHeaderStyle, ModalOverlayStyle,
        },
        filter_panel::FilterInputStyle,
        Style,
    },
    tauri_bridge::invoke_tauri_command,
};
use serde_json::json;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AddTrainerByIdProps {
    #[prop_or(Callback::noop())]
    pub on_complete: Callback<Result<String, String>>,
}

#[function_component(AddTrainerById)]
pub fn add_trainer_by_id(props: &AddTrainerByIdProps) -> Html {
    let open = use_state(|| false);
    let account_id = use_state(String::new);
    let busy = use_state(|| false);
    let error = use_state(|| None::<String>);

    let on_open = {
        let open = open.clone();
        let error = error.clone();
        let account_id = account_id.clone();
        Callback::from(move |_: MouseEvent| {
            error.set(None);
            account_id.set(String::new());
            open.set(true);
        })
    };

    let on_close = {
        let open = open.clone();
        let busy = busy.clone();
        Callback::from(move |_| {
            if *busy {
                return;
            }
            open.set(false);
        })
    };

    let on_cancel = {
        let open = open.clone();
        let busy = busy.clone();
        let account_id = account_id.clone();
        let error = error.clone();
        Callback::from(move |_: MouseEvent| {
            if *busy {
                return;
            }
            open.set(false);
            account_id.set(String::new());
            error.set(None);
        })
    };

    let on_add = {
        let account_id = account_id.clone();
        let busy = busy.clone();
        let error = error.clone();
        let open = open.clone();
        let on_complete = props.on_complete.clone();
        Callback::from(move |_: MouseEvent| {
            let account_id = account_id.clone();
            let busy = busy.clone();
            let error = error.clone();
            let open = open.clone();
            let on_complete = on_complete.clone();
            let id = (*account_id).trim().to_string();
            if id.is_empty() {
                return;
            }
            wasm_bindgen_futures::spawn_local(async move {
                busy.set(true);
                error.set(None);
                match invoke_tauri_command("add_uma_moe_trainer", json!({ "accountId": id }))
                    .await
                {
                    Ok(val) => {
                        let name = val
                            .as_str()
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| "trainer".to_string());
                        open.set(false);
                        account_id.set(String::new());
                        on_complete.emit(Ok(name));
                    }
                    Err(e) => {
                        error.set(Some(e.clone()));
                        on_complete.emit(Err(e));
                    }
                }
                busy.set(false);
            });
        })
    };

    html! {
        <>
            <button onclick={on_open}>{"Add via uma.moe"}</button>
            if *open {
                <div class={ModalOverlayStyle::CLASS_NAME} onclick={on_close}>
                    <div class={ModalContentStyle::CLASS_NAME} style="width:420px;"
                         onclick={|e: yew::MouseEvent| e.stop_propagation()}>
                        <div class={ModalHeaderStyle::CLASS_NAME}>
                            <div>{"Add Trainer from uma.moe"}</div>
                            <button class={ModalCloseStyle::CLASS_NAME} onclick={on_cancel.clone()}>{"\u{00D7}"}</button>
                        </div>
                        <div class={ModalBodyStyle::CLASS_NAME}>
                            <div style="color:#94a3b8;font-size:13px;margin-bottom:10px;">
                                {"Enter the trainer's account ID to import their profile (name, borrow veteran, support card, follower count) from uma.moe."}
                            </div>
                            <input
                                type="text"
                                class={FilterInputStyle::CLASS_NAME}
                                placeholder={"uma.moe trainer ID"}
                                value={(*account_id).clone()}
                                disabled={*busy}
                                style="width:100%;box-sizing:border-box;"
                                oninput={let v = account_id.clone(); Callback::from(move |e: InputEvent| v.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
                            />
                            if let Some(err) = &*error {
                                <div style="color:#f87171;font-size:12px;margin-top:8px;">{err}</div>
                            }
                            <div style="display:flex;justify-content:flex-end;gap:8px;margin-top:14px;">
                                <button onclick={on_cancel} disabled={*busy}>{"Cancel"}</button>
                                <button onclick={on_add} disabled={*busy}>
                                    { if *busy { "Importing..." } else { "Add Trainer" } }
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            }
        </>
    }
}
