use crate::styles::{Style, StyleDefinition};

pub struct PaginationStyle;

impl Style for PaginationStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 4px;
        }

        {{class}} button {
            padding: 2px 8px;
            font-size: 14px;
            min-width: 28px;
        }
    "#;

    const CLASS_NAME: &'static str = "pagination";
}

pub struct PageCurrentStyle;

impl Style for PageCurrentStyle {
    const CSS: &'static str = r#"
        {{class}} {
            min-width: 28px;
            text-align: center;
            font-size: 13px;
            font-weight: 600;
            color: #d1d5db;
            padding: 2px 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "page-current";
}

pub struct PageBtnStyle;

impl Style for PageBtnStyle {
    const CSS: &'static str = r#"
        {{class}} {
            min-width: 32px;
        }
    "#;

    const CLASS_NAME: &'static str = "page-btn";
}

pub struct PageActiveStyle;

impl Style for PageActiveStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #2563eb;
            font-weight: 600;
        }
    "#;

    const CLASS_NAME: &'static str = "page-active";
}

pub struct PageEllipsisStyle;

impl Style for PageEllipsisStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #64748b;
            padding: 0 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "page-ellipsis";
}

inventory::submit! { StyleDefinition { css: PaginationStyle::CSS, selector_type: PaginationStyle::SELECTOR_TYPE, class_name: PaginationStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PageCurrentStyle::CSS, selector_type: PageCurrentStyle::SELECTOR_TYPE, class_name: PageCurrentStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PageBtnStyle::CSS, selector_type: PageBtnStyle::SELECTOR_TYPE, class_name: PageBtnStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PageActiveStyle::CSS, selector_type: PageActiveStyle::SELECTOR_TYPE, class_name: PageActiveStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: PageEllipsisStyle::CSS, selector_type: PageEllipsisStyle::SELECTOR_TYPE, class_name: PageEllipsisStyle::CLASS_NAME } }
