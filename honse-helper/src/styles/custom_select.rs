use crate::styles::{Style, StyleDefinition};

pub struct CustomSelectRootStyle;

impl Style for CustomSelectRootStyle {
    const CSS: &'static str = r#"
        {{class}} {
            position: relative;
            width: 100%;
            outline: none;
        }
    "#;

    const CLASS_NAME: &'static str = "custom-select";
}

pub struct CustomSelectTriggerStyle;

impl Style for CustomSelectTriggerStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            justify-content: space-between;
            background: #0f172a;
            border: 1px solid #334155;
            border-radius: 6px;
            padding: 7px 10px;
            font-size: 13px;
            cursor: pointer;
            user-select: none;
        }

        {{class}}:hover {
            border-color: #475569;
        }
    "#;

    const CLASS_NAME: &'static str = "custom-select-trigger";
}

pub struct CustomSelectValueStyle;

impl Style for CustomSelectValueStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #f8fafc;
        }
    "#;

    const CLASS_NAME: &'static str = "custom-select-value";
}

pub struct CustomSelectPlaceholderStyle;

impl Style for CustomSelectPlaceholderStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #64748b;
        }
    "#;

    const CLASS_NAME: &'static str = "custom-select-placeholder";
}

pub struct CustomSelectArrowStyle;

impl Style for CustomSelectArrowStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #94a3b8;
            font-size: 10px;
            margin-left: 8px;
        }
    "#;

    const CLASS_NAME: &'static str = "custom-select-arrow";
}

pub struct CustomSelectDropdownStyle;

impl Style for CustomSelectDropdownStyle {
    const CSS: &'static str = r#"
        {{class}} {
            position: absolute;
            top: 100%;
            left: 0;
            right: 0;
            max-height: 200px;
            overflow-y: auto;
            background: #1e293b;
            border: 1px solid #334155;
            border-radius: 4px;
            box-shadow: 0 4px 12px rgba(0,0,0,0.4);
            z-index: 1000;
            margin-top: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "custom-select-dropdown";
}

pub struct CustomSelectOptionStyle;

impl Style for CustomSelectOptionStyle {
    const CSS: &'static str = r#"
        {{class}} {
            padding: 8px 12px;
            cursor: pointer;
            color: #f8fafc;
            font-size: 13px;
            border-bottom: 1px solid #334155;
        }

        {{class}}:hover {
            background: #334155;
        }
    "#;

    const CLASS_NAME: &'static str = "custom-select-option";
}

pub struct CustomSelectOptionSelectedStyle;

impl Style for CustomSelectOptionSelectedStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #1e3a5f;
            color: #93c5fd;
        }
    "#;

    const CLASS_NAME: &'static str = "selected";
}

inventory::submit! { StyleDefinition { css: CustomSelectRootStyle::CSS, selector_type: CustomSelectRootStyle::SELECTOR_TYPE, class_name: CustomSelectRootStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CustomSelectTriggerStyle::CSS, selector_type: CustomSelectTriggerStyle::SELECTOR_TYPE, class_name: CustomSelectTriggerStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CustomSelectValueStyle::CSS, selector_type: CustomSelectValueStyle::SELECTOR_TYPE, class_name: CustomSelectValueStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CustomSelectPlaceholderStyle::CSS, selector_type: CustomSelectPlaceholderStyle::SELECTOR_TYPE, class_name: CustomSelectPlaceholderStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CustomSelectArrowStyle::CSS, selector_type: CustomSelectArrowStyle::SELECTOR_TYPE, class_name: CustomSelectArrowStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CustomSelectDropdownStyle::CSS, selector_type: CustomSelectDropdownStyle::SELECTOR_TYPE, class_name: CustomSelectDropdownStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CustomSelectOptionStyle::CSS, selector_type: CustomSelectOptionStyle::SELECTOR_TYPE, class_name: CustomSelectOptionStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CustomSelectOptionSelectedStyle::CSS, selector_type: CustomSelectOptionSelectedStyle::SELECTOR_TYPE, class_name: CustomSelectOptionSelectedStyle::CLASS_NAME } }
