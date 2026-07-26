use yew::prelude::*;

use crate::styles::{shared_components::DeleteButtonStyle, Style};

#[derive(Properties, PartialEq)]
pub struct DeleteButtonProps {
    pub onclick: Callback<MouseEvent>,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or("Delete".to_string())]
    pub title: String,
}

#[function_component]
pub fn DeleteButton(props: &DeleteButtonProps) -> Html {
    html! {
        <button
            class={DeleteButtonStyle::CLASS_NAME}
            onclick={props.onclick.clone()}
            disabled={props.disabled}
            title={props.title.clone()}
        >
            {"\u{1F5D1}"}
        </button>
    }
}
