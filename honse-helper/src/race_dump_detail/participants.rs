use crate::race_dump_detail::replay::HorseSnapshot;
use shared::RaceDumpParticipant;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ParticipantsPanelProps {
    pub participants: Vec<RaceDumpParticipant>,
    pub snapshots: Vec<HorseSnapshot>,
}

#[function_component]
pub fn ParticipantsPanel(props: &ParticipantsPanelProps) -> Html {
    use crate::styles::{
        race_dump_detail::{
            PPBadgeStyle, PPBlockedStyle, PPEventsStyle, PPNameStyle, PPRowStyle, PPRushedStyle,
            PPSkillStyle, PPSpeedStyle, PPStatusStyle, ParticipantRowPlayerStyle,
            ParticipantsPanelStyle, ParticipantsRowsStyle,
        },
        Style,
    };

    let mut rows: Vec<(usize, &RaceDumpParticipant, &HorseSnapshot)> = props
        .snapshots
        .iter()
        .filter_map(|s| {
            let p = props.participants.get(s.horse_index)?;
            Some((s.horse_index, p, s))
        })
        .collect();
    rows.sort_by(|a, b| {
        b.2.distance
            .partial_cmp(&a.2.distance)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    html! {
        <div class={ParticipantsPanelStyle::CLASS_NAME}>
            <div class={ParticipantsRowsStyle::CLASS_NAME}>
                {rows.iter().map(|(_idx, p, s)| {
                    let is_player = p.is_player == 1;
                    let row_class = if is_player { Some(ParticipantRowPlayerStyle::CLASS_NAME) } else { None };

                    let status_badges = {
                        let mut b = Vec::new();
                        if s.is_blocked {
                            b.push(html! { <span class={classes!(PPBadgeStyle::CLASS_NAME, PPBlockedStyle::CLASS_NAME)}>{"B"}</span> });
                        }
                        if s.is_tempted {
                            b.push(html! { <span class={classes!(PPBadgeStyle::CLASS_NAME, PPRushedStyle::CLASS_NAME)}>{"R"}</span> });
                        }
                        b
                    };

                    let event_badges: Vec<Html> = s.active_event_labels.iter().map(|label| {
                        html! { <span class={classes!(PPBadgeStyle::CLASS_NAME, PPSkillStyle::CLASS_NAME)}>{label}</span> }
                    }).collect();

                    html! {
                        <div class={classes!(PPRowStyle::CLASS_NAME, row_class)} key={p.horse_index}>
                            <span class={PPSpeedStyle::CLASS_NAME}>{format!("{:.2}", s.speed)}</span>
                            <div class="rdd-pp-body">
                                <div class="rdd-pp-top">
                                    <span class={PPNameStyle::CLASS_NAME}>
                                        {p.chara_name.as_deref().unwrap_or("???")}
                                        if is_player { {" ☆"} }
                                    </span>
                                    <span class={PPStatusStyle::CLASS_NAME}>{status_badges}</span>
                                </div>
                                <div class={PPEventsStyle::CLASS_NAME}>{event_badges}</div>
                            </div>
                        </div>
                    }
                }).collect::<Html>()}
            </div>
        </div>
    }
}
