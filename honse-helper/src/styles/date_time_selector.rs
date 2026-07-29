use crate::styles::{Style, StyleDefinition};

pub struct DateTimeSelectorStyle;

impl Style for DateTimeSelectorStyle {
    const CSS: &'static str = r#"
        {{class}} {
            margin-top: 8px;
        }
    "#;

    const CLASS_NAME: &'static str = "date-time-selector";
}

pub struct DateCalendarStyle;

impl Style for DateCalendarStyle {
    const CSS: &'static str = r#"
        {{class}} .day {
            color: #e2e8f0 !important;
        }

        {{class}} .day--nc-month {
            opacity: 0.35;
        }

        {{class}} .days--month .day:hover {
            background-color: #334155 !important;
        }

        {{class}} .days--month .day--selected,
        {{class}} .days--month .day--selected:hover {
            background-color: #2563eb !important;
            color: #ffffff !important;
        }

        {{class}} .btn {
            background: #334155;
            color: #e2e8f0;
            border: none;
            border-radius: 6px;
            cursor: pointer;
            padding: 2px 10px;
            font-size: 13px;
            line-height: 1.4;
        }

        {{class}} .btn:hover {
            background: #475569;
        }

        {{class}} .header {
            font-size: 13px;
            font-weight: 600;
            color: #e2e8f0;
            margin-bottom: 6px;
        }

        {{class}} .d-flex {
            display: flex;
            align-items: center;
            justify-content: space-between;
        }

        {{class}} .text-nowrap {
            white-space: nowrap;
        }

        {{class}} .text-center {
            text-align: center;
        }
    "#;

    const CLASS_NAME: &'static str = "date-calendar";
}

pub struct DateTimeClearStyle;

impl Style for DateTimeClearStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: none;
            border: none;
            color: #94a3b8;
            cursor: pointer;
            padding: 0 4px;
            font-size: 13px;
            line-height: 1;
        }

        {{class}}:hover {
            color: #f8fafc;
        }
    "#;

    const CLASS_NAME: &'static str = "date-time-clear";
}

inventory::submit! { StyleDefinition { css: DateTimeSelectorStyle::CSS, selector_type: DateTimeSelectorStyle::SELECTOR_TYPE, class_name: DateTimeSelectorStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: DateCalendarStyle::CSS, selector_type: DateCalendarStyle::SELECTOR_TYPE, class_name: DateCalendarStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: DateTimeClearStyle::CSS, selector_type: DateTimeClearStyle::SELECTOR_TYPE, class_name: DateTimeClearStyle::CLASS_NAME } }
