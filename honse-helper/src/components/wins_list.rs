use crate::styles::{
    detail_modal::{
        WinBadgesStyle, WinNameStyle, WinRowStyle, WinSharedStyle, WinVeteranStyle, WinsListStyle,
    },
    Style,
};
use shared::veteran_browser::MajorWinRow;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct WinsListProps {
    pub wins: Vec<MajorWinRow>,
}

#[function_component]
pub fn WinsList(props: &WinsListProps) -> Html {
    let mut sorted_wins = props.wins.clone();
    sorted_wins.sort_by(|a, b| {
        a.priority
            .cmp(&b.priority)
            .then(a.on_veteran.cmp(&b.on_veteran))
            .then(a.name.cmp(&b.name))
    });

    if sorted_wins.is_empty() {
        return html! { <p>{"No major wins."}</p> };
    }

    html! {
        <div class={WinsListStyle::CLASS_NAME}>
            { for sorted_wins.iter().map(|w| {
                html! {
                    <div class={WinRowStyle::CLASS_NAME}>
                        <span class={WinNameStyle::CLASS_NAME}>{ w.name.as_deref().unwrap_or("Unknown") }</span>
                        <span class={WinBadgesStyle::CLASS_NAME}>
                            { if w.shared_count.map_or(false, |c| c > 1) { html! {
                                <span class={WinSharedStyle::CLASS_NAME}>{ format!("Shared ×{}", w.shared_count.unwrap()) }</span>
                            } } else { html! {} } }
                            { if w.on_veteran { html! { <span class={WinVeteranStyle::CLASS_NAME}>{"★Veteran"}</span> } } else { html! {} } }
                        </span>
                    </div>
                }
            }) }
        </div>
    }
}
