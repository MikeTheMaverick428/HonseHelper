use crate::styles::detail_modal::*;
use crate::styles::skill_pill::*;
use crate::styles::support_card_browser::*;
use crate::styles::Style;
use crate::support_card_browser::components::support_card_card::{
    parse_card_name, rarity_class, rarity_label, type_class, type_label,
};
use crate::tauri_bridge::invoke_tauri_command;
use crate::veteran_browser::components::skill_detail_modal::SkillDetailModal;
use crate::veteran_browser::components::skill_pill::SkillPill;
use shared::{
    models::SupportCardRarity,
    support_card_browser::{
        SupportCardDetail, SupportCardEventBranch, SupportCardEventChoiceDetail,
        SupportCardEventDetail, SupportCardEventRewardDetail, SupportCardPageItem,
        SupportCardSkillDetail,
    },
    SupportCardEffectRow, SupportCardUniqueEffectDetail,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SupportCardDetailModalProps {
    pub card: SupportCardPageItem,
    pub on_close: Callback<()>,
}

#[function_component]
pub fn SupportCardDetailModal(props: &SupportCardDetailModalProps) -> Html {
    let effects = use_state(Vec::new);
    let unique_effect = use_state(|| None);
    let skill_hints = use_state(Vec::new);
    let events = use_state(Vec::new);
    let loading = use_state(|| true);
    let load_error = use_state(|| None::<String>);
    let active_tab = use_state(|| 0usize);
    let selected_skill = use_state(|| None::<(i64, i64)>);

    {
        let effects = effects.clone();
        let unique_effect = unique_effect.clone();
        let skill_hints = skill_hints.clone();
        let events = events.clone();
        let loading = loading.clone();
        let load_error = load_error.clone();
        let card_id = props.card.support_card_id;
        use_effect_with((), move |_| {
            let card_id = card_id;
            wasm_bindgen_futures::spawn_local(async move {
                match invoke_tauri_command(
                    "get_support_card_detail",
                    serde_json::json!({ "supportCardId": card_id }),
                )
                .await
                {
                    Ok(val) => match serde_json::from_value::<SupportCardDetail>(val) {
                        Ok(detail) => {
                            effects.set(detail.effects);
                            unique_effect.set(detail.unique_effect);
                            skill_hints.set(detail.skill_hints);
                            events.set(detail.events);
                        }
                        Err(e) => {
                            load_error.set(Some(format!("Deserialize: {}", e)));
                        }
                    },
                    Err(e) => {
                        load_error.set(Some(format!("Invoke: {}", e)));
                    }
                }
                loading.set(false);
            });
            || ()
        });
    }

    let on_close = {
        let cb = props.on_close.clone();
        Callback::from(move |_| cb.emit(()))
    };

    let tab_labels = ["Overview", "Effects", "Skills", "Events"];

    let on_tab = {
        let active_tab = active_tab.clone();
        Callback::from(move |idx: usize| active_tab.set(idx))
    };

    let card = &props.card;

    let on_skill_click = {
        let selected_skill = selected_skill.clone();
        Callback::from(move |(id, lvl): (i64, i64)| {
            selected_skill.set(Some((id, lvl)));
        })
    };

    html! {
        <div class={ModalOverlayStyle::CLASS_NAME} onclick={on_close.clone()}>
            <div class={ModalContentStyle::CLASS_NAME}
                 onclick={|e: yew::MouseEvent| e.stop_propagation()}>
                <div class={ModalHeaderStyle::CLASS_NAME}>
                    <div>
                        <span class={format!("{} {}", SupportCardRarityStyle::CLASS_NAME, rarity_class(card.rarity))}>
                            {rarity_label(card.rarity)}
                        </span>
                        <span style="margin-left: 8px; font-weight: 600; font-size: 1.1em;">
                            {&card.name}
                        </span>
                    </div>
                    <button class={ModalCloseStyle::CLASS_NAME}
                            onclick={on_close}>{"\u{00D7}"}</button>
                </div>
                <div class={ModalTabsStyle::CLASS_NAME}>
                    {for tab_labels.iter().enumerate().map(|(i, label)| {
                        let on_tab = on_tab.clone();
                        let active = *active_tab == i;
                        let class = format!("{} {}", TabBtnStyle::CLASS_NAME,
                            if active { TabActiveStyle::CLASS_NAME } else { "" });
                        html! {
                            <button class={class}
                                    onclick={move |_: MouseEvent| on_tab.emit(i)}>
                                {label}
                            </button>
                        }
                    })}
                </div>
                <div class={ModalBodyStyle::CLASS_NAME}>
                    if *loading {
                        <div style="text-align: center; padding: 40px; color: #94a3b8;">
                            {"Loading..."}
                        </div>
                    } else if let Some(err) = &*load_error {
                        <div style="text-align: center; padding: 40px; color: #ef4444;">
                            {err}
                        </div>
                    } else if *active_tab == 0 {
                        {render_overview(card, &*effects, &*unique_effect)}
                    } else if *active_tab == 1 {
                        {render_effects(&effects, props.card.rarity)}
                    } else if *active_tab == 2 {
                        {render_skills(&skill_hints, on_skill_click.clone())}
                    } else if *active_tab == 3 {
                        {render_events(&events)}
                    }
                </div>
            </div>
            { if let Some((skill_id, level)) = *selected_skill {
                let on_close = {
                    let selected_skill = selected_skill.clone();
                    Callback::from(move |_| selected_skill.set(None))
                };
                html! {
                    <SkillDetailModal
                        skill_id={skill_id}
                        level={level}
                        on_close={on_close}
                    />
                }
            } else {
                html! {}
            } }
        </div>
    }
}

fn render_overview(
    card: &SupportCardPageItem,
    effects: &[SupportCardEffectRow],
    unique: &Option<SupportCardUniqueEffectDetail>,
) -> Html {
    let type_cls = type_class(card.card_type);
    let rarity_cls = rarity_class(card.rarity);
    let is_mlb = card.limit_break_count >= 4;
    let (variant, character_name) = parse_card_name(&card.name);
    let lv = card.level;

    let current_effects: Vec<_> = effects
        .iter()
        .filter(|e| e.value_at_level(lv) >= 0)
        .collect();

    html! {
        <div class={DetailTabStyle::CLASS_NAME}>
            {if let Some(v) = &variant {
                html! { <div class={SupportCardVariantStyle::CLASS_NAME}>{v}</div> }
            } else {
                html! {}
            }}
            <div style="font-size: 16px; font-weight: 700; color: #f1f5f9; margin-bottom: 8px;">
                {character_name}
            </div>
            <div class={SupportCardBadgeRowStyle::CLASS_NAME} style="margin-bottom: 12px;">
                <span class={format!("{} {}", SupportCardRarityStyle::CLASS_NAME, rarity_cls)}>
                    {rarity_label(card.rarity)}
                </span>
                <span class={format!("{} {}", SupportCardTypeStyle::CLASS_NAME, type_cls)}>
                    {type_label(card.card_type)}
                </span>
                <span class={format!("{}{}", SupportCardLbStyle::CLASS_NAME, if is_mlb { " mlb" } else { "" })}>
                    {(0..4).map(|i| {
                        let on = i < card.limit_break_count as usize;
                        html! { <span class={format!("diamond{}", if on { " on" } else { "" })}></span> }
                    }).collect::<Html>()}
                </span>
                <span style="color: #9ca3af; font-size: 13px;">
                    {format!("Lv{}/{}", lv, card.max_level)}
                </span>
            </div>
            <div style="display: flex; gap: 16px; font-size: 13px; color: #94a3b8; margin-bottom: 20px;">
                <span>{"EXP: "}<span style="color: #e2e8f0;">{card.exp}</span></span>
                { if card.favorite_flag { html! { <span>{"\u{2605}"}{" Favorite"}</span> } } else { html! {} } }
                <span>{"Stock: "}<span style="color: #e2e8f0;">{card.stock}</span></span>
            </div>

            {render_unique_section(unique, card.level)}

            if !current_effects.is_empty() {
                <div style="margin-bottom: 20px;">
                    <h3 style="margin-bottom: 8px;">{"Current Effects"}</h3>
                    {for current_effects.iter().map(|e| {
                        let val = e.value_at_level(lv);
                        html! {
                            <div style="padding: 6px 12px; background: #1e293b; border-radius: 8px; margin-bottom: 4px; display: flex; justify-content: space-between; align-items: center;">
                                <span style="color: #cbd5e1; font-size: 13px;">{effect_label(e.effect_type)}</span>
                                <span style="color: #34d399; font-weight: 600; font-size: 14px;">{val}</span>
                            </div>
                        }
                    })}
                </div>
            }
        </div>
    }
}

fn render_unique_section(unique: &Option<SupportCardUniqueEffectDetail>, card_level: i64) -> Html {
    match unique {
        None => html! {
            <div class={UniqueSectionStyle::CLASS_NAME}>
                <h3 class={UniqueSectionTitleStyle::CLASS_NAME}>{"Unique Effect"}</h3>
                <div style="color: #64748b; font-size: 13px;">{"No unique effect data available."}</div>
            </div>
        },
        Some(ue) => {
            let meets_level = card_level >= ue.limit_break_level;
            html! {
                <div class={UniqueSectionStyle::CLASS_NAME}>
                    <h3 class={UniqueSectionTitleStyle::CLASS_NAME}>{"Unique Effect"}</h3>
                    <div class={UniqueNameStyle::CLASS_NAME}>{&ue.name}</div>
                    <div class={UniqueRequiredLevelStyle::CLASS_NAME}>
                        {"Required Level: "}
                        <span>{format!("Lv {}", ue.limit_break_level)}</span>
                    </div>
                    {for ue.entries.iter().map(|entry| {
                        let entry_cls = if meets_level {
                            UniqueEntryStyle::CLASS_NAME.to_string()
                        } else {
                            format!("{} disabled", UniqueEntryStyle::CLASS_NAME)
                        };
                        html! {
                            <div class={entry_cls}>
                                <span class={UniqueEntryLabelStyle::CLASS_NAME}>{&entry.effect_label}</span>
                                <span class={UniqueEntrySeparatorStyle::CLASS_NAME}>{" + "}</span>
                                <span class={UniqueEntryValueStyle::CLASS_NAME}>{entry.effect_value}</span>
                            </div>
                        }
                    })}
                </div>
            }
        }
    }
}

fn effect_label(effect_type: i64) -> &'static str {
    shared::models::SupportCardEffectType::from_raw(effect_type).label()
}

fn render_effects(effects: &[SupportCardEffectRow], rarity: i64) -> Html {
    if effects.is_empty() {
        return html! {
            <div class={DetailTabStyle::CLASS_NAME}>
                <h3>{"Effects"}</h3>
                <div style="color: #64748b;">{"No effect data available. Run a sync to import from master.mdb."}</div>
            </div>
        };
    }

    let max_lvl = SupportCardRarity::from_raw(rarity).max_level();

    const ALL_COLS: [(i64, &str); 11] = [
        (1, "Lv1"),
        (5, "Lv5"),
        (10, "Lv10"),
        (15, "Lv15"),
        (20, "Lv20"),
        (25, "Lv25"),
        (30, "Lv30"),
        (35, "Lv35"),
        (40, "Lv40"),
        (45, "Lv45"),
        (50, "Lv50"),
    ];

    let cols: Vec<&(i64, &str)> = ALL_COLS.iter().filter(|(lvl, _)| *lvl <= max_lvl).collect();
    let lb_start = cols.len().saturating_sub(4);

    let value_at = |effect: &SupportCardEffectRow, threshold: i64| -> i64 {
        effect.value_at_level(threshold)
    };

    let header_bg = |is_lb: bool| -> &'static str {
        if is_lb { " background: #1e293b;" } else { "" }
    };

    let cell_bg = |is_lb: bool| -> &'static str {
        if is_lb { " background: #0f172a;" } else { "" }
    };

    html! {
        <div class={DetailTabStyle::CLASS_NAME}>
            <div style="overflow-x: auto;">
                <table style="width: 100%; border-collapse: collapse; font-size: 0.85em;">
                    <thead>
                        <tr>
                            <th style="text-align: left; padding: 6px 8px; border-bottom: 1px solid #334155; color: #94a3b8;">{"Effect"}</th>
                            {for cols.iter().enumerate().map(|(col_idx, (_, label))| {
                                let is_lb = col_idx >= lb_start;
                                html! {
                                    <th style={format!("text-align: right; padding: 6px 4px; border-bottom: 1px solid #334155; color: #94a3b8; min-width: 40px;{}", header_bg(is_lb))}>
                                        {label}
                                    </th>
                                }
                            })}
                        </tr>
                    </thead>
                    <tbody>
                        {for effects.iter().map(|e| {
                            html! {
                                <tr>
                                    <td style="padding: 4px 8px; border-bottom: 1px solid #1e293b; color: #cbd5e1;">
                                        {effect_label(e.effect_type)}
                                    </td>
                                    {for cols.iter().enumerate().map(|(col_idx, (threshold, _))| {
                                        let v = value_at(e, *threshold);
                                        let is_lb = col_idx >= lb_start;
                                        let (text, color) = if v < 0 {
                                            ("\u{2013}".to_string(), "#475569".to_string())
                                        } else {
                                            (v.to_string(), "#34d399".to_string())
                                        };
                                        html! {
                                            <td style={format!("text-align: right; padding: 4px 4px; border-bottom: 1px solid #1e293b; color: {};{}", color, cell_bg(is_lb))}>
                                                {text}
                                            </td>
                                        }
                                    })}
                                </tr>
                            }
                        })}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

fn render_skills(hints: &[SupportCardSkillDetail], on_skill_click: Callback<(i64, i64)>) -> Html {
    if hints.is_empty() {
        return html! {
            <div class={DetailTabStyle::CLASS_NAME}>
                <div style="color: #64748b;">{"No skill data available."}</div>
            </div>
        };
    }

    let hint_skills: Vec<_> = hints.iter().filter(|h| h.source == "hint").collect();
    let chain_skills: Vec<_> = hints.iter().filter(|h| h.source == "chain_event").collect();
    let random_skills: Vec<_> = hints
        .iter()
        .filter(|h| h.source == "random_event")
        .collect();

    fn skill_section(
        title: &str,
        skills: &[&SupportCardSkillDetail],
        on_skill_click: &Callback<(i64, i64)>,
    ) -> Html {
        if skills.is_empty() {
            return html! {};
        }
        html! {
            <div style="margin-bottom: 16px;">
                <div style="font-weight: 600; color: #94a3b8; margin-bottom: 8px; font-size: 0.9em;">
                    {title}
                </div>
                <div class={SkillPillListStyle::CLASS_NAME}>
                    {for skills.iter().map(|s| {
                        let on_click = {
                            let cb = on_skill_click.clone();
                            let skill_id = s.skill_id;
                            let level = s.skill_level;
                            Callback::from(move |_| cb.emit((skill_id, level)))
                        };
                        html! {
                            <div>
                                <SkillPill
                                    skill_id={s.skill_id}
                                    name={s.skill_name.clone()}
                                    level={s.skill_level}
                                    skill_type={s.skill_type.clone()}
                                    rarity={s.rarity}
                                    on_click={on_click}
                                />
                                if !s.source_name.is_empty() {
                                    <div style="font-size: 9px; color: #64748b; margin-top: 2px; margin-left: 4px;">
                                        {&s.source_name}
                                    </div>
                                }
                            </div>
                        }
                    })}
                </div>
            </div>
        }
    }

    html! {
        <div class={DetailTabStyle::CLASS_NAME}>
            {skill_section("Hints", &hint_skills, &on_skill_click)}
            {skill_section("Chain Events", &chain_skills, &on_skill_click)}
            {skill_section("Random Events", &random_skills, &on_skill_click)}
        </div>
    }
}

fn render_reward_pill(r: &SupportCardEventRewardDetail) -> Html {
    let color = if r.negative { "#ef4444" } else { "#34d399" };
    let sign = if r.negative { "-" } else { "+" };
    let size_text: Option<String> = r.size.map(|s| s.to_string());
    let alt_text: Option<String> = r.alternatives.as_ref().map(|a| {
        a.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join("/")
    });
    if let Some(ref skill_name) = r.skill_name {
        let skill_lv = match (&size_text, &alt_text) {
            (_, Some(alt)) => format!("Lv{}", alt),
            (Some(s), None) => format!("Lv{}", s),
            (None, None) => "???".to_string(),
        };
        html! {
            <span style={format!("font-size: 0.8em; padding: 2px 6px; border-radius: 4px; background: #8b5cf615; color: #a78bfa;")}>
                {format!("{} {}", skill_name, skill_lv)}
            </span>
        }
    } else {
        let label = match (&size_text, &alt_text) {
            (_, Some(alt)) => format!("{}{} {}", sign, alt, r.reward_label),
            (Some(s), None) if !s.is_empty() && s != "0" => {
                format!("{}{} {}", sign, s, r.reward_label)
            }
            _ => r.reward_label.clone(),
        };
        html! {
            <span style={format!("font-size: 0.8em; padding: 2px 6px; border-radius: 4px; background: {}15; color: {};", color, color)}>
                {label}
            </span>
        }
    }
}

fn render_branch(c: &SupportCardEventChoiceDetail, b: &SupportCardEventBranch) -> Html {
    let multi_branch = c.branches.len() > 1;
    html! {
        <div style={if multi_branch { "margin-bottom: 6px;" } else { "" }}>
            {if multi_branch {
                let label = b.probability.as_deref().unwrap_or("Random");
                html! { <div style="font-size: 0.75em; color: #fbbf24; margin-bottom: 2px;">{format!("{}:", label)}</div> }
            } else {
                html! {}
            }}
            <div style="display: flex; flex-wrap: wrap; gap: 4px;">
                {for b.rewards.iter().map(|r| render_reward_pill(r))}
            </div>
        </div>
    }
}

fn render_choice(c: &SupportCardEventChoiceDetail) -> Html {
    html! {
        <div style="margin-left: 16px; margin-bottom: 6px;">
            <div style="font-size: 0.85em; color: #94a3b8; margin-bottom: 4px;">
                {format!("Choice {}", c.choice_index + 1)}
            </div>
            {for c.branches.iter().map(|b| render_branch(c, b))}
        </div>
    }
}

fn render_event_card(e: &SupportCardEventDetail) -> Html {
    let is_chain = e.category == "arrows";
    let kind = if is_chain { "Chain" } else { "Random" };
    let kind_color = if is_chain { "#60a5fa" } else { "#a78bfa" };
    html! {
        <div style="margin-bottom: 12px; padding: 10px; background: #0f172a; border-radius: 8px; border: 1px solid #1e293b;">
            <div style="display: flex; align-items: center; margin-bottom: 8px;">
                <span style="font-weight: 600; flex: 1;">{&e.event_name}</span>
                <span style={format!("font-size: 0.75em; padding: 2px 8px; border-radius: 4px; background: {}22; color: {};", kind_color, kind_color)}>
                    {kind}
                </span>
            </div>
            {for e.choices.iter().map(|c| render_choice(c))}
        </div>
    }
}

fn render_events(events: &[SupportCardEventDetail]) -> Html {
    if events.is_empty() {
        return html! {
            <div class={DetailTabStyle::CLASS_NAME}>
                <div style="color: #64748b;">{"No event data available. Import supplementary data first."}</div>
            </div>
        };
    }

    html! {
        <div class={DetailTabStyle::CLASS_NAME}>
            <div class={SupportCardListStyle::CLASS_NAME}>
                {for events.iter().map(|e| render_event_card(e))}
            </div>
        </div>
    }
}
