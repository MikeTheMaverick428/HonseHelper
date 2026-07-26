use crate::styles::detail_modal::*;
use crate::styles::trainee_browser::*;
use crate::styles::veteran_card::VeteranVariantStyle;
use crate::styles::Style;
use crate::veteran_browser::components::skill_detail_modal::SkillDetailModal;
use crate::veteran_browser::components::skill_pill::SkillPill;
use crate::veteran_browser::components::veteran_card::parse_veteran_name;
use shared::trainee_browser::{TraineeDetail, TraineeSkillDetail};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TraineeDetailModalProps {
    pub trainee: Option<TraineeDetail>,
    pub loading: bool,
    pub on_close: Callback<()>,
}

#[derive(PartialEq, Clone, Copy)]
enum Tab {
    Stats,
    Skills,
    Events,
}

fn aptitude_level(val: i64) -> &'static str {
    match val {
        8 => "S",
        7 => "A",
        6 => "B",
        5 => "C",
        4 => "D",
        3 => "E",
        2 => "F",
        1 => "G",
        _ => "H",
    }
}

fn aptitude_color(val: i64) -> &'static str {
    match val {
        8 => "#7c3aed",
        7 => "#f59e0b",
        6 => "#3b82f6",
        5 => "#22c55e",
        4 => "#a3e635",
        3 => "#fb923c",
        2 => "#fca5a5",
        _ => "#e2e8f0",
    }
}

fn aptitude_weight(val: i64) -> &'static str {
    match val {
        8 => "800",
        _ => "700",
    }
}

fn growth_pct(val: i64) -> String {
    if val > 0 {
        format!("+{}%", val)
    } else {
        format!("{}%", val)
    }
}

fn render_aptitude(label: &str, val: i64) -> Html {
    let color = aptitude_color(val);
    let weight = aptitude_weight(val);
    html! {
        <div class={TrDetailAptCardStyle::CLASS_NAME} style={format!("border-top: 3px solid {};", color)}>
            <div class={TrDetailAptCardLabelStyle::CLASS_NAME}>{label}</div>
            <div class={TrDetailAptCardValueStyle::CLASS_NAME} style={format!("font-weight: {}; color: {};", weight, color)}>{ aptitude_level(val) }</div>
        </div>
    }
}

fn render_stat(label: &str, color: &str, val: i64) -> Html {
    html! {
        <div class={TrDetailStatCardStyle::CLASS_NAME} style={format!("border-top: 3px solid {};", color)}>
            <div class={TrDetailStatCardLabelStyle::CLASS_NAME}>{label}</div>
            <div class={TrDetailStatCardValueStyle::CLASS_NAME}>{ val }</div>
        </div>
    }
}

fn render_growth(label: &str, color: &str, val: i64) -> Html {
    html! {
        <div class={TrDetailGrowthCardStyle::CLASS_NAME} style={format!("border: 1px solid {};", color)}>
            <div class={TrDetailGrowthCardLabelStyle::CLASS_NAME}>{label}</div>
            <div class={TrDetailGrowthCardValueStyle::CLASS_NAME} style={format!("color: {};", color)}>{ growth_pct(val) }</div>
        </div>
    }
}

fn category_badge(cat: &str) -> (&'static str, &'static str) {
    match cat {
        "wchoice" => ("wchoice", "#60a5fa"),
        "version" => ("version", "#818cf8"),
        "nochoice" => ("nochoice", "#a78bfa"),
        "outings" => ("outing", "#f472b6"),
        "secret" => ("secret", "#fb923c"),
        "arrows" => ("Chain", "#60a5fa"),
        "random" => ("Random", "#a78bfa"),
        _ => ("?", "#94a3b8"),
    }
}

fn render_skill_pills(
    skills: &[&TraineeSkillDetail],
    on_skill_click: &Callback<(i64, i64)>,
) -> Html {
    html! {
        <div class={TrDetailSkillListStyle::CLASS_NAME}>
            {for skills.iter().map(|s| {
                let sid = s.skill_id;
                let lvl = s.level;
                let cb = on_skill_click.clone();
                let on_click = Callback::from(move |_| cb.emit((sid, lvl)));
                html! {
                    <div>
                        <SkillPill
                            skill_id={s.skill_id}
                            name={s.name.clone()}
                            level={s.level}
                            skill_type={s.skill_type.clone()}
                            rarity={s.rarity}
                            on_click={Some(on_click)}
                        />
                        if !s.source_name.is_empty() {
                            <div class={TrDetailSkillSourceStyle::CLASS_NAME}>
                                {&s.source_name}
                            </div>
                        }
                    </div>
                }
            })}
        </div>
    }
}

fn render_skills(
    trainee: &TraineeDetail,
    on_skill_click: &Callback<(i64, i64)>,
) -> Html {
    if trainee.skills.is_empty() {
        return html! {
            <div class={TrDetailEmptyStyle::CLASS_NAME}>{"No skill data available."}</div>
        };
    }

    let unique_skills: Vec<&TraineeSkillDetail> =
        trainee.skills.iter().filter(|s| s.source == "unique").collect();
    let base_skills: Vec<&TraineeSkillDetail> =
        trainee.skills.iter().filter(|s| s.source == "base").collect();
    let potential_skills: Vec<&TraineeSkillDetail> =
        trainee.skills.iter().filter(|s| s.source == "potential").collect();
    let unlocked: Vec<&TraineeSkillDetail> =
        potential_skills.iter().filter(|s| s.unlocked).copied().collect();
    let locked: Vec<&TraineeSkillDetail> =
        potential_skills.iter().filter(|s| !s.unlocked).copied().collect();
    let event_skills: Vec<&TraineeSkillDetail> =
        trainee.skills.iter().filter(|s| s.source == "event").collect();

    html! {
        <div>
            if !unique_skills.is_empty() {
                <div class={TrDetailSkillSectionStyle::CLASS_NAME}>
                    <div class={TrDetailSkillSectionLabelStyle::CLASS_NAME} style="color: #facc15;">
                        {format!("Unique ({})", unique_skills.len())}
                    </div>
                    {render_skill_pills(&unique_skills, on_skill_click)}
                </div>
            }

            if !base_skills.is_empty() {
                <div class={TrDetailSkillSectionStyle::CLASS_NAME}>
                    <div class={TrDetailSkillSectionLabelStyle::CLASS_NAME} style="color: #22c55e;">
                        {format!("Base Skills ({})", base_skills.len())}
                    </div>
                    {render_skill_pills(&base_skills, on_skill_click)}
                </div>
            }

            if !potential_skills.is_empty() {
                <div class={TrDetailSkillSectionStyle::CLASS_NAME}>
                    <div class={TrDetailSkillSectionLabelStyle::CLASS_NAME} style="color: #f59e0b;">
                        {format!("Potential Skills ({})", potential_skills.len())}
                    </div>
                    if !unlocked.is_empty() {
                        <div class={TrDetailSkillUnlockedLabelStyle::CLASS_NAME}>
                            {format!("Unlocked (potential lvl {})", trainee.talent_level)}
                        </div>
                        {render_skill_pills(&unlocked, on_skill_click)}
                    }
                    if !locked.is_empty() {
                        <div class={TrDetailSkillLockedLabelStyle::CLASS_NAME}>
                            {"Locked (needs higher potential level)"}
                        </div>
                        <div class={TrDetailSkillLockedContainerStyle::CLASS_NAME}>
                            {for locked.iter().map(|s| {
                                let sid = s.skill_id;
                                let lvl = s.level;
                                let cb = on_skill_click.clone();
                                let on_click = Callback::from(move |_| cb.emit((sid, lvl)));
                                html! {
                                    <div style="position: relative;">
                                        <SkillPill skill_id={s.skill_id} name={s.name.clone()} level={lvl} skill_type={s.skill_type.clone()} rarity={s.rarity} on_click={Some(on_click)} />
                                        <span class={TrDetailSkillLockedBadgeStyle::CLASS_NAME}>
                                            {format!("lvl {}", s.need_rank)}
                                        </span>
                                    </div>
                                }
                            })}
                        </div>
                    }
                </div>
            }

            if !event_skills.is_empty() {
                <div class={TrDetailSkillSectionStyle::CLASS_NAME}>
                    <div class={TrDetailSkillSectionLabelStyle::CLASS_NAME} style="color: #a78bfa;">
                        {format!("Event Skills ({})", event_skills.len())}
                    </div>
                    <div class={TrDetailSkillListStyle::CLASS_NAME}>
                        {for event_skills.iter().map(|s| {
                            let sid = s.skill_id;
                            let lvl = s.level;
                            let cb = on_skill_click.clone();
                            let on_click = Callback::from(move |_| cb.emit((sid, lvl)));
                            html! {
                                <div>
                                    <SkillPill skill_id={s.skill_id} name={s.name.clone()} level={s.level} skill_type={s.skill_type.clone()} rarity={s.rarity} on_click={Some(on_click)} />
                                    if !s.source_name.is_empty() {
                                        <div class={TrDetailSkillSourceStyle::CLASS_NAME}>
                                            {&s.source_name}
                                        </div>
                                    }
                                </div>
                            }
                        })}
                    </div>
                </div>
            }
        </div>
    }
}

fn render_events(trainee: &TraineeDetail) -> Html {
    if trainee.events.is_empty() {
        return html! {
            <div class={TrDetailEmptyStyle::CLASS_NAME}>{"No event data available."}</div>
        };
    }

    let mut by_category: std::collections::BTreeMap<
        &str,
        Vec<&shared::trainee_browser::TraineeEventDetail>,
    > = std::collections::BTreeMap::new();
    for e in &trainee.events {
        by_category.entry(e.category.as_str()).or_default().push(e);
    }

    let category_order = ["secret", "wchoice", "version", "nochoice", "outings"];
    let mut sorted_categories: Vec<&&str> = category_order
        .iter()
        .filter(|c| by_category.contains_key(**c))
        .collect();
    for cat in by_category.keys() {
        if !category_order.contains(cat) {
            sorted_categories.push(cat);
        }
    }

    html! {
        <div>
            {for sorted_categories.iter().map(|cat| {
                let events = &by_category[*cat];
                let (label, color) = category_badge(cat);

                let header = match **cat {
                    "wchoice" => "With Choice",
                    "nochoice" => "No Choice",
                    "outings" => "Outings",
                    "secret" => "Secret",
                    "version" => "Version",
                    _ => label,
                };

                html! {
                    <div class={TrDetailEventSectionStyle::CLASS_NAME}>
                        <div class={TrDetailEventSectionHeaderStyle::CLASS_NAME} style={format!("color: {}; border-bottom: 1px solid {}33;", color, color)}>
                            {format!("{} ({} events)", header, events.len())}
                        </div>
                        {for events.iter().map(|e| {
                            let cond_text = e.conditions_display.as_ref();
                            html! {
                                <div class={TrDetailEventCardStyle::CLASS_NAME}>
                                    <div class={TrDetailEventNameContainerStyle::CLASS_NAME}>
                                        <div class={TrDetailEventNameStyle::CLASS_NAME}>{&e.event_name}</div>
                                        {if let Some(ref cond) = cond_text {
                                            html! {
                                                <div class={TrDetailEventConditionStyle::CLASS_NAME}>
                                                    {cond}
                                                </div>
                                            }
                                        } else {
                                            html! {}
                                        }}
                                    </div>
                                    {for e.choices.iter().map(|c| {
                                        let show_header = e.choices.len() > 1;
                                        html! {
                                            <div class={TrDetailChoiceWrapperStyle::CLASS_NAME}>
                                                {if show_header {
                                                    html! { <div class={TrDetailChoiceHeaderStyle::CLASS_NAME}>{format!("Choice {}", c.choice_index + 1)}</div> }
                                                } else {
                                                    html! {}
                                                }}
                                                {for c.branches.iter().map(|b| {
                                                    let multi_branch = c.branches.len() > 1;
                                                    html! {
                                                        <div class={if multi_branch { TrDetailBranchWrapperStyle::CLASS_NAME } else { "" }}>
                                                            {if multi_branch {
                                                                let label = b.probability.as_deref().unwrap_or("Random");
                                                                html! {
                                                                    <div class={TrDetailProbLabelStyle::CLASS_NAME}>
                                                                        {format!("{}:", label)}
                                                                    </div>
                                                                }
                                                            } else {
                                                                html! {}
                                                            }}
                                                            <div class={TrDetailRewardListStyle::CLASS_NAME}>
                                                                {for b.rewards.iter().map(|r| {
                                                                    let color = if r.negative { "#ef4444" } else { "#34d399" };
                                                                    let sign = if r.negative { "-" } else { "+" };
                                                                    let size_text: Option<String> = r.size.map(|s| s.to_string());
                                                                    let alt_text: Option<String> = r.alternatives.as_ref().map(|a| {
                                                                        a.iter().map(|v| v.to_string()).collect::<Vec<_>>().join("/")
                                                                    });
                                                                    if let Some(ref skill_name) = r.skill_name {
                                                                        let skill_lv = match (&size_text, &alt_text) {
                                                                            (_, Some(alt)) => format!("Lv{}", alt),
                                                                            (Some(s), None) => format!("Lv{}", s),
                                                                            (None, None) => "???".to_string(),
                                                                        };
                                                                        html! {
                                                                            <span class={TrDetailRewardPillStyle::CLASS_NAME} style="background: #8b5cf615; color: #a78bfa;">
                                                                                {format!("{} {}", skill_name, skill_lv)}
                                                                            </span>
                                                                        }
                                                                    } else {
                                                                        let label = match (&size_text, &alt_text) {
                                                                            (_, Some(alt)) => format!("{}{} {}", sign, alt, r.reward_label),
                                                                            (Some(s), None) if !s.is_empty() && s != "0" => format!("{}{} {}", sign, s, r.reward_label),
                                                                            _ => r.reward_label.clone(),
                                                                        };
                                                                        html! {
                                                                            <span class={TrDetailRewardPillStyle::CLASS_NAME} style={format!("background: {}15; color: {};", color, color)}>
                                                                                {label}
                                                                            </span>
                                                                        }
                                                                    }
                                                                })}
                                                            </div>
                                                        </div>
                                                    }
                                                })}
                                            </div>
                                        }
                                    })}
                                </div>
                            }
                        })}
                    </div>
                }
            })}
        </div>
    }
}

#[function_component]
pub fn TraineeDetailModal(props: &TraineeDetailModalProps) -> Html {
    let on_close = {
        let cb = props.on_close.clone();
        Callback::from(move |_| cb.emit(()))
    };

    let tab = use_state(|| Tab::Stats);
    let selected_skill = use_state(|| Option::<(i64, i64)>::None);

    let on_tab_click = {
        let tab = tab.clone();
        Callback::from(move |t: Tab| tab.set(t))
    };

    let on_skill_click = {
        let selected_skill = selected_skill.clone();
        Callback::from(move |(sid, lvl): (i64, i64)| selected_skill.set(Some((sid, lvl))))
    };

    let on_skill_close = {
        let selected_skill = selected_skill.clone();
        Callback::from(move |_: ()| selected_skill.set(None))
    };

    let Some(d) = &props.trainee else {
        return html! {};
    };

    let (variant, _) = parse_veteran_name(&d.name);

    html! {
        <>
        <div class={ModalOverlayStyle::CLASS_NAME} onclick={on_close.clone()}>
            <div class={ModalContentStyle::CLASS_NAME} onclick={|e: yew::MouseEvent| e.stop_propagation()}>
                <div class={ModalHeaderStyle::CLASS_NAME}>
                    <div>
                        {if let Some(v) = &variant {
                            html! { <div class={VeteranVariantStyle::CLASS_NAME}>{v}</div> }
                        } else { html! {} }}
                        <h2>{&d.character_name}</h2>
                    </div>
                    <button class={ModalCloseStyle::CLASS_NAME} onclick={on_close}>{"\u{00D7}"}</button>
                </div>

                <div class={ModalTabsStyle::CLASS_NAME}>
                    <button class={classes!(TabBtnStyle::CLASS_NAME, (*tab == Tab::Stats).then_some(TabActiveStyle::CLASS_NAME))}
                        onclick={let t = on_tab_click.clone(); Callback::from(move |_| t.emit(Tab::Stats))}>
                        {"Stats"}
                    </button>
                    <button class={classes!(TabBtnStyle::CLASS_NAME, (*tab == Tab::Skills).then_some(TabActiveStyle::CLASS_NAME))}
                        onclick={let t = on_tab_click.clone(); Callback::from(move |_| t.emit(Tab::Skills))}>
                        {"Skills"}
                    </button>
                    <button class={classes!(TabBtnStyle::CLASS_NAME, (*tab == Tab::Events).then_some(TabActiveStyle::CLASS_NAME))}
                        onclick={let t = on_tab_click.clone(); Callback::from(move |_| t.emit(Tab::Events))}>
                        {"Events"}
                    </button>
                </div>

                <div class={ModalBodyStyle::CLASS_NAME}>
                    if props.loading {
                        <p>{"Loading..."}</p>
                    } else {
                        { match *tab {
                            Tab::Stats => html! {
                                <div class={DetailTabStyle::CLASS_NAME}>
                                    <div class={TrDetailSectionLabelStyle::CLASS_NAME}>
                                        {"Stats"}
                                    </div>
                                    <div class={TrDetailStatsGridStyle::CLASS_NAME}>
                                        {render_stat("SPD", "#06b6d4", d.stat_spe)}
                                        {render_stat("STA", "#f59e0b", d.stat_sta)}
                                        {render_stat("PWR", "#ef4444", d.stat_pwr)}
                                        {render_stat("GUT", "#f97316", d.stat_gut)}
                                        {render_stat("WIT", "#22c55e", d.stat_wit)}
                                    </div>

                                    <div class={TrDetailSectionLabelStyle::CLASS_NAME}>
                                        {"Growth Rates"}
                                    </div>
                                    <div class={TrDetailStatsGridStyle::CLASS_NAME}>
                                        {render_growth("SPD", if d.growth_spe > 0 { "#06b6d4" } else { "#86898a" }, d.growth_spe)}
                                        {render_growth("STA", if d.growth_sta > 0 { "#f59e0b" } else { "#86898a" }, d.growth_sta)}
                                        {render_growth("PWR", if d.growth_str > 0 { "#ef4444" } else { "#86898a" }, d.growth_str)}
                                        {render_growth("GUT", if d.growth_gut > 0 { "#f97316" } else { "#86898a" }, d.growth_gut)}
                                        {render_growth("WIT", if d.growth_wit > 0 { "#22c55e" } else { "#86898a" }, d.growth_wit)}
                                    </div>

                                    <div class={TrDetailAptSectionStyle::CLASS_NAME}>
                                        <div class={TrDetailSectionLabelStyle::CLASS_NAME}>{"Ground"}</div>
                                        <div class={TrDetailAptGridStyle::CLASS_NAME}>
                                            {render_aptitude("Turf", d.aptitude_turf)}
                                            {render_aptitude("Dirt", d.aptitude_dirt)}
                                        </div>
                                    </div>

                                    <div class={TrDetailAptSectionStyle::CLASS_NAME}>
                                        <div class={TrDetailSectionLabelStyle::CLASS_NAME}>{"Distance"}</div>
                                        <div class={TrDetailAptGridStyle::CLASS_NAME}>
                                            {render_aptitude("Sprint", d.aptitude_sprint)}
                                            {render_aptitude("Mile", d.aptitude_mile)}
                                            {render_aptitude("Medium", d.aptitude_medium)}
                                            {render_aptitude("Long", d.aptitude_long)}
                                        </div>
                                    </div>

                                    <div class={TrDetailAptSectionStyle::CLASS_NAME}>
                                        <div class={TrDetailSectionLabelStyle::CLASS_NAME}>{"Running Style"}</div>
                                        <div class={TrDetailAptGridStyle::CLASS_NAME}>
                                            {render_aptitude("Front", d.aptitude_front)}
                                            {render_aptitude("Pace Chaser", d.aptitude_pace_chaser)}
                                            {render_aptitude("Late Surger", d.aptitude_late_surger)}
                                            {render_aptitude("End Closer", d.aptitude_end_closer)}
                                        </div>
                                    </div>
                                </div>
                            },
                            Tab::Skills => html! {
                                <div class={DetailTabStyle::CLASS_NAME}>
                                    {render_skills(d, &on_skill_click)}
                                </div>
                            },
                            Tab::Events => html! {
                                <div class={DetailTabStyle::CLASS_NAME}>
                                    {render_events(d)}
                                </div>
                            },
                        } }
                    }
                </div>
            </div>
        </div>
        if let Some((skill_id, level)) = *selected_skill {
            <SkillDetailModal skill_id={skill_id} level={level} on_close={on_skill_close} />
        }
        </>
    }
}
