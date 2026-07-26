use crate::styles::{Style, StyleDefinition};

pub struct LegacyVeteranSlotContainerStyle;

impl Style for LegacyVeteranSlotContainerStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #1e293b;
            border: 1px solid #334155;
            border-radius: 10px;
            box-shadow: 0 1px 3px rgba(0,0,0,0.2);
            height: 100%;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-veteran-slot-container";
}

pub struct LegacyVeteranSlotHeaderStyle;

impl Style for LegacyVeteranSlotHeaderStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 8px;
            margin-bottom: 8px;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-veteran-slot-header";
}

pub struct LegacyVeteranSlotTitleStyle;

impl Style for LegacyVeteranSlotTitleStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin: 0;
            font-size: 14px;
            color: #e2e8f0;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-veteran-slot-title";
}

pub struct LegacyVeteranSlotClearButtonStyle;

impl Style for LegacyVeteranSlotClearButtonStyle {
    const CSS: &'static str = r#"
        {{class}} {
            padding: 4px 8px;
            color: #e2e8f0;
            border: 1px solid #475569;
            background: #374151;
            border-radius: 6px;
            cursor: pointer;
            font-size: 12px;
            transition: all 0.2s;
        }

        {{class}}:hover {
            background: #4b5563;
            color: #f8fafc;
            border-color: #64748b;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-veteran-slot-clear-button";
}

pub struct LegacyVeteranSlotCardHeaderStyle;

impl Style for LegacyVeteranSlotCardHeaderStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            gap: 8px;
            padding: 8px 12px;
            border-bottom: 1px solid #1f2937;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-veteran-slot-card-header";
}

pub struct LegacyVeteranSlotCardTitleStyle;

impl Style for LegacyVeteranSlotCardTitleStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-flex;
            align-items: center;
            padding: 3px 10px;
            border-radius: 999px;
            font-size: 11px;
            font-weight: 600;
            color: #fff;
            letter-spacing: 0.3px;
            white-space: nowrap;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-veteran-slot-card-title";
}

pub struct LegacyVeteranSlotCardClearStyle;

impl Style for LegacyVeteranSlotCardClearStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin-left: auto;
            padding: 3px 10px;
            border: 1px solid #475569;
            background: #374151;
            border-radius: 6px;
            cursor: pointer;
            font-size: 11px;
            color: #e2e8f0;
            transition: all 0.15s ease;
            white-space: nowrap;
        }

        {{class}}:hover:not(:disabled) {
            background: #4b5563;
            border-color: #64748b;
            color: #f8fafc;
        }

        {{class}}:disabled {
            opacity: 0.4;
            cursor: not-allowed;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-veteran-slot-card-clear";
}

pub struct LegacyVeteranSlotBodyStyle;

impl Style for LegacyVeteranSlotBodyStyle {
    const CSS: &'static str = r#"
        {{class}} {
            padding: 12px;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-veteran-slot-body";
}

pub struct LegacyVeteranSlotVeteranNameStyle;

impl Style for LegacyVeteranSlotVeteranNameStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 14px;
            font-weight: 600;
            color: #e2e8f0;
            line-height: 1.3;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-veteran-slot-veteran-name";
}

pub struct LegacyVeteranSlotVeteranHashStyle;

impl Style for LegacyVeteranSlotVeteranHashStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 12px;
            color: #94a3b8;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-veteran-slot-veteran-hash";
}

pub struct LegacyVeteranSlotActionsStyle;

impl Style for LegacyVeteranSlotActionsStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            gap: 6px;
            flex-wrap: wrap;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-veteran-slot-actions";
}

pub struct LegacyVeteranSlotCharacterNameStyle;

impl Style for LegacyVeteranSlotCharacterNameStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 14px;
            font-weight: 500;
            color: #94a3b8;
            margin-bottom: 3px;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-veteran-slot-character-name";
}

pub struct LegacyVeteranSlotCharacterIdStyle;

impl Style for LegacyVeteranSlotCharacterIdStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 12px;
            color: #94a3b8;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-veteran-slot-character-id";
}

pub struct LegacyVeteranSlotEmptyLabelStyle;

impl Style for LegacyVeteranSlotEmptyLabelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 11px;
            font-weight: 600;
            color: #64748b;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            margin-bottom: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-veteran-slot-empty-label";
}

pub struct LegacySlotBodyRowStyle;

impl Style for LegacySlotBodyRowStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: baseline;
            gap: 10px;
            flex-wrap: wrap;
            margin-bottom: 10px;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-slot-body-row";
}

pub struct LegacySlotEmptyBodyStyle;

impl Style for LegacySlotEmptyBodyStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
            gap: 10px;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-slot-empty-body";
}

pub struct LegacyMajorWinsGridStyle;

impl Style for LegacyMajorWinsGridStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
            gap: 10px;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-major-wins-grid";
}

pub struct LegacyMajorWinCardStyle;

impl Style for LegacyMajorWinCardStyle {
    const CSS: &'static str = r#"
        {{class}} {
            padding: 10px 12px;
            background: #fff;
            border: 1px solid #eee;
            border-radius: 6px;
            display: flex;
            flex-direction: column;
            gap: 6px;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-major-win-card";
}

pub struct LegacyMajorWinHeaderStyle;

impl Style for LegacyMajorWinHeaderStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            gap: 8px;
            align-items: center;
            justify-content: space-between;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-major-win-header";
}

pub struct LegacyMajorWinNameStyle;

impl Style for LegacyMajorWinNameStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-weight: 600;
            color: #333;
            font-size: 13px;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-major-win-name";
}

pub struct LegacyEmptyTextStyle;

impl Style for LegacyEmptyTextStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 13px;
            color: #64748b;
            padding: 12px 0;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-empty-text";
}

inventory::submit! { StyleDefinition { css: LegacyVeteranSlotContainerStyle::CSS, selector_type: LegacyVeteranSlotContainerStyle::SELECTOR_TYPE, class_name: LegacyVeteranSlotContainerStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacyVeteranSlotHeaderStyle::CSS, selector_type: LegacyVeteranSlotHeaderStyle::SELECTOR_TYPE, class_name: LegacyVeteranSlotHeaderStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacyVeteranSlotTitleStyle::CSS, selector_type: LegacyVeteranSlotTitleStyle::SELECTOR_TYPE, class_name: LegacyVeteranSlotTitleStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacyVeteranSlotClearButtonStyle::CSS, selector_type: LegacyVeteranSlotClearButtonStyle::SELECTOR_TYPE, class_name: LegacyVeteranSlotClearButtonStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacyVeteranSlotCardHeaderStyle::CSS, selector_type: LegacyVeteranSlotCardHeaderStyle::SELECTOR_TYPE, class_name: LegacyVeteranSlotCardHeaderStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacyVeteranSlotCardTitleStyle::CSS, selector_type: LegacyVeteranSlotCardTitleStyle::SELECTOR_TYPE, class_name: LegacyVeteranSlotCardTitleStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacyVeteranSlotCardClearStyle::CSS, selector_type: LegacyVeteranSlotCardClearStyle::SELECTOR_TYPE, class_name: LegacyVeteranSlotCardClearStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacyVeteranSlotBodyStyle::CSS, selector_type: LegacyVeteranSlotBodyStyle::SELECTOR_TYPE, class_name: LegacyVeteranSlotBodyStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacyVeteranSlotVeteranNameStyle::CSS, selector_type: LegacyVeteranSlotVeteranNameStyle::SELECTOR_TYPE, class_name: LegacyVeteranSlotVeteranNameStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacyVeteranSlotVeteranHashStyle::CSS, selector_type: LegacyVeteranSlotVeteranHashStyle::SELECTOR_TYPE, class_name: LegacyVeteranSlotVeteranHashStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacyVeteranSlotActionsStyle::CSS, selector_type: LegacyVeteranSlotActionsStyle::SELECTOR_TYPE, class_name: LegacyVeteranSlotActionsStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacyVeteranSlotCharacterNameStyle::CSS, selector_type: LegacyVeteranSlotCharacterNameStyle::SELECTOR_TYPE, class_name: LegacyVeteranSlotCharacterNameStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacyVeteranSlotCharacterIdStyle::CSS, selector_type: LegacyVeteranSlotCharacterIdStyle::SELECTOR_TYPE, class_name: LegacyVeteranSlotCharacterIdStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacyVeteranSlotEmptyLabelStyle::CSS, selector_type: LegacyVeteranSlotEmptyLabelStyle::SELECTOR_TYPE, class_name: LegacyVeteranSlotEmptyLabelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacySlotBodyRowStyle::CSS, selector_type: LegacySlotBodyRowStyle::SELECTOR_TYPE, class_name: LegacySlotBodyRowStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacySlotEmptyBodyStyle::CSS, selector_type: LegacySlotEmptyBodyStyle::SELECTOR_TYPE, class_name: LegacySlotEmptyBodyStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacyMajorWinsGridStyle::CSS, selector_type: LegacyMajorWinsGridStyle::SELECTOR_TYPE, class_name: LegacyMajorWinsGridStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacyMajorWinCardStyle::CSS, selector_type: LegacyMajorWinCardStyle::SELECTOR_TYPE, class_name: LegacyMajorWinCardStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacyMajorWinHeaderStyle::CSS, selector_type: LegacyMajorWinHeaderStyle::SELECTOR_TYPE, class_name: LegacyMajorWinHeaderStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacyMajorWinNameStyle::CSS, selector_type: LegacyMajorWinNameStyle::SELECTOR_TYPE, class_name: LegacyMajorWinNameStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacyEmptyTextStyle::CSS, selector_type: LegacyEmptyTextStyle::SELECTOR_TYPE, class_name: LegacyEmptyTextStyle::CLASS_NAME } }
