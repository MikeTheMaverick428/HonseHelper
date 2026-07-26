use crate::components::delete_button::DeleteButton;
use crate::styles::{
    race_dump::{
        PlayerBadgeStyle, PlayerNameStyle, TypeBadgeChampionsStyle, TypeBadgeRoomMatchStyle,
        TypeBadgeSingleStyle, TypeBadgeTeamStadiumStyle, TypeBadgeUnknownStyle,
    },
    race_dump_card::*,
    tag_modal::{CardTagMoreStyle, CardTagPillStyle},
    Style,
};
use shared::veteran_browser::TagRow;
use shared::RaceDumpPageItem;
use yew::prelude::*;

fn race_type_label(v: i64) -> &'static str {
    match v {
        5 => "Champions",
        6 => "Single",
        8 => "RoomMatch",
        14 => "TeamStadium",
        _ => "Unknown",
    }
}

fn type_badge_class(v: i64) -> &'static str {
    match v {
        5 => TypeBadgeChampionsStyle::CLASS_NAME,
        6 => TypeBadgeSingleStyle::CLASS_NAME,
        8 => TypeBadgeRoomMatchStyle::CLASS_NAME,
        14 => TypeBadgeTeamStadiumStyle::CLASS_NAME,
        _ => TypeBadgeUnknownStyle::CLASS_NAME,
    }
}

fn format_dist(v: Option<i64>) -> String {
    match v {
        Some(d) => format!("{}m", d),
        None => "—".into(),
    }
}

fn ground_label(v: Option<i64>) -> &'static str {
    match v {
        Some(1) => "Turf",
        Some(2) => "Dirt",
        _ => "—",
    }
}

fn season_label(v: Option<i64>) -> &'static str {
    match v {
        Some(1) => "Spring",
        Some(2) => "Summer",
        Some(3) => "Fall",
        Some(4) => "Winter",
        Some(5) => "CherryBlossom",
        _ => "—",
    }
}

fn weather_label(v: Option<i64>) -> &'static str {
    match v {
        Some(1) => "Sunny",
        Some(2) => "Rainy",
        Some(3) => "Snow",
        Some(4) => "Cloudy",
        Some(5) => "Star",
        Some(6) => "Firework",
        _ => "—",
    }
}

fn ground_condition_label(v: Option<i64>) -> &'static str {
    match v {
        Some(1) => "Firm",
        Some(2) => "Good",
        Some(3) => "Soft",
        Some(4) => "Heavy",
        _ => "—",
    }
}

fn turn_label(v: Option<i64>) -> &'static str {
    match v {
        Some(1) => "Right",
        Some(2) => "Left",
        _ => "—",
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct RaceDumpCardProps {
    pub item: RaceDumpPageItem,
    pub on_click: Callback<i64>,
    pub on_delete: Callback<i64>,
    pub deleting: bool,
}

#[function_component]
pub fn RaceDumpCard(props: &RaceDumpCardProps) -> Html {
    let s = &props.item.summary;
    let id = s.id;

    let onclick = {
        let cb = props.on_click.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            cb.emit(id);
        })
    };

    let ondelete = {
        let cb = props.on_delete.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            cb.emit(id);
        })
    };

    html! {
        <div class={RaceCardRootStyle::CLASS_NAME} onclick={onclick}>
            <div class={RaceCardTopRowStyle::CLASS_NAME}>
                <span class={type_badge_class(s.race_type)}>
                    {race_type_label(s.race_type)}
                </span>
                <span class={RaceCardIdStyle::CLASS_NAME}>
                    {"#"}
                    {id}
                </span>
            </div>

            if let Some(race_name) = &props.item.race_name {
                <div class={RaceCardRaceNameStyle::CLASS_NAME}>{race_name}</div>
            }

            <hr class={RaceCardDividerStyle::CLASS_NAME} />

            <div class={RaceCardInfoRowStyle::CLASS_NAME}>
                <span class={RaceCardInfoItemStyle::CLASS_NAME}>{format_dist(s.distance)}</span>
                <span class={RaceCardInfoItemStyle::CLASS_NAME}>{ground_label(s.ground_type)}</span>
                <span class={RaceCardInfoItemStyle::CLASS_NAME}>{turn_label(s.turn)}</span>
                if let Some(tn) = &s.track_name {
                    <span class={RaceCardInfoItemStyle::CLASS_NAME}>{tn}</span>
                }
            </div>

            <div class={RaceCardInfoRowStyle::CLASS_NAME}>
                <span class={RaceCardInfoItemStyle::CLASS_NAME}>{season_label(s.season)}</span>
                <span class={RaceCardInfoItemStyle::CLASS_NAME}>{weather_label(s.weather)}</span>
                <span class={RaceCardInfoItemStyle::CLASS_NAME}>{ground_condition_label(s.ground_condition)}</span>
            </div>

            <hr class={RaceCardDividerStyle::CLASS_NAME} />

            <div class={RaceCardParticipantsStyle::CLASS_NAME}>
                {format!("{} horses", s.participant_count)}
                if s.player_participant_count > 0 {
                    <span class={PlayerBadgeStyle::CLASS_NAME}>
                        {format!("{} yours", s.player_participant_count)}
                    </span>
                }
            </div>

            if !s.player_participants.is_empty() {
                <div>
                    {s.player_participants.iter().map(|name| {
                        html! { <span class={PlayerNameStyle::CLASS_NAME}>{name}</span> }
                    }).collect::<Html>()}
                </div>
            }

            <hr class={RaceCardDividerStyle::CLASS_NAME} />

            { if !props.item.tags.is_empty() {
                let display_tags: Vec<&TagRow> = props.item.tags.iter().take(3).collect();
                let remaining = props.item.tags.len().saturating_sub(3);
                html! {
                    <div class={RaceCardTagsStyle::CLASS_NAME}>
                        { for display_tags.iter().map(|t| {
                            html! { <span class={CardTagPillStyle::CLASS_NAME}>{ &t.tag_value }</span> }
                        })}
                        { if remaining > 0 {
                            html! { <span class={CardTagMoreStyle::CLASS_NAME}>{ format!("+{}", remaining) }</span> }
                        } else { html! {} } }
                    </div>
                }
            } else { html! {} } }

            <hr class={RaceCardDividerStyle::CLASS_NAME} />

            <div class={RaceCardFooterStyle::CLASS_NAME}>
                <div></div>
                <div>
                    <DeleteButton
                        onclick={ondelete}
                        disabled={props.deleting}
                        title="Delete this race dump"
                    />
                </div>
            </div>

            <div class={RaceCardTimeStyle::CLASS_NAME}>{&s.capture_time}</div>
        </div>
    }
}
