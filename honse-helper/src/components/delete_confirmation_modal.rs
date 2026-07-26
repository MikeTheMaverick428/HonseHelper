use yew::prelude::*;

use crate::styles::{
    detail_modal::{
        ModalBodyStyle, ModalCloseStyle, ModalContentStyle, ModalHeaderStyle, ModalOverlayStyle,
    },
    Style,
};

#[derive(Properties, PartialEq)]
pub struct DeleteConfirmationModalProps {
    pub show: bool,
    pub title: String,
    pub item_name: String,
    pub on_confirm: Callback<()>,
    pub on_close: Callback<()>,
}

#[function_component]
pub fn DeleteConfirmationModal(props: &DeleteConfirmationModalProps) -> Html {
    let on_overlay_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    let on_confirm = {
        let cb = props.on_confirm.clone();
        let close = props.on_close.clone();
        Callback::from(move |_| {
            cb.emit(());
            close.emit(());
        })
    };

    let on_cancel = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    if !props.show {
        return html! {};
    }

    html! {
        <div class={ModalOverlayStyle::CLASS_NAME} onclick={on_overlay_click.clone()}>
            <div class={ModalContentStyle::CLASS_NAME}
                style="max-width: 440px; width: 100%;"
                onclick={|e: yew::MouseEvent| e.stop_propagation()}>
                <div class={ModalHeaderStyle::CLASS_NAME}>
                    <h2>{ &props.title }</h2>
                    <button class={ModalCloseStyle::CLASS_NAME} onclick={on_overlay_click}>
                        {"\u{00D7}"}
                    </button>
                </div>
                <div class={ModalBodyStyle::CLASS_NAME} style="padding-bottom: 0;">
                    <p style="margin: 0 0 4px; color: #cbd5e1; font-size: 14px; line-height: 1.5;">
                        {"Are you sure you want to delete"}
                    </p>
                    <p style="margin: 0; color: #f8fafc; font-size: 15px; font-weight: 600; word-break: break-word;">
                        { &props.item_name }
                    </p>
                    <p style="margin: 16px 0 0; color: #94a3b8; font-size: 13px;">
                        {"This action cannot be undone."}
                    </p>
                </div>
                <div style="display: flex; gap: 8px; justify-content: flex-end; padding: 20px;">
                    <button
                        style="padding: 8px 16px; border: 1px solid #475569; border-radius: 6px; background: transparent; color: #94a3b8; cursor: pointer; font-size: 13px;"
                        onclick={on_cancel}>
                        {"Cancel"}
                    </button>
                    <button
                        style="padding: 8px 16px; border: none; border-radius: 6px; background: #dc2626; color: #fff; cursor: pointer; font-size: 13px; font-weight: 600;"
                        onclick={on_confirm}>
                        {"Confirm Delete"}
                    </button>
                </div>
            </div>
        </div>
    }
}
