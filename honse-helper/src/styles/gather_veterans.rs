use crate::styles::{Style, StyleDefinition};

pub struct GatherVeteransBtnStyle;

impl Style for GatherVeteransBtnStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-flex;
            align-items: center;
            justify-content: center;
            gap: 6px;
            min-height: 32px;
            padding: 0 14px;
            border: 1px solid #475569;
            border-radius: 999px;
            background: #1f2937;
            color: #e2e8f0;
            cursor: pointer;
            font-size: 12px;
            font-weight: 700;
            letter-spacing: 0.03em;
            transition: all 0.15s ease;
            white-space: nowrap;
        }

        {{class}}:hover:not(:disabled) {
            border-color: #64748b;
            background: #334155;
            color: #f8fafc;
        }

        {{class}}:disabled {
            opacity: 0.6;
            cursor: not-allowed;
        }
    "#;

    const CLASS_NAME: &'static str = "gather-veterans-btn";
}

inventory::submit! { StyleDefinition { css: GatherVeteransBtnStyle::CSS, selector_type: GatherVeteransBtnStyle::SELECTOR_TYPE, class_name: GatherVeteransBtnStyle::CLASS_NAME } }
