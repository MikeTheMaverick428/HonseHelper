use crate::styles::{Style, StyleDefinition};

pub struct FilterPanelStyle;

impl Style for FilterPanelStyle {
    const CSS: &'static str = r#"
        {{class}} {}
    "#;

    const CLASS_NAME: &'static str = "filter-panel";
}

pub struct FilterTitleStyle;

impl Style for FilterTitleStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 14px;
            font-weight: 600;
            margin-bottom: 12px;
            color: #e2e8f0;
        }
    "#;

    const CLASS_NAME: &'static str = "filter-title";
}

pub struct FilterSectionStyle;

impl Style for FilterSectionStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin-bottom: 12px;
        }

        {{class}} label {
            display: block;
            font-size: 12px;
            color: #94a3b8;
            margin-bottom: 4px;
        }

        {{class}} label input[type="checkbox"] {
            margin-right: 6px;
        }
    "#;

    const CLASS_NAME: &'static str = "filter-section";
}

pub struct FilterInputStyle;

impl Style for FilterInputStyle {
    const CSS: &'static str = r#"
        {{class}} {
            width: 100%;
            background: #0f172a;
            color: #f8fafc;
            border: 1px solid #334155;
            border-radius: 6px;
            padding: 7px 10px;
            font-size: 13px;
            color-scheme: dark;
        }
    "#;

    const CLASS_NAME: &'static str = "filter-input";
}

pub struct FilterRangeStyle;

impl Style for FilterRangeStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            gap: 6px;
            align-items: center;
        }

        {{class}} .filter-input {
            width: 0;
            flex: 1;
        }
    "#;

    const CLASS_NAME: &'static str = "filter-range";
}

pub struct RangeSepStyle;

impl Style for RangeSepStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #64748b;
            font-size: 13px;
        }
    "#;

    const CLASS_NAME: &'static str = "range-sep";
}

pub struct FilterInlineStyle;

impl Style for FilterInlineStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            gap: 6px;
        }
    "#;

    const CLASS_NAME: &'static str = "filter-inline";
}

pub struct FilterActionsStyle;

impl Style for FilterActionsStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            gap: 6px;
            margin-top: 16px;
        }

        {{class}} button {
            flex: 1;
            padding: 8px 10px;
            font-size: 13px;
        }
    "#;

    const CLASS_NAME: &'static str = "filter-actions";
}

pub struct FilterChipStyle;

impl Style for FilterChipStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            gap: 6px;
            background: #1e293b;
            border-radius: 6px;
            padding: 4px 10px;
            margin-bottom: 4px;
            font-size: 12px;
        }
    "#;

    const CLASS_NAME: &'static str = "filter-chip";
}

pub struct FilterChipTextStyle;

impl Style for FilterChipTextStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex: 1;
            color: #e2e8f0;
        }
    "#;

    const CLASS_NAME: &'static str = "filter-chip-text";
}

pub struct FilterChipRemoveStyle;

impl Style for FilterChipRemoveStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: none;
            border: none;
            color: #94a3b8;
            cursor: pointer;
            padding: 0;
            font-size: 14px;
            line-height: 1;
        }
    "#;

    const CLASS_NAME: &'static str = "filter-chip-remove";
}

pub struct FilterEmptyHintStyle;

impl Style for FilterEmptyHintStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #64748b;
            font-size: 12px;
            margin-bottom: 12px;
        }
    "#;

    const CLASS_NAME: &'static str = "filter-empty-hint";
}

pub struct FilterAddSectionStyle;

impl Style for FilterAddSectionStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin-top: 8px;
        }
    "#;

    const CLASS_NAME: &'static str = "filter-add-section";
}

pub struct FilterAddActionsStyle;

impl Style for FilterAddActionsStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin-top: 8px;
            display: flex;
            gap: 6px;
        }
    "#;

    const CLASS_NAME: &'static str = "filter-add-actions";
}

inventory::submit! { StyleDefinition { css: FilterPanelStyle::CSS, selector_type: FilterPanelStyle::SELECTOR_TYPE, class_name: FilterPanelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: FilterTitleStyle::CSS, selector_type: FilterTitleStyle::SELECTOR_TYPE, class_name: FilterTitleStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: FilterSectionStyle::CSS, selector_type: FilterSectionStyle::SELECTOR_TYPE, class_name: FilterSectionStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: FilterInputStyle::CSS, selector_type: FilterInputStyle::SELECTOR_TYPE, class_name: FilterInputStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: FilterRangeStyle::CSS, selector_type: FilterRangeStyle::SELECTOR_TYPE, class_name: FilterRangeStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RangeSepStyle::CSS, selector_type: RangeSepStyle::SELECTOR_TYPE, class_name: RangeSepStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: FilterInlineStyle::CSS, selector_type: FilterInlineStyle::SELECTOR_TYPE, class_name: FilterInlineStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: FilterActionsStyle::CSS, selector_type: FilterActionsStyle::SELECTOR_TYPE, class_name: FilterActionsStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: FilterChipStyle::CSS, selector_type: FilterChipStyle::SELECTOR_TYPE, class_name: FilterChipStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: FilterChipTextStyle::CSS, selector_type: FilterChipTextStyle::SELECTOR_TYPE, class_name: FilterChipTextStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: FilterChipRemoveStyle::CSS, selector_type: FilterChipRemoveStyle::SELECTOR_TYPE, class_name: FilterChipRemoveStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: FilterEmptyHintStyle::CSS, selector_type: FilterEmptyHintStyle::SELECTOR_TYPE, class_name: FilterEmptyHintStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: FilterAddSectionStyle::CSS, selector_type: FilterAddSectionStyle::SELECTOR_TYPE, class_name: FilterAddSectionStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: FilterAddActionsStyle::CSS, selector_type: FilterAddActionsStyle::SELECTOR_TYPE, class_name: FilterAddActionsStyle::CLASS_NAME } }
