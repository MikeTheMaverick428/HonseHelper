use crate::styles::{Style, StyleDefinition};

pub struct WorkerStatusContainerStyle;

impl Style for WorkerStatusContainerStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
            height: 100vh;
            padding: 0;
            background: #0f1220;
            color: #f3f4f6;
        }
    "#;

    const CLASS_NAME: &'static str = "worker-status-container";
}

pub struct WorkerStatusHeaderStyle;

impl Style for WorkerStatusHeaderStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex-shrink: 0;
            background: #0f1220;
            border-bottom: 1px solid #1f2937;
            padding: 20px 24px 14px 24px;
            min-height: 0;
        }

        {{class}} h1 {
            font-size: 28px;
            font-weight: 600;
            margin-bottom: 0;
        }
    "#;

    const CLASS_NAME: &'static str = "worker-status-header";
}

pub struct WorkerStatusScrollStyle;

impl Style for WorkerStatusScrollStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex: 1;
            overflow-y: auto;
            overflow-x: hidden;
            padding: 16px 24px 24px 24px;
        }

        {{class}}::-webkit-scrollbar { width: 10px; }
        {{class}}::-webkit-scrollbar-track { background: transparent; }
        {{class}}::-webkit-scrollbar-thumb {
            background: #4b5563;
            border-radius: 5px;
            border: 3px solid transparent;
            background-clip: content-box;
        }
        {{class}}::-webkit-scrollbar-thumb:hover {
            background: #6b7280;
            border-color: transparent;
            background-clip: content-box;
        }
    "#;

    const CLASS_NAME: &'static str = "worker-status-scroll";
}

pub struct ConfigRowStyle;

impl Style for ConfigRowStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            gap: 12px;
            margin-bottom: 10px;
            flex-wrap: wrap;
        }
    "#;

    const CLASS_NAME: &'static str = "config-row";
}

pub struct ConfigLabelStyle;

impl Style for ConfigLabelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 13px;
            color: #cbd5e1;
            min-width: 140px;
        }
    "#;

    const CLASS_NAME: &'static str = "config-label";
}

pub struct ConfigInputStyle;

impl Style for ConfigInputStyle {
    const CSS: &'static str = r#"
        {{class}} {
            width: 80px;
            background: #0b1220;
            color: #f8fafc;
            border: 1px solid #334155;
            border-radius: 8px;
            padding: 8px 10px;
            font-size: 13px;
            text-align: center;
        }
    "#;

    const CLASS_NAME: &'static str = "config-input";
}

pub struct ConfigUnitStyle;

impl Style for ConfigUnitStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 12px;
            color: #94a3b8;
        }
    "#;

    const CLASS_NAME: &'static str = "config-unit";
}

pub struct ToggleLabelStyle;

impl Style for ToggleLabelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 13px;
            color: #cbd5e1;
            cursor: pointer;
            user-select: none;
        }
    "#;

    const CLASS_NAME: &'static str = "toggle-label";
}

pub struct ToggleCheckboxStyle;

impl Style for ToggleCheckboxStyle {
    const CSS: &'static str = r#"
        {{class}} {
            width: 16px;
            height: 16px;
            cursor: pointer;
            accent-color: #38bdf8;
        }
    "#;

    const CLASS_NAME: &'static str = "toggle-checkbox";
}

pub struct ViewCardStyle;

impl Style for ViewCardStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #0b1220;
            border: 1px solid #22304a;
            border-radius: 10px;
            padding: 12px;
            margin-bottom: 8px;
        }
    "#;

    const CLASS_NAME: &'static str = "view-card";
}

pub struct ViewRowStyle;

impl Style for ViewRowStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            gap: 8px;
            font-size: 13px;
            padding: 2px 0;
        }

        {{class}} :first-child {
            color: #94a3b8;
            min-width: 120px;
        }

        {{class}} :last-child {
            color: #e2e8f0;
            font-family: 'Monaco', 'Menlo', 'Consolas', monospace;
            font-size: 12px;
        }
    "#;

    const CLASS_NAME: &'static str = "view-row";
}

pub struct KnownViewBadgeStyle;

impl Style for KnownViewBadgeStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-block;
            background: rgba(56, 189, 248, 0.15);
            color: #38bdf8;
            border: 1px solid rgba(56, 189, 248, 0.3);
            border-radius: 6px;
            padding: 4px 10px;
            font-size: 14px;
            font-weight: 600;
            margin-bottom: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "known-view-badge";
}

pub struct UnknownViewBadgeStyle;

impl Style for UnknownViewBadgeStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-block;
            background: rgba(251, 191, 36, 0.15);
            color: #fbbf24;
            border: 1px solid rgba(251, 191, 36, 0.3);
            border-radius: 6px;
            padding: 4px 10px;
            font-size: 14px;
            font-weight: 600;
            margin-bottom: 8px;
        }
    "#;

    const CLASS_NAME: &'static str = "unknown-view-badge";
}

pub struct ControlsGridStyle;

impl Style for ControlsGridStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-wrap: wrap;
            gap: 8px;
            margin-top: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "controls-grid";
}

pub struct RetryCountStyle;

impl Style for RetryCountStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 13px;
            color: #e2e8f0;
            margin-left: 8px;
        }

        {{class}} .retry-current {
            color: #38bdf8;
            font-weight: 600;
        }

        {{class}} .retry-max {
            color: #94a3b8;
        }
    "#;

    const CLASS_NAME: &'static str = "retry-count";
}

inventory::submit! { StyleDefinition { css: WorkerStatusContainerStyle::CSS, selector_type: WorkerStatusContainerStyle::SELECTOR_TYPE, class_name: WorkerStatusContainerStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: WorkerStatusHeaderStyle::CSS, selector_type: WorkerStatusHeaderStyle::SELECTOR_TYPE, class_name: WorkerStatusHeaderStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: WorkerStatusScrollStyle::CSS, selector_type: WorkerStatusScrollStyle::SELECTOR_TYPE, class_name: WorkerStatusScrollStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ConfigRowStyle::CSS, selector_type: ConfigRowStyle::SELECTOR_TYPE, class_name: ConfigRowStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ConfigLabelStyle::CSS, selector_type: ConfigLabelStyle::SELECTOR_TYPE, class_name: ConfigLabelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ConfigInputStyle::CSS, selector_type: ConfigInputStyle::SELECTOR_TYPE, class_name: ConfigInputStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ConfigUnitStyle::CSS, selector_type: ConfigUnitStyle::SELECTOR_TYPE, class_name: ConfigUnitStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ToggleLabelStyle::CSS, selector_type: ToggleLabelStyle::SELECTOR_TYPE, class_name: ToggleLabelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ToggleCheckboxStyle::CSS, selector_type: ToggleCheckboxStyle::SELECTOR_TYPE, class_name: ToggleCheckboxStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ViewCardStyle::CSS, selector_type: ViewCardStyle::SELECTOR_TYPE, class_name: ViewCardStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ViewRowStyle::CSS, selector_type: ViewRowStyle::SELECTOR_TYPE, class_name: ViewRowStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: KnownViewBadgeStyle::CSS, selector_type: KnownViewBadgeStyle::SELECTOR_TYPE, class_name: KnownViewBadgeStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: UnknownViewBadgeStyle::CSS, selector_type: UnknownViewBadgeStyle::SELECTOR_TYPE, class_name: UnknownViewBadgeStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ControlsGridStyle::CSS, selector_type: ControlsGridStyle::SELECTOR_TYPE, class_name: ControlsGridStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: RetryCountStyle::CSS, selector_type: RetryCountStyle::SELECTOR_TYPE, class_name: RetryCountStyle::CLASS_NAME } }
