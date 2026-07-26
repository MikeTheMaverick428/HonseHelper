use crate::styles::{Style, StyleDefinition};

pub struct LegacyPlannerRootStyle;

impl Style for LegacyPlannerRootStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
            gap: 16px;
            height: 100vh;
            padding: 20px 24px;
            background: #0f1220;
            color: #f3f4f6;
            overflow-y: auto;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-planner";
}

pub struct PlannerHeaderStyle;

impl Style for PlannerHeaderStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            justify-content: space-between;
            flex-shrink: 0;
        }

        {{class}} h2 {
            margin: 0;
            font-size: 20px;
            color: #f3f4f6;
        }
    "#;

    const CLASS_NAME: &'static str = "planner-header";
}

pub struct PlannerSectionLabelStyle;

impl Style for PlannerSectionLabelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 12px;
            text-transform: uppercase;
            letter-spacing: 0.05em;
            color: #6b7280;
            margin-bottom: 8px;
        }
    "#;

    const CLASS_NAME: &'static str = "planner-section-label";
}

pub struct TraineeSelectorRowStyle;

impl Style for TraineeSelectorRowStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            gap: 8px;
            align-items: flex-start;
        }

        {{class}} > :first-child {
            flex: 1;
        }
    "#;

    const CLASS_NAME: &'static str = "trainee-selector-row";
}

pub struct PlannerTreeGridStyle;

impl Style for PlannerTreeGridStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: grid;
            grid-template-columns: repeat(2, minmax(180px, 1fr)) 60px repeat(2, minmax(180px, 1fr));
            gap: 12px;
            align-items: start;
        }
    "#;

    const CLASS_NAME: &'static str = "planner-tree-grid";
}

pub struct TreeTraineeStyle;

impl Style for TreeTraineeStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #1e293b;
            border: 1px solid #334155;
            border-radius: 8px;
            padding: 12px 16px;
        }
    "#;

    const CLASS_NAME: &'static str = "tree-trainee";
}

pub struct PlannerSlotStyle;

impl Style for PlannerSlotStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
        }
    "#;

    const CLASS_NAME: &'static str = "planner-slot";
}

pub struct TreeAffinityStyle;

impl Style for TreeAffinityStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            justify-content: center;
            min-height: 40px;
        }
    "#;

    const CLASS_NAME: &'static str = "tree-affinity";
}

pub struct TreeAffinityCenterStyle;

impl Style for TreeAffinityCenterStyle {
    const CSS: &'static str = r#"
        {{class}} {
            align-items: center;
        }
    "#;

    const CLASS_NAME: &'static str = "tree-affinity-center";
}

pub struct AffinityValueStyle;

impl Style for AffinityValueStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 15px;
            font-weight: 700;
            color: #a78bfa;
            font-feature-settings: 'tnum' 1;
        }
    "#;

    const CLASS_NAME: &'static str = "affinity-value";
}

pub struct TreeAffinityEmptyStyle;

impl Style for TreeAffinityEmptyStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: transparent;
            border-color: #1f2937;
        }

        {{class}} .affinity-base {
            color: #374151;
        }
    "#;

    const CLASS_NAME: &'static str = "tree-affinity-empty";
}

pub struct AffinityBaseStyle;

impl Style for AffinityBaseStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 15px;
            font-weight: 700;
            color: #a78bfa;
            font-feature-settings: 'tnum' 1;
        }
    "#;

    const CLASS_NAME: &'static str = "affinity-base";
}

pub struct AffinityBonusStyle;

impl Style for AffinityBonusStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 13px;
            font-weight: 700;
            color: #fbbf24;
            font-feature-settings: 'tnum' 1;
        }
    "#;

    const CLASS_NAME: &'static str = "affinity-bonus";
}

pub struct AffinityPlusStyle;

impl Style for AffinityPlusStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 12px;
            color: #475569;
            margin: 0 2px;
        }
    "#;

    const CLASS_NAME: &'static str = "affinity-plus";
}

pub struct TreeAffinityBoxStyle;

impl Style for TreeAffinityBoxStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #1e1b4b;
            border: 1px solid #4c1d95;
            border-radius: 6px;
            padding: 6px 12px;
            display: inline-flex;
            align-items: center;
            justify-content: center;
            gap: 0;
        }
    "#;

    const CLASS_NAME: &'static str = "tree-affinity-box";
}

pub struct SecondaryBtnStyle;

impl Style for SecondaryBtnStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #374151;
        }

        {{class}}:hover {
            background: #4b5563;
        }
    "#;

    const CLASS_NAME: &'static str = "secondary";
}

inventory::submit! { StyleDefinition { css: LegacyPlannerRootStyle::CSS, selector_type: LegacyPlannerRootStyle::SELECTOR_TYPE, class_name: LegacyPlannerRootStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PlannerHeaderStyle::CSS, selector_type: PlannerHeaderStyle::SELECTOR_TYPE, class_name: PlannerHeaderStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PlannerSectionLabelStyle::CSS, selector_type: PlannerSectionLabelStyle::SELECTOR_TYPE, class_name: PlannerSectionLabelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TraineeSelectorRowStyle::CSS, selector_type: TraineeSelectorRowStyle::SELECTOR_TYPE, class_name: TraineeSelectorRowStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PlannerTreeGridStyle::CSS, selector_type: PlannerTreeGridStyle::SELECTOR_TYPE, class_name: PlannerTreeGridStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TreeTraineeStyle::CSS, selector_type: TreeTraineeStyle::SELECTOR_TYPE, class_name: TreeTraineeStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PlannerSlotStyle::CSS, selector_type: PlannerSlotStyle::SELECTOR_TYPE, class_name: PlannerSlotStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TreeAffinityStyle::CSS, selector_type: TreeAffinityStyle::SELECTOR_TYPE, class_name: TreeAffinityStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TreeAffinityCenterStyle::CSS, selector_type: TreeAffinityCenterStyle::SELECTOR_TYPE, class_name: TreeAffinityCenterStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TreeAffinityBoxStyle::CSS, selector_type: TreeAffinityBoxStyle::SELECTOR_TYPE, class_name: TreeAffinityBoxStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TreeAffinityEmptyStyle::CSS, selector_type: TreeAffinityEmptyStyle::SELECTOR_TYPE, class_name: TreeAffinityEmptyStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: AffinityValueStyle::CSS, selector_type: AffinityValueStyle::SELECTOR_TYPE, class_name: AffinityValueStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: AffinityValueStyle::CSS, selector_type: AffinityValueStyle::SELECTOR_TYPE, class_name: AffinityValueStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: AffinityBaseStyle::CSS, selector_type: AffinityBaseStyle::SELECTOR_TYPE, class_name: AffinityBaseStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: AffinityBonusStyle::CSS, selector_type: AffinityBonusStyle::SELECTOR_TYPE, class_name: AffinityBonusStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: AffinityPlusStyle::CSS, selector_type: AffinityPlusStyle::SELECTOR_TYPE, class_name: AffinityPlusStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SecondaryBtnStyle::CSS, selector_type: SecondaryBtnStyle::SELECTOR_TYPE, class_name: SecondaryBtnStyle::CLASS_NAME } }
