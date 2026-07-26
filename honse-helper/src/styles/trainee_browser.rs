use crate::styles::{Style, StyleDefinition};

pub struct TraineeBrowserRootStyle;

impl Style for TraineeBrowserRootStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
            height: 100vh;
            background: #0f1220;
            color: #f3f4f6;
            overflow: hidden;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-browser";
}

pub struct TrBrowserHeaderStyle;

impl Style for TrBrowserHeaderStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex-shrink: 0;
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 12px;
            padding: 16px 20px;
            border-bottom: 1px solid #1f2937;
            background: #0f1220;
        }

        {{class}} h1 {
            font-size: 22px;
            font-weight: 600;
            margin: 0;
        }

        @media (max-width: 768px) {
            {{class}} {
                flex-direction: column;
                align-items: flex-start;
            }
        }
    "#;

    const CLASS_NAME: &'static str = "tr-browser-header";
}

pub struct TrBrowserHeaderControlsStyle;

impl Style for TrBrowserHeaderControlsStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            gap: 10px;
            flex-wrap: wrap;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-browser-header-controls";
}

pub struct TrBrowserBodyStyle;

impl Style for TrBrowserBodyStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex: 1;
            overflow: hidden;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-browser-body";
}

pub struct TrBrowserSidebarStyle;

impl Style for TrBrowserSidebarStyle {
    const CSS: &'static str = r#"
        {{class}} {
            width: 260px;
            flex-shrink: 0;
            border-right: 1px solid #1f2937;
            padding: 16px;
            overflow-y: auto;
            background: #0c0f1a;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-browser-sidebar";
}

pub struct TrBrowserMainStyle;

impl Style for TrBrowserMainStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex: 1;
            display: flex;
            flex-direction: column;
            overflow: hidden;
            padding: 16px 20px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-browser-main";
}

pub struct TrBrowserTotalStyle;

impl Style for TrBrowserTotalStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 13px;
            color: #9ca3af;
            margin-bottom: 12px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-browser-total";
}

pub struct TrCardGridStyle;

impl Style for TrCardGridStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
            gap: 12px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-card-grid";
}

pub struct TrBrowserEmptyStyle;

impl Style for TrBrowserEmptyStyle {
    const CSS: &'static str = r#"
        {{class}} {
            text-align: center;
            padding: 60px 20px;
            color: #6b7280;
            font-size: 15px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-browser-empty";
}

pub struct TrBrowserLoadingStyle;

impl Style for TrBrowserLoadingStyle {
    const CSS: &'static str = r#"
        {{class}} {
            text-align: center;
            padding: 60px 20px;
            color: #9ca3af;
            font-size: 15px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-browser-loading";
}

pub struct TrBrowserErrorStyle;

impl Style for TrBrowserErrorStyle {
    const CSS: &'static str = r#"
        {{class}} {
            text-align: center;
            padding: 60px 20px;
            color: #f87171;
            font-size: 15px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-browser-error";
}

pub struct TraineeCardStyle;

impl Style for TraineeCardStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: linear-gradient(180deg, #10182b 0%, #0f172a 100%);
            border: 1px solid #263246;
            border-radius: 12px;
            padding: 14px 16px;
            display: flex;
            flex-direction: column;
            gap: 8px;
            cursor: pointer;
            transition: border-color 0.15s, box-shadow 0.15s;
        }

        {{class}}:hover {
            border-color: #3b4b6b;
            box-shadow: 0 4px 16px rgba(0, 0, 0, 0.3);
        }
    "#;

    const CLASS_NAME: &'static str = "trainee-card";
}

pub struct TraineeNameStyle;

impl Style for TraineeNameStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 15px;
            font-weight: 700;
            color: #f3f4f6;
            line-height: 1.3;
        }
    "#;

    const CLASS_NAME: &'static str = "trainee-name";
}

pub struct TraineeRarityRowStyle;

impl Style for TraineeRarityRowStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 14px;
            display: flex;
            align-items: center;
            gap: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "trainee-rarity-row";
}

pub struct TraineeCharNameStyle;

impl Style for TraineeCharNameStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 12px;
            color: #9ca3af;
        }
    "#;

    const CLASS_NAME: &'static str = "trainee-char-name";
}

pub struct TraineePieceBarStyle;

impl Style for TraineePieceBarStyle {
    const CSS: &'static str = r#"
        {{class}} {
            width: 100%;
            height: 10px;
            background: #2d3148;
            border-radius: 5px;
            overflow: hidden;
        }
    "#;

    const CLASS_NAME: &'static str = "trainee-piece-bar";
}

pub struct TraineePieceFillStyle;

impl Style for TraineePieceFillStyle {
    const CSS: &'static str = r#"
        {{class}} {
            height: 100%;
            border-radius: 5px;
            background: linear-gradient(90deg, #f97316, #f59e0b);
            transition: width 0.3s ease;
        }
    "#;

    const CLASS_NAME: &'static str = "trainee-piece-fill";
}

pub struct TraineePieceLabelStyle;

impl Style for TraineePieceLabelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 11px;
            color: #9ca3af;
            display: flex;
            justify-content: space-between;
        }
    "#;

    const CLASS_NAME: &'static str = "trainee-piece-label";
}

pub struct TraineeIdStyle;

impl Style for TraineeIdStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 11px;
            color: #4b5563;
            text-align: right;
        }
    "#;

    const CLASS_NAME: &'static str = "trainee-id";
}

pub struct TraineeSelectBtnStyle;

impl Style for TraineeSelectBtnStyle {
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

    const CLASS_NAME: &'static str = "trainee-select-btn";
}

// ── Detail modal styles ──────────────────────────────────────────────

pub struct TrDetailSectionLabelStyle;

impl Style for TrDetailSectionLabelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 11px;
            text-transform: uppercase;
            color: #64748b;
            letter-spacing: 0.5px;
            margin-bottom: 6px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-section-label";
}

pub struct TrDetailStatsGridStyle;

impl Style for TrDetailStatsGridStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: grid;
            grid-template-columns: repeat(5, 1fr);
            gap: 8px;
            margin-bottom: 16px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-stats-grid";
}

pub struct TrDetailStatCardStyle;

impl Style for TrDetailStatCardStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #1e293b;
            border-radius: 6px;
            padding: 8px;
            text-align: center;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-stat-card";
}

pub struct TrDetailStatCardLabelStyle;

impl Style for TrDetailStatCardLabelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 10px;
            text-transform: uppercase;
            color: #64748b;
            letter-spacing: 0.5px;
            margin-bottom: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-stat-card-label";
}

pub struct TrDetailStatCardValueStyle;

impl Style for TrDetailStatCardValueStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 16px;
            font-weight: 700;
            color: #f3f4f6;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-stat-card-value";
}

pub struct TrDetailGrowthCardStyle;

impl Style for TrDetailGrowthCardStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #1e293b;
            border-radius: 6px;
            padding: 8px;
            text-align: center;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-growth-card";
}

pub struct TrDetailGrowthCardLabelStyle;

impl Style for TrDetailGrowthCardLabelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 10px;
            text-transform: uppercase;
            color: #64748b;
            letter-spacing: 0.5px;
            margin-bottom: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-growth-card-label";
}

pub struct TrDetailGrowthCardValueStyle;

impl Style for TrDetailGrowthCardValueStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 14px;
            font-weight: 600;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-growth-card-value";
}

pub struct TrDetailAptSectionStyle;

impl Style for TrDetailAptSectionStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin-top: 12px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-apt-section";
}

pub struct TrDetailAptGridStyle;

impl Style for TrDetailAptGridStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-wrap: wrap;
            gap: 6px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-apt-grid";
}

pub struct TrDetailAptCardStyle;

impl Style for TrDetailAptCardStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #1e293b;
            border-radius: 6px;
            padding: 6px 12px;
            text-align: center;
            min-width: 70px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-apt-card";
}

pub struct TrDetailAptCardLabelStyle;

impl Style for TrDetailAptCardLabelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 10px;
            text-transform: uppercase;
            color: #64748b;
            margin-bottom: 2px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-apt-card-label";
}

pub struct TrDetailAptCardValueStyle;

impl Style for TrDetailAptCardValueStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 18px;
            font-weight: 700;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-apt-card-value";
}

pub struct TrDetailSkillSectionStyle;

impl Style for TrDetailSkillSectionStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin-bottom: 16px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-skill-section";
}

pub struct TrDetailSkillSectionLabelStyle;

impl Style for TrDetailSkillSectionLabelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 11px;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            margin-bottom: 6px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-skill-section-label";
}

pub struct TrDetailSkillListStyle;

impl Style for TrDetailSkillListStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
            gap: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-skill-list";
}

pub struct TrDetailSkillSourceStyle;

impl Style for TrDetailSkillSourceStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 9px;
            color: #64748b;
            margin-top: 2px;
            margin-left: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-skill-source";
}

pub struct TrDetailSkillUnlockedLabelStyle;

impl Style for TrDetailSkillUnlockedLabelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 10px;
            color: #34d399;
            margin-bottom: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-skill-unlocked-label";
}

pub struct TrDetailSkillLockedLabelStyle;

impl Style for TrDetailSkillLockedLabelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 10px;
            color: #f87171;
            margin-bottom: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-skill-locked-label";
}

pub struct TrDetailSkillLockedContainerStyle;

impl Style for TrDetailSkillLockedContainerStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
            gap: 4px;
            opacity: 0.4;
            margin-bottom: 6px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-skill-locked-container";
}

pub struct TrDetailSkillLockedBadgeStyle;

impl Style for TrDetailSkillLockedBadgeStyle {
    const CSS: &'static str = r#"
        {{class}} {
            position: absolute;
            top: -4px;
            right: -4px;
            font-size: 9px;
            background: #f87171;
            color: #fff;
            padding: 1px 5px;
            border-radius: 3px;
            font-weight: 600;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-skill-locked-badge";
}

pub struct TrDetailEmptyStyle;

impl Style for TrDetailEmptyStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #64748b;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-empty";
}

pub struct TrDetailEventSectionStyle;

impl Style for TrDetailEventSectionStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin-bottom: 20px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-event-section";
}

pub struct TrDetailEventSectionHeaderStyle;

impl Style for TrDetailEventSectionHeaderStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 0.85em;
            font-weight: 600;
            margin-bottom: 8px;
            padding-bottom: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-event-section-header";
}

pub struct TrDetailEventCardStyle;

impl Style for TrDetailEventCardStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin-bottom: 10px;
            padding: 10px;
            background: #0f172a;
            border-radius: 8px;
            border: 1px solid #1e293b;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-event-card";
}

pub struct TrDetailEventNameContainerStyle;

impl Style for TrDetailEventNameContainerStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: flex-start;
            flex-direction: column;
            margin-bottom: 6px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-event-name-container";
}

pub struct TrDetailEventNameStyle;

impl Style for TrDetailEventNameStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-weight: 600;
            font-size: 0.9em;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-event-name";
}

pub struct TrDetailEventConditionStyle;

impl Style for TrDetailEventConditionStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 0.75em;
            color: #fbbf24;
            margin-top: 2px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-event-condition";
}

pub struct TrDetailChoiceWrapperStyle;

impl Style for TrDetailChoiceWrapperStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin-left: 12px;
            margin-bottom: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-choice-wrapper";
}

pub struct TrDetailChoiceHeaderStyle;

impl Style for TrDetailChoiceHeaderStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 0.8em;
            color: #94a3b8;
            margin-bottom: 3px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-choice-header";
}

pub struct TrDetailBranchWrapperStyle;

impl Style for TrDetailBranchWrapperStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin-bottom: 6px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-branch-wrapper";
}

pub struct TrDetailProbLabelStyle;

impl Style for TrDetailProbLabelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 0.75em;
            color: #fbbf24;
            margin-bottom: 2px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-prob-label";
}

pub struct TrDetailRewardListStyle;

impl Style for TrDetailRewardListStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-wrap: wrap;
            gap: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-reward-list";
}

pub struct TrDetailRewardPillStyle;

impl Style for TrDetailRewardPillStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 0.8em;
            padding: 2px 6px;
            border-radius: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "tr-detail-reward-pill";
}

inventory::submit! { StyleDefinition { css: TraineeBrowserRootStyle::CSS, selector_type: TraineeBrowserRootStyle::SELECTOR_TYPE, class_name: TraineeBrowserRootStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrBrowserHeaderStyle::CSS, selector_type: TrBrowserHeaderStyle::SELECTOR_TYPE, class_name: TrBrowserHeaderStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrBrowserHeaderControlsStyle::CSS, selector_type: TrBrowserHeaderControlsStyle::SELECTOR_TYPE, class_name: TrBrowserHeaderControlsStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrBrowserBodyStyle::CSS, selector_type: TrBrowserBodyStyle::SELECTOR_TYPE, class_name: TrBrowserBodyStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrBrowserSidebarStyle::CSS, selector_type: TrBrowserSidebarStyle::SELECTOR_TYPE, class_name: TrBrowserSidebarStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrBrowserMainStyle::CSS, selector_type: TrBrowserMainStyle::SELECTOR_TYPE, class_name: TrBrowserMainStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrBrowserTotalStyle::CSS, selector_type: TrBrowserTotalStyle::SELECTOR_TYPE, class_name: TrBrowserTotalStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrCardGridStyle::CSS, selector_type: TrCardGridStyle::SELECTOR_TYPE, class_name: TrCardGridStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrBrowserEmptyStyle::CSS, selector_type: TrBrowserEmptyStyle::SELECTOR_TYPE, class_name: TrBrowserEmptyStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrBrowserLoadingStyle::CSS, selector_type: TrBrowserLoadingStyle::SELECTOR_TYPE, class_name: TrBrowserLoadingStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrBrowserErrorStyle::CSS, selector_type: TrBrowserErrorStyle::SELECTOR_TYPE, class_name: TrBrowserErrorStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TraineeCardStyle::CSS, selector_type: TraineeCardStyle::SELECTOR_TYPE, class_name: TraineeCardStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TraineeNameStyle::CSS, selector_type: TraineeNameStyle::SELECTOR_TYPE, class_name: TraineeNameStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TraineeRarityRowStyle::CSS, selector_type: TraineeRarityRowStyle::SELECTOR_TYPE, class_name: TraineeRarityRowStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TraineeCharNameStyle::CSS, selector_type: TraineeCharNameStyle::SELECTOR_TYPE, class_name: TraineeCharNameStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TraineePieceBarStyle::CSS, selector_type: TraineePieceBarStyle::SELECTOR_TYPE, class_name: TraineePieceBarStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TraineePieceFillStyle::CSS, selector_type: TraineePieceFillStyle::SELECTOR_TYPE, class_name: TraineePieceFillStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TraineePieceLabelStyle::CSS, selector_type: TraineePieceLabelStyle::SELECTOR_TYPE, class_name: TraineePieceLabelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TraineeIdStyle::CSS, selector_type: TraineeIdStyle::SELECTOR_TYPE, class_name: TraineeIdStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TraineeSelectBtnStyle::CSS, selector_type: TraineeSelectBtnStyle::SELECTOR_TYPE, class_name: TraineeSelectBtnStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailSectionLabelStyle::CSS, selector_type: TrDetailSectionLabelStyle::SELECTOR_TYPE, class_name: TrDetailSectionLabelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailStatsGridStyle::CSS, selector_type: TrDetailStatsGridStyle::SELECTOR_TYPE, class_name: TrDetailStatsGridStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailStatCardStyle::CSS, selector_type: TrDetailStatCardStyle::SELECTOR_TYPE, class_name: TrDetailStatCardStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailStatCardLabelStyle::CSS, selector_type: TrDetailStatCardLabelStyle::SELECTOR_TYPE, class_name: TrDetailStatCardLabelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailStatCardValueStyle::CSS, selector_type: TrDetailStatCardValueStyle::SELECTOR_TYPE, class_name: TrDetailStatCardValueStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailGrowthCardStyle::CSS, selector_type: TrDetailGrowthCardStyle::SELECTOR_TYPE, class_name: TrDetailGrowthCardStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailGrowthCardLabelStyle::CSS, selector_type: TrDetailGrowthCardLabelStyle::SELECTOR_TYPE, class_name: TrDetailGrowthCardLabelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailGrowthCardValueStyle::CSS, selector_type: TrDetailGrowthCardValueStyle::SELECTOR_TYPE, class_name: TrDetailGrowthCardValueStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailAptSectionStyle::CSS, selector_type: TrDetailAptSectionStyle::SELECTOR_TYPE, class_name: TrDetailAptSectionStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailAptGridStyle::CSS, selector_type: TrDetailAptGridStyle::SELECTOR_TYPE, class_name: TrDetailAptGridStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailAptCardStyle::CSS, selector_type: TrDetailAptCardStyle::SELECTOR_TYPE, class_name: TrDetailAptCardStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailAptCardLabelStyle::CSS, selector_type: TrDetailAptCardLabelStyle::SELECTOR_TYPE, class_name: TrDetailAptCardLabelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailAptCardValueStyle::CSS, selector_type: TrDetailAptCardValueStyle::SELECTOR_TYPE, class_name: TrDetailAptCardValueStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailSkillSectionStyle::CSS, selector_type: TrDetailSkillSectionStyle::SELECTOR_TYPE, class_name: TrDetailSkillSectionStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailSkillSectionLabelStyle::CSS, selector_type: TrDetailSkillSectionLabelStyle::SELECTOR_TYPE, class_name: TrDetailSkillSectionLabelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailSkillListStyle::CSS, selector_type: TrDetailSkillListStyle::SELECTOR_TYPE, class_name: TrDetailSkillListStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailSkillSourceStyle::CSS, selector_type: TrDetailSkillSourceStyle::SELECTOR_TYPE, class_name: TrDetailSkillSourceStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailSkillUnlockedLabelStyle::CSS, selector_type: TrDetailSkillUnlockedLabelStyle::SELECTOR_TYPE, class_name: TrDetailSkillUnlockedLabelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailSkillLockedLabelStyle::CSS, selector_type: TrDetailSkillLockedLabelStyle::SELECTOR_TYPE, class_name: TrDetailSkillLockedLabelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailSkillLockedContainerStyle::CSS, selector_type: TrDetailSkillLockedContainerStyle::SELECTOR_TYPE, class_name: TrDetailSkillLockedContainerStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailSkillLockedBadgeStyle::CSS, selector_type: TrDetailSkillLockedBadgeStyle::SELECTOR_TYPE, class_name: TrDetailSkillLockedBadgeStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailEmptyStyle::CSS, selector_type: TrDetailEmptyStyle::SELECTOR_TYPE, class_name: TrDetailEmptyStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailEventSectionStyle::CSS, selector_type: TrDetailEventSectionStyle::SELECTOR_TYPE, class_name: TrDetailEventSectionStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailEventSectionHeaderStyle::CSS, selector_type: TrDetailEventSectionHeaderStyle::SELECTOR_TYPE, class_name: TrDetailEventSectionHeaderStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailEventCardStyle::CSS, selector_type: TrDetailEventCardStyle::SELECTOR_TYPE, class_name: TrDetailEventCardStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailEventNameContainerStyle::CSS, selector_type: TrDetailEventNameContainerStyle::SELECTOR_TYPE, class_name: TrDetailEventNameContainerStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailEventNameStyle::CSS, selector_type: TrDetailEventNameStyle::SELECTOR_TYPE, class_name: TrDetailEventNameStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailEventConditionStyle::CSS, selector_type: TrDetailEventConditionStyle::SELECTOR_TYPE, class_name: TrDetailEventConditionStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailChoiceWrapperStyle::CSS, selector_type: TrDetailChoiceWrapperStyle::SELECTOR_TYPE, class_name: TrDetailChoiceWrapperStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailChoiceHeaderStyle::CSS, selector_type: TrDetailChoiceHeaderStyle::SELECTOR_TYPE, class_name: TrDetailChoiceHeaderStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailBranchWrapperStyle::CSS, selector_type: TrDetailBranchWrapperStyle::SELECTOR_TYPE, class_name: TrDetailBranchWrapperStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailProbLabelStyle::CSS, selector_type: TrDetailProbLabelStyle::SELECTOR_TYPE, class_name: TrDetailProbLabelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailRewardListStyle::CSS, selector_type: TrDetailRewardListStyle::SELECTOR_TYPE, class_name: TrDetailRewardListStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrDetailRewardPillStyle::CSS, selector_type: TrDetailRewardPillStyle::SELECTOR_TYPE, class_name: TrDetailRewardPillStyle::CLASS_NAME } }
