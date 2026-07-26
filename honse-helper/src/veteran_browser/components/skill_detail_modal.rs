use crate::styles::{
    detail_modal::{
        ModalBodyStyle, ModalCloseStyle, ModalContentStyle, ModalHeaderStyle, ModalOverlayStyle,
    },
    skill_pill::SkillPillTypeBadgeStyle,
    Style,
};
use crate::tauri_bridge::invoke_tauri_command;
use crate::veteran_browser::components::skill_pill::ACCENT_COLORS;
use serde_json::json;
use shared::{SkillDataRow, SkillType};
use yew::prelude::*;

fn fmt_cond(s: &Option<String>) -> String {
    s.as_deref()
        .unwrap_or("")
        .replace('&', " & ")
        .replace('@', " @ ")
}

fn fmt_value(raw: i64) -> String {
    let v = raw as f64 / 10000.0;
    if v == v.trunc() && v.abs() < 1000.0 {
        format!("{}", v as i64)
    } else {
        format!("{:.4}", v)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

fn fmt_time(raw: i64) -> String {
    if raw <= 0 {
        return "Infinite".to_string();
    }
    let secs = raw as f64 / 10000.0;
    if secs < 60.0 {
        format!("{:.1}s", secs)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    } else {
        let mins = secs / 60.0;
        format!("{:.1}m", mins)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

fn effect_label(ability_type: i64, raw_value: i64) -> String {
    let val = fmt_value(raw_value);
    let sign = if raw_value >= 0 { "+" } else { "" };
    let prefix = match ability_type {
        1 => "Speed",
        2 => "Stamina",
        3 => "Power",
        4 => "Guts",
        5 => "Wit",
        27 => "Target Speed",
        31 => "Acceleration",
        21 => "Speed",
        9 => "Energy",
        28 => "Position",
        8 => "Vision",
        _ => "",
    };
    let suffix = match ability_type {
        1..=5 => "Up",
        27 => "",
        31 => "",
        21 => {
            if raw_value < 0 {
                "Down"
            } else {
                "Up"
            }
        }
        9 => {
            if raw_value < 0 {
                "Down"
            } else {
                "Restore"
            }
        }
        28 => "Shift",
        8 => "",
        _ => "",
    };
    if prefix.is_empty() {
        format!("{}", val)
    } else {
        format!("{} {} ({}{})", prefix, suffix, sign, val)
    }
}

fn target_desc(target_type: i64, _target_value: i64) -> &'static str {
    match target_type {
        1 => "Self",
        4 => "Target in sight",
        9 => "Girls behind you",
        10 => "Closest girls",
        18 => "Nearby girls",
        20 => "Front runners",
        21 => "All runners",
        22 => "All girls",
        23 => "Rivals",
        _ => "Other",
    }
}

fn activation_desc(activate_lot: i64) -> &'static str {
    match activate_lot {
        0 => "Guaranteed",
        1 => "Wisdom check",
        _ => "Unknown",
    }
}

fn rarity_label(rarity: i64) -> &'static str {
    match rarity {
        1 => "Normal",
        2 => "Gold / Rare",
        3..=5 => "Unique",
        _ => "Unknown",
    }
}

fn fmt_cooldown(raw: i64) -> String {
    if raw <= 0 {
        return "None".to_string();
    }
    fmt_time(raw)
}

struct EffectSlot {
    ability_type: i64,
    value: i64,
    target_type: i64,
    target_value: i64,
}

fn collect_effects(sd: &SkillDataRow) -> Vec<EffectSlot> {
    let mut effects = Vec::new();
    if let Some(t) = sd.ability_type {
        if t != 0 {
            effects.push(EffectSlot {
                ability_type: t,
                value: sd.effect_value_1.unwrap_or(0),
                target_type: sd.target_type.unwrap_or(0),
                target_value: sd.target_value_1.unwrap_or(0),
            });
        }
    }
    if let Some(t) = sd.ability_type_2 {
        if t != 0 {
            effects.push(EffectSlot {
                ability_type: t,
                value: sd.effect_value_2.unwrap_or(0),
                target_type: sd.target_type_2.unwrap_or(0),
                target_value: sd.target_value_2.unwrap_or(0),
            });
        }
    }
    if let Some(t) = sd.ability_type_3 {
        if t != 0 {
            effects.push(EffectSlot {
                ability_type: t,
                value: sd.effect_value_3.unwrap_or(0),
                target_type: sd.target_type_3.unwrap_or(0),
                target_value: sd.target_value_3.unwrap_or(0),
            });
        }
    }
    effects
}

struct SkillDisplay<'a> {
    sd: &'a SkillDataRow,
    accent: &'static str,
    type_label: String,
    has_cond1: bool,
    has_cond2: bool,
    effects: Vec<EffectSlot>,
    effect_items: Vec<EffectItem>,
    activation: String,
    cost: String,
    duration: String,
    cooldown: String,
    rarity: String,
}

struct EffectItem {
    label: String,
    index: usize,
    target_desc: String,
    show_target: bool,
    is_multi: bool,
}

#[derive(Properties, Clone, PartialEq)]
pub struct SkillDetailModalProps {
    pub skill_id: i64,
    pub level: i64,
    pub on_close: Callback<()>,
}

fn accent_for(sd: &SkillDataRow) -> &'static str {
    let st = SkillType::from(sd);
    ACCENT_COLORS
        .iter()
        .find(|(t, _)| *t == st.label())
        .map(|(_, c)| *c)
        .unwrap_or("#475569")
}

fn type_label_for(sd: &SkillDataRow) -> String {
    SkillType::from(sd).label().to_string()
}

#[function_component]
pub fn SkillDetailModal(props: &SkillDetailModalProps) -> Html {
    let skill_data = use_state(|| Option::<SkillDataRow>::None);
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);

    {
        let skill_data = skill_data.clone();
        let loading = loading.clone();
        let error = error.clone();
        let skill_id = props.skill_id;

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match invoke_tauri_command("get_skill_detail", json!({ "skillId": skill_id })).await
                {
                    Ok(val) => match serde_json::from_value::<SkillDataRow>(val) {
                        Ok(row) => {
                            skill_data.set(Some(row));
                        }
                        Err(e) => {
                            error.set(Some(format!("Failed to parse: {e}")));
                        }
                    },
                    Err(e) => {
                        error.set(Some(e));
                    }
                }
                loading.set(false);
            });
            || ()
        });
    }

    let overlay_click = {
        let cb = props.on_close.clone();
        Callback::from(move |_| cb.emit(()))
    };
    let close_click = {
        let cb = props.on_close.clone();
        Callback::from(move |_| cb.emit(()))
    };

    let display = skill_data.as_ref().map(|sd| {
        let effects = collect_effects(sd);
        let is_multi = effects.len() > 1;
        let effect_items: Vec<EffectItem> = effects
            .iter()
            .enumerate()
            .map(|(i, e)| EffectItem {
                label: effect_label(e.ability_type, e.value),
                index: i + 1,
                target_desc: target_desc(e.target_type, e.target_value).to_string(),
                show_target: e.target_type != 0 && e.target_type != 1,
                is_multi,
            })
            .collect();
        SkillDisplay {
            effects,
            effect_items,
            activation: sd
                .activate_lot
                .map(|a| activation_desc(a).to_string())
                .unwrap_or_default(),
            cost: sd.skill_cost.map(|c| c.to_string()).unwrap_or_default(),
            duration: sd.effect_duration.map(|d| fmt_time(d)).unwrap_or_default(),
            cooldown: sd
                .effect_cooldown
                .map(|c| fmt_cooldown(c))
                .unwrap_or_default(),
            rarity: sd
                .rarity
                .map(|r| rarity_label(r).to_string())
                .unwrap_or_default(),
            accent: accent_for(sd),
            type_label: type_label_for(sd),
            has_cond1: fmt_cond(&sd.precondition1).is_empty() == false
                || fmt_cond(&sd.condition1).is_empty() == false,
            has_cond2: fmt_cond(&sd.precondition2).is_empty() == false
                || fmt_cond(&sd.condition2).is_empty() == false,
            sd,
        }
    });

    html! {
        <div class={ModalOverlayStyle::CLASS_NAME} onclick={overlay_click}>
            <div class={ModalContentStyle::CLASS_NAME} onclick={|e: yew::MouseEvent| e.stop_propagation()} style="max-width: 560px;">
                if *loading {
                    <div class={ModalHeaderStyle::CLASS_NAME}>
                        <h2>{"Skill Detail"}</h2>
                        <button class={ModalCloseStyle::CLASS_NAME} onclick={close_click.clone()}>{"\u{00D7}"}</button>
                    </div>
                    <div class={ModalBodyStyle::CLASS_NAME}>
                        <p>{"Loading..."}</p>
                    </div>
                } else if let Some(err) = error.as_ref() {
                    <div class={ModalHeaderStyle::CLASS_NAME}>
                        <h2>{"Skill Detail"}</h2>
                        <button class={ModalCloseStyle::CLASS_NAME} onclick={close_click.clone()}>{"\u{00D7}"}</button>
                    </div>
                    <div class={ModalBodyStyle::CLASS_NAME}>
                        <p style="color: #ef4444;">{ err }</p>
                    </div>
                } else if let Some(d) = display.as_ref() {
                    <div class={ModalHeaderStyle::CLASS_NAME}>
                        <h2>{ &d.sd.name }</h2>
                        <span class={SkillPillTypeBadgeStyle::CLASS_NAME}
                            style={format!("background: {}; color: #fff;", d.accent)}>
                            { &d.type_label }
                        </span>
                        <span style="color: #facc15; font-weight: 600; font-size: 13px; margin-left: 8px;">
                            { format!("Lv.{}", props.level) }
                        </span>
                        <button class={ModalCloseStyle::CLASS_NAME} onclick={close_click}>{"\u{00D7}"}</button>
                    </div>
                    <div class={ModalBodyStyle::CLASS_NAME}>
                        <div style="margin-bottom: 16px;">
                            <div style="font-size: 11px; font-weight: 700; text-transform: uppercase; color: #94a3b8; margin-bottom: 6px;">
                                {"Description"}
                            </div>
                            <div style="color: #e2e8f0; font-size: 13px; line-height: 1.5;">
                                { d.sd.description.as_deref().unwrap_or("No description available.") }
                            </div>
                        </div>

                        if d.has_cond1 {
                            <div style="margin-bottom: 12px;">
                                <div style="font-size: 11px; font-weight: 700; text-transform: uppercase; color: #94a3b8; margin-bottom: 6px;">
                                    {"Activation Condition"}
                                </div>
                                <div style="background: #1e293b; border-radius: 6px; padding: 10px 12px; font-size: 12px; font-family: monospace; color: #94a3b8; line-height: 1.6; word-break: break-word;">
                                    if let Some(pre) = &d.sd.precondition1 {
                                        if !pre.is_empty() {
                                            <div style="margin-bottom: 4px;">
                                                <span style="color: #64748b;">{"Precondition: "}</span>
                                                <span style="color: #cbd5e1;">{ fmt_cond(&d.sd.precondition1) }</span>
                                            </div>
                                        }
                                    }
                                    if let Some(cond) = &d.sd.condition1 {
                                        if !cond.is_empty() {
                                            <div>
                                                <span style="color: #64748b;">{"Condition: "}</span>
                                                <span style="color: #cbd5e1;">{ fmt_cond(&d.sd.condition1) }</span>
                                            </div>
                                        }
                                    }
                                </div>
                            </div>
                        }

                        if d.has_cond2 {
                            <div style="margin-bottom: 12px;">
                                <div style="font-size: 11px; font-weight: 700; text-transform: uppercase; color: #94a3b8; margin-bottom: 6px;">
                                    {"Activation Condition 2"}
                                </div>
                                <div style="background: #1e293b; border-radius: 6px; padding: 10px 12px; font-size: 12px; font-family: monospace; color: #94a3b8; line-height: 1.6; word-break: break-word;">
                                    if let Some(pre) = &d.sd.precondition2 {
                                        if !pre.is_empty() {
                                            <div style="margin-bottom: 4px;">
                                                <span style="color: #64748b;">{"Precondition: "}</span>
                                                <span style="color: #cbd5e1;">{ fmt_cond(&d.sd.precondition2) }</span>
                                            </div>
                                        }
                                    }
                                    if let Some(cond) = &d.sd.condition2 {
                                        if !cond.is_empty() {
                                            <div>
                                                <span style="color: #64748b;">{"Condition: "}</span>
                                                <span style="color: #cbd5e1;">{ fmt_cond(&d.sd.condition2) }</span>
                                            </div>
                                        }
                                    }
                                </div>
                            </div>
                        }

                        if !d.effects.is_empty() {
                            <div style="margin-bottom: 12px;">
                                <div style="font-size: 11px; font-weight: 700; text-transform: uppercase; color: #94a3b8; margin-bottom: 6px;">
                                    {"Effects"}
                                </div>
                                <div style="display: flex; flex-direction: column; gap: 6px;">
                                { for d.effect_items.iter().map(|item| {
                                    html! {
                                        <div style="background: #1e293b; border-radius: 6px; padding: 10px 12px; font-size: 13px; line-height: 1.5;">
                                            <div style="color: #e2e8f0; font-weight: 600;">
                                                { if item.is_multi { format!("Effect {}: ", item.index) } else { String::new() } }
                                                { &item.label }
                                            </div>
                                            if item.show_target {
                                                <div style="color: #64748b; font-size: 11px; margin-top: 2px;">
                                                    {"Target: "}{ &item.target_desc }
                                                </div>
                                            }
                                        </div>
                                    }
                                }) }
                                </div>
                            </div>
                        }

                        <div style="display: flex; gap: 24px; flex-wrap: wrap; margin-bottom: 12px;">
                            if !d.activation.is_empty() {
                                <div style="font-size: 12px;">
                                    <span style="color: #64748b;">{"Activation: "}</span>
                                    <span style="color: #cbd5e1;">{ &d.activation }</span>
                                </div>
                            }
                            if !d.cost.is_empty() {
                                <div style="font-size: 12px;">
                                    <span style="color: #64748b;">{"Base cost: "}</span>
                                    <span style="color: #cbd5e1;">{ &d.cost }</span>
                                </div>
                            }
                            if !d.duration.is_empty() {
                                <div style="font-size: 12px;">
                                    <span style="color: #64748b;">{"Duration: "}</span>
                                    <span style="color: #cbd5e1;">{ &d.duration }</span>
                                </div>
                            }
                            if !d.cooldown.is_empty() {
                                <div style="font-size: 12px;">
                                    <span style="color: #64748b;">{"Cooldown: "}</span>
                                    <span style="color: #cbd5e1;">{ &d.cooldown }</span>
                                </div>
                            }
                        </div>

                        <div style="margin-top: 12px; padding-top: 12px; border-top: 1px solid #1e293b;">
                            <div style="display: flex; gap: 20px; flex-wrap: wrap; font-size: 11px; color: #64748b;">
                                <div>
                                    <span style="color: #475569;">{"ID: "}</span>
                                    <span style="color: #94a3b8;">{ format!("#{}", d.sd.id) }</span>
                                </div>
                                if !d.rarity.is_empty() {
                                    <div>
                                        <span style="color: #475569;">{"Rarity: "}</span>
                                        <span style="color: #94a3b8;">{ &d.rarity }</span>
                                    </div>
                                }
                                if let Some(cat) = d.sd.skill_category {
                                    <div>
                                        <span style="color: #475569;">{"Category: "}</span>
                                        <span style="color: #94a3b8;">{ cat.to_string() }</span>
                                    </div>
                                }
                                if let Some(group) = d.sd.group_id {
                                    <div>
                                        <span style="color: #475569;">{"Group: "}</span>
                                        <span style="color: #94a3b8;">{ group.to_string() }</span>
                                    </div>
                                }
                            </div>
                        </div>
                    </div>
                }
            </div>
        </div>
    }
}
