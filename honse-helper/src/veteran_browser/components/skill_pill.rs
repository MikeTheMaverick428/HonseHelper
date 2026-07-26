use crate::styles::skill_pill::*;
use crate::styles::Style;
use yew::prelude::*;

pub const ACCENT_COLORS: &[(&str, &str)] = &[
    ("Passive", "#22c55e"),
    ("Velocity", "#eab308"),
    ("Acceleration", "#f97316"),
    ("Recovery", "#3b82f6"),
    ("Navigation", "#8b5cf6"),
    ("Visibility", "#64748b"),
    ("Debuff", "#ef4444"),
    ("Self Debuff", "#a855f7"),
    ("Special", "#ec4899"),
];

fn accent_color(skill_type: &str) -> &'static str {
    ACCENT_COLORS
        .iter()
        .find(|(t, _)| *t == skill_type)
        .map(|(_, c)| *c)
        .unwrap_or("#475569")
}

#[derive(Properties, Clone, PartialEq)]
pub struct SkillPillProps {
    pub skill_id: i64,
    pub name: String,
    pub level: i64,
    pub skill_type: String,
    #[prop_or(1)]
    pub rarity: i64,
    #[prop_or_default]
    pub on_click: Option<Callback<i64>>,
}

#[function_component]
pub fn SkillPill(props: &SkillPillProps) -> Html {
    let accent = accent_color(&props.skill_type);

    let style = format!(
        "--skill-accent: {};{}",
        accent,
        if props.on_click.is_some() {
            "cursor: pointer;"
        } else {
            ""
        }
    );

    let rare_border = if props.rarity >= 2 {
        Some("box-shadow: 0 0 0 1px #ca8a04, 0 0 6px rgba(202,138,4,0.3);")
    } else {
        None
    };

    let onclick = {
        let on_click = props.on_click.clone();
        let skill_id = props.skill_id;
        Callback::from(move |_| {
            if let Some(ref cb) = on_click {
                cb.emit(skill_id);
            }
        })
    };

    html! {
        <div
            class={SkillPillStyle::CLASS_NAME}
            style={format!("{}{}", style, rare_border.unwrap_or(""))}
            onclick={onclick}
        >
            <span class={SkillPillTypeBadgeStyle::CLASS_NAME}>
                { &props.skill_type }
            </span>
            <span class={SkillPillNameStyle::CLASS_NAME}>
                { &props.name }
            </span>
            <span class={SkillPillLevelStyle::CLASS_NAME}>
                { format!("Lv.{}", props.level) }
            </span>
            <span class={SkillPillIdStyle::CLASS_NAME}>
                { format!("#{}", props.skill_id) }
            </span>
        </div>
    }
}
