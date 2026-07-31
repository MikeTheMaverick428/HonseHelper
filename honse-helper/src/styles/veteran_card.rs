use crate::styles::{Style, StyleDefinition};

pub struct VeteranCardRootStyle;

impl Style for VeteranCardRootStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: linear-gradient(180deg, #10182b 0%, #0f172a 100%);
            border: 1px solid #263246;
            border-radius: 12px;
            padding: 14px;
            cursor: pointer;
            transition: border-color 0.15s, box-shadow 0.15s;
            position: relative;
        }

        {{class}}:hover {
            border-color: #3b4b6b;
            box-shadow: 0 4px 16px rgba(0, 0, 0, 0.3);
        }
    "#;

    const CLASS_NAME: &'static str = "veteran-card";
}

pub struct CardHeaderStyle;

impl Style for CardHeaderStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 8px;
        }
    "#;

    const CLASS_NAME: &'static str = "card-header";
}

pub struct CardNameStyle;

impl Style for CardNameStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 15px;
            font-weight: 600;
            color: #f1f5f9;
        }
    "#;

    const CLASS_NAME: &'static str = "card-name";
}

pub struct VeteranVariantStyle;

impl Style for VeteranVariantStyle {
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

    const CLASS_NAME: &'static str = "veteran-variant";
}

pub struct IndepTrainBadgeStyle;

impl Style for IndepTrainBadgeStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-flex;
            align-items: center;
            gap: 4px;
            background: linear-gradient(135deg, #fbbf24, #f59e0b);
            color: #1e1b4b;
            padding: 2px 8px;
            border-radius: 4px;
            font-size: 11px;
            font-weight: 700;
            letter-spacing: 0.3px;
            white-space: nowrap;
        }
    "#;

    const CLASS_NAME: &'static str = "indep-train-badge";
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

pub struct CardBorrowedStyle;

impl Style for CardBorrowedStyle {
    const CSS: &'static str = r#"
        {{class}} .card-rank {
            color: #f87171;
        }
        {{class}} .rank-badge {
            opacity: 0.6;
        }
    "#;

    const CLASS_NAME: &'static str = "card-borrowed";
}

pub struct RankScoreStyle;

impl Style for RankScoreStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin-left: 6px;
        }
    "#;

    const CLASS_NAME: &'static str = "rank-score";
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

pub struct CardScenarioStyle;

impl Style for CardScenarioStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #1e293b;
            padding: 2px 8px;
            border-radius: 4px;
            font-size: 11px;
            font-weight: 500;
        }
    "#;

    const CLASS_NAME: &'static str = "card-scenario";
}

pub struct CardDateStyle;

impl Style for CardDateStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #64748b;
        }
    "#;

    const CLASS_NAME: &'static str = "card-date";
}

pub struct BorrowedBadgeStyle;

impl Style for BorrowedBadgeStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: rgba(248, 113, 113, 0.15);
            color: #fca5a5;
            padding: 2px 8px;
            border-radius: 4px;
            font-size: 11px;
            font-weight: 500;
        }
    "#;

    const CLASS_NAME: &'static str = "borrowed-badge";
}

pub struct OwnerIdBadgeStyle;

impl Style for OwnerIdBadgeStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-flex;
            align-items: center;
            gap: 4px;
            background: rgba(251, 191, 36, 0.15);
            color: #fcd34d;
            padding: 2px 8px;
            border-radius: 4px;
            font-size: 11px;
            font-weight: 500;
            cursor: pointer;
            border: 1px solid rgba(251, 191, 36, 0.3);
            transition: background 0.15s, border-color 0.15s;
        }
        {{class}}:hover {
            background: rgba(251, 191, 36, 0.25);
        }
        {{class}}.owner-id-copied {
            background: rgba(34, 197, 94, 0.2);
            border-color: rgba(34, 197, 94, 0.5);
            color: #86efac;
        }
    "#;

    const CLASS_NAME: &'static str = "owner-id-badge";
}

pub struct OwnerIdPrefixStyle;

impl Style for OwnerIdPrefixStyle {
    const CSS: &'static str = r#"
        {{class}} {
            opacity: 0.7;
            font-weight: 400;
        }
    "#;

    const CLASS_NAME: &'static str = "owner-id-prefix";
}

pub struct CardStatsRowStyle;

impl Style for CardStatsRowStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            gap: 12px;
            font-size: 12px;
            margin-bottom: 8px;
        }
    "#;

    const CLASS_NAME: &'static str = "card-stats-row";
}

pub struct StatLabelStyle;

impl Style for StatLabelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #64748b;
        }
    "#;

    const CLASS_NAME: &'static str = "stat-label";
}

pub struct StatValueStyle;

impl Style for StatValueStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #e2e8f0;
            font-weight: 600;
        }
    "#;

    const CLASS_NAME: &'static str = "stat-value";
}

pub struct StatSubStyle;

impl Style for StatSubStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #93c5fd;
            font-size: 11px;
        }
    "#;

    const CLASS_NAME: &'static str = "stat-sub";
}

pub struct CardAffinityStyle;

impl Style for CardAffinityStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 13px;
            font-weight: 600;
            color: #a78bfa;
            margin-bottom: 6px;
            padding: 2px 8px;
            background: #1e1b4b;
            border-radius: 4px;
            display: inline-block;
        }
    "#;

    const CLASS_NAME: &'static str = "card-affinity";
}

pub struct CardSparksStyle;

impl Style for CardSparksStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-wrap: wrap;
            gap: 4px;
            margin-bottom: 8px;
        }
    "#;

    const CLASS_NAME: &'static str = "card-sparks";
}

pub struct CardFooterStyle;

impl Style for CardFooterStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            justify-content: space-between;
            align-items: center;
            font-size: 11px;
        }
    "#;

    const CLASS_NAME: &'static str = "card-footer";
}

pub struct CardHashStyle;

impl Style for CardHashStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-family: 'Monaco', 'Menlo', 'Consolas', monospace;
            font-size: 11px;
            color: #64748b;
            cursor: pointer;
            background: rgba(100, 116, 139, 0.1);
            padding: 2px 6px;
            border-radius: 4px;
            border: 1px solid rgba(100, 116, 139, 0.2);
            transition: all 0.15s;
            letter-spacing: 0.5px;
        }

        {{class}}:hover {
            color: #94a3b8;
            background: rgba(100, 116, 139, 0.2);
            border-color: rgba(100, 116, 139, 0.4);
        }

        {{class}}.hash-copied {
            color: #86efac;
            background: rgba(34, 197, 94, 0.15);
            border-color: rgba(34, 197, 94, 0.4);
        }
    "#;

    const CLASS_NAME: &'static str = "card-hash";
}

pub struct CardFooterRightStyle;

impl Style for CardFooterRightStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            gap: 8px;
            min-width: 0;
        }
    "#;

    const CLASS_NAME: &'static str = "card-footer-right";
}

pub struct CardFavIconStyle;

impl Style for CardFavIconStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 11px;
            color: #fbbf24;
            white-space: nowrap;
        }
    "#;

    const CLASS_NAME: &'static str = "card-fav-icon";
}

pub struct CardTagsStyle;

impl Style for CardTagsStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-wrap: wrap;
            gap: 4px;
            margin-bottom: 8px;
        }
    "#;
    const CLASS_NAME: &'static str = "card-tags";
}

pub struct CardFavMemoStyle;

impl Style for CardFavMemoStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 11px;
            color: #fbbf24;
            font-style: italic;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
            max-width: 120px;
        }
    "#;

    const CLASS_NAME: &'static str = "card-fav-memo";
}

pub struct SelectBtnStyle;

impl Style for SelectBtnStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin-top: 10px;
            width: 100%;
            padding: 8px;
            font-size: 13px;
            font-weight: 600;
            background: #2563eb;
            color: white;
            border: none;
            border-radius: 8px;
            cursor: pointer;
        }

        {{class}}:hover {
            background: #1d4ed8;
        }
    "#;

    const CLASS_NAME: &'static str = "select-btn";
}

pub struct RankBadgeStyle;

impl Style for RankBadgeStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-flex;
            align-items: center;
            justify-content: center;
            min-width: 28px;
            height: 20px;
            padding: 0 6px;
            border-radius: 4px;
            font-size: 12px;
            font-weight: 800;
            letter-spacing: 0.5px;
        }
    "#;

    const CLASS_NAME: &'static str = "rank-badge";
}

pub struct RankTier1Style;
impl Style for RankTier1Style {
    const CSS: &'static str = "{{class}} { background: #5c3a1e; color: #d4a373; }";
    const CLASS_NAME: &'static str = "rank-tier-1";
}
pub struct RankTier2Style;
impl Style for RankTier2Style {
    const CSS: &'static str = "{{class}} { background: #374151; color: #9ca3af; }";
    const CLASS_NAME: &'static str = "rank-tier-2";
}
pub struct RankTier3Style;
impl Style for RankTier3Style {
    const CSS: &'static str = "{{class}} { background: #1e3a2f; color: #34d399; }";
    const CLASS_NAME: &'static str = "rank-tier-3";
}
pub struct RankTier4Style;
impl Style for RankTier4Style {
    const CSS: &'static str = "{{class}} { background: #1e3a5f; color: #60a5fa; }";
    const CLASS_NAME: &'static str = "rank-tier-4";
}
pub struct RankTier5Style;
impl Style for RankTier5Style {
    const CSS: &'static str = "{{class}} { background: #3b1e5c; color: #c084fc; }";
    const CLASS_NAME: &'static str = "rank-tier-5";
}
pub struct RankTier6Style;
impl Style for RankTier6Style {
    const CSS: &'static str = "{{class}} { background: #5c1e3a; color: #f472b6; }";
    const CLASS_NAME: &'static str = "rank-tier-6";
}
pub struct RankTier7Style;
impl Style for RankTier7Style {
    const CSS: &'static str = "{{class}} { background: #5c2e1e; color: #fb923c; }";
    const CLASS_NAME: &'static str = "rank-tier-7";
}
pub struct RankTier8Style;
impl Style for RankTier8Style {
    const CSS: &'static str = "{{class}} { background: #4a3a0e; color: #facc15; }";
    const CLASS_NAME: &'static str = "rank-tier-8";
}
pub struct RankTier9Style;
impl Style for RankTier9Style {
    const CSS: &'static str = "{{class}} { background: #134e4a; color: #5eead4; }";
    const CLASS_NAME: &'static str = "rank-tier-9";
}
pub struct RankTier10Style;
impl Style for RankTier10Style {
    const CSS: &'static str = "{{class}} { background: #155e4a; color: #6ee7b7; }";
    const CLASS_NAME: &'static str = "rank-tier-10";
}
pub struct RankTier11Style;
impl Style for RankTier11Style {
    const CSS: &'static str = "{{class}} { background: #1a4a5e; color: #67e8f9; }";
    const CLASS_NAME: &'static str = "rank-tier-11";
}
pub struct RankTier12Style;
impl Style for RankTier12Style {
    const CSS: &'static str = "{{class}} { background: #1e3a5e; color: #7dd3fc; }";
    const CLASS_NAME: &'static str = "rank-tier-12";
}
pub struct RankTier13Style;
impl Style for RankTier13Style {
    const CSS: &'static str = "{{class}} { background: #2e2a5e; color: #a5b4fc; }";
    const CLASS_NAME: &'static str = "rank-tier-13";
}
pub struct RankTier14Style;
impl Style for RankTier14Style {
    const CSS: &'static str = "{{class}} { background: #3e1a5e; color: #c4b5fd; }";
    const CLASS_NAME: &'static str = "rank-tier-14";
}
pub struct RankTierUnknownStyle;
impl Style for RankTierUnknownStyle {
    const CSS: &'static str = "{{class}} { background: #1e1e1e; color: #6b7280; }";
    const CLASS_NAME: &'static str = "rank-tier-unknown";
}

inventory::submit! { StyleDefinition { css: VeteranCardRootStyle::CSS, selector_type: VeteranCardRootStyle::SELECTOR_TYPE, class_name: VeteranCardRootStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CardHeaderStyle::CSS, selector_type: CardHeaderStyle::SELECTOR_TYPE, class_name: CardHeaderStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CardNameStyle::CSS, selector_type: CardNameStyle::SELECTOR_TYPE, class_name: CardNameStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: VeteranVariantStyle::CSS, selector_type: VeteranVariantStyle::SELECTOR_TYPE, class_name: VeteranVariantStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: IndepTrainBadgeStyle::CSS, selector_type: IndepTrainBadgeStyle::SELECTOR_TYPE, class_name: IndepTrainBadgeStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CardRankStyle::CSS, selector_type: CardRankStyle::SELECTOR_TYPE, class_name: CardRankStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CardBorrowedStyle::CSS, selector_type: CardBorrowedStyle::SELECTOR_TYPE, class_name: CardBorrowedStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RankScoreStyle::CSS, selector_type: RankScoreStyle::SELECTOR_TYPE, class_name: RankScoreStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CardMetaStyle::CSS, selector_type: CardMetaStyle::SELECTOR_TYPE, class_name: CardMetaStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CardScenarioStyle::CSS, selector_type: CardScenarioStyle::SELECTOR_TYPE, class_name: CardScenarioStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CardDateStyle::CSS, selector_type: CardDateStyle::SELECTOR_TYPE, class_name: CardDateStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: BorrowedBadgeStyle::CSS, selector_type: BorrowedBadgeStyle::SELECTOR_TYPE, class_name: BorrowedBadgeStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: OwnerIdBadgeStyle::CSS, selector_type: OwnerIdBadgeStyle::SELECTOR_TYPE, class_name: OwnerIdBadgeStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: OwnerIdPrefixStyle::CSS, selector_type: OwnerIdPrefixStyle::SELECTOR_TYPE, class_name: OwnerIdPrefixStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CardStatsRowStyle::CSS, selector_type: CardStatsRowStyle::SELECTOR_TYPE, class_name: CardStatsRowStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: StatLabelStyle::CSS, selector_type: StatLabelStyle::SELECTOR_TYPE, class_name: StatLabelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: StatValueStyle::CSS, selector_type: StatValueStyle::SELECTOR_TYPE, class_name: StatValueStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: StatSubStyle::CSS, selector_type: StatSubStyle::SELECTOR_TYPE, class_name: StatSubStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CardAffinityStyle::CSS, selector_type: CardAffinityStyle::SELECTOR_TYPE, class_name: CardAffinityStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CardSparksStyle::CSS, selector_type: CardSparksStyle::SELECTOR_TYPE, class_name: CardSparksStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CardFooterStyle::CSS, selector_type: CardFooterStyle::SELECTOR_TYPE, class_name: CardFooterStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CardHashStyle::CSS, selector_type: CardHashStyle::SELECTOR_TYPE, class_name: CardHashStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CardFooterRightStyle::CSS, selector_type: CardFooterRightStyle::SELECTOR_TYPE, class_name: CardFooterRightStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CardFavIconStyle::CSS, selector_type: CardFavIconStyle::SELECTOR_TYPE, class_name: CardFavIconStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CardTagsStyle::CSS, selector_type: CardTagsStyle::SELECTOR_TYPE, class_name: CardTagsStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CardFavMemoStyle::CSS, selector_type: CardFavMemoStyle::SELECTOR_TYPE, class_name: CardFavMemoStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SelectBtnStyle::CSS, selector_type: SelectBtnStyle::SELECTOR_TYPE, class_name: SelectBtnStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RankBadgeStyle::CSS, selector_type: RankBadgeStyle::SELECTOR_TYPE, class_name: RankBadgeStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RankTier1Style::CSS, selector_type: RankTier1Style::SELECTOR_TYPE, class_name: RankTier1Style::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RankTier2Style::CSS, selector_type: RankTier2Style::SELECTOR_TYPE, class_name: RankTier2Style::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RankTier3Style::CSS, selector_type: RankTier3Style::SELECTOR_TYPE, class_name: RankTier3Style::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RankTier4Style::CSS, selector_type: RankTier4Style::SELECTOR_TYPE, class_name: RankTier4Style::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RankTier5Style::CSS, selector_type: RankTier5Style::SELECTOR_TYPE, class_name: RankTier5Style::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RankTier6Style::CSS, selector_type: RankTier6Style::SELECTOR_TYPE, class_name: RankTier6Style::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RankTier7Style::CSS, selector_type: RankTier7Style::SELECTOR_TYPE, class_name: RankTier7Style::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RankTier8Style::CSS, selector_type: RankTier8Style::SELECTOR_TYPE, class_name: RankTier8Style::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RankTier9Style::CSS, selector_type: RankTier9Style::SELECTOR_TYPE, class_name: RankTier9Style::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RankTier10Style::CSS, selector_type: RankTier10Style::SELECTOR_TYPE, class_name: RankTier10Style::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RankTier11Style::CSS, selector_type: RankTier11Style::SELECTOR_TYPE, class_name: RankTier11Style::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RankTier12Style::CSS, selector_type: RankTier12Style::SELECTOR_TYPE, class_name: RankTier12Style::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RankTier13Style::CSS, selector_type: RankTier13Style::SELECTOR_TYPE, class_name: RankTier13Style::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RankTier14Style::CSS, selector_type: RankTier14Style::SELECTOR_TYPE, class_name: RankTier14Style::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RankTierUnknownStyle::CSS, selector_type: RankTierUnknownStyle::SELECTOR_TYPE, class_name: RankTierUnknownStyle::CLASS_NAME } }
