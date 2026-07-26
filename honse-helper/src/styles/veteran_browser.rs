use crate::styles::{Style, StyleDefinition};

pub struct VeteranBrowserRootStyle;

impl Style for VeteranBrowserRootStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            flex-direction: column;
            height: 100vh;
            background: #0f1220;
            color: #f3f4f6;
            overflow: hidden;
        }
    "#;

    const CLASS_NAME: &'static str = "veteran-browser";
}

pub struct BrowserHeaderStyle;

impl Style for BrowserHeaderStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex-shrink: 0;
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 12px;
            padding: 16px 20px;
            border-bottom: 1px solid #1f2937;
            background: #0f1220;
        }

        {{class}} h1 {
            font-size: 22px;
            font-weight: 600;
            margin: 0;
        }

        @media (max-width: 768px) {
            {{class}} {
                flex-direction: column;
                align-items: flex-start;
            }
        }
    "#;

    const CLASS_NAME: &'static str = "browser-header";
}

pub struct BrowserHeaderControlsStyle;

impl Style for BrowserHeaderControlsStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            gap: 12px;
        }
    "#;

    const CLASS_NAME: &'static str = "browser-header-controls";
}

pub struct BrowserBodyStyle;

impl Style for BrowserBodyStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex: 1;
            display: flex;
            overflow: hidden;
        }

        @media (max-width: 768px) {
            {{class}} {
                flex-direction: column;
            }
        }
    "#;

    const CLASS_NAME: &'static str = "browser-body";
}

pub struct BrowserSidebarStyle;

impl Style for BrowserSidebarStyle {
    const CSS: &'static str = r#"
        {{class}} {
            width: 280px;
            flex-shrink: 0;
            overflow-y: auto;
            border-right: 1px solid #1f2937;
            padding: 12px;
            background: #0b1220;
        }

        {{class}}::-webkit-scrollbar { width: 6px; }
        {{class}}::-webkit-scrollbar-track { background: transparent; }
        {{class}}::-webkit-scrollbar-thumb { background: #374151; border-radius: 3px; }

        @media (max-width: 768px) {
            {{class}} {
                width: 100%;
                max-height: 200px;
                border-right: none;
                border-bottom: 1px solid #1f2937;
            }
        }
    "#;

    const CLASS_NAME: &'static str = "browser-sidebar";
}

pub struct BrowserMainStyle;

impl Style for BrowserMainStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex: 1;
            display: flex;
            flex-direction: column;
            overflow: hidden;
            padding: 16px 20px;
        }

        {{class}}::-webkit-scrollbar { width: 10px; }
        {{class}}::-webkit-scrollbar-track { background: transparent; }
        {{class}}::-webkit-scrollbar-thumb {
            background: #4b5563;
            border-radius: 5px;
            border: 3px solid transparent;
            background-clip: content-box;
        }
    "#;

    const CLASS_NAME: &'static str = "browser-main";
}

pub struct BrowserLoadingStyle;
pub struct BrowserErrorStyle;
pub struct BrowserEmptyStyle;

impl Style for BrowserLoadingStyle {
    const CSS: &'static str = r#"
        {{class}} {
            padding: 40px;
            text-align: center;
            color: #94a3b8;
            font-size: 15px;
        }
    "#;

    const CLASS_NAME: &'static str = "browser-loading";
}

impl Style for BrowserErrorStyle {
    const CSS: &'static str = r#"
        {{class}} {
            padding: 40px;
            text-align: center;
            color: #fca5a5;
            font-size: 15px;
        }
    "#;

    const CLASS_NAME: &'static str = "browser-error";
}

impl Style for BrowserEmptyStyle {
    const CSS: &'static str = r#"
        {{class}} {
            padding: 40px;
            text-align: center;
            color: #94a3b8;
            font-size: 15px;
        }
    "#;

    const CLASS_NAME: &'static str = "browser-empty";
}

pub struct BrowserTotalStyle;

impl Style for BrowserTotalStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin-bottom: 12px;
            font-size: 13px;
            color: #94a3b8;
        }
    "#;

    const CLASS_NAME: &'static str = "browser-total";
}

pub struct CardGridStyle;

impl Style for CardGridStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
            gap: 12px;
        }

        @media (max-width: 768px) {
            {{class}} {
                grid-template-columns: 1fr;
            }
        }
    "#;

    const CLASS_NAME: &'static str = "card-grid";
}

inventory::submit! { StyleDefinition { css: VeteranBrowserRootStyle::CSS, selector_type: VeteranBrowserRootStyle::SELECTOR_TYPE, class_name: VeteranBrowserRootStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: BrowserHeaderStyle::CSS, selector_type: BrowserHeaderStyle::SELECTOR_TYPE, class_name: BrowserHeaderStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: BrowserHeaderControlsStyle::CSS, selector_type: BrowserHeaderControlsStyle::SELECTOR_TYPE, class_name: BrowserHeaderControlsStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: BrowserBodyStyle::CSS, selector_type: BrowserBodyStyle::SELECTOR_TYPE, class_name: BrowserBodyStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: BrowserSidebarStyle::CSS, selector_type: BrowserSidebarStyle::SELECTOR_TYPE, class_name: BrowserSidebarStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: BrowserMainStyle::CSS, selector_type: BrowserMainStyle::SELECTOR_TYPE, class_name: BrowserMainStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: BrowserLoadingStyle::CSS, selector_type: BrowserLoadingStyle::SELECTOR_TYPE, class_name: BrowserLoadingStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: BrowserErrorStyle::CSS, selector_type: BrowserErrorStyle::SELECTOR_TYPE, class_name: BrowserErrorStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: BrowserEmptyStyle::CSS, selector_type: BrowserEmptyStyle::SELECTOR_TYPE, class_name: BrowserEmptyStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: BrowserTotalStyle::CSS, selector_type: BrowserTotalStyle::SELECTOR_TYPE, class_name: BrowserTotalStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: CardGridStyle::CSS, selector_type: CardGridStyle::SELECTOR_TYPE, class_name: CardGridStyle::CLASS_NAME } }
