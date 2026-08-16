use shared::{
    legacy_planner::{InspirationSummaryRow, SparkGroupInfo, SparkSummaryRow},
    models::SparkType,
};
use yew::prelude::*;
use std::cmp::Ordering;

use crate::{
    components::{
        sortable_table::{Align, ColumnCfg, SortDir, SortableTable},
        sparks::SparksList,
    },
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
                <div class={ModalBodyStyle::CLASS_NAME} style="display: flex; flex-direction: column; overflow: hidden; min-height: 0;">
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum WhiteSortCol {
    Name,
    Type,
    Umas,
    Stars,
    White,
    Maru,
    Gold,
}

fn white_columns() -> Vec<ColumnCfg<SparkSummaryRow, WhiteSortCol>> {
    vec![
        ColumnCfg {
            key: WhiteSortCol::Name,
            label: "Spark",
            title: None,
            sortable: true,
            default_dir: SortDir::Asc,
            align: Align::Left,
            compare: |a, b| a.spark_name.cmp(&b.spark_name),
        },
        ColumnCfg {
            key: WhiteSortCol::Type,
            label: "Type",
            title: None,
            sortable: true,
            default_dir: SortDir::Asc,
            align: Align::Left,
            compare: |a, b| a.spark_type.label().cmp(&b.spark_type.label()),
        },
        ColumnCfg {
            key: WhiteSortCol::Umas,
            label: "Legacy Umas",
            title: None,
            sortable: true,
            default_dir: SortDir::Desc,
            align: Align::Center,
            compare: |a, b| a.legacy_uma_count.cmp(&b.legacy_uma_count),
        },
        ColumnCfg {
            key: WhiteSortCol::Stars,
            label: "Total Stars",
            title: None,
            sortable: true,
            default_dir: SortDir::Desc,
            align: Align::Center,
            compare: |a, b| a.total_stars.cmp(&b.total_stars),
        },
        ColumnCfg {
            key: WhiteSortCol::White,
            label: "White",
            title: None,
            sortable: true,
            default_dir: SortDir::Desc,
            align: Align::Center,
            compare: |a, b| {
                white_spark_probabilities(a.legacy_uma_count)
                    .0
                    .partial_cmp(&white_spark_probabilities(b.legacy_uma_count).0)
                    .unwrap_or(Ordering::Equal)
            },
        },
        ColumnCfg {
            key: WhiteSortCol::Maru,
            label: "◎ Skill",
            title: None,
            sortable: false,
            default_dir: SortDir::Desc,
            align: Align::Center,
            compare: |_, _| Ordering::Equal,
        },
        ColumnCfg {
            key: WhiteSortCol::Gold,
            label: "Gold Skill",
            title: None,
            sortable: false,
            default_dir: SortDir::Desc,
            align: Align::Center,
            compare: |_, _| Ordering::Equal,
        },
    ]
}

fn render_white_cell(row: &SparkSummaryRow, col: usize) -> Html {
    let (w, m, g) = white_spark_probabilities(row.legacy_uma_count);
    match col {
        0 => html! { <td style="padding: 8px 12px; color: #f3f4f6; font-weight: 500;">{&row.spark_name}</td> },
        1 => html! { <td style="padding: 8px 12px; color: #94a3b8; font-size: 12px;">{row.spark_type.label()}</td> },
        2 => html! { <td style="padding: 8px 12px; text-align: center; color: #94a3b8;">{row.legacy_uma_count}</td> },
        3 => html! { <td style="padding: 8px 12px; text-align: center; color: #f3f4f6; font-weight: 600;">{format!("{}★", row.total_stars)}</td> },
        4 => html! { <td style="padding: 8px 12px; text-align: center; color: #fbbf24; font-weight: 600;">{format!("{:.1}%", w)}</td> },
        5 => html! { <td style="padding: 8px 12px; text-align: center; color: #a78bfa; font-weight: 600;">
            {if row.spark_type == SparkType::Skill { format!("{:.1}%", m) } else { "-".to_string() }}
        </td> },
        _ => html! { <td style="padding: 8px 12px; text-align: center; color: #fbbf24; font-weight: 600;">
            {if row.spark_type == SparkType::Skill { format!("{:.1}%", g) } else { "-".to_string() }}
        </td> },
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
        rows.into_iter()
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
            .collect::<Vec<SparkSummaryRow>>()
    };

    html! {
        <div class={ModalOverlayStyle::CLASS_NAME} onclick={on_close.clone()}>
            <div class={ModalContentStyle::CLASS_NAME} onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                <div class={ModalHeaderStyle::CLASS_NAME}>
                    <h2 style="margin: 0;">{"White Spark Generating Chance"}</h2>
                    <button onclick={on_close.clone()} class={ModalCloseStyle::CLASS_NAME}>{"\u{00D7}"}</button>
                </div>
                <div class={ModalBodyStyle::CLASS_NAME} style="display: flex; flex-direction: column; overflow: hidden; min-height: 0;">
                    <div style="display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-bottom: 12px; flex-shrink: 0;">
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

                    <div style="flex: 1; min-height: 0; overflow-y: auto;">
                        <SortableTable::<SparkSummaryRow, WhiteSortCol>
                            rows={filtered}
                            columns={white_columns()}
                            default_sort={Some((WhiteSortCol::Stars, SortDir::Desc))}
                            empty_message="No white spark data."
                            cell_renderer={render_white_cell as fn(&SparkSummaryRow, usize) -> Html}
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}

// ── Inspiration Spark Chance Modal ───────────────────────────────

#[derive(Clone, Copy, PartialEq)]
struct SparkMetrics {
    expected: f64,
    avg_pct: f64,
    at_least_one_pct: f64,
}

type InspireRow = (InspirationSummaryRow, SparkMetrics);

#[derive(Clone, Copy, PartialEq, Eq)]
enum InspireSortCol {
    Name,
    Type,
    Umas,
    Stars,
    Expected,
    Avg,
    AtLeastOne,
}

fn inspire_columns() -> Vec<ColumnCfg<InspireRow, InspireSortCol>> {
    vec![
        ColumnCfg {
            key: InspireSortCol::Name,
            label: "Spark",
            title: None,
            sortable: true,
            default_dir: SortDir::Asc,
            align: Align::Left,
            compare: |a, b| a.0.spark_name.cmp(&b.0.spark_name),
        },
        ColumnCfg {
            key: InspireSortCol::Type,
            label: "Type",
            title: None,
            sortable: true,
            default_dir: SortDir::Asc,
            align: Align::Left,
            compare: |a, b| a.0.spark_type.label().cmp(&b.0.spark_type.label()),
        },
        ColumnCfg {
            key: InspireSortCol::Umas,
            label: "Umas",
            title: None,
            sortable: true,
            default_dir: SortDir::Desc,
            align: Align::Center,
            compare: |a, b| a.0.total_umas.cmp(&b.0.total_umas),
        },
        ColumnCfg {
            key: InspireSortCol::Stars,
            label: "Total Stars",
            title: None,
            sortable: true,
            default_dir: SortDir::Desc,
            align: Align::Center,
            compare: |a, b| a.0.total_stars.cmp(&b.0.total_stars),
        },
        ColumnCfg {
            key: InspireSortCol::Expected,
            label: "Expected value",
            title: Some("E[X] — expected number of sparks fired: sum of each carrier's chance, each clamped at 100% per event"),
            sortable: true,
            default_dir: SortDir::Desc,
            align: Align::Right,
            compare: |a, b| {
                a.1.expected
                    .partial_cmp(&b.1.expected)
                    .unwrap_or(Ordering::Equal)
            },
        },
        ColumnCfg {
            key: InspireSortCol::Avg,
            label: "Average Chance",
            title: Some("S_w — weighted rating score with equal weights, i.e. the simple average chance"),
            sortable: true,
            default_dir: SortDir::Desc,
            align: Align::Right,
            compare: |a, b| a.1.avg_pct.partial_cmp(&b.1.avg_pct).unwrap_or(Ordering::Equal),
        },
        ColumnCfg {
            key: InspireSortCol::AtLeastOne,
            label: "At Least One",
            title: Some("P(≥1) — probability that at least one carrier fires the spark: 1 − Π(1 − p)"),
            sortable: true,
            default_dir: SortDir::Desc,
            align: Align::Right,
            compare: |a, b| {
                a.1.at_least_one_pct
                    .partial_cmp(&b.1.at_least_one_pct)
                    .unwrap_or(Ordering::Equal)
            },
        },
    ]
}

fn render_inspire_cell(row: &InspireRow, col: usize) -> Html {
    let (r, m) = row;
    match col {
        0 => html! { <td style="padding: 8px 12px; color: #f3f4f6; font-weight: 500;">{&r.spark_name}</td> },
        1 => html! { <td style="padding: 8px 12px; color: #94a3b8; font-size: 12px;">{r.spark_type.label()}</td> },
        2 => html! { <td style="padding: 8px 12px; text-align: center; color: #94a3b8;">{r.total_umas}</td> },
        3 => html! { <td style="padding: 8px 12px; text-align: center; color: #f3f4f6; font-weight: 600;">{format!("{}★", r.total_stars)}</td> },
        4 => html! { <td style="padding: 8px 12px; text-align: right; color: #fbbf24; font-weight: 600; font-feature-settings: 'tnum' 1;">{format!("{:.2}", m.expected)}</td> },
        5 => html! { <td style="padding: 8px 12px; text-align: right; color: #94a3b8; font-feature-settings: 'tnum' 1;">{format!("{:.2}%", m.avg_pct)}</td> },
        _ => html! { <td style="padding: 8px 12px; text-align: right; color: #34d399; font-weight: 600; font-feature-settings: 'tnum' 1;">{format!("{:.2}%", m.at_least_one_pct)}</td> },
    }
}

/// Computes the guide metrics from per-carrier chances (percent):
/// E[X] (expected sparks), S_w (average chance, equal weights),
/// P(≥1) (at-least-one rate). Each chance is clamped at 100% per
/// event; `events` repeats every chance (2 for a career of two
/// inspirations), doubling E[X] and re-applying the at-least-one
/// product.
fn spark_metrics(chances: &[f64], events: usize) -> SparkMetrics {
    let mut expected = 0.0;
    let mut prob_none = 1.0;
    let mut count = 0usize;
    for _ in 0..events {
        for &p in chances {
            let q = (p / 100.0).min(1.0);
            expected += q;
            prob_none *= 1.0 - q;
            count += 1;
        }
    }
    let avg_pct = if count == 0 { 0.0 } else { expected / count as f64 * 100.0 };
    SparkMetrics {
        expected,
        avg_pct,
        at_least_one_pct: (1.0 - prob_none) * 100.0,
    }
}

#[derive(Properties, PartialEq)]
pub struct InspirationChanceModalProps {
    pub on_close: Callback<()>,
}

#[function_component]
pub fn InspirationChanceModal(props: &InspirationChanceModalProps) -> Html {
    let data = use_state(Vec::<InspirationSummaryRow>::new);
    let filter_text = use_state(String::new);
    let show_career = use_state(|| false);
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

    let events = if *show_career { 2 } else { 1 };

    let filtered = {
        let rows = (*data).clone();
        let ft = filter_text.to_lowercase();
        let tf = (*type_filters).clone();
        rows.into_iter()
            .filter(|r| {
                if !tf.contains(&r.spark_type) {
                    return false;
                }
                if !ft.is_empty() && !r.spark_name.to_lowercase().contains(&ft) {
                    return false;
                }
                true
            })
            .map(|r| {
                let chances: Vec<f64> = r.carriers.iter().map(|c| c.chance_pct).collect();
                let metrics = spark_metrics(&chances, events);
                (r, metrics)
            })
            .collect::<Vec<InspireRow>>()
    };

    html! {
        <div class={ModalOverlayStyle::CLASS_NAME} onclick={on_close.clone()}>
                <div class={ModalContentStyle::CLASS_NAME} style="max-width: 620px;" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                <div class={ModalHeaderStyle::CLASS_NAME}>
                    <h2 style="margin: 0;">{"Spark Inspiration Chance"}</h2>
                    <button onclick={on_close.clone()} class={ModalCloseStyle::CLASS_NAME}>{"\u{00D7}"}</button>
                </div>
                <div class={ModalBodyStyle::CLASS_NAME} style="display: flex; flex-direction: column; overflow: hidden; min-height: 0;">
                    <div style="display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-bottom: 12px; flex-shrink: 0;">
                        <input
                            type="text"
                            placeholder="Filter by name..."
                            oninput={on_filter_input}
                            style="background: #1f2937; color: #e2e8f0; border: 1px solid #475569; border-radius: 4px; padding: 6px 10px; font-size: 13px; flex: 1; min-width: 150px;"
                        />
                        <button
                            onclick={let s=show_career.clone(); Callback::from(move |_| s.set(!*s))}
                            style={format!(
                                "padding: 4px 10px; border-radius: 999px; border: 1px solid {}; background: {}; color: {}; cursor: pointer; font-size: 12px; font-weight: 600; white-space: nowrap;",
                                if *show_career { "#f59e0b" } else { "#475569" },
                                if *show_career { "#451a1a" } else { "transparent" },
                                if *show_career { "#fbbf24" } else { "#94a3b8" },
                            )}
                        >
                            {"Career (2 inspirations)"}
                        </button>
                    </div>

                    <div style="display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-bottom: 12px; flex-shrink: 0;">
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

                    <div style="flex: 1; min-height: 0; overflow-y: auto;">
                        <SortableTable::<InspireRow, InspireSortCol>
                            rows={filtered}
                            columns={inspire_columns()}
                            default_sort={Some((InspireSortCol::Expected, SortDir::Desc))}
                            empty_message="No inspiration data."
                            cell_renderer={render_inspire_cell as fn(&InspireRow, usize) -> Html}
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}
