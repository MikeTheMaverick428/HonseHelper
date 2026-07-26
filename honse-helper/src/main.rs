mod api_config_window;
mod app;
mod components;
mod db_status_window;
mod dev_tools;
mod legacy_planner;
mod race_dump;
mod race_dump_detail;
mod styles;
mod supplementary_data_window;
mod support_card_browser;
mod tauri_bridge;
mod trainee_browser;
mod veteran_browser;
mod worker_status;

use api_config_window::ApiConfigWindow;
use app::App;
use db_status_window::DbStatusWindow;
use dev_tools::DevTools;
use legacy_planner::LegacyPlanner;
use race_dump::RaceDumpBrowser;
use race_dump_detail::RaceDumpDetailWindow;
use supplementary_data_window::SupplementaryDataWindow;
use support_card_browser::SupportCardBrowser;
use tauri_bridge::get_window_label;
use trainee_browser::TraineeBrowser;
use veteran_browser::VeteranBrowser;
use worker_status::window::WorkerStatusWindow;
use yew::prelude::*;

#[function_component]
fn Root() -> Html {
    let window_label = use_state(String::new);

    {
        let window_label = window_label.clone();
        use_effect_with((), move |_| {
            let label = get_window_label().unwrap_or_default();
            window_label.set(label);
            || {}
        });
    }

    match window_label.as_str() {
        "db-status" => html! { <DbStatusWindow /> },
        "veteran-browser" => html! { <VeteranBrowser /> },
        "support-card-browser" => html! { <SupportCardBrowser /> },
        "trainee-browser" => html! { <TraineeBrowser /> },
        "legacy-planner" => html! { <LegacyPlanner /> },
        "worker-status" => html! { <WorkerStatusWindow /> },
        "api-config" => html! { <ApiConfigWindow /> },
        "race-dump" => html! { <RaceDumpBrowser /> },
        "race-dump-detail" => html! { <RaceDumpDetailWindow /> },
        "dev-tools" => html! { <DevTools /> },
        "supplementary-data" => html! { <SupplementaryDataWindow /> },
        _ => html! { <App /> },
    }
}

fn main() {
    yew::Renderer::<Root>::new().render();
}
