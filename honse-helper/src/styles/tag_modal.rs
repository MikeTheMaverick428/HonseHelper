use crate::styles::{Style, StyleDefinition};

pub struct TagPillStyle;

impl Style for TagPillStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-flex;
            align-items: center;
            gap: 4px;
            padding: 3px 10px;
            border: 1px solid #475569;
            border-radius: 999px;
            background: #1e293b;
            color: #e2e8f0;
            font-size: 12px;
            font-weight: 600;
            white-space: nowrap;
        }
    "#;
    const CLASS_NAME: &'static str = "tag-pill";
}

pub struct TagPillRemoveStyle;

impl Style for TagPillRemoveStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-flex;
            align-items: center;
            justify-content: center;
            width: 16px;
            height: 16px;
            border: none;
            border-radius: 50%;
            background: #475569;
            color: #0f172a;
            cursor: pointer;
            font-size: 11px;
            font-weight: 700;
            line-height: 1;
            padding: 0;
            transition: background 0.15s;
        }

        {{class}}:hover {
            background: #ef4444;
            color: #fff;
        }
    "#;
    const CLASS_NAME: &'static str = "tag-pill-remove";
}

pub struct TagPillListStyle;

impl Style for TagPillListStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-wrap: wrap;
            gap: 4px;
            margin-bottom: 8px;
        }
    "#;
    const CLASS_NAME: &'static str = "tag-pill-list";
}

pub struct TagInputContainerStyle;

impl Style for TagInputContainerStyle {
    const CSS: &'static str = r#"
        {{class}} {
            width: 100%;
        }
    "#;
    const CLASS_NAME: &'static str = "tag-input-container";
}

pub struct TagInputStyle;

impl Style for TagInputStyle {
    const CSS: &'static str = r#"
        {{class}} {
            width: 100%;
            padding: 8px 12px;
            border: 1px solid #475569;
            border-radius: 4px;
            background: #0f172a;
            color: #f8fafc;
            font-size: 13px;
            outline: none;
            box-sizing: border-box;
        }

        {{class}}:focus {
            border-color: #60a5fa;
        }

        {{class}}::placeholder {
            color: #64748b;
        }
    "#;
    const CLASS_NAME: &'static str = "tag-input";
}

pub struct TagDropdownStyle;

impl Style for TagDropdownStyle {
    const CSS: &'static str = r#"
        {{class}} {
            width: 100%;
            max-height: 200px;
            overflow-y: auto;
            background: #1e293b;
            border: 1px solid #334155;
            border-radius: 4px;
            box-shadow: 0 4px 12px rgba(0,0,0,0.4);
            margin-top: 4px;
        }
    "#;
    const CLASS_NAME: &'static str = "tag-dropdown";
}

pub struct TagDropdownItemStyle;

impl Style for TagDropdownItemStyle {
    const CSS: &'static str = r#"
        {{class}} {
            padding: 8px 12px;
            cursor: pointer;
            border-bottom: 1px solid #334155;
            color: #f8fafc;
            font-size: 13px;
            transition: background 0.1s;
        }

        {{class}}:hover {
            background: #334155;
        }

        {{class}}:last-child {
            border-bottom: none;
        }
    "#;
    const CLASS_NAME: &'static str = "tag-dropdown-item";
}

pub struct TagCreateItemStyle;

impl Style for TagCreateItemStyle {
    const CSS: &'static str = r#"
        {{class}} {
            padding: 8px 12px;
            cursor: pointer;
            border-bottom: 1px solid #334155;
            color: #22d3ee;
            font-size: 13px;
            font-style: italic;
            transition: background 0.1s;
        }

        {{class}}:hover {
            background: #334155;
        }

        {{class}}:last-child {
            border-bottom: none;
        }
    "#;
    const CLASS_NAME: &'static str = "tag-create-item";
}

pub struct TagNoResultsStyle;

impl Style for TagNoResultsStyle {
    const CSS: &'static str = r#"
        {{class}} {
            padding: 8px 12px;
            color: #64748b;
            font-size: 12px;
        }
    "#;
    const CLASS_NAME: &'static str = "tag-no-results";
}

pub struct TagSectionTitleStyle;

impl Style for TagSectionTitleStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 14px;
            font-weight: 700;
            color: #e2e8f0;
            margin-bottom: 8px;
        }
    "#;
    const CLASS_NAME: &'static str = "tag-section-title";
}

// ── Tag on veteran card (smaller variant) ─────────────────────────

pub struct CardTagPillStyle;

impl Style for CardTagPillStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-flex;
            align-items: center;
            padding: 1px 6px;
            border: 1px solid #475569;
            border-radius: 999px;
            background: #1e293b;
            color: #94a3b8;
            font-size: 10px;
            font-weight: 600;
            white-space: nowrap;
        }
    "#;
    const CLASS_NAME: &'static str = "card-tag-pill";
}

pub struct CardTagMoreStyle;

impl Style for CardTagMoreStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-flex;
            align-items: center;
            padding: 1px 6px;
            border-radius: 999px;
            background: transparent;
            color: #64748b;
            font-size: 10px;
            font-weight: 600;
            cursor: pointer;
        }

        {{class}}:hover {
            color: #94a3b8;
        }
    "#;
    const CLASS_NAME: &'static str = "card-tag-more";
}

inventory::submit! { StyleDefinition { css: TagPillStyle::CSS, selector_type: TagPillStyle::SELECTOR_TYPE, class_name: TagPillStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TagPillRemoveStyle::CSS, selector_type: TagPillRemoveStyle::SELECTOR_TYPE, class_name: TagPillRemoveStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TagPillListStyle::CSS, selector_type: TagPillListStyle::SELECTOR_TYPE, class_name: TagPillListStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TagInputContainerStyle::CSS, selector_type: TagInputContainerStyle::SELECTOR_TYPE, class_name: TagInputContainerStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TagInputStyle::CSS, selector_type: TagInputStyle::SELECTOR_TYPE, class_name: TagInputStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TagDropdownStyle::CSS, selector_type: TagDropdownStyle::SELECTOR_TYPE, class_name: TagDropdownStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TagDropdownItemStyle::CSS, selector_type: TagDropdownItemStyle::SELECTOR_TYPE, class_name: TagDropdownItemStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TagCreateItemStyle::CSS, selector_type: TagCreateItemStyle::SELECTOR_TYPE, class_name: TagCreateItemStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TagNoResultsStyle::CSS, selector_type: TagNoResultsStyle::SELECTOR_TYPE, class_name: TagNoResultsStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TagSectionTitleStyle::CSS, selector_type: TagSectionTitleStyle::SELECTOR_TYPE, class_name: TagSectionTitleStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CardTagPillStyle::CSS, selector_type: CardTagPillStyle::SELECTOR_TYPE, class_name: CardTagPillStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CardTagMoreStyle::CSS, selector_type: CardTagMoreStyle::SELECTOR_TYPE, class_name: CardTagMoreStyle::CLASS_NAME } }
