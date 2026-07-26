use crate::styles::detail_modal::*;
use crate::styles::Style;
use shared::support_card_browser::SupportCardPageItem;
use yew::prelude::*;

pub fn rarity_class(rarity: i64) -> &'static str {
    match rarity {
        1 => "rarity-r",
        2 => "rarity-sr",
        3 => "rarity-ssr",
        _ => "",
    }
}

pub fn rarity_label(rarity: i64) -> &'static str {
    match rarity {
        1 => "R",
        2 => "SR",
        3 => "SSR",
        _ => "?",
    }
}

pub fn type_class(card_type: i64) -> &'static str {
    match card_type {
        1 => "type-speed",
        2 => "type-stamina",
        3 => "type-power",
        4 => "type-guts",
        5 => "type-wit",
        6 => "type-pal",
        7 => "type-group",
        _ => "type-unknown",
    }
}

pub fn type_label(card_type: i64) -> &'static str {
    match card_type {
        1 => "Speed",
        2 => "Stamina",
        3 => "Power",
        4 => "Guts",
        5 => "Wisdom",
        6 => "Friend",
        7 => "Group",
        _ => "?",
    }
}

pub fn parse_card_name(name: &str) -> (Option<String>, &str) {
    let name = name.trim();
    if let Some(end_bracket) = name.find(']') {
        if name.starts_with('[') && end_bracket > 0 {
            let variant = name[1..end_bracket].trim().to_string();
            let character = name[end_bracket + 1..].trim();
            let variant = if variant.is_empty() {
                None
            } else {
                Some(variant)
            };
            return (variant, character);
        }
    }
    (None, name)
}

#[derive(Properties, PartialEq)]
pub struct SupportCardCardProps {
    pub card: SupportCardPageItem,
    #[prop_or_default]
    pub on_click: Option<Callback<()>>,
}

#[function_component]
pub fn SupportCardCard(props: &SupportCardCardProps) -> Html {
    let onclick = props
        .on_click
        .clone()
        .map(|cb| Callback::from(move |_: MouseEvent| cb.emit(())));

    let lb = props.card.limit_break_count;
    let is_mlb = lb >= 4;
    let (variant, character_name) = parse_card_name(&props.card.name);

    html! {
        <div class={SupportCardCardStyle::CLASS_NAME} onclick={onclick}>
            {if let Some(v) = &variant {
                html! { <div class={SupportCardVariantStyle::CLASS_NAME}>{v}</div> }
            } else {
                html! {}
            }}
            <div class={SupportCardNameStyle::CLASS_NAME}>{character_name}</div>
            <div class={SupportCardBadgeRowStyle::CLASS_NAME}>
                <span class={format!("{} {}", SupportCardRarityStyle::CLASS_NAME, rarity_class(props.card.rarity))}>
                    {rarity_label(props.card.rarity)}
                </span>
                <span class={format!("{} {}", SupportCardTypeStyle::CLASS_NAME, type_class(props.card.card_type))}>
                    {type_label(props.card.card_type)}
                </span>
                <span class={format!("{}{}", SupportCardLbStyle::CLASS_NAME, if is_mlb { " mlb" } else { "" })}>
                    {(0..4).map(|i| {
                        let on = i < lb;
                        html! {
                            <span class={format!("diamond{}", if on { " on" } else { "" })}></span>
                        }
                    }).collect::<Html>()}
                </span>
                <span style="color:#9ca3af;font-size:12px;">{format!("Lv{} /{}", props.card.level, props.card.max_level)}</span>
            </div>
        </div>
    }
}
