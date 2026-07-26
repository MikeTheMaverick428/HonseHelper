use crate::styles::{Style, StyleDefinition};

pub struct RaceCardRootStyle;

impl Style for RaceCardRootStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #111627;
            border: 1px solid #1f2937;
            border-radius: 8px;
            padding: 14px 16px;
            cursor: pointer;
            transition: border-color 0.15s, background 0.15s;
            display: flex;
            flex-direction: column;
            gap: 8px;
        }
        {{class}}:hover {
            border-color: #374151;
            background: #151a30;
        }
    "#;

    const CLASS_NAME: &'static str = "race-card";
}

pub struct RaceCardTopRowStyle;

impl Style for RaceCardTopRowStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 8px;
        }
    "#;

    const CLASS_NAME: &'static str = "race-card-top-row";
}

pub struct RaceCardIdStyle;

impl Style for RaceCardIdStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #64748b;
            font-size: 11px;
        }
    "#;

    const CLASS_NAME: &'static str = "race-card-id";
}

pub struct RaceCardRaceNameStyle;

impl Style for RaceCardRaceNameStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #facc15;
            font-size: 14px;
            font-weight: 500;
        }
    "#;

    const CLASS_NAME: &'static str = "race-card-race-name";
}

pub struct RaceCardDividerStyle;

impl Style for RaceCardDividerStyle {
    const CSS: &'static str = r#"
        {{class}} {
            border: none;
            border-top: 1px solid #1e293b;
            margin: 0;
        }
    "#;

    const CLASS_NAME: &'static str = "race-card-divider";
}

pub struct RaceCardInfoRowStyle;

impl Style for RaceCardInfoRowStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-wrap: wrap;
            gap: 8px;
            align-items: center;
            font-size: 13px;
            color: #cbd5e1;
        }
    "#;

    const CLASS_NAME: &'static str = "race-card-info-row";
}

pub struct RaceCardInfoItemStyle;

impl Style for RaceCardInfoItemStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-flex;
            align-items: center;
            gap: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "race-card-info-item";
}

pub struct RaceCardParticipantsStyle;

impl Style for RaceCardParticipantsStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 12px;
            color: #94a3b8;
        }
    "#;

    const CLASS_NAME: &'static str = "race-card-participants";
}

pub struct RaceCardTimeStyle;

impl Style for RaceCardTimeStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 11px;
            color: #64748b;
        }
    "#;

    const CLASS_NAME: &'static str = "race-card-time";
}

pub struct RaceCardFooterStyle;

impl Style for RaceCardFooterStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 8px;
        }
    "#;

    const CLASS_NAME: &'static str = "race-card-footer";
}

pub struct RaceCardTagsStyle;

impl Style for RaceCardTagsStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-wrap: wrap;
            gap: 4px;
            align-items: center;
        }
    "#;

    const CLASS_NAME: &'static str = "race-card-tags";
}

inventory::submit! { StyleDefinition { css: RaceCardRootStyle::CSS, selector_type: RaceCardRootStyle::SELECTOR_TYPE, class_name: RaceCardRootStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RaceCardTopRowStyle::CSS, selector_type: RaceCardTopRowStyle::SELECTOR_TYPE, class_name: RaceCardTopRowStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RaceCardIdStyle::CSS, selector_type: RaceCardIdStyle::SELECTOR_TYPE, class_name: RaceCardIdStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RaceCardRaceNameStyle::CSS, selector_type: RaceCardRaceNameStyle::SELECTOR_TYPE, class_name: RaceCardRaceNameStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RaceCardDividerStyle::CSS, selector_type: RaceCardDividerStyle::SELECTOR_TYPE, class_name: RaceCardDividerStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RaceCardInfoRowStyle::CSS, selector_type: RaceCardInfoRowStyle::SELECTOR_TYPE, class_name: RaceCardInfoRowStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RaceCardInfoItemStyle::CSS, selector_type: RaceCardInfoItemStyle::SELECTOR_TYPE, class_name: RaceCardInfoItemStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RaceCardParticipantsStyle::CSS, selector_type: RaceCardParticipantsStyle::SELECTOR_TYPE, class_name: RaceCardParticipantsStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RaceCardTimeStyle::CSS, selector_type: RaceCardTimeStyle::SELECTOR_TYPE, class_name: RaceCardTimeStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RaceCardFooterStyle::CSS, selector_type: RaceCardFooterStyle::SELECTOR_TYPE, class_name: RaceCardFooterStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RaceCardTagsStyle::CSS, selector_type: RaceCardTagsStyle::SELECTOR_TYPE, class_name: RaceCardTagsStyle::CLASS_NAME } }
