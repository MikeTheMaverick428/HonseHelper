use crate::{
    components::{copyable::{HashBadge, OwnerBadge}, parse_variant_name},
    styles::{
        Style, detail_modal::{
            SupportCardBadgeRowStyle, SupportCardLbStyle, SupportCardRarityStyle,
            SupportCardTypeStyle,
        }, legacy_planner::{AffinityBaseStyle, AffinityBonusStyle, AffinityPlusStyle},
        trainer_browser::{
            TrainerBorrowBlockStyle, TrainerFollowingBadgeStyle, TrainerMetaStyle,
            TrainerNameStyle, TrainerRowStyle, TrainerSubCardHeaderStyle, TrainerSubCardStyle,
        }, veteran_card::{
            CardAffinityStyle, CardRankStyle, CardSparksStyle, RankScoreStyle, VeteranVariantStyle,
        },
    },
    support_card_browser::components::support_card_card::{
        rarity_class, rarity_label, type_class, type_label,
    },
    veteran_browser::components::{rank_badge::RankBadge, spark_item::SparkItem},
};
use shared::{
    models::UmaRank,
    trainer_browser::{max_level_for_limit_break, BorrowSupportCardInfo, TrainerPageItem},
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TrainerCardProps {
    pub trainer: TrainerPageItem,
    pub on_view_veteran: Callback<i64>,
    pub on_view_support_card: Callback<BorrowSupportCardInfo>,
    #[prop_or(false)]
    pub api_key_configured: bool,
    #[prop_or_default]
    pub refreshing_ids: Vec<i64>,
    #[prop_or(Callback::noop())]
    pub on_refresh_trainer: Callback<i64>,
}

fn format_followers(n: i64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 10_000 {
        format!("{:.1}k", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

fn update_source_label(source: &str) -> &'static str {
    match source {
        "game" | "followed" => "Follow list",
        "uma_moe" => "uma.moe",
        _ => "Unknown",
    }
}

fn format_rank(score: i64) -> String {
    if score >= 100_000_000 {
        format!("UG+{:.1}", (score as f64 - 100_000_000.0) / 10_000_000.0)
    } else if score >= 50_000_000 {
        format!("UF+{:.1}", (score as f64 - 50_000_000.0) / 10_000_000.0)
    } else if score >= 25_000_000 {
        format!("UE+{:.1}", (score as f64 - 25_000_000.0) / 10_000_000.0)
    } else if score >= 12_000_000 {
        format!("UD+{:.1}", (score as f64 - 12_000_000.0) / 10_000_000.0)
    } else if score >= 6_000_000 {
        format!("UC+{:.1}", (score as f64 - 6_000_000.0) / 10_000_000.0)
    } else if score >= 3_000_000 {
        format!("UB+{:.1}", (score as f64 - 3_000_000.0) / 10_000_000.0)
    } else if score >= 1_500_000 {
        format!("UA+{:.1}", (score as f64 - 1_500_000.0) / 10_000_000.0)
    } else {
        format!("{}", score)
    }
}

#[derive(Clone, Copy, PartialEq)]
enum ChipSource {
    Game,
    UmaMoe,
    Shared,
}

fn meta_chip(label: &str, value: impl ToString, source: ChipSource) -> Html {
    let (label_color, dot_color, title) = match source {
        ChipSource::Game => ("#34d399", "#34d399", "Only available from the game follow list"),
        ChipSource::UmaMoe => ("#60a5fa", "#60a5fa", "Only available from uma.moe API"),
        ChipSource::Shared => ("#64748b", "transparent", "Available from both sources"),
    };
    html! {
        <span title={title} style="display:inline-flex;align-items:center;gap:5px;background:#1e293b;padding:1px 8px;border-radius:999px;white-space:nowrap;font-size:11px;line-height:1.4;">
            <span style={format!("width:5px;height:5px;border-radius:50%;background:{};", dot_color)}></span>
            <span style={format!("color:{};font-size:10px;text-transform:uppercase;letter-spacing:0.03em;", label_color)}>{label}</span>
            <span>{value.to_string()}</span>
        </span>
    }
}

#[function_component]
pub fn TrainerCard(props: &TrainerCardProps) -> Html {
    let t = &props.trainer;
    let hash_copied = use_state(|| false);

    let on_view = {
        let on_view_veteran = props.on_view_veteran.clone();
        match &t.borrow_veteran {
            Some(v) => {
                let hash = v.hash.as_i64();
                Some(Callback::from(move |_: MouseEvent| {
                    on_view_veteran.emit(hash);
                }))
            }
            None => None,
        }
    };

    let on_view_sc = match &t.borrow_support_card {
        Some(sc) => {
            let sc = sc.clone();
            let cb = props.on_view_support_card.clone();
            Some(Callback::from(move |_: MouseEvent| {
                cb.emit(sc.clone());
            }))
        }
        None => None,
    };

    let (uma_variant, uma_character) = {
        let (v, c) = parse_variant_name(&t.borrow_uma_trainee_name);
        let character = if c.is_empty() { "Unknown" } else { c };
        (v.map(String::from), character.to_string())
    };

    let (sc_variant, sc_character) = match &t.borrow_support_card {
        Some(sc) if !sc.name.is_empty() => {
            let (v, c) = parse_variant_name(&sc.name);
            (v.map(String::from), c.to_string())
        }
        _ => (None, String::new()),
    };

    let refreshing = props.refreshing_ids.contains(&t.trainer_id);
    let on_refresh = {
        let cb = props.on_refresh_trainer.clone();
        let trainer_id = t.trainer_id;
        Callback::from(move |_: MouseEvent| cb.emit(trainer_id))
    };
    let recheck_date = t.last_recheck_at.as_deref().filter(|s| !s.is_empty()).map(|s| {
        if s.len() >= 10 { &s[..10] } else { s }
    });
    let updated_date = t.updated_at.as_deref().filter(|s| !s.is_empty()).map(|s| {
        if s.len() >= 10 { &s[..10] } else { s }
    });
    let update_source = t
        .last_update_source
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(update_source_label);

    html! {
        <div class={TrainerRowStyle::CLASS_NAME}>
            <div style="flex:1;min-width:180px;">
                <div style="display:flex;align-items:center;gap:8px;">
                    <span class={TrainerNameStyle::CLASS_NAME}>{&t.name}</span>
                    if t.is_following {
                        <span class={classes!(TrainerFollowingBadgeStyle::CLASS_NAME, "following")}>{"Following"}</span>
                    } else {
                        <span class={TrainerFollowingBadgeStyle::CLASS_NAME}>{"Added"}</span>
                    }
                    if props.api_key_configured {
                        <button
                            onclick={on_refresh.clone()}
                            disabled={refreshing}
                            title="Refresh trainer data from uma.moe"
                            style="cursor:pointer;padding:2px 7px;border:1px solid #334155;border-radius:999px;background:transparent;color:#93c5fd;font-size:12px;line-height:1.2;"
                        >
                            { if refreshing { "…" } else { "↻" } }
                        </button>
                    }
                </div>
                <div style="height:1px;background:#1f2937;margin:6px 0 8px 0;"></div>
                <div style="display:flex;align-items:center;gap:8px;flex-wrap:wrap;">
                    <OwnerBadge owner_id={t.trainer_id as u64} label={"Trainer".to_string()} title="Click to copy trainer ID" />
                    if t.fan > 0 {
                        {meta_chip("Fans", t.fan, ChipSource::Shared)}
                    }
                    if let Some(follower_num) = t.follower_num {
                        {meta_chip("Followers", format_followers(follower_num), ChipSource::UmaMoe)}
                    }
                    if let Some(date) = recheck_date {
                        {meta_chip("Recheck", date, ChipSource::UmaMoe)}
                    }
                    if let Some(date) = updated_date {
                        if let Some(source) = update_source {
                            {meta_chip("Updated", format!("{} · {}", date, source), ChipSource::Shared)}
                        }
                    }
                    if !t.circle_name.is_empty() {
                        {meta_chip("Circle", &t.circle_name, ChipSource::Shared)}
                    }
                    if let Some(ref last_login) = t.last_login {
                        if !last_login.is_empty() {
                            {meta_chip("Login", last_login, ChipSource::Game)}
                        }
                    }
                </div>
                if !t.comment.is_empty() {
                    <div class={TrainerMetaStyle::CLASS_NAME} style="margin-top:2px;color:#64748b;font-style:italic;">
                        {&t.comment}
                    </div>
                }
            </div>

            <div style="width:1px;align-self:stretch;background:#1f2937;"></div>

            <div
                class={format!("{}{}", TrainerBorrowBlockStyle::CLASS_NAME, if on_view.is_some() { " clickable" } else { "" })}
                onclick={on_view.clone()}
            >
                <span class="borrow-label">{"Borrow Veteran"}</span>
                if let Some(v) = &t.borrow_veteran {
                    <div class={TrainerSubCardStyle::CLASS_NAME}>
                        <div class={TrainerSubCardHeaderStyle::CLASS_NAME}>
                            <div style="display:flex;flex-direction:column;gap:2px;">
                                if let Some(ref variant) = uma_variant {
                                    <span class={VeteranVariantStyle::CLASS_NAME}>{variant}</span>
                                }
                                <span class={TrainerNameStyle::CLASS_NAME}>{&uma_character}</span>
                            </div>
                            <span class={CardRankStyle::CLASS_NAME}>
                                <RankBadge rank={UmaRank::from_raw(v.rank as u16)} />
                                <span class={RankScoreStyle::CLASS_NAME}>{ format_rank(v.rank_score) }</span>
                            </span>
                        </div>
                        if let Some(aff) = v.affinity {
                            <div class={CardAffinityStyle::CLASS_NAME}>
                                <span>{"Affinity: "}</span>
                                <span class={AffinityBaseStyle::CLASS_NAME}>{aff.base}</span>
                                { if aff.bonus > 0 {
                                    html! {
                                        <>
                                            <span class={AffinityPlusStyle::CLASS_NAME}>{" + "}</span>
                                            <span class={AffinityBonusStyle::CLASS_NAME}>{aff.bonus}</span>
                                            <span>{" (Total: "}{aff.total()}{")"}</span>
                                        </>
                                    }
                                } else { html! {} } }
                            </div>
                        }
                        if !v.blue_sparks.is_empty() {
                            <div class={CardSparksStyle::CLASS_NAME}>
                                { for v.blue_sparks.iter().map(|s| html! { <SparkItem spark={s.clone()} /> }) }
                            </div>
                        }
                        <div style="display:flex;gap:4px;flex-wrap:wrap;">
                            <HashBadge hash={v.hash.as_u64()} label="VET" title="Click to copy veteran hash" />
                            { for v.min_hash.map(|hash| html! {
                                <HashBadge label="PRT" hash={hash.as_u64()} title="Click to copy parent identity hash" />
                            }) }
                        </div>
                    </div>
                } else {
                    <span style="color:#64748b;">{"None"}</span>
                }
            </div>

            <div style="width:1px;align-self:stretch;background:#1f2937;"></div>

            <div
                class={format!("{}{}", TrainerBorrowBlockStyle::CLASS_NAME, if on_view_sc.is_some() { " clickable" } else { "" })}
                onclick={on_view_sc.clone()}
            >
                <span class="borrow-label">{"Borrow Support Card"}</span>
                if let Some(sc) = &t.borrow_support_card {
                    <div class={TrainerSubCardStyle::CLASS_NAME}>
                        if !sc.name.is_empty() {
                            <div style="display:flex;flex-direction:column;gap:2px;">
                                if let Some(ref variant) = sc_variant {
                                    <span class={VeteranVariantStyle::CLASS_NAME}>{variant}</span>
                                }
                                <span class={TrainerNameStyle::CLASS_NAME}>{&sc_character}</span>
                            </div>
                        } else {
                            <span class={TrainerNameStyle::CLASS_NAME}>{"Unknown"}</span>
                        }
                        <div class={SupportCardBadgeRowStyle::CLASS_NAME}>
                            <span class={format!("{} {}", SupportCardRarityStyle::CLASS_NAME, rarity_class(sc.card_rarity))}>
                                {rarity_label(sc.card_rarity)}
                            </span>
                            <span class={format!("{} {}", SupportCardTypeStyle::CLASS_NAME, type_class(sc.card_type))}>
                                {type_label(sc.card_type)}
                            </span>
                            <span class={format!("{}{}", SupportCardLbStyle::CLASS_NAME, if sc.limit_break_count >= 4 { " mlb" } else { "" })}>
                                {(0..4).map(|i| {
                                    let on = i < sc.limit_break_count;
                                    html! {
                                        <span class={format!("diamond{}", if on { " on" } else { "" })}></span>
                                    }
                                }).collect::<Html>()}
                            </span>
                            <span style="color:#9ca3af;font-size:12px;">{format!("Lv{} /{}", sc.level, max_level_for_limit_break(sc.card_rarity, sc.limit_break_count))}</span>
                        </div>
                    </div>
                } else {
                    <span style="color:#64748b;">{"None"}</span>
                }
            </div>
        </div>
    }
}
