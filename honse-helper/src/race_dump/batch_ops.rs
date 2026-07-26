use crate::{
    styles::{
        detail_modal::{
            ModalBodyStyle, ModalCloseStyle, ModalContentStyle, ModalHeaderStyle, ModalOverlayStyle,
        },
        Style,
    },
    tauri_bridge::invoke_tauri_command,
};
use serde_json::json;
use shared::race_dump_types::RaceDumpFilter;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BatchOperationsModalProps {
    pub show: bool,
    pub filters: Vec<RaceDumpFilter>,
    pub total: u32,
    pub on_close: Callback<()>,
    pub on_export_done: Callback<Result<String, String>>,
}

fn filter_label(filter: &RaceDumpFilter) -> String {
    match filter {
        RaceDumpFilter::RaceType(v) => format!("Race Type: {:?}", v),
        RaceDumpFilter::DistanceMeters { min, max } => match (min, max) {
            (Some(lo), Some(hi)) => format!("Distance: {}–{}m", lo, hi),
            (Some(lo), None) => format!("Distance: ≥{}m", lo),
            (None, Some(hi)) => format!("Distance: ≤{}m", hi),
            _ => "Distance: any".to_string(),
        },
        RaceDumpFilter::Distance(d) => format!("Distance: {:?}", d),
        RaceDumpFilter::GroundType(v) => format!("Ground: {:?}", v),
        RaceDumpFilter::Season(v) => format!("Season: {:?}", v),
        RaceDumpFilter::Weather(v) => format!("Weather: {:?}", v),
        RaceDumpFilter::GroundCondition(v) => format!("Condition: {:?}", v),
        RaceDumpFilter::Character(id) => format!("Character ID: {}", id),
        RaceDumpFilter::Trainee(id) => format!("Trainee ID: {}", id),
        RaceDumpFilter::VeteranHash(h) => format!("Veteran: {}", h),
        RaceDumpFilter::HasTag(s) => format!("Tag: {}", s),
        RaceDumpFilter::CaptureDate { after, before } => {
            let parts: Vec<&str> = vec![
                after
                    .as_deref()
                    .filter(|_| true)
                    .map(|_| "after")
                    .unwrap_or(""),
                before
                    .as_deref()
                    .filter(|_| true)
                    .map(|_| "before")
                    .unwrap_or(""),
            ]
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect();
            if parts.is_empty() {
                "Date: any".into()
            } else {
                format!("Date {}", parts.join(" & "))
            }
        }
    }
}

#[function_component(BatchOperationsModal)]
pub fn batch_operations_modal(props: &BatchOperationsModalProps) -> Html {
    let busy = use_state(|| false);

    if !props.show {
        return html! {};
    }

    let on_export = {
        let busy = busy.clone();
        let on_done = props.on_export_done.clone();
        let filters = props.filters.clone();
        Callback::from(move |_: MouseEvent| {
            let busy = busy.clone();
            let on_done = on_done.clone();
            let filters = filters.clone();
            wasm_bindgen_futures::spawn_local(async move {
                busy.set(true);
                let payload = json!({
                    "query": {
                        "filters": &filters,
                        "sort": { "key": "capture_time", "direction": "desc" },
                        "page": 1u32,
                        "page_size": 10000u32,
                    }
                });
                match invoke_tauri_command("export_race_dumps_batch", payload).await {
                    Ok(val) => {
                        let msg = val
                            .as_str()
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| val.to_string());
                        on_done.emit(Ok(msg));
                    }
                    Err(e) => on_done.emit(Err(e)),
                }
                busy.set(false);
            });
        })
    };

    let on_overlay_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_: MouseEvent| on_close.emit(()))
    };

    let on_close_btn = {
        let on_close = props.on_close.clone();
        Callback::from(move |_: MouseEvent| on_close.emit(()))
    };

    html! {
        <div class={ModalOverlayStyle::CLASS_NAME} onclick={on_overlay_click}>
            <div class={ModalContentStyle::CLASS_NAME} onclick={|e: MouseEvent| e.stop_propagation()}>
                <div class={ModalHeaderStyle::CLASS_NAME}>
                    <h3>{"Batch Operations"}</h3>
                    <button class={ModalCloseStyle::CLASS_NAME} onclick={on_close_btn}>{"\u{00d7}"}</button>
                </div>
                <div class={ModalBodyStyle::CLASS_NAME}>
                    <div style="margin-bottom: 10px;">
                        <strong>{format!("{} race dumps selected", props.total)}</strong>
                    </div>
                    if !props.filters.is_empty() {
                        <div style="margin-bottom: 14px; color: #94a3b8; font-size: 12px;">
                            <div style="margin-bottom: 3px;">{"Active filters:"}</div>
                            {for props.filters.iter().map(|f| {
                                html! { <div style="padding-left: 10px;">{format!("- {}", filter_label(f))}</div> }
                            })}
                        </div>
                    } else {
                        <div style="margin-bottom: 14px; color: #94a3b8; font-size: 12px;">
                            {"No filters — all races will be exported."}
                        </div>
                    }

                    <hr style="border-color: #1f2937; margin: 12px 0;" />

                    <div style="margin-bottom: 10px; font-weight: 600; font-size: 12px; color: #64748b;">{"ACTIONS"}</div>

                    <button
                        style="
                            width: 100%; padding: 10px 16px;
                            background: #1f2937; border: 1px solid #475569;
                            color: #e2e8f0; border-radius: 8px; cursor: pointer;
                            font-size: 14px; font-weight: 600;
                        "
                        onclick={on_export}
                        disabled={*busy}
                    >
                        if *busy {
                            {"Exporting\u{2026}"}
                        } else {
                            {"Export Hakuraku"}
                        }
                    </button>
                </div>
            </div>
        </div>
    }
}
