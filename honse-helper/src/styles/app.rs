use crate::styles::{Style, StyleDefinition};

pub struct AppContainerStyle;

impl Style for AppContainerStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
            height: 100vh;
            max-width: 460px;
            margin: 0 auto;
            padding: 20px 16px;
            overflow-y: auto;
        }
    "#;

    const CLASS_NAME: &'static str = "container";
}

pub struct TopRowStyle;

impl Style for TopRowStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 12px;
            margin-bottom: 16px;
        }

        @media (max-width: 768px) {
            {{class}} {
                align-items: flex-start;
                flex-direction: column;
            }
        }
    "#;

    const CLASS_NAME: &'static str = "top-row";
}

pub struct MasterDbIndicatorStyle;

impl Style for MasterDbIndicatorStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #1f2937;
            border: 1px solid #334155;
            color: #e2e8f0;
            display: inline-flex;
            align-items: center;
            gap: 8px;
            padding: 8px 12px;
            border-radius: 999px;
            font-size: 13px;
        }
    "#;

    const CLASS_NAME: &'static str = "masterdb-indicator";
}

pub struct MasterDbDotStyle;

impl Style for MasterDbDotStyle {
    const CSS: &'static str = r#"
        {{class}} {
            width: 9px;
            height: 9px;
            border-radius: 50%;
            background: #94a3b8;
            box-shadow: 0 0 0 2px rgba(148, 163, 184, 0.15);
        }
    "#;

    const CLASS_NAME: &'static str = "masterdb-dot";
}

pub struct ButtonGroupStyle;

impl Style for ButtonGroupStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            gap: 8px;
            margin-bottom: 16px;
            flex-wrap: wrap;
        }
    "#;

    const CLASS_NAME: &'static str = "button-group";
}

pub struct FeatureCardStyle;

impl Style for FeatureCardStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #111827;
            border: 1px solid #263246;
            border-radius: 12px;
            padding: 16px 20px;
            margin-bottom: 12px;
            text-align: center;
        }

        {{class}} h2 {
            font-size: 15px;
            font-weight: 600;
            color: #f1f5f9;
            margin: 0 0 2px 0;
        }

        {{class}} p {
            font-size: 12px;
            color: #64748b;
            margin: 0 0 10px 0;
        }

        {{class}} > div {
            justify-content: center;
        }
    "#;

    const CLASS_NAME: &'static str = "feature-card";
}

pub struct LogViewerStyle;

impl Style for LogViewerStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex: 1;
            background: #111827;
            border: 1px solid #374151;
            border-radius: 8px;
            padding: 12px;
            overflow-y: auto;
            font-family: 'Monaco', 'Menlo', 'Consolas', monospace;
            font-size: 12px;
            line-height: 1.4;
        }

        {{class}}::-webkit-scrollbar { width: 8px; }
        {{class}}::-webkit-scrollbar-track { background: #1f2937; }
        {{class}}::-webkit-scrollbar-thumb { background: #4b5563; border-radius: 4px; }
        {{class}}::-webkit-scrollbar-thumb:hover { background: #6b7280; }
    "#;

    const CLASS_NAME: &'static str = "log-viewer";
}

pub struct LogEntryStyle;

impl Style for LogEntryStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            gap: 12px;
            margin-bottom: 4px;
            padding: 4px 0;
            border-bottom: 1px solid #1f2937;
        }
    "#;

    const CLASS_NAME: &'static str = "log-entry";
}

pub struct LogErrorStyle;

impl Style for LogErrorStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #fca5a5;
        }
    "#;

    const CLASS_NAME: &'static str = "log-error";
}

pub struct LogWarningStyle;

impl Style for LogWarningStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #fde047;
        }
    "#;

    const CLASS_NAME: &'static str = "log-warning";
}

pub struct LogInfoStyle;

impl Style for LogInfoStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #a5f3fc;
        }
    "#;

    const CLASS_NAME: &'static str = "log-info";
}

pub struct LogTimeStyle;

impl Style for LogTimeStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #9ca3af;
            min-width: 80px;
            text-align: right;
        }
    "#;

    const CLASS_NAME: &'static str = "log-time";
}

pub struct LogLabelStyle;

impl Style for LogLabelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #d1d5db;
            min-width: 100px;
            font-weight: 600;
        }
    "#;

    const CLASS_NAME: &'static str = "log-label";
}

pub struct LogMessageStyle;

impl Style for LogMessageStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #e5e7eb;
            word-break: break-word;
            flex: 1;
        }
    "#;

    const CLASS_NAME: &'static str = "log-message";
}

pub struct MasterDbReadyStyle;

impl Style for MasterDbReadyStyle {
    const CSS: &'static str = r#"
        {{class}} .masterdb-dot {
            background: #22c55e;
            box-shadow: 0 0 0 2px rgba(34, 197, 94, 0.25);
        }
    "#;

    const CLASS_NAME: &'static str = "masterdb-ready";
}

pub struct MasterDbMissingStyle;

impl Style for MasterDbMissingStyle {
    const CSS: &'static str = r#"
        {{class}} .masterdb-dot {
            background: #f97316;
            box-shadow: 0 0 0 2px rgba(249, 115, 22, 0.25);
        }
    "#;

    const CLASS_NAME: &'static str = "masterdb-missing";
}

pub struct MasterDbBusyStyle;

impl Style for MasterDbBusyStyle {
    const CSS: &'static str = r#"
        {{class}} .masterdb-dot {
            background: #38bdf8;
            box-shadow: 0 0 0 2px rgba(56, 189, 248, 0.25);
        }
    "#;

    const CLASS_NAME: &'static str = "masterdb-busy";
}

pub struct MasterDbPartialStyle;

impl Style for MasterDbPartialStyle {
    const CSS: &'static str = r#"
        {{class}} .masterdb-dot {
            background: #fbbf24;
            box-shadow: 0 0 0 2px rgba(251, 191, 36, 0.25);
        }
    "#;

    const CLASS_NAME: &'static str = "masterdb-partial";
}

pub struct WorkerIndicatorStyle;

impl Style for WorkerIndicatorStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #1f2937;
            border: 1px solid #334155;
            color: #e2e8f0;
            display: inline-flex;
            align-items: center;
            gap: 8px;
            padding: 8px 12px;
            border-radius: 999px;
            font-size: 13px;
            cursor: pointer;
            transition: background 0.15s ease;
        }

        {{class}}:hover {
            background: #283245;
        }
    "#;

    const CLASS_NAME: &'static str = "worker-indicator";
}

pub struct WorkerDotStyle;

impl Style for WorkerDotStyle {
    const CSS: &'static str = r#"
        {{class}} {
            width: 9px;
            height: 9px;
            border-radius: 50%;
            background: #94a3b8;
            box-shadow: 0 0 0 2px rgba(148, 163, 184, 0.15);
        }
    "#;

    const CLASS_NAME: &'static str = "worker-dot";
}

pub struct WorkerReadyStyle;

impl Style for WorkerReadyStyle {
    const CSS: &'static str = r#"
        {{class}} .worker-dot {
            background: #22c55e;
            box-shadow: 0 0 0 2px rgba(34, 197, 94, 0.25);
        }
    "#;

    const CLASS_NAME: &'static str = "worker-ready";
}

pub struct WorkerSearchingStyle;

impl Style for WorkerSearchingStyle {
    const CSS: &'static str = r#"
        {{class}} .worker-dot {
            background: #38bdf8;
            box-shadow: 0 0 0 2px rgba(56, 189, 248, 0.25);
        }
    "#;

    const CLASS_NAME: &'static str = "worker-searching";
}

pub struct WorkerStoppedStyle;

impl Style for WorkerStoppedStyle {
    const CSS: &'static str = r#"
        {{class}} .worker-dot {
            background: #94a3b8;
            box-shadow: 0 0 0 2px rgba(148, 163, 184, 0.15);
        }
    "#;

    const CLASS_NAME: &'static str = "worker-stopped";
}

pub struct WorkerErrorStyle;

impl Style for WorkerErrorStyle {
    const CSS: &'static str = r#"
        {{class}} .worker-dot {
            background: #f97316;
            box-shadow: 0 0 0 2px rgba(249, 115, 22, 0.25);
        }
    "#;

    const CLASS_NAME: &'static str = "worker-error";
}

pub struct HeaderStatusGroupStyle;

impl Style for HeaderStatusGroupStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            gap: 8px;
        }
    "#;

    const CLASS_NAME: &'static str = "header-status-group";
}

pub struct VersionPillStyle;

impl Style for VersionPillStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #1f2937;
            border: 1px solid #334155;
            color: #e2e8f0;
            display: inline-flex;
            align-items: center;
            gap: 8px;
            padding: 8px 12px;
            border-radius: 999px;
            font-size: 13px;
            white-space: nowrap;
            cursor: default;
        }
    "#;

    const CLASS_NAME: &'static str = "version-pill";
}

pub struct ApiIndicatorStyle;

impl Style for ApiIndicatorStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #1f2937;
            border: 1px solid #334155;
            color: #e2e8f0;
            display: inline-flex;
            align-items: center;
            gap: 8px;
            padding: 8px 12px;
            border-radius: 999px;
            font-size: 13px;
            cursor: pointer;
            transition: background 0.15s ease;
        }

        {{class}}:hover {
            background: #283245;
        }
    "#;

    const CLASS_NAME: &'static str = "api-indicator";
}

pub struct ApiDotStyle;

impl Style for ApiDotStyle {
    const CSS: &'static str = r#"
        {{class}} {
            width: 9px;
            height: 9px;
            border-radius: 50%;
            background: #94a3b8;
            box-shadow: 0 0 0 2px rgba(148, 163, 184, 0.15);
        }
    "#;

    const CLASS_NAME: &'static str = "api-dot";
}

pub struct ApiConfiguredStyle;

impl Style for ApiConfiguredStyle {
    const CSS: &'static str = r#"
        {{class}} .api-dot {
            background: #22c55e;
            box-shadow: 0 0 0 2px rgba(34, 197, 94, 0.25);
        }
    "#;

    const CLASS_NAME: &'static str = "api-configured";
}

pub struct ApiUnconfiguredStyle;

impl Style for ApiUnconfiguredStyle {
    const CSS: &'static str = r#"
        {{class}} .api-dot {
            background: #f97316;
            box-shadow: 0 0 0 2px rgba(249, 115, 22, 0.25);
        }
    "#;

    const CLASS_NAME: &'static str = "api-unconfigured";
}

pub struct ApiConfigContainerStyle;

impl Style for ApiConfigContainerStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
            height: 100vh;
            padding: 24px;
            background: #0f172a;
            color: #e2e8f0;
            overflow: hidden;
        }
    "#;

    const CLASS_NAME: &'static str = "api-config-container";
}

pub struct ApiConfigHeaderStyle;

impl Style for ApiConfigHeaderStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin-bottom: 24px;
        }

        {{class}} h1 {
            font-size: 20px;
            font-weight: 700;
            color: #f1f5f9;
            margin: 0;
        }

        {{class}} p {
            margin: 4px 0 0 0;
            font-size: 13px;
            color: #94a3b8;
        }
    "#;

    const CLASS_NAME: &'static str = "api-config-header";
}

pub struct ApiConfigFormStyle;

impl Style for ApiConfigFormStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
            gap: 16px;
        }

        {{class}} label {
            display: flex;
            flex-direction: column;
            gap: 6px;
            font-size: 13px;
            font-weight: 600;
            color: #cbd5e1;
        }

        {{class}} input {
            background: #1e293b;
            border: 1px solid #334155;
            border-radius: 8px;
            padding: 10px 14px;
            font-size: 14px;
            color: #e2e8f0;
            font-family: inherit;
        }

        {{class}} input:focus {
            outline: none;
            border-color: #6366f1;
            box-shadow: 0 0 0 2px rgba(99, 102, 241, 0.2);
        }

        {{class}} input::placeholder {
            color: #64748b;
        }
    "#;

    const CLASS_NAME: &'static str = "api-config-form";
}

pub struct ApiConfigStatusStyle;

impl Style for ApiConfigStatusStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            gap: 8px;
            padding: 10px 14px;
            border-radius: 8px;
            font-size: 13px;
            margin-top: 8px;
        }

        {{class}}.configured {
            background: #064e3b;
            border: 1px solid #065f46;
            color: #6ee7b7;
        }

        {{class}}.unconfigured {
            background: #451a03;
            border: 1px solid #78350f;
            color: #fdba74;
        }
    "#;

    const CLASS_NAME: &'static str = "api-config-status";
}

pub struct SupplIndicatorStyle;

impl Style for SupplIndicatorStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #1f2937;
            border: 1px solid #334155;
            color: #e2e8f0;
            display: inline-flex;
            align-items: center;
            gap: 8px;
            padding: 8px 12px;
            border-radius: 999px;
            font-size: 13px;
            cursor: pointer;
            transition: background 0.15s ease;
        }

        {{class}}:hover {
            background: #283245;
        }
    "#;

    const CLASS_NAME: &'static str = "suppl-indicator";
}

pub struct SupplDotStyle;

impl Style for SupplDotStyle {
    const CSS: &'static str = r#"
        {{class}} {
            width: 9px;
            height: 9px;
            border-radius: 50%;
            background: #94a3b8;
            box-shadow: 0 0 0 2px rgba(148, 163, 184, 0.15);
        }
    "#;

    const CLASS_NAME: &'static str = "suppl-dot";
}

pub struct SupplReadyStyle;

impl Style for SupplReadyStyle {
    const CSS: &'static str = r#"
        {{class}} .suppl-dot {
            background: #22c55e;
            box-shadow: 0 0 0 2px rgba(34, 197, 94, 0.25);
        }
    "#;

    const CLASS_NAME: &'static str = "suppl-ready";
}

pub struct SupplMissingStyle;

impl Style for SupplMissingStyle {
    const CSS: &'static str = r#"
        {{class}} .suppl-dot {
            background: #f97316;
            box-shadow: 0 0 0 2px rgba(249, 115, 22, 0.25);
        }
    "#;

    const CLASS_NAME: &'static str = "suppl-missing";
}

pub struct SupplUpdateStyle;

impl Style for SupplUpdateStyle {
    const CSS: &'static str = r#"
        {{class}} .suppl-dot {
            background: #eab308;
            box-shadow: 0 0 0 2px rgba(234, 179, 8, 0.3);
        }
    "#;

    const CLASS_NAME: &'static str = "suppl-update";
}

inventory::submit! { StyleDefinition { css: AppContainerStyle::CSS, selector_type: AppContainerStyle::SELECTOR_TYPE, class_name: AppContainerStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TopRowStyle::CSS, selector_type: TopRowStyle::SELECTOR_TYPE, class_name: TopRowStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: MasterDbIndicatorStyle::CSS, selector_type: MasterDbIndicatorStyle::SELECTOR_TYPE, class_name: MasterDbIndicatorStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: MasterDbDotStyle::CSS, selector_type: MasterDbDotStyle::SELECTOR_TYPE, class_name: MasterDbDotStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ButtonGroupStyle::CSS, selector_type: ButtonGroupStyle::SELECTOR_TYPE, class_name: ButtonGroupStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: FeatureCardStyle::CSS, selector_type: FeatureCardStyle::SELECTOR_TYPE, class_name: FeatureCardStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LogViewerStyle::CSS, selector_type: LogViewerStyle::SELECTOR_TYPE, class_name: LogViewerStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LogEntryStyle::CSS, selector_type: LogEntryStyle::SELECTOR_TYPE, class_name: LogEntryStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LogErrorStyle::CSS, selector_type: LogErrorStyle::SELECTOR_TYPE, class_name: LogErrorStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LogWarningStyle::CSS, selector_type: LogWarningStyle::SELECTOR_TYPE, class_name: LogWarningStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LogInfoStyle::CSS, selector_type: LogInfoStyle::SELECTOR_TYPE, class_name: LogInfoStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LogTimeStyle::CSS, selector_type: LogTimeStyle::SELECTOR_TYPE, class_name: LogTimeStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LogLabelStyle::CSS, selector_type: LogLabelStyle::SELECTOR_TYPE, class_name: LogLabelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LogMessageStyle::CSS, selector_type: LogMessageStyle::SELECTOR_TYPE, class_name: LogMessageStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: MasterDbReadyStyle::CSS, selector_type: MasterDbReadyStyle::SELECTOR_TYPE, class_name: MasterDbReadyStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: MasterDbMissingStyle::CSS, selector_type: MasterDbMissingStyle::SELECTOR_TYPE, class_name: MasterDbMissingStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: MasterDbBusyStyle::CSS, selector_type: MasterDbBusyStyle::SELECTOR_TYPE, class_name: MasterDbBusyStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: MasterDbPartialStyle::CSS, selector_type: MasterDbPartialStyle::SELECTOR_TYPE, class_name: MasterDbPartialStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: WorkerIndicatorStyle::CSS, selector_type: WorkerIndicatorStyle::SELECTOR_TYPE, class_name: WorkerIndicatorStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: WorkerDotStyle::CSS, selector_type: WorkerDotStyle::SELECTOR_TYPE, class_name: WorkerDotStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: WorkerReadyStyle::CSS, selector_type: WorkerReadyStyle::SELECTOR_TYPE, class_name: WorkerReadyStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: WorkerSearchingStyle::CSS, selector_type: WorkerSearchingStyle::SELECTOR_TYPE, class_name: WorkerSearchingStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: WorkerStoppedStyle::CSS, selector_type: WorkerStoppedStyle::SELECTOR_TYPE, class_name: WorkerStoppedStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: WorkerErrorStyle::CSS, selector_type: WorkerErrorStyle::SELECTOR_TYPE, class_name: WorkerErrorStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: HeaderStatusGroupStyle::CSS, selector_type: HeaderStatusGroupStyle::SELECTOR_TYPE, class_name: HeaderStatusGroupStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: VersionPillStyle::CSS, selector_type: VersionPillStyle::SELECTOR_TYPE, class_name: VersionPillStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ApiIndicatorStyle::CSS, selector_type: ApiIndicatorStyle::SELECTOR_TYPE, class_name: ApiIndicatorStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ApiDotStyle::CSS, selector_type: ApiDotStyle::SELECTOR_TYPE, class_name: ApiDotStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ApiConfiguredStyle::CSS, selector_type: ApiConfiguredStyle::SELECTOR_TYPE, class_name: ApiConfiguredStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ApiUnconfiguredStyle::CSS, selector_type: ApiUnconfiguredStyle::SELECTOR_TYPE, class_name: ApiUnconfiguredStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ApiConfigContainerStyle::CSS, selector_type: ApiConfigContainerStyle::SELECTOR_TYPE, class_name: ApiConfigContainerStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ApiConfigHeaderStyle::CSS, selector_type: ApiConfigHeaderStyle::SELECTOR_TYPE, class_name: ApiConfigHeaderStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ApiConfigFormStyle::CSS, selector_type: ApiConfigFormStyle::SELECTOR_TYPE, class_name: ApiConfigFormStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: ApiConfigStatusStyle::CSS, selector_type: ApiConfigStatusStyle::SELECTOR_TYPE, class_name: ApiConfigStatusStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SupplIndicatorStyle::CSS, selector_type: SupplIndicatorStyle::SELECTOR_TYPE, class_name: SupplIndicatorStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SupplDotStyle::CSS, selector_type: SupplDotStyle::SELECTOR_TYPE, class_name: SupplDotStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SupplReadyStyle::CSS, selector_type: SupplReadyStyle::SELECTOR_TYPE, class_name: SupplReadyStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SupplMissingStyle::CSS, selector_type: SupplMissingStyle::SELECTOR_TYPE, class_name: SupplMissingStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SupplUpdateStyle::CSS, selector_type: SupplUpdateStyle::SELECTOR_TYPE, class_name: SupplUpdateStyle::CLASS_NAME } }
