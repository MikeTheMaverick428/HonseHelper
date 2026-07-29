use shared::models::UmaRank;
use yew::prelude::*;

fn rank_tier(rank: &UmaRank) -> &'static str {
    match *rank as u16 {
        1..=4 => "rank-tier-1",
        5..=8 => "rank-tier-2",
        9..=12 => "rank-tier-3",
        13..=14 => "rank-tier-4",
        15..=16 => "rank-tier-5",
        17..=18 => "rank-tier-6",
        19..=28 => "rank-tier-7",
        29..=38 => "rank-tier-8",
        39..=48 => "rank-tier-9",
        49..=58 => "rank-tier-10",
        59..=68 => "rank-tier-11",
        69..=78 => "rank-tier-12",
        79..=88 => "rank-tier-13",
        89..=98 => "rank-tier-14",
        _ => "rank-tier-unknown",
    }
}

#[derive(Properties, PartialEq)]
pub struct RankBadgeProps {
    pub rank: UmaRank,
}

#[function_component]
pub fn RankBadge(props: &RankBadgeProps) -> Html {
    let tier = rank_tier(&props.rank);
    html! {
        <span class={classes!("rank-badge", tier)}>
            { props.rank.label() }
        </span>
    }
}
