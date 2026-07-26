use crate::styles::{Style, StyleDefinition};

// ── Delete Button ────────────────────────────────────────────────

pub struct DeleteButtonStyle;

impl Style for DeleteButtonStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: rgba(239, 68, 68, 0.15);
            border: 1px solid rgba(239, 68, 68, 0.35);
            color: #ef4444;
            cursor: pointer;
            padding: 4px 8px;
            border-radius: 6px;
            font-size: 14px;
            line-height: 1;
            transition: all 0.15s;
        }
        {{class}}:hover:not(:disabled) {
            background: rgba(239, 68, 68, 0.3);
            border-color: #ef4444;
        }
        {{class}}:disabled {
            opacity: 0.5;
            cursor: not-allowed;
        }
    "#;
    const CLASS_NAME: &'static str = "delete-btn-reusable";
}

// ── Header Action Button ─────────────────────────────────────────

pub struct HeaderActionButtonStyle;

impl Style for HeaderActionButtonStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-flex;
            align-items: center;
            justify-content: center;
            min-height: 32px;
            padding: 0 12px;
            border: 1px solid #475569;
            border-radius: 999px;
            background: #1f2937;
            color: #e2e8f0;
            cursor: pointer;
            font-size: 12px;
            font-weight: 700;
            letter-spacing: 0.03em;
            transition: border-color 0.15s ease, background-color 0.15s ease, color 0.15s ease;
            white-space: nowrap;
        }

        {{class}}:hover {
            border-color: #64748b;
            background: #334155;
            color: #f8fafc;
        }
    "#;

    const CLASS_NAME: &'static str = "veteran-card-header-action-button";
}

// ── Copyable Value ───────────────────────────────────────────────

pub struct CopyableValueStyle;

impl Style for CopyableValueStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-flex;
            align-items: center;
            gap: 8px;
        }
    "#;

    const CLASS_NAME: &'static str = "copyable-value";
}

pub struct CopyableButtonStyle;

impl Style for CopyableButtonStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-flex;
            align-items: center;
            gap: 8px;
            padding: 8px 12px;
            background: #1f2937;
            border: 1px solid #475569;
            border-radius: 4px;
            cursor: pointer;
            font-family: monospace;
            font-size: 13px;
            transition: all 0.2s ease;
            color: #e2e8f0;
        }

        {{class}}:hover {
            background: #334155;
            border-color: #64748b;
        }

        {{class}}.copied {
            background: #064e3b;
            border-color: #10b981;
            color: #6ee7b7;
        }
    "#;

    const CLASS_NAME: &'static str = "copyable-button";
}

pub struct CopyableLabelStyle;

impl Style for CopyableLabelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 11px;
            color: #64748b;
            text-transform: uppercase;
            font-weight: 600;
            letter-spacing: 0.5px;
        }
    "#;

    const CLASS_NAME: &'static str = "copyable-label";
}

pub struct CopyableDisplayStyle;

impl Style for CopyableDisplayStyle {
    const CSS: &'static str = r#"
        {{class}} {
            word-break: break-all;
            max-width: 300px;
        }
    "#;

    const CLASS_NAME: &'static str = "copyable-display";
}

pub struct CopyIconStyle;

impl Style for CopyIconStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 14px;
            opacity: 0.7;
        }
    "#;

    const CLASS_NAME: &'static str = "copy-icon";
}

pub struct CopyFeedbackStyle;

impl Style for CopyFeedbackStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 12px;
            font-weight: 600;
            color: #28a745;
            animation: fadeInOut 2s ease;
        }

        @keyframes fadeInOut {
            0% { opacity: 0; }
            10% { opacity: 1; }
            90% { opacity: 1; }
            100% { opacity: 0; }
        }
    "#;

    const CLASS_NAME: &'static str = "copy-feedback";
}

// ── Sparks Shared ────────────────────────────────────────────────

pub struct SparksContainerStyle;

impl Style for SparksContainerStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
            gap: 6px;
        }
    "#;

    const CLASS_NAME: &'static str = "sparks-container";
}

pub struct SparksSectionStyle;

impl Style for SparksSectionStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin-top: 6px;
        }

        {{class}}:first-child {
            margin-top: 0;
        }
    "#;

    const CLASS_NAME: &'static str = "sparks-section";
}

pub struct SparksGridStyle;

impl Style for SparksGridStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-wrap: wrap;
            gap: 4px;
            justify-content: flex-start;
            align-items: center;
        }
    "#;

    const CLASS_NAME: &'static str = "sparks-grid";
}

// ── Shared Modal ─────────────────────────────────────────────────

pub struct SharedModalOverlayStyle;

impl Style for SharedModalOverlayStyle {
    const CSS: &'static str = r#"
        {{class}} {
            position: fixed;
            inset: 0;
            background: rgba(0, 0, 0, 0.45);
            display: flex;
            align-items: center;
            justify-content: center;
            z-index: 1190;
        }
    "#;

    const CLASS_NAME: &'static str = "shared-modal-overlay";
}

pub struct SharedModalContentStyle;

impl Style for SharedModalContentStyle {
    const CSS: &'static str = r#"
        {{class}} {
            width: min(860px, 94vw);
            max-height: 90vh;
            overflow-y: auto;
            background: white;
            border-radius: 10px;
            box-shadow: 0 10px 30px rgba(0, 0, 0, 0.25);
            padding: 16px;
        }
    "#;

    const CLASS_NAME: &'static str = "shared-modal-content";
}

pub struct SharedModalHeaderStyle;

impl Style for SharedModalHeaderStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 12px;
            margin-bottom: 8px;
        }
    "#;

    const CLASS_NAME: &'static str = "shared-modal-header";
}

pub struct SharedModalCloseButtonStyle;

impl Style for SharedModalCloseButtonStyle {
    const CSS: &'static str = r#"
        {{class}} {
            padding: 8px 12px;
            border: 1px solid #d1d5db;
            border-radius: 6px;
            background: white;
            cursor: pointer;
        }
    "#;

    const CLASS_NAME: &'static str = "shared-modal-close-button";
}

// ── Loading Overlay ────────────────────────────────────────────

pub struct LoadingOverlayStyle;

impl Style for LoadingOverlayStyle {
    const CSS: &'static str = r#"
        {{class}} {
            position: fixed;
            inset: 0;
            background: rgba(0, 0, 0, 0.35);
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            z-index: 1200;
            gap: 14px;
        }
        {{class}} .loading-spinner {
            width: 36px;
            height: 36px;
            border: 3px solid rgba(255, 255, 255, 0.25);
            border-top-color: #e2e8f0;
            border-radius: 50%;
            animation: loading-spin 0.7s linear infinite;
        }
        {{class}} .loading-label {
            color: #e2e8f0;
            font-size: 13px;
            font-weight: 500;
        }
        @keyframes loading-spin {
            to { transform: rotate(360deg); }
        }
    "#;

    const CLASS_NAME: &'static str = "shared-loading-overlay";
}

inventory::submit! { StyleDefinition { css: DeleteButtonStyle::CSS, selector_type: DeleteButtonStyle::SELECTOR_TYPE, class_name: DeleteButtonStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: HeaderActionButtonStyle::CSS, selector_type: HeaderActionButtonStyle::SELECTOR_TYPE, class_name: HeaderActionButtonStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CopyableValueStyle::CSS, selector_type: CopyableValueStyle::SELECTOR_TYPE, class_name: CopyableValueStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CopyableButtonStyle::CSS, selector_type: CopyableButtonStyle::SELECTOR_TYPE, class_name: CopyableButtonStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CopyableLabelStyle::CSS, selector_type: CopyableLabelStyle::SELECTOR_TYPE, class_name: CopyableLabelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CopyableDisplayStyle::CSS, selector_type: CopyableDisplayStyle::SELECTOR_TYPE, class_name: CopyableDisplayStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CopyIconStyle::CSS, selector_type: CopyIconStyle::SELECTOR_TYPE, class_name: CopyIconStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CopyFeedbackStyle::CSS, selector_type: CopyFeedbackStyle::SELECTOR_TYPE, class_name: CopyFeedbackStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SparksContainerStyle::CSS, selector_type: SparksContainerStyle::SELECTOR_TYPE, class_name: SparksContainerStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SparksSectionStyle::CSS, selector_type: SparksSectionStyle::SELECTOR_TYPE, class_name: SparksSectionStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SparksGridStyle::CSS, selector_type: SparksGridStyle::SELECTOR_TYPE, class_name: SparksGridStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SharedModalOverlayStyle::CSS, selector_type: SharedModalOverlayStyle::SELECTOR_TYPE, class_name: SharedModalOverlayStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SharedModalContentStyle::CSS, selector_type: SharedModalContentStyle::SELECTOR_TYPE, class_name: SharedModalContentStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SharedModalHeaderStyle::CSS, selector_type: SharedModalHeaderStyle::SELECTOR_TYPE, class_name: SharedModalHeaderStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SharedModalCloseButtonStyle::CSS, selector_type: SharedModalCloseButtonStyle::SELECTOR_TYPE, class_name: SharedModalCloseButtonStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LoadingOverlayStyle::CSS, selector_type: LoadingOverlayStyle::SELECTOR_TYPE, class_name: LoadingOverlayStyle::CLASS_NAME } }
