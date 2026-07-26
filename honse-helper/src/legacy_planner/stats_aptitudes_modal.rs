use shared::{
    legacy_planner::SparkGroupInfo,
    models::{AptitudeLevel, SparkType},
    TraineeStatsDataRow,
};
use yew::prelude::*;

use crate::{
    components::SelectOption,
    styles::{
        detail_modal::{
            ModalBodyStyle, ModalCloseStyle, ModalContentStyle, ModalHeaderStyle, ModalOverlayStyle,
        },
        Style,
    },
    tauri_bridge::invoke_tauri_command,
    veteran_browser::components::custom_select::CustomSelect,
};
use serde_json::json;

#[derive(Properties, PartialEq)]
pub struct StatsAptitudesModalProps {
    pub trainee_id: i64,
    pub individual_spark_groups: Vec<SparkGroupInfo>,
    pub on_close: Callback<()>,
}

fn star_to_bonus(stars: i8) -> i32 {
    match stars {
        1 => 5,
        2 => 11,
        3 => 21,
        _ => 0,
    }
}

fn compute_aptitude_boost(total_stars: i32) -> i32 {
    let mut remaining = total_stars;
    let mut steps = 0;
    let mut cost = 1;
    while remaining >= cost && steps < 4 {
        steps += 1;
        remaining -= cost;
        cost = 3;
    }
    steps
}

fn sum_stat_bonus(spark_group_id: i64, individual: &[SparkGroupInfo]) -> i32 {
    individual
        .iter()
        .filter(|s| s.spark_group_id == spark_group_id && s.spark_type == SparkType::Stat)
        .map(|s| star_to_bonus(s.total_stars))
        .sum()
}

fn aptitude_from_i64(v: i64) -> AptitudeLevel {
    match v {
        8 => AptitudeLevel::S,
        7 => AptitudeLevel::A,
        6 => AptitudeLevel::B,
        5 => AptitudeLevel::C,
        4 => AptitudeLevel::D,
        3 => AptitudeLevel::E,
        2 => AptitudeLevel::F,
        1 => AptitudeLevel::G,
        _ => AptitudeLevel::H,
    }
}

#[function_component]
pub fn StatsAptitudesModal(props: &StatsAptitudesModalProps) -> Html {
    let rarities = use_state(Vec::<i64>::new);
    let selected_rarity = use_state(|| 3i64);
    let stats_data = use_state(|| None::<TraineeStatsDataRow>);
    let loading = use_state(|| false);

    {
        let rarities = rarities.clone();
        let tid = props.trainee_id;
        use_effect_with(tid, move |_| {
            let rarities = rarities.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(result) = invoke_tauri_command(
                    "get_trainee_available_rarities",
                    json!({"traineeId": tid}),
                )
                .await
                {
                    if let Ok(list) = serde_json::from_value::<Vec<i64>>(result) {
                        rarities.set(list);
                    }
                }
            });
            || {}
        });
    }

    {
        let stats_data = stats_data.clone();
        let loading = loading.clone();
        let tid = props.trainee_id;
        let rarity = *selected_rarity;
        use_effect_with((tid, rarity), move |_| {
            let stats_data = stats_data.clone();
            let loading = loading.clone();
            loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(result) = invoke_tauri_command(
                    "get_trainee_stats",
                    json!({"traineeId": tid, "rarity": rarity}),
                )
                .await
                {
                    if let Ok(row) = serde_json::from_value::<Option<TraineeStatsDataRow>>(result) {
                        stats_data.set(row);
                    }
                }
                loading.set(false);
            });
            || {}
        });
    }

    let on_rarity_change = {
        let selected_rarity = selected_rarity.clone();
        Callback::from(move |val: String| {
            if let Ok(v) = val.parse::<i64>() {
                selected_rarity.set(v);
            }
        })
    };

    let on_close = {
        let cb = props.on_close.clone();
        Callback::from(move |_| cb.emit(()))
    };

    let individual_spark_groups = &props.individual_spark_groups;

    html! {
        <div class={ModalOverlayStyle::CLASS_NAME} onclick={on_close.clone()}>
            <div class={ModalContentStyle::CLASS_NAME} onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                <div class={ModalHeaderStyle::CLASS_NAME}>
                    <h2 style="margin: 0;">{"Stats + Aptitudes"}</h2>
                    <button onclick={on_close.clone()} class={ModalCloseStyle::CLASS_NAME}>{"\u{00D7}"}</button>
                </div>

                <div class={ModalBodyStyle::CLASS_NAME}>
                    if *loading {
                        <p>{"Loading..."}</p>
                    } else if let Some(data) = &*stats_data {
                        <div style="margin-bottom: 12px; display: flex; align-items: center; gap: 8px;">
                            <label style="font-size: 12px; color: #94a3b8;">{"Rarity:"}</label>
                            <CustomSelect
                                options={{
                                    let list: Vec<i64> = if !rarities.is_empty() { (*rarities).clone() } else { (1i64..=5).collect() };
                                    list.iter().map(|r| SelectOption {
                                        value: r.to_string(),
                                        label: format!("{}★", r),
                                    }).collect::<Vec<_>>()
                                }}
                                on_change={on_rarity_change}
                                selected={Some(selected_rarity.to_string())}
                            />
                        </div>

                        <StatsGridComponent data={data.clone()} individual_spark_groups={individual_spark_groups.clone()} />
                        <hr style="border: none; border-top: 1px solid #334155; margin: 16px 0;" />
                        <AptitudesSectionComponent data={data.clone()} individual_spark_groups={individual_spark_groups.clone()} />
                    } else {
                        <p style="color: #94a3b8;">{"No stats data available for this trainee."}</p>
                    }
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct StatsGridProps {
    data: TraineeStatsDataRow,
    individual_spark_groups: Vec<SparkGroupInfo>,
}

#[function_component]
fn StatsGridComponent(props: &StatsGridProps) -> Html {
    let d = &props.data;
    let ind = &props.individual_spark_groups;

    let stat_items = vec![
        ("SPD", d.spe, 1i64, "#06b6d4"),
        ("STA", d.sta, 2, "#f59e0b"),
        ("PWR", d.pwr, 3, "#ef4444"),
        ("GUT", d.gut, 4, "#f97316"),
        ("WIT", d.wit, 5, "#22c55e"),
    ];

    html! {
        <div style="display: grid; grid-template-columns: repeat(5, 1fr); gap: 8px; margin-top: 8px;">
            {for stat_items.iter().map(|(label, base, group_id, color)| {
                let bonus = sum_stat_bonus(*group_id, ind);
                let total = *base as i32 + bonus;
                let boosted = bonus > 0;
                let border_color = if boosted { "#f59e0b" } else { color };
                let bg = if boosted { "linear-gradient(135deg, #1a1a2e, #2d1b1b)" } else { "#1e293b" };
                html! {
                    <div
                        style={format!(
                            "background: {}; border-top: 3px solid {}; border-radius: 6px; padding: 8px; text-align: center; position: relative; overflow: hidden;",
                            bg, border_color
                        )}
                    >
                        <div style="font-size: 10px; text-transform: uppercase; color: #64748b; letter-spacing: 0.5px; margin-bottom: 4px;">{label}</div>
                        <div style="font-size: 16px; font-weight: 700; color: #f3f4f6;">{total}</div>
                        if boosted {
                            <div style="font-size: 11px; font-weight: 600; color: #f59e0b;">{format!("+{}", bonus)}</div>
                        }
                    </div>
                }
            })}
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct AptitudesSectionProps {
    data: TraineeStatsDataRow,
    individual_spark_groups: Vec<SparkGroupInfo>,
}

fn aptitude_color(level: &AptitudeLevel) -> &'static str {
    match level {
        AptitudeLevel::S => "#7c3aed",
        AptitudeLevel::A => "#f59e0b",
        AptitudeLevel::B => "#3b82f6",
        AptitudeLevel::C => "#22c55e",
        AptitudeLevel::D => "#a3e635",
        AptitudeLevel::E => "#fb923c",
        AptitudeLevel::F => "#fca5a5",
        AptitudeLevel::G | AptitudeLevel::H => "#e2e8f0",
    }
}

fn aptitude_weight(level: &AptitudeLevel) -> &'static str {
    match level {
        AptitudeLevel::S => "800",
        _ => "700",
    }
}

#[function_component]
fn AptitudesSectionComponent(props: &AptitudesSectionProps) -> Html {
    let d = &props.data;
    let ind = &props.individual_spark_groups;

    fn apt_total_stars(group_id: i64, individual: &[SparkGroupInfo]) -> i32 {
        individual
            .iter()
            .filter(|s| s.spark_group_id == group_id && s.spark_type == SparkType::Aptitude)
            .map(|s| s.total_stars as i32)
            .sum()
    }

    let ground: Vec<(&str, i64, i64)> = vec![
        ("Turf", 11, d.aptitude_ground_turf),
        ("Dirt", 12, d.aptitude_ground_dirt),
    ];
    let distance: Vec<(&str, i64, i64)> = vec![
        ("Sprint", 31, d.aptitude_dist_sprint),
        ("Mile", 32, d.aptitude_dist_mile),
        ("Medium", 33, d.aptitude_dist_medium),
        ("Long", 34, d.aptitude_dist_long),
    ];
    let style: Vec<(&str, i64, i64)> = vec![
        ("Front", 21, d.aptitude_style_front),
        ("Pace Chaser", 22, d.aptitude_style_pace_chaser),
        ("Late Surger", 23, d.aptitude_style_late_surger),
        ("End Closer", 24, d.aptitude_style_end_closer),
    ];

    let render_group = |items: Vec<(&str, i64, i64)>, label: &str| {
        html! {
            <div style="margin-bottom: 12px;">
                <div style="font-size: 11px; text-transform: uppercase; color: #64748b; letter-spacing: 0.5px; margin-bottom: 6px;">{label}</div>
                <div style="display: flex; flex-wrap: wrap; gap: 6px;">
                    {for items.iter().map(|(name, group_id, db_val)| {
                        let base = aptitude_from_i64(*db_val);
                        let total = apt_total_stars(*group_id, ind);
                        let boost = compute_aptitude_boost(total);
                        let current = base;
                        let mut applied = 0;
                        let mut remaining = boost;
                        let mut c = current;
                        while remaining > 0 && c != AptitudeLevel::A {
                            c.increase();
                            applied += 1;
                            remaining -= 1;
                        }
                        let display_level = c;
                        let color = aptitude_color(&display_level);
                        let weight = aptitude_weight(&display_level);
                        let grade_changed = applied > 0;
                        let spark_available = total > 0;
                        let border_color = if grade_changed || spark_available { "#f59e0b" } else { color };
                        let bg = if grade_changed {
                            "linear-gradient(135deg, #1a1a2e, #2d1b1b)"
                        } else if spark_available {
                            "#1e293b"
                        } else {
                            "#1e293b"
                        };
                        let glow = if grade_changed {
                            format!("box-shadow: 0 0 8px rgba(245, 158, 11, 0.4);")
                        } else if spark_available {
                            format!("box-shadow: 0 0 4px rgba(245, 158, 11, 0.25);")
                        } else {
                            String::new()
                        };
                        html! {
                            <div
                                style={format!(
                                    "background: {}; border-top: 3px solid {}; border-radius: 6px; padding: 6px 12px; text-align: center; min-width: 70px; {}",
                                    bg, border_color, glow
                                )}
                            >
                                <div style="font-size: 10px; text-transform: uppercase; color: #64748b; margin-bottom: 2px;">{name}</div>
                                <div style={format!("font-size: 18px; font-weight: {}; color: {};", weight, color)}>
                                    {display_level.to_string()}
                                </div>
                                if grade_changed {
                                    <div style="font-size: 10px; font-weight: 600; color: #f59e0b;">{format!("+{}", applied)}</div>
                                } else if spark_available {
                                    <div style="font-size: 10px; font-weight: 600; color: #f59e0b;">{"★"}</div>
                                }
                            </div>
                        }
                    })}
                </div>
            </div>
        }
    };

    html! {
        <>
            {render_group(ground, "Ground")}
            {render_group(distance, "Distance")}
            {render_group(style, "Running Style")}
        </>
    }
}
