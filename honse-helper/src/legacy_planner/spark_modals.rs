use shared::{
    legacy_planner::{InspirationSummaryRow, SparkGroupInfo, SparkSummaryRow},
    models::SparkType,
};
use yew::prelude::*;

use crate::{
    components::sparks::SparksList,
    styles::{
        detail_modal::{
            ModalBodyStyle, ModalCloseStyle, ModalContentStyle, ModalHeaderStyle, ModalOverlayStyle,
        },
        Style,
    },
    tauri_bridge::invoke_tauri_command,
};
use serde_json::json;

// ── Sparks List Modal ────────────────────────────────────────────

#[derive(Properties, PartialEq)]
pub struct SparksListModalProps {
    pub all_spark_groups: Vec<SparkGroupInfo>,
    pub on_close: Callback<()>,
}

#[function_component]
pub fn SparksListModal(props: &SparksListModalProps) -> Html {
    let on_close = {
        let cb = props.on_close.clone();
        Callback::from(move |_| cb.emit(()))
    };

    html! {
        <div class={ModalOverlayStyle::CLASS_NAME} onclick={on_close.clone()}>
            <div class={ModalContentStyle::CLASS_NAME} onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                <div class={ModalHeaderStyle::CLASS_NAME}>
                    <h2 style="margin: 0;">{"Sparks"}</h2>
                    <button onclick={on_close.clone()} class={ModalCloseStyle::CLASS_NAME}>{"\u{00D7}"}</button>
                </div>
                <div class={ModalBodyStyle::CLASS_NAME}>
                    <div style="max-height: 60vh; overflow-y: auto;">
                        <SparksList spark_groups={props.all_spark_groups.clone()} active_spark_filters={Vec::new()} />
                    </div>
                </div>
            </div>
        </div>
    }
}

// ── White Spark Generating Chance Modal ──────────────────────────

#[derive(Properties, PartialEq)]
pub struct WhiteSparkChanceModalProps {
    pub on_close: Callback<()>,
}

fn white_spark_probabilities(carriers: usize) -> (f64, f64, f64) {
    let c = carriers.min(6);
    match c {
        0 => (20.0, 25.0, 40.0),
        1 => (22.5, 27.5, 45.0),
        2 => (25.0, 30.0, 50.0),
        3 => (27.5, 32.5, 55.0),
        4 => (30.0, 35.0, 60.0),
        5 => (32.5, 37.5, 65.0),
        _ => (35.0, 40.0, 70.0),
    }
}

#[function_component]
pub fn WhiteSparkChanceModal(props: &WhiteSparkChanceModalProps) -> Html {
    let data = use_state(Vec::<SparkSummaryRow>::new);
    let filter_text = use_state(String::new);
    let type_filters = use_state(|| vec![SparkType::Skill, SparkType::Race, SparkType::Scenario]);

    {
        let data = data.clone();
        use_effect_with((), move |_| {
            let data = data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(result) =
                    invoke_tauri_command("get_planner_spark_summary", json!({})).await
                {
                    if let Ok(rows) = serde_json::from_value::<Vec<SparkSummaryRow>>(result) {
                        data.set(rows);
                    }
                }
            });
            || {}
        });
    }

    let on_filter_input = {
        let filter_text = filter_text.clone();
        Callback::from(move |e: web_sys::InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                filter_text.set(input.value());
            }
        })
    };

    let toggle_type = {
        let type_filters = type_filters.clone();
        Callback::from(move |t: SparkType| {
            let mut v = (*type_filters).clone();
            if let Some(pos) = v.iter().position(|x| *x == t) {
                v.remove(pos);
            } else {
                v.push(t);
            }
            type_filters.set(v);
        })
    };

    let on_close = {
        let cb = props.on_close.clone();
        Callback::from(move |_| cb.emit(()))
    };

    let filtered = {
        let rows = (*data).clone();
        let ft = filter_text.to_lowercase();
        let tf = (*type_filters).clone();
        let mut filtered: Vec<SparkSummaryRow> = rows
            .into_iter()
            .filter(|r| {
                if !r.spark_type.is_white() {
                    return false;
                }
                if !tf.contains(&r.spark_type) {
                    return false;
                }
                if !ft.is_empty() && !r.spark_name.to_lowercase().contains(&ft) {
                    return false;
                }
                true
            })
            .collect();
        filtered.sort_by(|a, b| {
            b.total_stars
                .cmp(&a.total_stars)
                .then(a.spark_name.cmp(&b.spark_name))
        });
        filtered
    };

    html! {
        <div class={ModalOverlayStyle::CLASS_NAME} onclick={on_close.clone()}>
            <div class={ModalContentStyle::CLASS_NAME} onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                <div class={ModalHeaderStyle::CLASS_NAME}>
                    <h2 style="margin: 0;">{"White Spark Generating Chance"}</h2>
                    <button onclick={on_close.clone()} class={ModalCloseStyle::CLASS_NAME}>{"\u{00D7}"}</button>
                </div>
                <div class={ModalBodyStyle::CLASS_NAME}>
                    <div style="display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-bottom: 12px;">
                        <input
                            type="text"
                            placeholder="Filter by name..."
                            oninput={on_filter_input}
                            style="background: #1f2937; color: #e2e8f0; border: 1px solid #475569; border-radius: 4px; padding: 6px 10px; font-size: 13px; flex: 1; min-width: 150px;"
                        />
                        {for [SparkType::Skill, SparkType::Race, SparkType::Scenario].iter().map(|t| {
                            let active = (*type_filters).contains(t);
                            let tt = toggle_type.clone();
                            let t2 = *t;
                            html! {
                                <button
                                    onclick={Callback::from(move |_| tt.emit(t2))}
                                    style={format!(
                                        "padding: 4px 10px; border-radius: 999px; border: 1px solid {}; background: {}; color: {}; cursor: pointer; font-size: 12px; font-weight: 600;",
                                        if active { "#f59e0b" } else { "#475569" },
                                        if active { "#451a1a" } else { "transparent" },
                                        if active { "#fbbf24" } else { "#94a3b8" },
                                    )}
                                >
                                    {t.label()}
                                </button>
                            }
                        })}
                    </div>

                    <div style="overflow-x: auto;">
                        <table style="width: 100%; border-collapse: collapse; font-size: 13px;">
                            <thead>
                                <tr style="background: #1e293b; color: #94a3b8; text-transform: uppercase; font-size: 11px; letter-spacing: 0.05em;">
                                    <th style="padding: 8px 12px; text-align: left; border-bottom: 1px solid #334155;">{"Spark"}</th>
                                    <th style="padding: 8px 12px; text-align: left; border-bottom: 1px solid #334155;">{"Type"}</th>
                                    <th style="padding: 8px 12px; text-align: center; border-bottom: 1px solid #334155;">{"Legacy Umas"}</th>
                                    <th style="padding: 8px 12px; text-align: center; border-bottom: 1px solid #334155;">{"Total Stars"}</th>
                                    <th style="padding: 8px 12px; text-align: center; border-bottom: 1px solid #334155;">{"White"}</th>
                                    <th style="padding: 8px 12px; text-align: center; border-bottom: 1px solid #334155;">{"◎ Skill"}</th>
                                    <th style="padding: 8px 12px; text-align: center; border-bottom: 1px solid #334155;">{"Gold Skill"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {for filtered.iter().map(|row| {
                                    let (w, m, g) = white_spark_probabilities(row.legacy_uma_count);
                                    html! {
                                        <tr style="border-bottom: 1px solid #1e293b;">
                                            <td style="padding: 8px 12px; color: #f3f4f6; font-weight: 500;">{&row.spark_name}</td>
                                            <td style="padding: 8px 12px; color: #94a3b8; font-size: 12px;">{row.spark_type.label()}</td>
                                            <td style="padding: 8px 12px; text-align: center; color: #94a3b8;">{row.legacy_uma_count}</td>
                                            <td style="padding: 8px 12px; text-align: center; color: #f3f4f6; font-weight: 600;">{format!("{}★", row.total_stars)}</td>
                                            <td style="padding: 8px 12px; text-align: center; color: #fbbf24; font-weight: 600;">{format!("{:.1}%", w)}</td>
                                            <td style="padding: 8px 12px; text-align: center; color: #a78bfa; font-weight: 600;">
                                                {if row.spark_type == SparkType::Skill { format!("{:.1}%", m) } else { "-".to_string() }}
                                            </td>
                                            <td style="padding: 8px 12px; text-align: center; color: #fbbf24; font-weight: 600;">
                                                {if row.spark_type == SparkType::Skill { format!("{:.1}%", g) } else { "-".to_string() }}
                                            </td>
                                        </tr>
                                    }
                                })}
                                if filtered.is_empty() {
                                    <tr>
                                        <td colspan="7" style="padding: 24px; text-align: center; color: #64748b;">{"No white spark data."}</td>
                                    </tr>
                                }
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
    }
}

// ── Inspiration Spark Chance Modal ───────────────────────────────

#[derive(Properties, PartialEq)]
pub struct InspirationChanceModalProps {
    pub on_close: Callback<()>,
}

#[function_component]
pub fn InspirationChanceModal(props: &InspirationChanceModalProps) -> Html {
    let data = use_state(Vec::<InspirationSummaryRow>::new);
    let filter_text = use_state(String::new);
    let type_filters = use_state(|| {
        vec![
            SparkType::Stat,
            SparkType::Aptitude,
            SparkType::Unique,
            SparkType::Skill,
            SparkType::Race,
            SparkType::Scenario,
        ]
    });

    {
        let data = data.clone();
        use_effect_with((), move |_| {
            let data = data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(result) =
                    invoke_tauri_command("get_planner_inspiration_summary", json!({})).await
                {
                    if let Ok(rows) = serde_json::from_value::<Vec<InspirationSummaryRow>>(result) {
                        data.set(rows);
                    }
                }
            });
            || {}
        });
    }

    let on_filter_input = {
        let filter_text = filter_text.clone();
        Callback::from(move |e: web_sys::InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                filter_text.set(input.value());
            }
        })
    };

    let toggle_type = {
        let type_filters = type_filters.clone();
        Callback::from(move |t: SparkType| {
            let mut v = (*type_filters).clone();
            if let Some(pos) = v.iter().position(|x| *x == t) {
                v.remove(pos);
            } else {
                v.push(t);
            }
            type_filters.set(v);
        })
    };

    let on_close = {
        let cb = props.on_close.clone();
        Callback::from(move |_| cb.emit(()))
    };

    let filtered = {
        let rows = (*data).clone();
        let ft = filter_text.to_lowercase();
        let tf = (*type_filters).clone();
        let mut filtered: Vec<InspirationSummaryRow> = rows
            .into_iter()
            .filter(|r| {
                if !tf.contains(&r.spark_type) {
                    return false;
                }
                if !ft.is_empty() && !r.spark_name.to_lowercase().contains(&ft) {
                    return false;
                }
                true
            })
            .collect();
        filtered.sort_by(|a, b| {
            b.sparking_chance
                .partial_cmp(&a.sparking_chance)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.spark_name.cmp(&b.spark_name))
        });
        filtered
    };

    html! {
        <div class={ModalOverlayStyle::CLASS_NAME} onclick={on_close.clone()}>
            <div class={ModalContentStyle::CLASS_NAME} onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                <div class={ModalHeaderStyle::CLASS_NAME}>
                    <h2 style="margin: 0;">{"Spark Inspiration Chance"}</h2>
                    <button onclick={on_close.clone()} class={ModalCloseStyle::CLASS_NAME}>{"\u{00D7}"}</button>
                </div>
                <div class={ModalBodyStyle::CLASS_NAME}>
                    <div style="display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-bottom: 12px;">
                        <input
                            type="text"
                            placeholder="Filter by name..."
                            oninput={on_filter_input}
                            style="background: #1f2937; color: #e2e8f0; border: 1px solid #475569; border-radius: 4px; padding: 6px 10px; font-size: 13px; flex: 1; min-width: 150px;"
                        />
                        {for [SparkType::Stat, SparkType::Aptitude, SparkType::Unique, SparkType::Skill, SparkType::Race, SparkType::Scenario].iter().map(|t| {
                            let active = (*type_filters).contains(t);
                            let tt = toggle_type.clone();
                            let t2 = *t;
                            html! {
                                <button
                                    onclick={Callback::from(move |_| tt.emit(t2))}
                                    style={format!(
                                        "padding: 4px 10px; border-radius: 999px; border: 1px solid {}; background: {}; color: {}; cursor: pointer; font-size: 12px; font-weight: 600;",
                                        if active { "#f59e0b" } else { "#475569" },
                                        if active { "#451a1a" } else { "transparent" },
                                        if active { "#fbbf24" } else { "#94a3b8" },
                                    )}
                                >
                                    {t.label()}
                                </button>
                            }
                        })}
                    </div>

                    <div style="overflow-x: auto;">
                        <table style="width: 100%; border-collapse: collapse; font-size: 13px;">
                            <thead>
                                <tr style="background: #1e293b; color: #94a3b8; text-transform: uppercase; font-size: 11px; letter-spacing: 0.05em;">
                                    <th style="padding: 8px 12px; text-align: left; border-bottom: 1px solid #334155;">{"Spark"}</th>
                                    <th style="padding: 8px 12px; text-align: left; border-bottom: 1px solid #334155;">{"Type"}</th>
                                    <th style="padding: 8px 12px; text-align: right; border-bottom: 1px solid #334155;">{"Sparking Chance"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {for filtered.iter().map(|row| {
                                    html! {
                                        <tr style="border-bottom: 1px solid #1e293b;">
                                            <td style="padding: 8px 12px; color: #f3f4f6; font-weight: 500;">{&row.spark_name}</td>
                                            <td style="padding: 8px 12px; color: #94a3b8; font-size: 12px;">{row.spark_type.label()}</td>
                                            <td style="padding: 8px 12px; text-align: right; color: #fbbf24; font-weight: 600; font-feature-settings: 'tnum' 1;">
                                                {format!("{:.2}%", row.sparking_chance)}
                                            </td>
                                        </tr>
                                    }
                                })}
                                if filtered.is_empty() {
                                    <tr>
                                        <td colspan="3" style="padding: 24px; text-align: center; color: #64748b;">{"No inspiration data."}</td>
                                    </tr>
                                }
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
    }
}
