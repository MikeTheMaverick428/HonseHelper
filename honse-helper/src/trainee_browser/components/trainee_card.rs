use crate::styles::trainee_browser::*;
use crate::styles::legacy_planner::{AffinityBaseStyle, AffinityBonusStyle, AffinityPlusStyle};
use crate::styles::veteran_card::{CardAffinityStyle, CardNameStyle, VeteranVariantStyle};
use crate::styles::Style;
use crate::veteran_browser::components::veteran_card::parse_veteran_name;
use shared::trainee_browser::TraineePageItem;
use yew::prelude::*;

fn rarity_stars(rarity: i64) -> Html {
    if rarity <= 0 {
        html! { <span style="color:#6b7280;">{"Not owned"}</span> }
    } else {
        html! {
            <span style="color:#fbbf24;">
                { (0..rarity).map(|_| { "★" }).collect::<String>() }
            </span>
        }
    }
}

fn piece_progress(current: i64, needed: i64) -> Html {
    if needed <= 0 {
        return html! {
            <div class={TraineePieceLabelStyle::CLASS_NAME}>
                <span>{"MAX"}</span>
                <span>{format!("{}★", current)}</span>
            </div>
        };
    }
    let pct = ((current as f64 / needed as f64) * 100.0).min(100.0);
    html! {
        <>
            <div class={TraineePieceLabelStyle::CLASS_NAME}>
                <span>{format!("{} / {}", current, needed)}</span>
                <span>{format!("{:.0}%", (current as f64 / needed as f64 * 100.0).min(100.0).floor())}</span>
            </div>
            <div class={TraineePieceBarStyle::CLASS_NAME}>
                <div class={TraineePieceFillStyle::CLASS_NAME} style={format!("width:{}%", pct)}></div>
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct TraineeCardProps {
    pub card: TraineePageItem,
    #[prop_or_default]
    pub on_click: Option<Callback<()>>,
    #[prop_or_default]
    pub on_select: Option<Callback<i64>>,
}

#[function_component]
pub fn TraineeCard(props: &TraineeCardProps) -> Html {
    let onclick = props
        .on_click
        .clone()
        .map(|cb| Callback::from(move |_: MouseEvent| cb.emit(())));

    let on_select = props.on_select.clone().map(|cb| {
        let trainee_id = props.card.id;
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            cb.emit(trainee_id);
        })
    });

    let (variant, _) = parse_veteran_name(&props.card.name);

    html! {
        <div class={TraineeCardStyle::CLASS_NAME} onclick={onclick}>
            <div style="display:flex;flex-direction:column;">
                {if let Some(v) = &variant {
                    html! { <span class={VeteranVariantStyle::CLASS_NAME}>{v}</span> }
                } else { html! {} }}
                <span class={CardNameStyle::CLASS_NAME}>{ &props.card.character_name }</span>
            </div>
            <div class={TraineeRarityRowStyle::CLASS_NAME}>
                {rarity_stars(props.card.owned_rarity)}
            </div>
            <div class={TraineeCharNameStyle::CLASS_NAME}>
                { &props.card.name }
            </div>
            {piece_progress(props.card.piece_count, props.card.piece_needed)}
            <div class={TraineeIdStyle::CLASS_NAME}>{format!("ID: {}", props.card.id)}</div>
            { if let Some(aff) = props.card.affinity {
                if aff.total() > 0 {
                    html! { <div class={CardAffinityStyle::CLASS_NAME}>
                        <span>{"Affinity: "}</span>
                        <span class={AffinityBaseStyle::CLASS_NAME}>{aff.base}</span>
                        { if aff.bonus > 0 {
                            html! {
                                <>
                                    <span class={AffinityPlusStyle::CLASS_NAME}>{" + "}</span>
                                    <span class={AffinityBonusStyle::CLASS_NAME}>{aff.bonus}</span>
                                    <span>{" (Total: "}{aff.total()}{")"}</span>
                                </>
                            }
                        } else { html! {} } }
                    </div> }
                } else {
                    html! { <div class={CardAffinityStyle::CLASS_NAME}>
                        <span>{"Affinity: "}</span>
                        <span class={AffinityBaseStyle::CLASS_NAME}>{0}</span>
                    </div> }
                }
            } else { html! {} } }
            { if let Some(cb) = on_select {
                html! { <button class={TraineeSelectBtnStyle::CLASS_NAME} onclick={cb}>{"Select"}</button> }
            } else { html! {} } }
        </div>
    }
}
