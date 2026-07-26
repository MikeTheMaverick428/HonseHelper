mod participants;
mod participants_tab;
mod replay;
mod tags_tab;

use crate::race_dump_detail::participants::ParticipantsPanel;
use crate::race_dump_detail::participants_tab::ParticipantsTab;
use crate::race_dump_detail::replay::{HorseSnapshot, ReplayPanel};
use crate::race_dump_detail::tags_tab::TagsTab;
use crate::{
    components::notifications::{use_timed_notification, Notification, NotificationOverlay},
    styles::{
        race_dump_detail::{
            DetailBodyStyle, DetailContainerStyle, DetailHeaderStyle, DetailTabActiveStyle,
            DetailTabBtnStyle, DetailTabsStyle, ErrorOverlayStyle, LoadingOverlayStyle,
            ReplayPanelStyle,
        },
        Style, StyleManager,
    },
    tauri_bridge::invoke_tauri_command,
};
use serde_json::json;
use shared::RaceDumpDetail;
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

fn ground_label(v: Option<i64>) -> &'static str {
    match v.unwrap_or(0) {
        1 => "Turf",
        2 => "Dirt",
        _ => "Unknown",
    }
}

fn weather_label(v: Option<i64>) -> &'static str {
    match v.unwrap_or(0) {
        1 => "Sunny",
        2 => "Rainy",
        3 => "Snow",
        4 => "Cloudy",
        _ => "—",
    }
}

fn season_label(v: Option<i64>) -> &'static str {
    match v.unwrap_or(0) {
        1 => "Spring",
        2 => "Summer",
        3 => "Fall",
        4 => "Winter",
        5 => "CherryBlossom",
        _ => "—",
    }
}

#[derive(Clone, Copy, PartialEq)]
enum DetailTab {
    RaceReplay,
    RaceParticipants,
    Tags,
}

#[function_component]
pub fn RaceDumpDetailWindow() -> Html {
    let detail = use_state(|| None as Option<RaceDumpDetail>);
    let loading = use_state(|| true);
    let error = use_state(|| None as Option<String>);
    let snapshots = use_state(|| Vec::new() as Vec<HorseSnapshot>);
    let active_tab = use_state(|| DetailTab::RaceParticipants);

    let (notif_state, push, _remove) = use_timed_notification(5000);

    {
        let detail = detail.clone();
        let loading = loading.clone();
        let error = error.clone();
        use_effect_with((), move |_| {
            let detail = detail.clone();
            let loading = loading.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match invoke_tauri_command("get_race_dump_detail", json!({})).await {
                    Ok(result) => match serde_json::from_value::<RaceDumpDetail>(result) {
                        Ok(d) => {
                            detail.set(Some(d));
                            loading.set(false);
                        }
                        Err(e) => {
                            error.set(Some(format!("Parse error: {e}")));
                            loading.set(false);
                        }
                    },
                    Err(e) => {
                        error.set(Some(format!("Failed to load: {e}")));
                        loading.set(false);
                    }
                }
            });
            || {}
        });
    }

    let on_frame = {
        let snapshots = snapshots.clone();
        Callback::from(move |data: Vec<HorseSnapshot>| {
            snapshots.set(data);
        })
    };

    let stylesheet = StyleManager::render_stylesheet();

    let on_race_replay_click = {
        let t = active_tab.clone();
        Callback::from(move |_| t.set(DetailTab::RaceReplay))
    };

    let on_race_participants_click = {
        let t = active_tab.clone();
        Callback::from(move |_| t.set(DetailTab::RaceParticipants))
    };

    let on_tags_click = {
        let t = active_tab.clone();
        Callback::from(move |_| t.set(DetailTab::Tags))
    };

    html! {
        <div class={DetailContainerStyle::CLASS_NAME}>
            {stylesheet}

            if let Some(ref d) = *detail {
                <div class={DetailHeaderStyle::CLASS_NAME}>
                    <h2>
                        {format!("Race Dump #{}", d.summary.id)}
                        {if let Some(ref rn) = d.summary.race_name {
                            html! { {format!(" — {}", rn)} }
                        } else { html! {} }}
                    </h2>
                    <span class="rdd-header-meta">
                        {format!("{} · {} · {}m · {} · {} · {} · {} horses",
                            race_type_label(d.summary.race_type),
                            d.summary.track_name.clone().unwrap_or_else(|| "?".into()),
                            d.summary.distance.map(|v| v.to_string()).unwrap_or_else(|| "?".into()),
                            ground_label(d.summary.ground_type),
                            season_label(d.summary.season),
                            weather_label(d.summary.weather),
                            d.participants.len(),
                        )}
                    </span>
                    <button
                        class={DetailTabBtnStyle::CLASS_NAME}
                        onclick={{
                            let push = push.clone();
                            Callback::from(move |_: MouseEvent| {
                                let push = push.clone();
                                wasm_bindgen_futures::spawn_local(async move {
                                    match invoke_tauri_command("export_race_dump_hakuraku", json!({})).await {
                                        Ok(val) => {
                                            if let Some(s) = val.as_str() {
                                                if s == "canceled" { return; }
                                                push(Notification::success(format!("Saved: {}", s)));
                                            }
                                        }
                                        Err(e) => push(Notification::error(format!("Export failed: {}", e))),
                                    }
                                });
                            })
                        }}
                    >{"Export Hakuraku"}</button>
                </div>
                <div class={DetailTabsStyle::CLASS_NAME}>
                    <button class={classes!(DetailTabBtnStyle::CLASS_NAME, (*active_tab == DetailTab::RaceParticipants).then_some(DetailTabActiveStyle::CLASS_NAME))}
                        onclick={on_race_participants_click}>
                        {"Race Participants"}
                    </button>
                    <button class={classes!(DetailTabBtnStyle::CLASS_NAME, (*active_tab == DetailTab::RaceReplay).then_some(DetailTabActiveStyle::CLASS_NAME))}
                        onclick={on_race_replay_click}>
                        {"Race Replay"}
                    </button>
                    <button class={classes!(DetailTabBtnStyle::CLASS_NAME, (*active_tab == DetailTab::Tags).then_some(DetailTabActiveStyle::CLASS_NAME))}
                        onclick={on_tags_click}>
                        {"Tags"}
                    </button>
                </div>
                if *active_tab == DetailTab::RaceParticipants {
                    <ParticipantsTab participants={d.participants.clone()} />
                } else if *active_tab == DetailTab::Tags {
                    <TagsTab race_dump_id={d.summary.id} initial_tags={d.summary.tags.clone()} />
                } else if *active_tab == DetailTab::RaceReplay {
                    <div class={DetailBodyStyle::CLASS_NAME}>
                        <ParticipantsPanel participants={d.participants.clone()} snapshots={(*snapshots).clone()} />
                        <div class={ReplayPanelStyle::CLASS_NAME}>
                            <ReplayPanel
                                summary={d.summary.clone()}
                                participants={d.participants.clone()}
                                frames={d.frames.clone()}
                                events={d.events.clone()}
                                on_frame={on_frame}
                                paused={*active_tab != DetailTab::RaceReplay}
                            />
                        </div>
                    </div>
                }
            } else if *loading {
                <div class={LoadingOverlayStyle::CLASS_NAME}>{"Loading..."}</div>
            } else if let Some(ref err) = *error {
                <div class={ErrorOverlayStyle::CLASS_NAME}>{err}</div>
            }

            <NotificationOverlay notifications={notif_state.0.clone()} on_close={{
                let _remove = _remove.clone();
                Callback::from(move |id| _remove(id))
            }} />
        </div>
    }
}
