use crate::styles::{Style, StyleDefinition};

pub struct SupportCardBrowserRootStyle;

impl Style for SupportCardBrowserRootStyle {
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

    const CLASS_NAME: &'static str = "sc-browser";
}

pub struct ScBrowserHeaderStyle;

impl Style for ScBrowserHeaderStyle {
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

    const CLASS_NAME: &'static str = "sc-browser-header";
}

pub struct ScBrowserHeaderControlsStyle;

impl Style for ScBrowserHeaderControlsStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            gap: 10px;
            flex-wrap: wrap;
        }
    "#;

    const CLASS_NAME: &'static str = "sc-browser-header-controls";
}

pub struct ScBrowserBodyStyle;

impl Style for ScBrowserBodyStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex: 1;
            overflow: hidden;
        }
    "#;

    const CLASS_NAME: &'static str = "sc-browser-body";
}

pub struct ScBrowserSidebarStyle;

impl Style for ScBrowserSidebarStyle {
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

    const CLASS_NAME: &'static str = "sc-browser-sidebar";
}

pub struct ScBrowserMainStyle;

impl Style for ScBrowserMainStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex: 1;
            display: flex;
            flex-direction: column;
            overflow: hidden;
            padding: 16px 20px;
        }
    "#;

    const CLASS_NAME: &'static str = "sc-browser-main";
}

pub struct ScBrowserTotalStyle;

impl Style for ScBrowserTotalStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 13px;
            color: #9ca3af;
            margin-bottom: 12px;
        }
    "#;

    const CLASS_NAME: &'static str = "sc-browser-total";
}

pub struct ScCardGridStyle;

impl Style for ScCardGridStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
            gap: 12px;
        }
    "#;

    const CLASS_NAME: &'static str = "sc-card-grid";
}

pub struct ScBrowserEmptyStyle;

impl Style for ScBrowserEmptyStyle {
    const CSS: &'static str = r#"
        {{class}} {
            text-align: center;
            padding: 60px 20px;
            color: #6b7280;
            font-size: 15px;
        }
    "#;

    const CLASS_NAME: &'static str = "sc-browser-empty";
}

pub struct ScBrowserLoadingStyle;

impl Style for ScBrowserLoadingStyle {
    const CSS: &'static str = r#"
        {{class}} {
            text-align: center;
            padding: 60px 20px;
            color: #9ca3af;
            font-size: 15px;
        }
    "#;

    const CLASS_NAME: &'static str = "sc-browser-loading";
}

pub struct ScBrowserErrorStyle;

impl Style for ScBrowserErrorStyle {
    const CSS: &'static str = r#"
        {{class}} {
            text-align: center;
            padding: 60px 20px;
            color: #f87171;
            font-size: 15px;
        }
    "#;

    const CLASS_NAME: &'static str = "sc-browser-error";
}

pub struct UniqueSectionStyle;

impl Style for UniqueSectionStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin-bottom: 20px;
        }
    "#;

    const CLASS_NAME: &'static str = "unique-section";
}

pub struct UniqueSectionTitleStyle;

impl Style for UniqueSectionTitleStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin-bottom: 8px;
        }
    "#;

    const CLASS_NAME: &'static str = "unique-section-title";
}

pub struct UniqueNameStyle;

impl Style for UniqueNameStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #e2e8f0;
            font-weight: 600;
            font-size: 14px;
            margin-bottom: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "unique-name";
}

pub struct UniqueRequiredLevelStyle;

impl Style for UniqueRequiredLevelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #94a3b8;
            font-size: 12px;
            margin-bottom: 12px;
        }

        {{class}} span {
            color: #fbbf24;
            font-weight: 600;
        }
    "#;

    const CLASS_NAME: &'static str = "unique-required-level";
}

pub struct UniqueEntryStyle;

impl Style for UniqueEntryStyle {
    const CSS: &'static str = r#"
        {{class}} {
            padding: 8px 12px;
            background: #1e293b;
            border-radius: 8px;
            margin-bottom: 8px;
        }

        {{class}}.disabled {
            opacity: 0.4;
        }
    "#;

    const CLASS_NAME: &'static str = "unique-entry";
}

pub struct UniqueEntryLabelStyle;

impl Style for UniqueEntryLabelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #34d399;
            font-weight: 600;
        }
    "#;

    const CLASS_NAME: &'static str = "unique-entry-label";
}

pub struct UniqueEntrySeparatorStyle;

impl Style for UniqueEntrySeparatorStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #94a3b8;
        }
    "#;

    const CLASS_NAME: &'static str = "unique-entry-separator";
}

pub struct UniqueEntryValueStyle;

impl Style for UniqueEntryValueStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #fbbf24;
            font-weight: 600;
        }
    "#;

    const CLASS_NAME: &'static str = "unique-entry-value";
}

inventory::submit! { StyleDefinition { css: SupportCardBrowserRootStyle::CSS, selector_type: SupportCardBrowserRootStyle::SELECTOR_TYPE, class_name: SupportCardBrowserRootStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ScBrowserHeaderStyle::CSS, selector_type: ScBrowserHeaderStyle::SELECTOR_TYPE, class_name: ScBrowserHeaderStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ScBrowserHeaderControlsStyle::CSS, selector_type: ScBrowserHeaderControlsStyle::SELECTOR_TYPE, class_name: ScBrowserHeaderControlsStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ScBrowserBodyStyle::CSS, selector_type: ScBrowserBodyStyle::SELECTOR_TYPE, class_name: ScBrowserBodyStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ScBrowserSidebarStyle::CSS, selector_type: ScBrowserSidebarStyle::SELECTOR_TYPE, class_name: ScBrowserSidebarStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ScBrowserMainStyle::CSS, selector_type: ScBrowserMainStyle::SELECTOR_TYPE, class_name: ScBrowserMainStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ScBrowserTotalStyle::CSS, selector_type: ScBrowserTotalStyle::SELECTOR_TYPE, class_name: ScBrowserTotalStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ScCardGridStyle::CSS, selector_type: ScCardGridStyle::SELECTOR_TYPE, class_name: ScCardGridStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ScBrowserEmptyStyle::CSS, selector_type: ScBrowserEmptyStyle::SELECTOR_TYPE, class_name: ScBrowserEmptyStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ScBrowserLoadingStyle::CSS, selector_type: ScBrowserLoadingStyle::SELECTOR_TYPE, class_name: ScBrowserLoadingStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ScBrowserErrorStyle::CSS, selector_type: ScBrowserErrorStyle::SELECTOR_TYPE, class_name: ScBrowserErrorStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: UniqueSectionStyle::CSS, selector_type: UniqueSectionStyle::SELECTOR_TYPE, class_name: UniqueSectionStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: UniqueSectionTitleStyle::CSS, selector_type: UniqueSectionTitleStyle::SELECTOR_TYPE, class_name: UniqueSectionTitleStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: UniqueNameStyle::CSS, selector_type: UniqueNameStyle::SELECTOR_TYPE, class_name: UniqueNameStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: UniqueRequiredLevelStyle::CSS, selector_type: UniqueRequiredLevelStyle::SELECTOR_TYPE, class_name: UniqueRequiredLevelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: UniqueEntryStyle::CSS, selector_type: UniqueEntryStyle::SELECTOR_TYPE, class_name: UniqueEntryStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: UniqueEntryLabelStyle::CSS, selector_type: UniqueEntryLabelStyle::SELECTOR_TYPE, class_name: UniqueEntryLabelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: UniqueEntrySeparatorStyle::CSS, selector_type: UniqueEntrySeparatorStyle::SELECTOR_TYPE, class_name: UniqueEntrySeparatorStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: UniqueEntryValueStyle::CSS, selector_type: UniqueEntryValueStyle::SELECTOR_TYPE, class_name: UniqueEntryValueStyle::CLASS_NAME } }
