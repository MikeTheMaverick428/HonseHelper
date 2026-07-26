use crate::styles::{Style, StyleDefinition};

pub struct RaceDumpRootStyle;

impl Style for RaceDumpRootStyle {
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

    const CLASS_NAME: &'static str = "race-dump-browser";
}

pub struct RaceDumpHeaderStyle;

impl Style for RaceDumpHeaderStyle {
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

    const CLASS_NAME: &'static str = "race-dump-header";
}

pub struct RaceDumpHeaderControlsStyle;

impl Style for RaceDumpHeaderControlsStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            gap: 12px;
        }
    "#;

    const CLASS_NAME: &'static str = "race-dump-header-controls";
}

pub struct RaceDumpBodyStyle;

impl Style for RaceDumpBodyStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex: 1;
            display: flex;
            overflow: hidden;
        }

        @media (max-width: 768px) {
            {{class}} {
                flex-direction: column;
            }
        }
    "#;

    const CLASS_NAME: &'static str = "race-dump-body";
}

pub struct RaceDumpSidebarStyle;

impl Style for RaceDumpSidebarStyle {
    const CSS: &'static str = r#"
        {{class}} {
            width: 280px;
            flex-shrink: 0;
            overflow-y: auto;
            border-right: 1px solid #1f2937;
            padding: 12px;
            background: #0b1220;
        }

        {{class}}::-webkit-scrollbar { width: 6px; }
        {{class}}::-webkit-scrollbar-track { background: transparent; }
        {{class}}::-webkit-scrollbar-thumb { background: #374151; border-radius: 3px; }

        @media (max-width: 768px) {
            {{class}} {
                width: 100%;
                max-height: 200px;
                border-right: none;
                border-bottom: 1px solid #1f2937;
            }
        }
    "#;

    const CLASS_NAME: &'static str = "race-dump-sidebar";
}

pub struct RaceDumpMainStyle;

impl Style for RaceDumpMainStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex: 1;
            display: flex;
            flex-direction: column;
            overflow: hidden;
            padding: 16px 20px;
        }

        {{class}}::-webkit-scrollbar { width: 10px; }
        {{class}}::-webkit-scrollbar-track { background: transparent; }
        {{class}}::-webkit-scrollbar-thumb {
            background: #4b5563;
            border-radius: 5px;
            border: 3px solid transparent;
            background-clip: content-box;
        }
    "#;

    const CLASS_NAME: &'static str = "race-dump-main";
}

pub struct RaceDumpLoadingStyle;

impl Style for RaceDumpLoadingStyle {
    const CSS: &'static str = r#"
        {{class}} {
            padding: 40px;
            text-align: center;
            color: #94a3b8;
            font-size: 15px;
        }
    "#;

    const CLASS_NAME: &'static str = "race-dump-loading";
}

pub struct RaceDumpErrorStyle;

impl Style for RaceDumpErrorStyle {
    const CSS: &'static str = r#"
        {{class}} {
            padding: 40px;
            text-align: center;
            color: #fca5a5;
            font-size: 15px;
        }
    "#;

    const CLASS_NAME: &'static str = "race-dump-error";
}

pub struct RaceDumpEmptyStyle;

impl Style for RaceDumpEmptyStyle {
    const CSS: &'static str = r#"
        {{class}} {
            padding: 40px;
            text-align: center;
            color: #94a3b8;
            font-size: 15px;
        }
    "#;

    const CLASS_NAME: &'static str = "race-dump-empty";
}

pub struct RaceDumpTotalStyle;

impl Style for RaceDumpTotalStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin-bottom: 12px;
            font-size: 13px;
            color: #94a3b8;
        }
    "#;

    const CLASS_NAME: &'static str = "race-dump-total";
}

pub struct RaceDumpCardGridStyle;

impl Style for RaceDumpCardGridStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
            gap: 12px;
        }

        @media (max-width: 768px) {
            {{class}} {
                grid-template-columns: 1fr;
            }
        }
    "#;

    const CLASS_NAME: &'static str = "race-dump-card-grid";
}

pub struct TypeBadgeTeamStadiumStyle;

impl Style for TypeBadgeTeamStadiumStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-block;
            padding: 2px 8px;
            border-radius: 4px;
            font-size: 11px;
            font-weight: 600;
            background: #1e3a5f;
            color: #60a5fa;
        }
    "#;
    const CLASS_NAME: &'static str = "badge-team-stadium";
}

pub struct TypeBadgeSingleStyle;

impl Style for TypeBadgeSingleStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-block;
            padding: 2px 8px;
            border-radius: 4px;
            font-size: 11px;
            font-weight: 600;
            background: #1a3a2a;
            color: #4ade80;
        }
    "#;
    const CLASS_NAME: &'static str = "badge-single";
}

pub struct TypeBadgeUnknownStyle;

impl Style for TypeBadgeUnknownStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-block;
            padding: 2px 8px;
            border-radius: 4px;
            font-size: 11px;
            font-weight: 600;
            background: #2d2d2d;
            color: #94a3b8;
        }
    "#;
    const CLASS_NAME: &'static str = "badge-unknown";
}

pub struct TypeBadgeRoomMatchStyle;

impl Style for TypeBadgeRoomMatchStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-block;
            padding: 2px 8px;
            border-radius: 4px;
            font-size: 11px;
            font-weight: 600;
            background: #2d1b4e;
            color: #c084fc;
        }
    "#;
    const CLASS_NAME: &'static str = "badge-room-match";
}

pub struct TypeBadgeChampionsStyle;

impl Style for TypeBadgeChampionsStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-block;
            padding: 2px 8px;
            border-radius: 4px;
            font-size: 11px;
            font-weight: 600;
            background: #3d2e00;
            color: #fbbf24;
        }
    "#;
    const CLASS_NAME: &'static str = "badge-champions";
}

pub struct DeleteBtnStyle;

impl Style for DeleteBtnStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: none;
            border: 1px solid transparent;
            color: #64748b;
            cursor: pointer;
            padding: 2px 6px;
            border-radius: 4px;
            font-size: 14px;
            line-height: 1;
            transition: all 0.15s;
        }
        {{class}}:hover {
            color: #ef4444;
            background: rgba(239, 68, 68, 0.1);
            border-color: rgba(239, 68, 68, 0.3);
        }
    "#;
    const CLASS_NAME: &'static str = "delete-btn";
}

pub struct PlayerBadgeStyle;

impl Style for PlayerBadgeStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-block;
            background: #2d1b69;
            color: #a78bfa;
            padding: 1px 7px;
            border-radius: 4px;
            font-size: 11px;
            font-weight: 600;
        }
    "#;
    const CLASS_NAME: &'static str = "player-badge";
}

pub struct PlayerNameStyle;

impl Style for PlayerNameStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-block;
            color: #a78bfa;
            font-size: 12px;
        }
        {{class}} + {{class}}::before {
            content: ", ";
            color: #64748b;
        }
    "#;
    const CLASS_NAME: &'static str = "player-name";
}

inventory::submit! { StyleDefinition { css: RaceDumpRootStyle::CSS, selector_type: RaceDumpRootStyle::SELECTOR_TYPE, class_name: RaceDumpRootStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RaceDumpHeaderStyle::CSS, selector_type: RaceDumpHeaderStyle::SELECTOR_TYPE, class_name: RaceDumpHeaderStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RaceDumpHeaderControlsStyle::CSS, selector_type: RaceDumpHeaderControlsStyle::SELECTOR_TYPE, class_name: RaceDumpHeaderControlsStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RaceDumpBodyStyle::CSS, selector_type: RaceDumpBodyStyle::SELECTOR_TYPE, class_name: RaceDumpBodyStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RaceDumpSidebarStyle::CSS, selector_type: RaceDumpSidebarStyle::SELECTOR_TYPE, class_name: RaceDumpSidebarStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RaceDumpMainStyle::CSS, selector_type: RaceDumpMainStyle::SELECTOR_TYPE, class_name: RaceDumpMainStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RaceDumpLoadingStyle::CSS, selector_type: RaceDumpLoadingStyle::SELECTOR_TYPE, class_name: RaceDumpLoadingStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RaceDumpErrorStyle::CSS, selector_type: RaceDumpErrorStyle::SELECTOR_TYPE, class_name: RaceDumpErrorStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RaceDumpEmptyStyle::CSS, selector_type: RaceDumpEmptyStyle::SELECTOR_TYPE, class_name: RaceDumpEmptyStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RaceDumpTotalStyle::CSS, selector_type: RaceDumpTotalStyle::SELECTOR_TYPE, class_name: RaceDumpTotalStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RaceDumpCardGridStyle::CSS, selector_type: RaceDumpCardGridStyle::SELECTOR_TYPE, class_name: RaceDumpCardGridStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TypeBadgeTeamStadiumStyle::CSS, selector_type: TypeBadgeTeamStadiumStyle::SELECTOR_TYPE, class_name: TypeBadgeTeamStadiumStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TypeBadgeSingleStyle::CSS, selector_type: TypeBadgeSingleStyle::SELECTOR_TYPE, class_name: TypeBadgeSingleStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TypeBadgeUnknownStyle::CSS, selector_type: TypeBadgeUnknownStyle::SELECTOR_TYPE, class_name: TypeBadgeUnknownStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TypeBadgeRoomMatchStyle::CSS, selector_type: TypeBadgeRoomMatchStyle::SELECTOR_TYPE, class_name: TypeBadgeRoomMatchStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TypeBadgeChampionsStyle::CSS, selector_type: TypeBadgeChampionsStyle::SELECTOR_TYPE, class_name: TypeBadgeChampionsStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: DeleteBtnStyle::CSS, selector_type: DeleteBtnStyle::SELECTOR_TYPE, class_name: DeleteBtnStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PlayerBadgeStyle::CSS, selector_type: PlayerBadgeStyle::SELECTOR_TYPE, class_name: PlayerBadgeStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PlayerNameStyle::CSS, selector_type: PlayerNameStyle::SELECTOR_TYPE, class_name: PlayerNameStyle::CLASS_NAME } }
