use crate::styles::{Style, StyleDefinition};

pub struct SparkItemStyle;

impl Style for SparkItemStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-flex;
            align-items: center;
            gap: 4px;
            font-size: 11px;
            padding: 2px 8px;
            border-radius: 4px;
            border: 1px solid transparent;
            white-space: nowrap;
        }
    "#;

    const CLASS_NAME: &'static str = "spark-item";
}

pub struct SparkStatStyle;

impl Style for SparkStatStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #1e3a5f;
            color: #93c5fd;
            border-color: #2563eb44;
        }
    "#;

    const CLASS_NAME: &'static str = "spark-stat";
}

pub struct SparkAptStyle;

impl Style for SparkAptStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #14402a;
            color: #6ee7b7;
            border-color: #059669aa;
        }
    "#;

    const CLASS_NAME: &'static str = "spark-apt";
}

pub struct SparkUniqueStyle;

impl Style for SparkUniqueStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #3b1f5e;
            color: #c4b5fd;
            border-color: #7c3aed44;
        }
    "#;

    const CLASS_NAME: &'static str = "spark-unique";
}

pub struct SparkOtherStyle;

impl Style for SparkOtherStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #1f2937;
            color: #9ca3af;
            border-color: #374151;
        }
    "#;

    const CLASS_NAME: &'static str = "spark-other";
}

pub struct SparkHighlightedStyle;

impl Style for SparkHighlightedStyle {
    const CSS: &'static str = r#"
        {{class}} {
            outline: 2px solid #f59e0b;
            outline-offset: 1px;
        }
    "#;

    const CLASS_NAME: &'static str = "spark-highlighted";
}

pub struct SparkItemNameStyle;

impl Style for SparkItemNameStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-weight: 500;
        }
    "#;

    const CLASS_NAME: &'static str = "spark-item-name";
}

pub struct SparkItemVeteranStyle;

impl Style for SparkItemVeteranStyle {
    const CSS: &'static str = r#"
        {{class}} {
            color: #fbbf24;
            font-size: 10px;
        }
    "#;

    const CLASS_NAME: &'static str = "spark-item-veteran";
}

pub struct SparkItemTotalStyle;

impl Style for SparkItemTotalStyle {
    const CSS: &'static str = r#"
        {{class}} {
            opacity: 0.8;
        }
    "#;

    const CLASS_NAME: &'static str = "spark-item-total";
}

pub struct SparkItemUmasStyle;

impl Style for SparkItemUmasStyle {
    const CSS: &'static str = r#"
        {{class}} {
            opacity: 0.65;
            font-size: 10px;
        }
    "#;

    const CLASS_NAME: &'static str = "spark-item-umas";
}

inventory::submit! { StyleDefinition { css: SparkItemStyle::CSS, selector_type: SparkItemStyle::SELECTOR_TYPE, class_name: SparkItemStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SparkStatStyle::CSS, selector_type: SparkStatStyle::SELECTOR_TYPE, class_name: SparkStatStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SparkAptStyle::CSS, selector_type: SparkAptStyle::SELECTOR_TYPE, class_name: SparkAptStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SparkUniqueStyle::CSS, selector_type: SparkUniqueStyle::SELECTOR_TYPE, class_name: SparkUniqueStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SparkOtherStyle::CSS, selector_type: SparkOtherStyle::SELECTOR_TYPE, class_name: SparkOtherStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SparkHighlightedStyle::CSS, selector_type: SparkHighlightedStyle::SELECTOR_TYPE, class_name: SparkHighlightedStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SparkItemNameStyle::CSS, selector_type: SparkItemNameStyle::SELECTOR_TYPE, class_name: SparkItemNameStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SparkItemVeteranStyle::CSS, selector_type: SparkItemVeteranStyle::SELECTOR_TYPE, class_name: SparkItemVeteranStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SparkItemTotalStyle::CSS, selector_type: SparkItemTotalStyle::SELECTOR_TYPE, class_name: SparkItemTotalStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: SparkItemUmasStyle::CSS, selector_type: SparkItemUmasStyle::SELECTOR_TYPE, class_name: SparkItemUmasStyle::CLASS_NAME } }
