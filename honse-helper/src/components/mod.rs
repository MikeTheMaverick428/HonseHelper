pub mod copyable;
pub mod date_time_selector;
pub mod delete_button;
pub mod delete_confirmation_modal;
pub mod external_link;
pub mod gather_support_cards;
pub mod gather_trainees;
pub mod gather_veterans;
pub mod loading_overlay;
pub mod notifications;
pub mod sparks;
pub mod tag_modal;
pub mod wins_list;

pub use crate::veteran_browser::components::searchable_select::SelectOption;

use yew::prelude::*;

pub fn render_gather_time(time: &Option<String>) -> Html {
    let label = time.as_ref().map(|t| {
        let local = local_time_str(t);
        format!("Last gathered: {}", local)
    });
    html! {
        <div style="font-size: 11px; color: #64748b; padding-top: 4px; min-height: 16px;">
            {label.unwrap_or_default()}
        </div>
    }
}

fn local_time_str(utc_rfc3339: &str) -> String {
    let millis = js_sys::Date::parse(utc_rfc3339);
    if millis.is_nan() {
        let time = utc_rfc3339.replace('T', " ");
        if time.len() > 16 {
            time[..16].to_string()
        } else {
            time
        }
    } else {
        let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(millis));
        let year = date.get_full_year();
        let month = date.get_month() + 1;
        let day = date.get_date();
        let hours = date.get_hours();
        let mins = date.get_minutes();
        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}",
            year, month, day, hours, mins
        )
    }
}
