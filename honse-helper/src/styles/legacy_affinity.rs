use crate::styles::{Style, StyleDefinition};

pub struct LegacyAffinityTotalsBarStyle;

impl Style for LegacyAffinityTotalsBarStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: transparent;
            margin-top: 8px;
        }

        {{class}} .affinity-bar-header {
            padding: 12px 0 0 0;
            font-size: 11px;
            font-weight: 600;
            color: #94a3b8;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }

        {{class}} .affinity-bar-content {
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 16px;
            padding: 6px 0 0 0;
        }

        {{class}} .affinity-stat {
            display: flex;
            align-items: center;
            gap: 8px;
            font-size: 13px;
            color: #e2e8f0;
        }

        {{class}} .affinity-stat-label {
            font-size: 12px;
            color: #94a3b8;
            font-weight: 500;
        }

        {{class}} .affinity-divider {
            width: 1px;
            height: 24px;
            background: #475569;
        }

        {{class}} .affinity-value {
            display: inline-flex;
            align-items: center;
            justify-content: center;
            min-width: 34px;
            height: 24px;
            padding: 0 8px;
            border-radius: 999px;
            font-size: 13px;
            font-weight: 700;
        }

        {{class}} .affinity-value-total {
            background: #374151;
            color: #f3f4f6;
            border: 1px solid #4b5563;
        }

        {{class}} .affinity-value-base {
            background: #1e1b4b;
            color: #a78bfa;
            border: 1px solid #4c1d95;
        }

        {{class}} .affinity-value-bonus {
            background: #451a1a;
            color: #fbbf24;
            border: 1px solid #78350f;
        }

        @media (max-width: 560px) {
            {{class}} .affinity-bar-content {
                flex-direction: column;
                gap: 8px;
            }
            {{class}} .affinity-divider {
                width: 60px;
                height: 1px;
            }
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-affinity-totals-bar";
}

pub struct LegacySparkPillsRowStyle;

impl Style for LegacySparkPillsRowStyle {
    const CSS: &'static str = r#"
        {{class}} {
            padding: 0 0 0 0;
        }

        {{class}} .spark-section-header {
            padding: 8px 0 0 0;
            font-size: 11px;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            margin-bottom: 6px;
        }

        {{class}} .spark-section-header.blue {
            color: #60a5fa;
        }

        {{class}} .spark-section-header.pink {
            color: #f472b6;
        }

        {{class}} .spark-pills-list {
            display: flex;
            flex-wrap: wrap;
            gap: 4px;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-spark-pills-row";
}

pub struct LegacySparkPillStyle;

impl Style for LegacySparkPillStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: inline-flex;
            align-items: center;
            gap: 6px;
            padding: 4px 10px;
            border-radius: 3px;
            font-size: 12px;
            border: 1px solid #475569;
            background: #1f2937;
            white-space: nowrap;
        }

        {{class}}.spark-blue {
            border-left: 3px solid #3b82f6;
            background: #172554;
        }

        {{class}}.spark-pink {
            border-left: 3px solid #ec4899;
            background: #2d1b2e;
        }

        {{class}} .spark-name {
            font-weight: 500;
            color: #e2e8f0;
        }

        {{class}} .spark-stars {
            font-weight: bold;
            color: #60a5fa;
            font-size: 12px;
        }
    "#;

    const CLASS_NAME: &'static str = "legacy-spark-pill";
}

inventory::submit! { StyleDefinition { css: LegacyAffinityTotalsBarStyle::CSS, selector_type: LegacyAffinityTotalsBarStyle::SELECTOR_TYPE, class_name: LegacyAffinityTotalsBarStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacySparkPillsRowStyle::CSS, selector_type: LegacySparkPillsRowStyle::SELECTOR_TYPE, class_name: LegacySparkPillsRowStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: LegacySparkPillStyle::CSS, selector_type: LegacySparkPillStyle::SELECTOR_TYPE, class_name: LegacySparkPillStyle::CLASS_NAME } }
