use super::{Style, StyleDefinition};

pub struct SkillPillStyle;

impl Style for SkillPillStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            gap: 10px;
            padding: 6px 12px;
            border-radius: 6px;
            background: #0f172a;
            border-left: 3px solid var(--skill-accent, #475569);
            font-size: 13px;
        }
    "#;

    const CLASS_NAME: &'static str = "skill-pill";
}

pub struct SkillPillTypeBadgeStyle;

impl Style for SkillPillTypeBadgeStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-flex;
            align-items: center;
            padding: 2px 8px;
            border-radius: 4px;
            font-size: 10px;
            font-weight: 700;
            text-transform: uppercase;
            letter-spacing: 0.3px;
            background: var(--skill-accent, #475569);
            color: #fff;
            white-space: nowrap;
            flex-shrink: 0;
        }
    "#;

    const CLASS_NAME: &'static str = "skill-pill-type-badge";
}

pub struct SkillPillNameStyle;

impl Style for SkillPillNameStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #e2e8f0;
            flex: 1;
            min-width: 0;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }
    "#;

    const CLASS_NAME: &'static str = "skill-pill-name";
}

pub struct SkillPillLevelStyle;

impl Style for SkillPillLevelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #facc15;
            font-weight: 600;
            font-size: 12px;
            white-space: nowrap;
            flex-shrink: 0;
        }
    "#;

    const CLASS_NAME: &'static str = "skill-pill-level";
}

pub struct SkillPillIdStyle;

impl Style for SkillPillIdStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #475569;
            font-size: 11px;
            white-space: nowrap;
            flex-shrink: 0;
        }
    "#;

    const CLASS_NAME: &'static str = "skill-pill-id";
}

pub struct SkillPillListStyle;

impl Style for SkillPillListStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
            gap: 6px;
        }
    "#;

    const CLASS_NAME: &'static str = "skill-pill-list";
}

pub struct SkillPillGroupStyle;

impl Style for SkillPillGroupStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
            gap: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "skill-pill-group";
}

pub struct SkillPillCategoryStyle;

impl Style for SkillPillCategoryStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 11px;
            font-weight: 700;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            border-bottom: 1px solid #1e293b;
            padding-bottom: 3px;
            margin-bottom: 2px;
        }
    "#;

    const CLASS_NAME: &'static str = "skill-pill-category";
}

inventory::submit! { StyleDefinition { css: SkillPillStyle::CSS, selector_type: SkillPillStyle::SELECTOR_TYPE, class_name: SkillPillStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SkillPillTypeBadgeStyle::CSS, selector_type: SkillPillTypeBadgeStyle::SELECTOR_TYPE, class_name: SkillPillTypeBadgeStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SkillPillNameStyle::CSS, selector_type: SkillPillNameStyle::SELECTOR_TYPE, class_name: SkillPillNameStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SkillPillLevelStyle::CSS, selector_type: SkillPillLevelStyle::SELECTOR_TYPE, class_name: SkillPillLevelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SkillPillIdStyle::CSS, selector_type: SkillPillIdStyle::SELECTOR_TYPE, class_name: SkillPillIdStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SkillPillListStyle::CSS, selector_type: SkillPillListStyle::SELECTOR_TYPE, class_name: SkillPillListStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SkillPillGroupStyle::CSS, selector_type: SkillPillGroupStyle::SELECTOR_TYPE, class_name: SkillPillGroupStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SkillPillCategoryStyle::CSS, selector_type: SkillPillCategoryStyle::SELECTOR_TYPE, class_name: SkillPillCategoryStyle::CLASS_NAME } }
