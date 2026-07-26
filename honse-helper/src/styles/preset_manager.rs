use crate::styles::{Style, StyleDefinition};

pub struct PresetManagerStyle;

impl Style for PresetManagerStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            gap: 6px;
            flex-wrap: wrap;
        }

        {{class}} button {
            padding: 6px 10px;
            font-size: 12px;
        }
    "#;

    const CLASS_NAME: &'static str = "preset-manager";
}

pub struct PresetSaveRowStyle;

impl Style for PresetSaveRowStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            gap: 4px;
            width: 100%;
        }

        {{class}} .filter-input {
            flex: 1;
            font-size: 12px;
            padding: 5px 8px;
        }
    "#;

    const CLASS_NAME: &'static str = "preset-save-row";
}

inventory::submit! { StyleDefinition { css: PresetManagerStyle::CSS, selector_type: PresetManagerStyle::SELECTOR_TYPE, class_name: PresetManagerStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PresetSaveRowStyle::CSS, selector_type: PresetSaveRowStyle::SELECTOR_TYPE, class_name: PresetSaveRowStyle::CLASS_NAME } }
