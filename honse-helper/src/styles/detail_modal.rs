use crate::styles::{Style, StyleDefinition};

pub struct ModalOverlayStyle;

impl Style for ModalOverlayStyle {
    const CSS: &'static str = r#"
        {{class}} {
            position: fixed;
            inset: 0;
            background: rgba(0, 0, 0, 0.65);
            display: flex;
            align-items: center;
            justify-content: center;
            z-index: 100;
            padding: 20px;
        }
    "#;

    const CLASS_NAME: &'static str = "modal-overlay";
}

pub struct ModalContentStyle;

impl Style for ModalContentStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #0f172a;
            border: 1px solid #334155;
            border-radius: 14px;
            width: 100%;
            max-width: 600px;
            max-height: 85vh;
            display: flex;
            flex-direction: column;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
        }
    "#;

    const CLASS_NAME: &'static str = "modal-content";
}

pub struct ModalHeaderStyle;

impl Style for ModalHeaderStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 16px 20px;
            border-bottom: 1px solid #1f2937;
        }

        {{class}} h2 {
            font-size: 18px;
            font-weight: 600;
            margin: 0;
        }
    "#;

    const CLASS_NAME: &'static str = "modal-header";
}

pub struct ModalCloseStyle;

impl Style for ModalCloseStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: none;
            border: none;
            color: #94a3b8;
            font-size: 22px;
            cursor: pointer;
            padding: 0 4px;
            line-height: 1;
        }

        {{class}}:hover {
            color: #f8fafc;
        }
    "#;

    const CLASS_NAME: &'static str = "modal-close";
}

pub struct ModalTabsStyle;

impl Style for ModalTabsStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            border-bottom: 1px solid #1f2937;
            padding: 0 20px;
            gap: 0;
        }
    "#;

    const CLASS_NAME: &'static str = "modal-tabs";
}

pub struct TabBtnStyle;

impl Style for TabBtnStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: none;
            border: none;
            color: #94a3b8;
            padding: 10px 16px;
            font-size: 13px;
            cursor: pointer;
            border-bottom: 2px solid transparent;
            transition: color 0.15s, border-color 0.15s;
            border-radius: 0;
        }

        {{class}}:hover {
            color: #e2e8f0;
        }
    "#;

    const CLASS_NAME: &'static str = "tab-btn";
}

pub struct TabActiveStyle;

impl Style for TabActiveStyle {
    const CSS: &'static str = r#"
        .tab-btn{{class}} {
            color: #ffffff;
            background: rgba(96, 165, 250, 0.12);
            border-bottom-color: #60a5fa;
            border-bottom-width: 3px;
            border-radius: 6px 6px 0 0;
        }
        .tab-btn{{class}}:hover {
            color: #ffffff;
        }
    "#;

    const CLASS_NAME: &'static str = "tab-active";
}

pub struct ModalBodyStyle;

impl Style for ModalBodyStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex: 1;
            overflow-y: auto;
            padding: 20px;
        }
    "#;

    const CLASS_NAME: &'static str = "modal-body";
}

pub struct DetailTabStyle;

impl Style for DetailTabStyle {
    const CSS: &'static str = r#"
        {{class}} h3 {
            font-size: 14px;
            font-weight: 600;
            margin-bottom: 12px;
            color: #e2e8f0;
        }
    "#;

    const CLASS_NAME: &'static str = "detail-tab";
}

pub struct StatsGridStyle;

impl Style for StatsGridStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            gap: 8px;
            margin-bottom: 20px;
        }

        @media (max-width: 768px) {
            {{class}} {
                grid-template-columns: repeat(2, 1fr);
            }
        }
    "#;

    const CLASS_NAME: &'static str = "stats-grid";
}

pub struct StatItemStyle;

impl Style for StatItemStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #0b1220;
            border: 1px solid #263246;
            border-radius: 8px;
            padding: 10px;
            display: flex;
            justify-content: space-between;
            font-size: 13px;
        }
    "#;

    const CLASS_NAME: &'static str = "stat-item";
}

pub struct StatNameStyle;

impl Style for StatNameStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #94a3b8;
            font-weight: 600;
        }
    "#;

    const CLASS_NAME: &'static str = "stat-name";
}

pub struct AptGridStyle;

impl Style for AptGridStyle {
    const CSS: &'static str = r#"
        {{class}} {
            grid-template-columns: 1fr;
        }

        @media (max-width: 768px) {
            {{class}} {
                grid-template-columns: 1fr;
            }
        }
    "#;

    const CLASS_NAME: &'static str = "apt-grid";
}

pub struct AptItemStyle;

impl Style for AptItemStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            justify-content: space-between;
            padding: 6px 10px;
            background: #0b1220;
            border: 1px solid #263246;
            border-radius: 6px;
            font-size: 13px;
            margin-bottom: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "apt-item";
}

pub struct AptNameStyle;

impl Style for AptNameStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #94a3b8;
        }
    "#;

    const CLASS_NAME: &'static str = "apt-name";
}

pub struct AptLevelStyle;

impl Style for AptLevelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #e2e8f0;
            font-weight: 700;
        }
    "#;

    const CLASS_NAME: &'static str = "apt-level";
}

pub struct SparkDetailListStyle;

impl Style for SparkDetailListStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
            gap: 6px;
        }
    "#;

    const CLASS_NAME: &'static str = "spark-detail-list";
}

pub struct SparkColorRowStyle;

impl Style for SparkColorRowStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-wrap: wrap;
            gap: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "spark-color-row";
}

pub struct ParentListStyle;

impl Style for ParentListStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
            gap: 12px;
        }
    "#;

    const CLASS_NAME: &'static str = "parent-list";
}

pub struct ParentCardStyle;

impl Style for ParentCardStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: linear-gradient(180deg, #10182b 0%, #0f172a 100%);
            border: 1px solid #263246;
            border-radius: 12px;
            padding: 14px;
            cursor: pointer;
            transition: border-color 0.15s;
        }

        {{class}}:hover {
            border-color: #3b4b6b;
        }
    "#;

    const CLASS_NAME: &'static str = "parent-card";
}

pub struct ParentRarityStyle;

impl Style for ParentRarityStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 12px;
            color: #94a3b8;
        }
    "#;

    const CLASS_NAME: &'static str = "parent-rarity";
}

pub struct ParentTalentStyle;

impl Style for ParentTalentStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 12px;
            color: #fbbf24;
        }
    "#;

    const CLASS_NAME: &'static str = "parent-talent";
}

pub struct WinsListStyle;

impl Style for WinsListStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
            gap: 6px;
        }
    "#;

    const CLASS_NAME: &'static str = "wins-list";
}

pub struct WinRowStyle;

impl Style for WinRowStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 8px 12px;
            background: #0b1220;
            border: 1px solid #263246;
            border-radius: 8px;
        }
    "#;

    const CLASS_NAME: &'static str = "win-row";
}

pub struct WinNameStyle;

impl Style for WinNameStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 13px;
            color: #e2e8f0;
        }
    "#;

    const CLASS_NAME: &'static str = "win-name";
}

pub struct WinBadgesStyle;

impl Style for WinBadgesStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            gap: 6px;
            align-items: center;
        }
    "#;

    const CLASS_NAME: &'static str = "win-badges";
}

pub struct WinSharedStyle;

impl Style for WinSharedStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 11px;
            color: #fbbf24;
            background: rgba(251, 191, 36, 0.1);
            padding: 2px 8px;
            border-radius: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "win-shared";
}

pub struct WinVeteranStyle;

impl Style for WinVeteranStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 11px;
            color: #7dd3fc;
            background: rgba(56, 189, 248, 0.1);
            padding: 2px 8px;
            border-radius: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "win-veteran";
}

pub struct ParentDetailOverlayStyle;

impl Style for ParentDetailOverlayStyle {
    const CSS: &'static str = r#"
        {{class}} {
            position: fixed;
            inset: 0;
            background: rgba(0, 0, 0, 0.65);
            display: flex;
            align-items: center;
            justify-content: center;
            z-index: 110;
            padding: 20px;
        }
    "#;

    const CLASS_NAME: &'static str = "parent-detail-overlay";
}

pub struct ParentDetailContentStyle;

impl Style for ParentDetailContentStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #0f172a;
            border: 1px solid #334155;
            border-radius: 14px;
            width: 100%;
            max-width: 600px;
            max-height: 80vh;
            display: flex;
            flex-direction: column;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
        }
    "#;

    const CLASS_NAME: &'static str = "parent-detail-content";
}

pub struct CardRankStyle;

impl Style for CardRankStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 14px;
            font-weight: 700;
            color: #fbbf24;
            font-feature-settings: 'tnum' 1;
        }
    "#;

    const CLASS_NAME: &'static str = "card-rank";
}

pub struct CardMetaStyle;

impl Style for CardMetaStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            gap: 8px;
            align-items: center;
            font-size: 12px;
            color: #94a3b8;
            margin-bottom: 8px;
        }
    "#;

    const CLASS_NAME: &'static str = "card-meta";
}

pub struct SkillsListStyle;

impl Style for SkillsListStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
            gap: 14px;
        }
    "#;

    const CLASS_NAME: &'static str = "skills-list";
}

pub struct SkillGroupStyle;

impl Style for SkillGroupStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
            gap: 5px;
        }
    "#;

    const CLASS_NAME: &'static str = "skill-group";
}

pub struct SkillCategoryStyle;

impl Style for SkillCategoryStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 11px;
            font-weight: 700;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            border-bottom: 1px solid #1e293b;
            padding-bottom: 3px;
            margin-bottom: 2px;
        }
    "#;

    const CLASS_NAME: &'static str = "skill-category";
}

pub struct SkillRowStyle;

impl Style for SkillRowStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            gap: 12px;
            padding: 4px 0;
            font-size: 13px;
        }
    "#;

    const CLASS_NAME: &'static str = "skill-row";
}

pub struct SkillNameStyle;

impl Style for SkillNameStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #e2e8f0;
            flex: 1;
        }
    "#;

    const CLASS_NAME: &'static str = "skill-name";
}

pub struct SkillLevelStyle;

impl Style for SkillLevelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #facc15;
            font-weight: 600;
            font-size: 12px;
        }
    "#;

    const CLASS_NAME: &'static str = "skill-level";
}

pub struct SkillIdStyle;

impl Style for SkillIdStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #475569;
            font-size: 11px;
            min-width: 48px;
            text-align: right;
        }
    "#;

    const CLASS_NAME: &'static str = "skill-id";
}

inventory::submit! { StyleDefinition { css: ModalOverlayStyle::CSS, selector_type: ModalOverlayStyle::SELECTOR_TYPE, class_name: ModalOverlayStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ModalContentStyle::CSS, selector_type: ModalContentStyle::SELECTOR_TYPE, class_name: ModalContentStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ModalHeaderStyle::CSS, selector_type: ModalHeaderStyle::SELECTOR_TYPE, class_name: ModalHeaderStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ModalCloseStyle::CSS, selector_type: ModalCloseStyle::SELECTOR_TYPE, class_name: ModalCloseStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ModalTabsStyle::CSS, selector_type: ModalTabsStyle::SELECTOR_TYPE, class_name: ModalTabsStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TabBtnStyle::CSS, selector_type: TabBtnStyle::SELECTOR_TYPE, class_name: TabBtnStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TabActiveStyle::CSS, selector_type: TabActiveStyle::SELECTOR_TYPE, class_name: TabActiveStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ModalBodyStyle::CSS, selector_type: ModalBodyStyle::SELECTOR_TYPE, class_name: ModalBodyStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: DetailTabStyle::CSS, selector_type: DetailTabStyle::SELECTOR_TYPE, class_name: DetailTabStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: StatsGridStyle::CSS, selector_type: StatsGridStyle::SELECTOR_TYPE, class_name: StatsGridStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: StatItemStyle::CSS, selector_type: StatItemStyle::SELECTOR_TYPE, class_name: StatItemStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: StatNameStyle::CSS, selector_type: StatNameStyle::SELECTOR_TYPE, class_name: StatNameStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: AptGridStyle::CSS, selector_type: AptGridStyle::SELECTOR_TYPE, class_name: AptGridStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: AptItemStyle::CSS, selector_type: AptItemStyle::SELECTOR_TYPE, class_name: AptItemStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: AptNameStyle::CSS, selector_type: AptNameStyle::SELECTOR_TYPE, class_name: AptNameStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: AptLevelStyle::CSS, selector_type: AptLevelStyle::SELECTOR_TYPE, class_name: AptLevelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SparkDetailListStyle::CSS, selector_type: SparkDetailListStyle::SELECTOR_TYPE, class_name: SparkDetailListStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SparkColorRowStyle::CSS, selector_type: SparkColorRowStyle::SELECTOR_TYPE, class_name: SparkColorRowStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ParentListStyle::CSS, selector_type: ParentListStyle::SELECTOR_TYPE, class_name: ParentListStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ParentCardStyle::CSS, selector_type: ParentCardStyle::SELECTOR_TYPE, class_name: ParentCardStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ParentRarityStyle::CSS, selector_type: ParentRarityStyle::SELECTOR_TYPE, class_name: ParentRarityStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ParentTalentStyle::CSS, selector_type: ParentTalentStyle::SELECTOR_TYPE, class_name: ParentTalentStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: WinsListStyle::CSS, selector_type: WinsListStyle::SELECTOR_TYPE, class_name: WinsListStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: WinRowStyle::CSS, selector_type: WinRowStyle::SELECTOR_TYPE, class_name: WinRowStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: WinNameStyle::CSS, selector_type: WinNameStyle::SELECTOR_TYPE, class_name: WinNameStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: WinBadgesStyle::CSS, selector_type: WinBadgesStyle::SELECTOR_TYPE, class_name: WinBadgesStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: WinSharedStyle::CSS, selector_type: WinSharedStyle::SELECTOR_TYPE, class_name: WinSharedStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: WinVeteranStyle::CSS, selector_type: WinVeteranStyle::SELECTOR_TYPE, class_name: WinVeteranStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ParentDetailOverlayStyle::CSS, selector_type: ParentDetailOverlayStyle::SELECTOR_TYPE, class_name: ParentDetailOverlayStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ParentDetailContentStyle::CSS, selector_type: ParentDetailContentStyle::SELECTOR_TYPE, class_name: ParentDetailContentStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CardRankStyle::CSS, selector_type: CardRankStyle::SELECTOR_TYPE, class_name: CardRankStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CardMetaStyle::CSS, selector_type: CardMetaStyle::SELECTOR_TYPE, class_name: CardMetaStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SkillsListStyle::CSS, selector_type: SkillsListStyle::SELECTOR_TYPE, class_name: SkillsListStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SkillGroupStyle::CSS, selector_type: SkillGroupStyle::SELECTOR_TYPE, class_name: SkillGroupStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SkillCategoryStyle::CSS, selector_type: SkillCategoryStyle::SELECTOR_TYPE, class_name: SkillCategoryStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SkillRowStyle::CSS, selector_type: SkillRowStyle::SELECTOR_TYPE, class_name: SkillRowStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SkillNameStyle::CSS, selector_type: SkillNameStyle::SELECTOR_TYPE, class_name: SkillNameStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SkillLevelStyle::CSS, selector_type: SkillLevelStyle::SELECTOR_TYPE, class_name: SkillLevelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SkillIdStyle::CSS, selector_type: SkillIdStyle::SELECTOR_TYPE, class_name: SkillIdStyle::CLASS_NAME } }

pub struct SupportCardListStyle;

impl Style for SupportCardListStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
            gap: 6px;
        }
    "#;
    const CLASS_NAME: &'static str = "support-card-list";
}

pub struct SupportCardRowStyle;

impl Style for SupportCardRowStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            gap: 12px;
            padding: 10px 14px;
            background: #1e293b;
            border: 1px solid #334155;
            border-radius: 6px;
            font-size: 13px;
        }
        {{class}}.borrow-row {
            border-color: #9b59b6;
            box-shadow: inset 0 0 0 1px rgba(155, 89, 182, 0.3),
                        0 0 6px rgba(155, 89, 182, 0.25);
        }
    "#;
    const CLASS_NAME: &'static str = "support-card-row";
}

pub struct SupportCardNameStyle;

impl Style for SupportCardNameStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #e2e8f0;
            font-weight: 600;
            font-size: 14px;
        }
    "#;
    const CLASS_NAME: &'static str = "support-card-name";
}

pub struct SupportCardBadgeRowStyle;

impl Style for SupportCardBadgeRowStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            gap: 8px;
            flex-wrap: wrap;
        }
    "#;
    const CLASS_NAME: &'static str = "support-card-badge-row";
}

/* --- Card container (used in browser grid, vertical layout) --- */

pub struct SupportCardCardStyle;

impl Style for SupportCardCardStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
            gap: 4px;
            padding: 10px 14px;
            background: linear-gradient(180deg, #10182b 0%, #0f172a 100%);
            border: 1px solid #263246;
            border-radius: 12px;
            font-size: 13px;
            cursor: pointer;
            transition: border-color 0.15s, box-shadow 0.15s;
        }

        {{class}}:hover {
            border-color: #3b4b6b;
            box-shadow: 0 4px 16px rgba(0, 0, 0, 0.3);
        }

        {{class}}.borrow-row {
            border-color: #9b59b6;
            box-shadow: inset 0 0 0 1px rgba(155, 89, 182, 0.3),
                        0 0 6px rgba(155, 89, 182, 0.25);
        }
    "#;
    const CLASS_NAME: &'static str = "support-card-card";
}

/* --- Variant text (the [bracket] part) --- */

pub struct SupportCardVariantStyle;

impl Style for SupportCardVariantStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #f59e0b;
            font-size: 11px;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            line-height: 1.2;
        }
    "#;
    const CLASS_NAME: &'static str = "support-card-variant";
}

/* --- Rarity badge --- */

pub struct SupportCardRarityStyle;

impl Style for SupportCardRarityStyle {
    const CSS: &'static str = r#"
        {{class}} {
            position: relative;
            display: inline-flex;
            align-items: center;
            justify-content: center;
            box-sizing: border-box;
            min-width: 2.4em;
            min-height: 1.8em;
            padding: 0.3em 0.7em;
            border-radius: 999px;
            overflow: hidden;
            font-size: 11px;
            font-weight: 900;
            line-height: 1;
            color: #ffffff;
            text-shadow: -1px -1px 1px rgba(0,0,0,0.6), 1px -1px 1px rgba(0,0,0,0.6), -1px 1px 1px rgba(0,0,0,0.6), 1px 1px 1px rgba(0,0,0,0.6);
            box-shadow: inset 0 0.15em 0.3em rgba(255,255,255,0.45),
                        inset 0 -0.2em 0.4em rgba(0,0,0,0.35),
                        0 0.25em 0.5em rgba(0,0,0,0.4);
            white-space: nowrap;
            user-select: none;
        }
        {{class}}::before {
            content: "";
            position: absolute;
            top: 0;
            left: -50%;
            width: 200%;
            height: 100%;
            pointer-events: none;
            z-index: 1;
            background: linear-gradient(120deg, transparent 30%, rgba(255,255,255,0.35), transparent 70%);
            transform: skewX(-20deg);
        }
        {{class}}.rarity-r {
            background: linear-gradient(145deg, #8aa0c6, #5c6f96);
        }
        {{class}}.rarity-sr {
            background: linear-gradient(145deg, #ffd84a, #d6a800);
            color: #fff8dc;
        }
        {{class}}.rarity-ssr {
            background: linear-gradient(135deg, #b06cff 0%, #6a5cff 40%, #4db8ff 100%);
            box-shadow: inset 0 0.2em 0.4em rgba(255,255,255,0.5),
                        inset 0 -0.3em 0.6em rgba(0,0,0,0.4),
                        0 0 0.5em rgba(120,140,255,0.7),
                        0 0 1em rgba(80,180,255,0.5);
        }
        {{class}}.rarity-ssr::after {
            content: "";
            position: absolute;
            inset: 0;
            border-radius: inherit;
            pointer-events: none;
            z-index: 1;
            background: linear-gradient(120deg, transparent, rgba(255,255,255,0.25),
                                        rgba(120,200,255,0.25), transparent);
            background-size: 200% 100%;
            mix-blend-mode: screen;
        }
        {{class}}.rarity-ssr .rarity-text {
            color: #ffffff;
            text-shadow: -1px -1px 1px rgba(0,0,0,0.6), 1px -1px 1px rgba(0,0,0,0.6), -1px 1px 1px rgba(0,0,0,0.6), 1px 1px 1px rgba(0,0,0,0.6);
        }
    "#;
    const CLASS_NAME: &'static str = "support-card-rarity";
}

/* --- Type badge --- */

pub struct SupportCardTypeStyle;

impl Style for SupportCardTypeStyle {
    const CSS: &'static str = r#"
        {{class}} {
            position: relative;
            display: inline-flex;
            align-items: center;
            justify-content: center;
            box-sizing: border-box;
            min-width: 3.2em;
            min-height: 1.8em;
            padding: 0.3em 0.7em;
            border-radius: 999px;
            overflow: hidden;
            font-size: 11px;
            font-weight: 800;
            letter-spacing: 0.03em;
            line-height: 1;
            text-shadow: -1px -1px 1px rgba(0,0,0,0.6), 1px -1px 1px rgba(0,0,0,0.6), -1px 1px 1px rgba(0,0,0,0.6), 1px 1px 1px rgba(0,0,0,0.6);
            white-space: nowrap;
            user-select: none;
            box-shadow: inset 0 0.15em 0.3em rgba(255,255,255,0.45),
                        inset 0 -0.2em 0.4em rgba(0,0,0,0.35),
                        0 0.2em 0.4em rgba(0,0,0,0.35);
        }
        {{class}}.type-speed   { background: linear-gradient(145deg, #4da6ff, #2d6fd6); color: #eaf4ff; }
        {{class}}.type-power   { background: linear-gradient(145deg, #ffb347, #e07a00); color: #fff3e0; }
        {{class}}.type-stamina { background: linear-gradient(145deg, #ff6b6b, #c0392b); color: #ffeaea; }
        {{class}}.type-guts    { background: linear-gradient(145deg, #ff8ad6, #d94fa3); color: #fff0fa; }
        {{class}}.type-wit     { background: linear-gradient(145deg, #5fdc9c, #2e9e63); color: #eafff4; }
        {{class}}.type-pal {
            background: linear-gradient(145deg, #ffffff, #eaeaea);
            color: #d4a800;
            text-shadow: 0 0.05em 0.15em rgba(180, 140, 0, 0.4);
            box-shadow: inset 0 0.15em 0.25em rgba(255,255,255,0.9),
                        inset 0 -0.15em 0.3em rgba(0,0,0,0.15),
                        0 0.2em 0.4em rgba(0,0,0,0.25);
        }
        {{class}}.type-group {
            background: linear-gradient(145deg, #ffffff, #eaeaea);
            color: #2e9e63;
            text-shadow: 0 0.05em 0.15em rgba(0, 120, 70, 0.4);
            box-shadow: inset 0 0.15em 0.25em rgba(255,255,255,0.9),
                        inset 0 -0.15em 0.3em rgba(0,0,0,0.15),
                        0 0.2em 0.4em rgba(0,0,0,0.25);
        }
        {{class}}.type-unknown { background: linear-gradient(145deg, #9ca3af, #4b5563); color: #f8fafc; }
    "#;
    const CLASS_NAME: &'static str = "support-card-type";
}

/* --- Limit break diamonds --- */

pub struct SupportCardLbStyle;

impl Style for SupportCardLbStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-flex;
            align-items: center;
            gap: 3px;
            padding: 0.2em 0.5em;
            background: linear-gradient(145deg, #4b5a858e, #37589e);
            border-radius: 4px;
        }
        {{class}} .diamond {
            width: 0.6em;
            height: 0.8em;
            background: linear-gradient(145deg, #3a3f50, #232837);
            clip-path: polygon(50% 0%, 100% 50%, 50% 100%, 0% 50%);
            display: inline-block;
        }
        {{class}} .diamond.on {
            background: linear-gradient(145deg, #5fb8ff, #2d7be6);
            box-shadow: inset 0 0.12em 0.25em rgba(255,255,255,0.5),
                        inset 0 -0.15em 0.3em rgba(0,0,0,0.4),
                        0 0 0.25em rgba(80,160,255,0.7);
        }
        {{class}}.mlb {
            box-shadow: 0 0 0.5em rgba(80,160,255,0.6),
                        0 0 0.75em rgba(80,160,255,0.4);
        }
        {{class}}.mlb .diamond.on {
            background: linear-gradient(145deg, #6fe0ff, #2bb3ff);
            box-shadow: inset 0 0.12em 0.25em rgba(255,255,255,0.6),
                        inset 0 -0.15em 0.3em rgba(0,0,0,0.4),
                        0 0 0.3em rgba(80,200,255,0.9);
        }
    "#;
    const CLASS_NAME: &'static str = "support-card-lb";
}

/* --- Exp text --- */

inventory::submit! { StyleDefinition { css: SupportCardListStyle::CSS, selector_type: SupportCardListStyle::SELECTOR_TYPE, class_name: SupportCardListStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SupportCardCardStyle::CSS, selector_type: SupportCardCardStyle::SELECTOR_TYPE, class_name: SupportCardCardStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SupportCardVariantStyle::CSS, selector_type: SupportCardVariantStyle::SELECTOR_TYPE, class_name: SupportCardVariantStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SupportCardRowStyle::CSS, selector_type: SupportCardRowStyle::SELECTOR_TYPE, class_name: SupportCardRowStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SupportCardNameStyle::CSS, selector_type: SupportCardNameStyle::SELECTOR_TYPE, class_name: SupportCardNameStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SupportCardBadgeRowStyle::CSS, selector_type: SupportCardBadgeRowStyle::SELECTOR_TYPE, class_name: SupportCardBadgeRowStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SupportCardRarityStyle::CSS, selector_type: SupportCardRarityStyle::SELECTOR_TYPE, class_name: SupportCardRarityStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SupportCardTypeStyle::CSS, selector_type: SupportCardTypeStyle::SELECTOR_TYPE, class_name: SupportCardTypeStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SupportCardLbStyle::CSS, selector_type: SupportCardLbStyle::SELECTOR_TYPE, class_name: SupportCardLbStyle::CLASS_NAME } }
