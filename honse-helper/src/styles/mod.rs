pub mod app;
pub mod custom_select;
pub mod db_status;
pub mod detail_modal;
pub mod filter_panel;
pub mod gather_veterans;
pub mod legacy_affinity;
pub mod legacy_planner;
pub mod legacy_summary;
pub mod legacy_veteran_slots;
pub mod notifications;
pub mod pagination;
pub mod preset_manager;
pub mod race_dump;
pub mod race_dump_card;
pub mod race_dump_detail;
pub mod shared_components;
pub mod skill_pill;
pub mod sort_selector;
pub mod spark_item;
pub mod support_card_browser;
pub mod tag_modal;
pub mod trainee_browser;
pub mod veteran_browser;
pub mod veteran_card;
pub mod worker_status;

use yew::{html, Html};

inventory::collect!(StyleDefinition);

#[derive(Clone)]
pub struct StyleDefinition {
    pub css: &'static str,
    pub selector_type: SelectorType,
    pub class_name: &'static str,
}

#[derive(Clone, PartialEq)]
pub enum SelectorType {
    Class,
    Id,
}

pub trait Style {
    const CLASS_NAME: &'static str;
    const CSS: &'static str;
    const SELECTOR_TYPE: SelectorType = SelectorType::Class;
}

pub struct StyleManager;

impl StyleManager {
    pub fn render_stylesheet() -> Html {
        let mut css_rules = Vec::new();

        inventory::iter::<StyleDefinition>
            .into_iter()
            .for_each(|def| {
                let selector = match def.selector_type {
                    SelectorType::Class => format!(".{}", def.class_name),
                    SelectorType::Id => format!("#{}", def.class_name),
                };
                let css = def.css.replace("{{class}}", &selector);
                css_rules.push(css);
            });

        if css_rules.is_empty() {
            html! {
                <style></style>
            }
        } else {
            html! {
                <style>
                    {css_rules.join("\n\n")}
                </style>
            }
        }
    }
}
