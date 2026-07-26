use crate::styles::{Style, StyleDefinition};

pub struct DbStatusContainerStyle;

impl Style for DbStatusContainerStyle {
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

    const CLASS_NAME: &'static str = "db-status-container";
}

pub struct DbStatusHeaderStyle;

impl Style for DbStatusHeaderStyle {
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

    const CLASS_NAME: &'static str = "db-status-header";
}

pub struct DbStatusScrollStyle;

impl Style for DbStatusScrollStyle {
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

    const CLASS_NAME: &'static str = "db-status-scroll";
}

pub struct PanelStyle;

impl Style for PanelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: linear-gradient(180deg, #10182b 0%, #0f172a 100%);
            border: 1px solid #263246;
            border-radius: 14px;
            padding: 18px;
            margin-bottom: 18px;
            box-shadow: 0 10px 30px rgba(0, 0, 0, 0.25);
        }

        @media (max-width: 768px) {
            {{class}} {
                padding: 14px;
            }
        }
    "#;

    const CLASS_NAME: &'static str = "panel";
}

pub struct PanelCompactStyle;

impl Style for PanelCompactStyle {
    const CSS: &'static str = r#"
        {{class}} {
            padding: 14px;
        }
    "#;

    const CLASS_NAME: &'static str = "panel-compact";
}

pub struct PanelHeaderStyle;

impl Style for PanelHeaderStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            justify-content: space-between;
            gap: 16px;
            align-items: flex-start;
            margin-bottom: 14px;
        }

        {{class}} h2 {
            font-size: 18px;
            margin-bottom: 4px;
        }

        {{class}} p {
            color: #94a3b8;
            font-size: 13px;
        }
    "#;

    const CLASS_NAME: &'static str = "panel-header";
}

pub struct StatusPillStyle;

impl Style for StatusPillStyle {
    const CSS: &'static str = r#"
        {{class}} {
            border: 1px solid #334155;
            border-radius: 999px;
            padding: 6px 10px;
            color: #cbd5e1;
            font-size: 12px;
            background: rgba(15, 23, 42, 0.75);
            white-space: nowrap;
        }
    "#;

    const CLASS_NAME: &'static str = "status-pill";
}

pub struct StatusGridStyle;

impl Style for StatusGridStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: grid;
            grid-template-columns: repeat(3, minmax(0, 1fr));
            gap: 10px;
            margin-bottom: 14px;
        }

        @media (max-width: 768px) {
            {{class}} {
                grid-template-columns: 1fr;
            }
        }
    "#;

    const CLASS_NAME: &'static str = "status-grid";
}

pub struct StatusCardStyle;

impl Style for StatusCardStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #0b1220;
            border: 1px solid #22304a;
            border-radius: 10px;
            padding: 12px;
            display: flex;
            flex-direction: column;
            gap: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "status-card";
}

pub struct StatusLabelStyle;

impl Style for StatusLabelStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 12px;
            color: #94a3b8;
        }
    "#;

    const CLASS_NAME: &'static str = "status-label";
}

pub struct StatusMessageStyle;

impl Style for StatusMessageStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #e2e8f0;
            font-size: 13px;
            margin-bottom: 8px;
        }
    "#;

    const CLASS_NAME: &'static str = "status-message";
}

pub struct PathRowStyle;

impl Style for PathRowStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            gap: 8px;
            flex-wrap: wrap;
            margin-bottom: 10px;
        }
    "#;

    const CLASS_NAME: &'static str = "path-row";
}

pub struct PathInputStyle;

impl Style for PathInputStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex: 1;
            min-width: 260px;
            background: #0b1220;
            color: #f8fafc;
            border: 1px solid #334155;
            border-radius: 8px;
            padding: 10px 12px;
            font-size: 14px;
        }
    "#;

    const CLASS_NAME: &'static str = "path-input";
}

pub struct DetectedPathStyle;

impl Style for DetectedPathStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #7dd3fc;
            font-size: 12px;
            word-break: break-all;
        }
    "#;

    const CLASS_NAME: &'static str = "detected-path";
}

pub struct TableWrapperStyle;

impl Style for TableWrapperStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin: 12px 0;
            overflow-x: auto;
            border-radius: 8px;
            border: 1px solid #22304a;
        }
    "#;

    const CLASS_NAME: &'static str = "table-wrapper";
}

pub struct SyncTableStyle;

impl Style for SyncTableStyle {
    const CSS: &'static str = r#"
        {{class}} {
            width: 100%;
            border-collapse: collapse;
            font-size: 13px;
        }

        {{class}} thead {
            background: #0b1220;
            border-bottom: 2px solid #334155;
        }

        {{class}} th {
            padding: 12px 14px;
            text-align: left;
            font-weight: 600;
            color: #cbd5e1;
            letter-spacing: 0.5px;
        }

        {{class}} tbody tr {
            border-bottom: 1px solid #22304a;
            transition: background 0.15s ease;
        }

        {{class}} tbody tr:hover {
            background: rgba(15, 23, 42, 0.6);
        }

        {{class}} td {
            padding: 12px 14px;
            color: #e2e8f0;
        }
    "#;

    const CLASS_NAME: &'static str = "sync-table";
}

pub struct TextRightStyle;

impl Style for TextRightStyle {
    const CSS: &'static str = r#"
        {{class}} {
            text-align: right;
            font-feature-settings: 'tnum' 1;
            color: #94a3b8;
        }
    "#;

    const CLASS_NAME: &'static str = "text-right";
}

pub struct TextMonoStyle;

impl Style for TextMonoStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-family: 'Monaco', 'Menlo', 'Consolas', monospace;
            font-size: 12px;
            color: #7dd3fc;
            letter-spacing: -0.5px;
        }
    "#;

    const CLASS_NAME: &'static str = "text-mono";
}

inventory::submit! { StyleDefinition { css: DbStatusContainerStyle::CSS, selector_type: DbStatusContainerStyle::SELECTOR_TYPE, class_name: DbStatusContainerStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: DbStatusHeaderStyle::CSS, selector_type: DbStatusHeaderStyle::SELECTOR_TYPE, class_name: DbStatusHeaderStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: DbStatusScrollStyle::CSS, selector_type: DbStatusScrollStyle::SELECTOR_TYPE, class_name: DbStatusScrollStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PanelStyle::CSS, selector_type: PanelStyle::SELECTOR_TYPE, class_name: PanelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PanelCompactStyle::CSS, selector_type: PanelCompactStyle::SELECTOR_TYPE, class_name: PanelCompactStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PanelHeaderStyle::CSS, selector_type: PanelHeaderStyle::SELECTOR_TYPE, class_name: PanelHeaderStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: StatusPillStyle::CSS, selector_type: StatusPillStyle::SELECTOR_TYPE, class_name: StatusPillStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: StatusGridStyle::CSS, selector_type: StatusGridStyle::SELECTOR_TYPE, class_name: StatusGridStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: StatusCardStyle::CSS, selector_type: StatusCardStyle::SELECTOR_TYPE, class_name: StatusCardStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: StatusLabelStyle::CSS, selector_type: StatusLabelStyle::SELECTOR_TYPE, class_name: StatusLabelStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: StatusMessageStyle::CSS, selector_type: StatusMessageStyle::SELECTOR_TYPE, class_name: StatusMessageStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PathRowStyle::CSS, selector_type: PathRowStyle::SELECTOR_TYPE, class_name: PathRowStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PathInputStyle::CSS, selector_type: PathInputStyle::SELECTOR_TYPE, class_name: PathInputStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: DetectedPathStyle::CSS, selector_type: DetectedPathStyle::SELECTOR_TYPE, class_name: DetectedPathStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TableWrapperStyle::CSS, selector_type: TableWrapperStyle::SELECTOR_TYPE, class_name: TableWrapperStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SyncTableStyle::CSS, selector_type: SyncTableStyle::SELECTOR_TYPE, class_name: SyncTableStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TextRightStyle::CSS, selector_type: TextRightStyle::SELECTOR_TYPE, class_name: TextRightStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: TextMonoStyle::CSS, selector_type: TextMonoStyle::SELECTOR_TYPE, class_name: TextMonoStyle::CLASS_NAME } }
