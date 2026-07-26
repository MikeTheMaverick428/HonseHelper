use crate::styles::{Style, StyleDefinition};

pub struct LegacySummaryContainerStyle;

impl Style for LegacySummaryContainerStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #fff;
            border: 1px solid #e5e7eb;
            border-radius: 10px;
            padding: 12px;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-summary-container";
}

pub struct LegacySummaryTitleStyle;

impl Style for LegacySummaryTitleStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin: 0 0 10px 0;
            font-size: 14px;
            color: #334155;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-summary-title";
}

pub struct LegacySummarySectionStyle;

impl Style for LegacySummarySectionStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin-bottom: 14px;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-summary-section";
}

pub struct LegacySummarySectionTitleStyle;

impl Style for LegacySummarySectionTitleStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin: 0 0 8px 0;
            font-size: 13px;
            color: #475569;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-summary-section-title";
}

pub struct LegacySummaryStatGridStyle;

impl Style for LegacySummaryStatGridStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: grid;
            grid-template-columns: repeat(3, minmax(120px, 1fr));
            gap: 8px;
            font-size: 12px;
        }

        @media (max-width: 900px) {
            {{class}} {
                grid-template-columns: repeat(2, minmax(120px, 1fr));
            }
        }

        @media (max-width: 560px) {
            {{class}} {
                grid-template-columns: 1fr;
            }
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-summary-stat-grid";
}

pub struct LegacySummaryStatCardStyle;

impl Style for LegacySummaryStatCardStyle {
    const CSS: &'static str = r#"
        {{class}} {
            padding: 8px;
            background: #f8fafc;
            border: 1px solid #e2e8f0;
            border-radius: 8px;
            color: #0f172a;
        }

        {{class}} strong {
            display: block;
            margin-bottom: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-summary-stat-card";
}

pub struct LegacySummarySubSectionStyle;

impl Style for LegacySummarySubSectionStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin-bottom: 12px;
        }

        {{class}}:last-child {
            margin-bottom: 0;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-summary-sub-section";
}

pub struct LegacySummarySubSectionTitleStyle;

impl Style for LegacySummarySubSectionTitleStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin: 0 0 8px 0;
            font-size: 12px;
            color: #334155;
            font-weight: 600;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-summary-sub-section-title";
}

pub struct LegacySummaryTableWrapStyle;

impl Style for LegacySummaryTableWrapStyle {
    const CSS: &'static str = r#"
        {{class}} {
            overflow-x: auto;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-summary-table-wrap";
}

pub struct LegacySummaryTableStyle;

impl Style for LegacySummaryTableStyle {
    const CSS: &'static str = r#"
        {{class}} {
            width: 100%;
            border-collapse: collapse;
            font-size: 12px;
        }

        {{class}} thead tr {
            background: #f8fafc;
            text-align: left;
        }

        {{class}} th {
            padding: 8px;
            border-bottom: 1px solid #e2e8f0;
            white-space: nowrap;
        }

        {{class}} td {
            padding: 8px;
            border-bottom: 1px solid #f1f5f9;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-summary-table";
}

pub struct LegacySummaryEmptyTextStyle;

impl Style for LegacySummaryEmptyTextStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 12px;
            color: #64748b;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-summary-empty-text";
}

pub struct LegacySummaryPlaceholderStyle;

impl Style for LegacySummaryPlaceholderStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 12px;
            color: #64748b;
            padding: 4px 0;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-summary-placeholder";
}

pub struct LegacySummaryFootnoteStyle;

impl Style for LegacySummaryFootnoteStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin-top: 6px;
            font-size: 11px;
            color: #64748b;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-summary-footnote";
}

inventory::submit! { StyleDefinition { css: LegacySummaryContainerStyle::CSS, selector_type: LegacySummaryContainerStyle::SELECTOR_TYPE, class_name: LegacySummaryContainerStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacySummaryTitleStyle::CSS, selector_type: LegacySummaryTitleStyle::SELECTOR_TYPE, class_name: LegacySummaryTitleStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacySummarySectionStyle::CSS, selector_type: LegacySummarySectionStyle::SELECTOR_TYPE, class_name: LegacySummarySectionStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacySummarySectionTitleStyle::CSS, selector_type: LegacySummarySectionTitleStyle::SELECTOR_TYPE, class_name: LegacySummarySectionTitleStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacySummaryStatGridStyle::CSS, selector_type: LegacySummaryStatGridStyle::SELECTOR_TYPE, class_name: LegacySummaryStatGridStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacySummaryStatCardStyle::CSS, selector_type: LegacySummaryStatCardStyle::SELECTOR_TYPE, class_name: LegacySummaryStatCardStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacySummarySubSectionStyle::CSS, selector_type: LegacySummarySubSectionStyle::SELECTOR_TYPE, class_name: LegacySummarySubSectionStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacySummarySubSectionTitleStyle::CSS, selector_type: LegacySummarySubSectionTitleStyle::SELECTOR_TYPE, class_name: LegacySummarySubSectionTitleStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacySummaryTableWrapStyle::CSS, selector_type: LegacySummaryTableWrapStyle::SELECTOR_TYPE, class_name: LegacySummaryTableWrapStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacySummaryTableStyle::CSS, selector_type: LegacySummaryTableStyle::SELECTOR_TYPE, class_name: LegacySummaryTableStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacySummaryEmptyTextStyle::CSS, selector_type: LegacySummaryEmptyTextStyle::SELECTOR_TYPE, class_name: LegacySummaryEmptyTextStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacySummaryPlaceholderStyle::CSS, selector_type: LegacySummaryPlaceholderStyle::SELECTOR_TYPE, class_name: LegacySummaryPlaceholderStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacySummaryFootnoteStyle::CSS, selector_type: LegacySummaryFootnoteStyle::SELECTOR_TYPE, class_name: LegacySummaryFootnoteStyle::CLASS_NAME } }
