use crate::styles::{Style, StyleDefinition};

pub struct TrainerBrowserRootStyle;

impl Style for TrainerBrowserRootStyle {
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

    const CLASS_NAME: &'static str = "trainer-browser";
}

pub struct TrainerBrowserHeaderStyle;

impl Style for TrainerBrowserHeaderStyle {
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

    const CLASS_NAME: &'static str = "trainer-browser-header";
}

pub struct TrainerBrowserHeaderControlsStyle;

impl Style for TrainerBrowserHeaderControlsStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            gap: 10px;
            flex-wrap: wrap;
        }
    "#;

    const CLASS_NAME: &'static str = "trainer-browser-header-controls";
}

pub struct TrainerBrowserBodyStyle;

impl Style for TrainerBrowserBodyStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex: 1;
            overflow: hidden;
        }
    "#;

    const CLASS_NAME: &'static str = "trainer-browser-body";
}

pub struct TrainerBrowserSidebarStyle;

impl Style for TrainerBrowserSidebarStyle {
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

    const CLASS_NAME: &'static str = "trainer-browser-sidebar";
}

pub struct TrainerBrowserMainStyle;

impl Style for TrainerBrowserMainStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex: 1;
            display: flex;
            flex-direction: column;
            overflow: hidden;
            padding: 16px 20px;
        }
    "#;

    const CLASS_NAME: &'static str = "trainer-browser-main";
}

pub struct TrainerBrowserEmptyStyle;

impl Style for TrainerBrowserEmptyStyle {
    const CSS: &'static str = r#"
        {{class}} {
            text-align: center;
            padding: 60px 20px;
            color: #6b7280;
            font-size: 15px;
        }
    "#;

    const CLASS_NAME: &'static str = "trainer-browser-empty";
}

pub struct TrainerBrowserLoadingStyle;

impl Style for TrainerBrowserLoadingStyle {
    const CSS: &'static str = r#"
        {{class}} {
            text-align: center;
            padding: 60px 20px;
            color: #9ca3af;
            font-size: 15px;
        }
    "#;

    const CLASS_NAME: &'static str = "trainer-browser-loading";
}

pub struct TrainerBrowserErrorStyle;

impl Style for TrainerBrowserErrorStyle {
    const CSS: &'static str = r#"
        {{class}} {
            text-align: center;
            padding: 60px 20px;
            color: #f87171;
            font-size: 15px;
        }
    "#;

    const CLASS_NAME: &'static str = "trainer-browser-error";
}

pub struct TrainerRowStyle;

impl Style for TrainerRowStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: stretch;
            gap: 16px;
            padding: 12px 16px;
            border: 1px solid #1f2937;
            border-radius: 8px;
            background: #141828;
            margin-bottom: 8px;
        }
    "#;

    const CLASS_NAME: &'static str = "trainer-row";
}

pub struct TrainerNameStyle;

impl Style for TrainerNameStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 15px;
            font-weight: 600;
            color: #e2e8f0;
        }
    "#;

    const CLASS_NAME: &'static str = "trainer-name";
}

pub struct TrainerIdStyle;

impl Style for TrainerIdStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 11px;
            color: #64748b;
            font-family: ui-monospace, monospace;
        }
    "#;

    const CLASS_NAME: &'static str = "trainer-id";
}

pub struct TrainerMetaStyle;

impl Style for TrainerMetaStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 12px;
            color: #9ca3af;
            display: flex;
            gap: 12px;
            flex-wrap: wrap;
        }
    "#;

    const CLASS_NAME: &'static str = "trainer-meta";
}

pub struct TrainerBorrowBlockStyle;

impl Style for TrainerBorrowBlockStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
            gap: 2px;
            font-size: 12px;
            color: #cbd5e1;
            min-width: 140px;
            flex: 1;
        }

        {{class}} .borrow-label {
            font-size: 10px;
            text-transform: uppercase;
            letter-spacing: 0.05em;
            color: #64748b;
        }

        {{class}}.clickable {
            cursor: pointer;
            border-radius: 6px;
            padding: 4px 6px;
            margin: -4px -6px;
            transition: background 0.15s;
        }

        {{class}}.clickable:hover {
            background: rgba(59, 130, 246, 0.08);
        }
    "#;

    const CLASS_NAME: &'static str = "trainer-borrow-block";
}

pub struct TrainerSubCardStyle;

impl Style for TrainerSubCardStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
            align-items: flex-start;
            gap: 4px;
            width: 100%;
            box-sizing: border-box;
            flex: 1;
            padding: 10px 12px;
            background: linear-gradient(180deg, #10182b 0%, #0f172a 100%);
            border: 1px solid #263246;
            border-radius: 12px;
        }
    "#;

    const CLASS_NAME: &'static str = "trainer-sub-card";
}

pub struct TrainerSubCardHeaderStyle;

impl Style for TrainerSubCardHeaderStyle {
    const CSS: &'static str = r#"
        {{class}} {
            width: 100%;
            display: flex;
            justify-content: space-between;
            align-items: flex-start;
            gap: 8px;
        }
    "#;

    const CLASS_NAME: &'static str = "trainer-sub-card-header";
}

pub struct TrainerFollowingBadgeStyle;

impl Style for TrainerFollowingBadgeStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 11px;
            padding: 2px 8px;
            border-radius: 999px;
            border: 1px solid #334155;
            color: #94a3b8;
        }

        {{class}}.following {
            border-color: #22c55e;
            color: #4ade80;
        }
    "#;

    const CLASS_NAME: &'static str = "trainer-following-badge";
}

pub struct TrainerAddPanelStyle;

impl Style for TrainerAddPanelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            gap: 8px;
        }

        {{class}} input {
            width: 180px;
        }
    "#;

    const CLASS_NAME: &'static str = "trainer-add-panel";
}

pub struct TrainerIdListStyle;

impl Style for TrainerIdListStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-wrap: wrap;
            gap: 4px;
            margin-top: 6px;
        }
    "#;

    const CLASS_NAME: &'static str = "trainer-id-list";
}

inventory::submit! { StyleDefinition { css: TrainerBrowserRootStyle::CSS, selector_type: TrainerBrowserRootStyle::SELECTOR_TYPE, class_name: TrainerBrowserRootStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrainerBrowserHeaderStyle::CSS, selector_type: TrainerBrowserHeaderStyle::SELECTOR_TYPE, class_name: TrainerBrowserHeaderStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrainerBrowserHeaderControlsStyle::CSS, selector_type: TrainerBrowserHeaderControlsStyle::SELECTOR_TYPE, class_name: TrainerBrowserHeaderControlsStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrainerBrowserBodyStyle::CSS, selector_type: TrainerBrowserBodyStyle::SELECTOR_TYPE, class_name: TrainerBrowserBodyStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrainerBrowserSidebarStyle::CSS, selector_type: TrainerBrowserSidebarStyle::SELECTOR_TYPE, class_name: TrainerBrowserSidebarStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrainerBrowserMainStyle::CSS, selector_type: TrainerBrowserMainStyle::SELECTOR_TYPE, class_name: TrainerBrowserMainStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrainerBrowserEmptyStyle::CSS, selector_type: TrainerBrowserEmptyStyle::SELECTOR_TYPE, class_name: TrainerBrowserEmptyStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrainerBrowserLoadingStyle::CSS, selector_type: TrainerBrowserLoadingStyle::SELECTOR_TYPE, class_name: TrainerBrowserLoadingStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrainerBrowserErrorStyle::CSS, selector_type: TrainerBrowserErrorStyle::SELECTOR_TYPE, class_name: TrainerBrowserErrorStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrainerRowStyle::CSS, selector_type: TrainerRowStyle::SELECTOR_TYPE, class_name: TrainerRowStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrainerNameStyle::CSS, selector_type: TrainerNameStyle::SELECTOR_TYPE, class_name: TrainerNameStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrainerIdStyle::CSS, selector_type: TrainerIdStyle::SELECTOR_TYPE, class_name: TrainerIdStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrainerMetaStyle::CSS, selector_type: TrainerMetaStyle::SELECTOR_TYPE, class_name: TrainerMetaStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrainerBorrowBlockStyle::CSS, selector_type: TrainerBorrowBlockStyle::SELECTOR_TYPE, class_name: TrainerBorrowBlockStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrainerSubCardStyle::CSS, selector_type: TrainerSubCardStyle::SELECTOR_TYPE, class_name: TrainerSubCardStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrainerSubCardHeaderStyle::CSS, selector_type: TrainerSubCardHeaderStyle::SELECTOR_TYPE, class_name: TrainerSubCardHeaderStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrainerFollowingBadgeStyle::CSS, selector_type: TrainerFollowingBadgeStyle::SELECTOR_TYPE, class_name: TrainerFollowingBadgeStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrainerAddPanelStyle::CSS, selector_type: TrainerAddPanelStyle::SELECTOR_TYPE, class_name: TrainerAddPanelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TrainerIdListStyle::CSS, selector_type: TrainerIdListStyle::SELECTOR_TYPE, class_name: TrainerIdListStyle::CLASS_NAME } }
