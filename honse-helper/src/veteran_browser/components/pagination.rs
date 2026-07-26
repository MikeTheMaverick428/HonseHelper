use crate::styles::{pagination::*, Style};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PaginationProps {
    pub page: u32,
    pub total_pages: u32,
    pub on_page_change: Callback<u32>,
}

#[function_component]
pub fn Pagination(props: &PaginationProps) -> Html {
    let on_first = {
        let cb = props.on_page_change.clone();
        Callback::from(move |_| {
            cb.emit(1);
        })
    };

    let on_prev = {
        let page = props.page;
        let cb = props.on_page_change.clone();
        Callback::from(move |_| {
            if page > 1 {
                cb.emit(page - 1);
            }
        })
    };

    let on_next = {
        let page = props.page;
        let total = props.total_pages;
        let cb = props.on_page_change.clone();
        Callback::from(move |_| {
            if page < total {
                cb.emit(page + 1);
            }
        })
    };

    let on_last = {
        let total = props.total_pages;
        let cb = props.on_page_change.clone();
        Callback::from(move |_| {
            cb.emit(total);
        })
    };

    html! {
        <div class={PaginationStyle::CLASS_NAME}>
            <button onclick={on_first} disabled={props.page <= 1}>{"\u{00AB}"}</button>
            <button onclick={on_prev} disabled={props.page <= 1}>{"\u{2039}"}</button>
            <span class={PageCurrentStyle::CLASS_NAME}>{ props.page }</span>
            <button onclick={on_next} disabled={props.page >= props.total_pages}>{"\u{203A}"}</button>
            <button onclick={on_last} disabled={props.page >= props.total_pages}>{"\u{00BB}"}</button>
        </div>
    }
}
