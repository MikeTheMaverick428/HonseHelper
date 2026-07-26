use crate::styles::{Style, StyleDefinition};

pub struct DetailContainerStyle;

impl Style for DetailContainerStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
            height: 100vh;
            background: #0f1220;
            color: #f3f4f6;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-container";
}

pub struct DetailHeaderStyle;

impl Style for DetailHeaderStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex-shrink: 0;
            padding: 14px 20px;
            border-bottom: 1px solid #1f2937;
            display: flex;
            align-items: center;
            gap: 16px;
        }
        {{class}} h2 {
            margin: 0;
            font-size: 20px;
            font-weight: 600;
        }
        {{class}} .rdd-header-meta {
            font-size: 12px;
            color: #94a3b8;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-header";
}

pub struct DetailBodyStyle;

impl Style for DetailBodyStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex: 1;
            min-height: 0;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-body";
}

pub struct ParticipantsPanelStyle;

impl Style for ParticipantsPanelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            width: 320px;
            flex-shrink: 0;
            border-right: 1px solid #1f2937;
            display: flex;
            flex-direction: column;
            overflow: hidden;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-participants-panel";
}

pub struct ParticipantsRowsStyle;

impl Style for ParticipantsRowsStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
            height: 100%;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-pp-rows";
}

pub struct PPRowStyle;

impl Style for PPRowStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex: 1;
            display: flex;
            align-items: stretch;
            padding: 2px 8px;
            border-bottom: 1px solid #1a1f33;
            min-height: 0;
            transition: background 0.15s;
        }
        {{class}}:hover {
            background: #1a1f33;
        }
        {{class}} .rdd-pp-body {
            flex: 1;
            display: flex;
            flex-direction: column;
            min-width: 0;
        }
        {{class}} .rdd-pp-top {
            display: flex;
            align-items: center;
            justify-content: space-between;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-pp-row";
}

pub struct PPSpeedStyle;

impl Style for PPSpeedStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 11px;
            color: #94a3b8;
            margin-right: 8px;
            flex-shrink: 0;
            font-variant-numeric: tabular-nums;
            display: flex;
            align-items: center;
            width: 36px;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-pp-speed";
}

pub struct PPNameStyle;

impl Style for PPNameStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-weight: 500;
            font-size: 12px;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-pp-name";
}

pub struct PPStatusStyle;

impl Style for PPStatusStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            gap: 3px;
            flex-shrink: 0;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-pp-status";
}

pub struct PPBadgeStyle;

impl Style for PPBadgeStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-block;
            padding: 0 4px;
            border-radius: 2px;
            font-size: 10px;
            font-weight: 600;
            line-height: 1.5;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-pp-badge";
}

pub struct PPBlockedStyle;

impl Style for PPBlockedStyle {
    const CSS: &'static str = r#"
        {{class}} { background: rgba(239,68,68,0.2); color: #ef4444; }
    "#;
    const CLASS_NAME: &'static str = "rdd-pp-blocked";
}

pub struct PPRushedStyle;

impl Style for PPRushedStyle {
    const CSS: &'static str = r#"
        {{class}} { background: rgba(255,170,0,0.2); color: #f90; }
    "#;
    const CLASS_NAME: &'static str = "rdd-pp-rushed";
}

pub struct PPSkillStyle;

impl Style for PPSkillStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: rgba(139,92,246,0.2);
            color: #a78bfa;
            max-width: 100px;
            overflow: hidden;
            text-overflow: ellipsis;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-pp-skill";
}

pub struct PPEventsStyle;

impl Style for PPEventsStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-wrap: wrap;
            gap: 2px;
            flex: 1;
            align-items: flex-end;
            padding-bottom: 1px;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-pp-events";
}

pub struct ParticipantRowPlayerStyle;

impl Style for ParticipantRowPlayerStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: rgba(139, 92, 246, 0.08);
        }
        {{class}}:hover {
            background: rgba(139, 92, 246, 0.15) !important;
        }
        {{class}} .rdd-pp-name {
            color: #c4b5fd;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-participant-player";
}

pub struct ReplayPanelStyle;

impl Style for ReplayPanelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex: 1;
            display: flex;
            flex-direction: column;
            min-width: 0;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-replay-panel";
}

pub struct CanvasContainerStyle;

impl Style for CanvasContainerStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex: 1;
            position: relative;
            min-height: 0;
            background: #080c14;
        }
        {{class}} canvas {
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-canvas-container";
}

pub struct ControlBarStyle;

impl Style for ControlBarStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex-shrink: 0;
            display: flex;
            align-items: center;
            gap: 10px;
            padding: 10px 16px;
            border-top: 1px solid #1f2937;
            background: #13182a;
        }
        {{class}} button {
            background: #1e293b;
            border: 1px solid #334155;
            color: #f3f4f6;
            cursor: pointer;
            padding: 4px 10px;
            border-radius: 4px;
            font-size: 13px;
            transition: all 0.15s;
        }
        {{class}} button:hover {
            background: #334155;
        }
        {{class}} input[type=range] {
            flex: 1;
            accent-color: #8b5cf6;
        }
        {{class}} select {
            background: #1e293b;
            color: #f3f4f6;
            border: 1px solid #334155;
            border-radius: 4px;
            padding: 3px 20px 3px 6px;
            font-size: 12px;
            appearance: none;
            cursor: pointer;
        }
        {{class}} .rdd-control-label {
            font-size: 12px;
            color: #94a3b8;
            min-width: 60px;
            text-align: right;
        }
        {{class}} .rdd-frame-counter {
            font-size: 11px;
            color: #64748b;
            min-width: 70px;
            text-align: center;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-control-bar";
}

pub struct LoadingOverlayStyle;

impl Style for LoadingOverlayStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            justify-content: center;
            height: 100%;
            color: #64748b;
            font-size: 14px;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-loading";
}

pub struct ErrorOverlayStyle;

impl Style for ErrorOverlayStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            justify-content: center;
            height: 100%;
            color: #ef4444;
            font-size: 14px;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-error";
}

pub struct DetailTabsStyle;

impl Style for DetailTabsStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            background: #13182a;
            border-bottom: 1px solid #334155;
            padding: 0 20px;
            gap: 0;
            flex-shrink: 0;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-tabs";
}

pub struct DetailTabBtnStyle;

impl Style for DetailTabBtnStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: none;
            border: none;
            color: #94a3b8;
            padding: 10px 20px;
            font-size: 13px;
            cursor: pointer;
            border-bottom: 2px solid transparent;
            margin-bottom: -1px;
            transition: color 0.15s, border-color 0.15s, background 0.15s;
            border-radius: 0;
        }
        {{class}}:hover {
            color: #e2e8f0;
            background: rgba(255, 255, 255, 0.03);
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-tab-btn";
}

pub struct DetailTabActiveStyle;

impl Style for DetailTabActiveStyle {
    const CSS: &'static str = r#"
        .rdd-tab-btn{{class}} {
            color: #93c5fd;
            border-bottom-color: #93c5fd;
            background: rgba(147, 197, 253, 0.06);
        }
        .rdd-tab-btn{{class}}:hover {
            color: #93c5fd;
            background: rgba(147, 197, 253, 0.08);
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-tab-active";
}

pub struct PartTabStyle;

impl Style for PartTabStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex: 1;
            overflow-y: auto;
            padding: 12px 20px;
            display: flex;
            flex-direction: column;
            gap: 8px;
        }
        {{class}}::-webkit-scrollbar { width: 8px; }
        {{class}}::-webkit-scrollbar-track { background: transparent; }
        {{class}}::-webkit-scrollbar-thumb {
            background: #4b5563;
            border-radius: 4px;
            border: 2px solid transparent;
            background-clip: content-box;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-part-tab";
}

pub struct PartRowStyle;

impl Style for PartRowStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            gap: 16px;
            background: linear-gradient(180deg, #10182b 0%, #0f172a 100%);
            border: 1px solid #263246;
            border-radius: 10px;
            padding: 12px 16px;
            transition: border-color 0.15s;
        }
        {{class}}:hover {
            border-color: #3b4b6b;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-part-row";
}

pub struct PartFinishStyle;

impl Style for PartFinishStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 22px;
            font-weight: 700;
            min-width: 52px;
            text-align: center;
            flex-shrink: 0;
            font-feature-settings: 'tnum' 1;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-part-finish";
}

pub struct PartFinish1stStyle;

impl Style for PartFinish1stStyle {
    const CSS: &'static str = r#"
        {{class}} { color: #fbbf24; }
    "#;
    const CLASS_NAME: &'static str = "rdd-part-1st";
}

pub struct PartFinish2ndStyle;

impl Style for PartFinish2ndStyle {
    const CSS: &'static str = r#"
        {{class}} { color: #94a3b8; }
    "#;
    const CLASS_NAME: &'static str = "rdd-part-2nd";
}

pub struct PartFinish3rdStyle;

impl Style for PartFinish3rdStyle {
    const CSS: &'static str = r#"
        {{class}} { color: #d97706; }
    "#;
    const CLASS_NAME: &'static str = "rdd-part-3rd";
}

pub struct PartFinishOtherStyle;

impl Style for PartFinishOtherStyle {
    const CSS: &'static str = r#"
        {{class}} { color: #64748b; }
    "#;
    const CLASS_NAME: &'static str = "rdd-part-other";
}

pub struct PartInfoSectionStyle;

impl Style for PartInfoSectionStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex: 1;
            min-width: 0;
            display: flex;
            flex-direction: column;
            gap: 4px;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-part-info";
}

pub struct PartNameStyle;

impl Style for PartNameStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 15px;
            font-weight: 600;
            color: #f1f5f9;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-part-name";
}

pub struct PartPlayerBadgeStyle;

impl Style for PartPlayerBadgeStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #c4b5fd;
            font-size: 14px;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-part-player-badge";
}

pub struct PartMetaStyle;

impl Style for PartMetaStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            gap: 6px;
            font-size: 12px;
            color: #94a3b8;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-part-meta";
}

pub struct PartRankScoreStyle;

impl Style for PartRankScoreStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #fbbf24;
            font-weight: 600;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-part-rank";
}

pub struct PartNpcBadgeStyle;

impl Style for PartNpcBadgeStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #1e293b;
            padding: 1px 8px;
            border-radius: 4px;
            font-size: 11px;
            color: #64748b;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-part-npc";
}

pub struct PartStatsRowStyle;

impl Style for PartStatsRowStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            gap: 8px;
            flex-wrap: wrap;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-part-stats";
}

pub struct PartStatStyle;

impl Style for PartStatStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 11px;
            color: #94a3b8;
            background: #1e293b;
            padding: 1px 8px;
            border-radius: 4px;
            font-variant-numeric: tabular-nums;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-part-stat";
}

pub struct PartScenarioBadgeStyle;

impl Style for PartScenarioBadgeStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #1e293b;
            padding: 1px 8px;
            border-radius: 4px;
            font-size: 11px;
            color: #a78bfa;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-part-scenario";
}

pub struct PartBorrowedBadgeStyle;

impl Style for PartBorrowedBadgeStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: rgba(251, 191, 36, 0.15);
            color: #fbbf24;
            padding: 1px 8px;
            border-radius: 4px;
            font-size: 11px;
            font-weight: 600;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-part-borrowed";
}

pub struct PartActiveBadgeStyle;

impl Style for PartActiveBadgeStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: rgba(34, 197, 94, 0.15);
            color: #22c55e;
            padding: 1px 8px;
            border-radius: 4px;
            font-size: 11px;
            font-weight: 600;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-part-active";
}

pub struct PartPastBadgeStyle;

impl Style for PartPastBadgeStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: rgba(100, 116, 139, 0.2);
            color: #64748b;
            padding: 1px 8px;
            border-radius: 4px;
            font-size: 11px;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-part-past";
}

pub struct PartDetailBtnStyle;

impl Style for PartDetailBtnStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #1e293b;
            border: 1px solid #334155;
            color: #93c5fd;
            cursor: pointer;
            padding: 4px 12px;
            border-radius: 4px;
            font-size: 12px;
            transition: all 0.15s;
            margin-top: 4px;
            white-space: nowrap;
        }
        {{class}}:hover {
            background: #334155;
            border-color: #93c5fd;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-part-detail-btn";
}

pub struct PartResultSectionStyle;

impl Style for PartResultSectionStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex-shrink: 0;
            text-align: right;
            display: flex;
            flex-direction: column;
            align-items: flex-end;
            gap: 4px;
            min-width: 80px;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-part-result";
}

pub struct PartTimeStyle;

impl Style for PartTimeStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 14px;
            font-weight: 600;
            color: #e2e8f0;
            font-feature-settings: 'tnum' 1;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-part-time";
}

pub struct PartRunningStyleStyle;

impl Style for PartRunningStyleStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 11px;
            color: #64748b;
            background: #1e293b;
            padding: 1px 8px;
            border-radius: 4px;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-part-running-style";
}

pub struct ScrollPanelStyle;

impl Style for ScrollPanelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex: 1;
            overflow-y: auto;
            overflow-x: hidden;
        }
        {{class}}::-webkit-scrollbar { width: 8px; }
        {{class}}::-webkit-scrollbar-track { background: transparent; }
        {{class}}::-webkit-scrollbar-thumb {
            background: #4b5563;
            border-radius: 4px;
            border: 2px solid transparent;
            background-clip: content-box;
        }
    "#;
    const CLASS_NAME: &'static str = "rdd-scroll";
}

inventory::submit! { StyleDefinition { css: DetailContainerStyle::CSS, selector_type: DetailContainerStyle::SELECTOR_TYPE, class_name: DetailContainerStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: DetailHeaderStyle::CSS, selector_type: DetailHeaderStyle::SELECTOR_TYPE, class_name: DetailHeaderStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: DetailBodyStyle::CSS, selector_type: DetailBodyStyle::SELECTOR_TYPE, class_name: DetailBodyStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ParticipantsPanelStyle::CSS, selector_type: ParticipantsPanelStyle::SELECTOR_TYPE, class_name: ParticipantsPanelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ParticipantsRowsStyle::CSS, selector_type: ParticipantsRowsStyle::SELECTOR_TYPE, class_name: ParticipantsRowsStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PPRowStyle::CSS, selector_type: PPRowStyle::SELECTOR_TYPE, class_name: PPRowStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PPSpeedStyle::CSS, selector_type: PPSpeedStyle::SELECTOR_TYPE, class_name: PPSpeedStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PPNameStyle::CSS, selector_type: PPNameStyle::SELECTOR_TYPE, class_name: PPNameStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PPStatusStyle::CSS, selector_type: PPStatusStyle::SELECTOR_TYPE, class_name: PPStatusStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PPBadgeStyle::CSS, selector_type: PPBadgeStyle::SELECTOR_TYPE, class_name: PPBadgeStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PPBlockedStyle::CSS, selector_type: PPBlockedStyle::SELECTOR_TYPE, class_name: PPBlockedStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PPRushedStyle::CSS, selector_type: PPRushedStyle::SELECTOR_TYPE, class_name: PPRushedStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PPSkillStyle::CSS, selector_type: PPSkillStyle::SELECTOR_TYPE, class_name: PPSkillStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PPEventsStyle::CSS, selector_type: PPEventsStyle::SELECTOR_TYPE, class_name: PPEventsStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ParticipantRowPlayerStyle::CSS, selector_type: ParticipantRowPlayerStyle::SELECTOR_TYPE, class_name: ParticipantRowPlayerStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ReplayPanelStyle::CSS, selector_type: ReplayPanelStyle::SELECTOR_TYPE, class_name: ReplayPanelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CanvasContainerStyle::CSS, selector_type: CanvasContainerStyle::SELECTOR_TYPE, class_name: CanvasContainerStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ControlBarStyle::CSS, selector_type: ControlBarStyle::SELECTOR_TYPE, class_name: ControlBarStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LoadingOverlayStyle::CSS, selector_type: LoadingOverlayStyle::SELECTOR_TYPE, class_name: LoadingOverlayStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ErrorOverlayStyle::CSS, selector_type: ErrorOverlayStyle::SELECTOR_TYPE, class_name: ErrorOverlayStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: DetailTabsStyle::CSS, selector_type: DetailTabsStyle::SELECTOR_TYPE, class_name: DetailTabsStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: DetailTabBtnStyle::CSS, selector_type: DetailTabBtnStyle::SELECTOR_TYPE, class_name: DetailTabBtnStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: DetailTabActiveStyle::CSS, selector_type: DetailTabActiveStyle::SELECTOR_TYPE, class_name: DetailTabActiveStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PartTabStyle::CSS, selector_type: PartTabStyle::SELECTOR_TYPE, class_name: PartTabStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PartRowStyle::CSS, selector_type: PartRowStyle::SELECTOR_TYPE, class_name: PartRowStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PartFinishStyle::CSS, selector_type: PartFinishStyle::SELECTOR_TYPE, class_name: PartFinishStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PartFinish1stStyle::CSS, selector_type: PartFinish1stStyle::SELECTOR_TYPE, class_name: PartFinish1stStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PartFinish2ndStyle::CSS, selector_type: PartFinish2ndStyle::SELECTOR_TYPE, class_name: PartFinish2ndStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PartFinish3rdStyle::CSS, selector_type: PartFinish3rdStyle::SELECTOR_TYPE, class_name: PartFinish3rdStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PartFinishOtherStyle::CSS, selector_type: PartFinishOtherStyle::SELECTOR_TYPE, class_name: PartFinishOtherStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PartInfoSectionStyle::CSS, selector_type: PartInfoSectionStyle::SELECTOR_TYPE, class_name: PartInfoSectionStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PartNameStyle::CSS, selector_type: PartNameStyle::SELECTOR_TYPE, class_name: PartNameStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PartPlayerBadgeStyle::CSS, selector_type: PartPlayerBadgeStyle::SELECTOR_TYPE, class_name: PartPlayerBadgeStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PartMetaStyle::CSS, selector_type: PartMetaStyle::SELECTOR_TYPE, class_name: PartMetaStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PartRankScoreStyle::CSS, selector_type: PartRankScoreStyle::SELECTOR_TYPE, class_name: PartRankScoreStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PartNpcBadgeStyle::CSS, selector_type: PartNpcBadgeStyle::SELECTOR_TYPE, class_name: PartNpcBadgeStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PartStatsRowStyle::CSS, selector_type: PartStatsRowStyle::SELECTOR_TYPE, class_name: PartStatsRowStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PartStatStyle::CSS, selector_type: PartStatStyle::SELECTOR_TYPE, class_name: PartStatStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PartScenarioBadgeStyle::CSS, selector_type: PartScenarioBadgeStyle::SELECTOR_TYPE, class_name: PartScenarioBadgeStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PartBorrowedBadgeStyle::CSS, selector_type: PartBorrowedBadgeStyle::SELECTOR_TYPE, class_name: PartBorrowedBadgeStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PartActiveBadgeStyle::CSS, selector_type: PartActiveBadgeStyle::SELECTOR_TYPE, class_name: PartActiveBadgeStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PartPastBadgeStyle::CSS, selector_type: PartPastBadgeStyle::SELECTOR_TYPE, class_name: PartPastBadgeStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PartDetailBtnStyle::CSS, selector_type: PartDetailBtnStyle::SELECTOR_TYPE, class_name: PartDetailBtnStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PartResultSectionStyle::CSS, selector_type: PartResultSectionStyle::SELECTOR_TYPE, class_name: PartResultSectionStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PartTimeStyle::CSS, selector_type: PartTimeStyle::SELECTOR_TYPE, class_name: PartTimeStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PartRunningStyleStyle::CSS, selector_type: PartRunningStyleStyle::SELECTOR_TYPE, class_name: PartRunningStyleStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ScrollPanelStyle::CSS, selector_type: ScrollPanelStyle::SELECTOR_TYPE, class_name: ScrollPanelStyle::CLASS_NAME } }
