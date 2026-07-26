use crate::styles::{Style, StyleDefinition};

pub struct SortSelectorStyle;

impl Style for SortSelectorStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            gap: 6px;
            font-size: 13px;
        }

        {{class}} label {
            color: #94a3b8;
        }

        {{class}} .custom-select {
            width: auto;
            min-width: 110px;
        }
    "#;

    const CLASS_NAME: &'static str = "sort-selector";
}

inventory::submit! { StyleDefinition { css: SortSelectorStyle::CSS, selector_type: SortSelectorStyle::SELECTOR_TYPE, class_name: SortSelectorStyle::CLASS_NAME } }
