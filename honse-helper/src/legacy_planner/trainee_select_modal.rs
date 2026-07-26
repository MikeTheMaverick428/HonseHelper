use crate::{
    components::{SearchableSelect, SelectOption},
    styles::{
        detail_modal::{
            ModalBodyStyle, ModalCloseStyle, ModalContentStyle, ModalHeaderStyle, ModalOverlayStyle,
        },
        Style,
    },
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TraineeSelectModalProps {
    pub options: Vec<SelectOption<i64>>,
    pub selected: Option<i64>,
    pub on_select: Callback<i64>,
    pub on_close: Callback<()>,
}

#[function_component]
pub fn TraineeSelectModal(props: &TraineeSelectModalProps) -> Html {
    let on_close = {
        let cb = props.on_close.clone();
        Callback::from(move |_| cb.emit(()))
    };

    let on_select = {
        let cb = props.on_select.clone();
        let close = props.on_close.clone();
        Callback::from(move |id: i64| {
            cb.emit(id);
            close.emit(());
        })
    };

    html! {
        <div class={ModalOverlayStyle::CLASS_NAME} onclick={on_close.clone()}>
            <div class={ModalContentStyle::CLASS_NAME} onclick={Callback::from(|e: MouseEvent| e.stop_propagation())} style="max-width: 500px; width: 100%;">
                <div class={ModalHeaderStyle::CLASS_NAME}>
                    <h2 style="margin: 0;">{"Select Trainee"}</h2>
                    <button onclick={on_close.clone()} class={ModalCloseStyle::CLASS_NAME}>{"\u{00D7}"}</button>
                </div>
                <div class={ModalBodyStyle::CLASS_NAME} style="min-height: 250px;">
                    <SearchableSelect<i64>
                        options={props.options.clone()}
                        on_select={on_select}
                        selected={props.selected}
                        placeholder={"Search trainee character..."}
                    />
                </div>
            </div>
        </div>
    }
}
