use crate::styles::{Style, StyleDefinition};

pub struct ToastOverlayStyle;

impl Style for ToastOverlayStyle {
    const CSS: &'static str = r#"
        {{class}} {
            position: fixed;
            top: 20px;
            left: 20px;
            z-index: 1000;
            display: flex;
            flex-direction: column;
            gap: 8px;
        }
    "#;

    const CLASS_NAME: &'static str = "toast-overlay";
}

pub struct NotificationRootStyle;

impl Style for NotificationRootStyle {
    const CSS: &'static str = r#"
        {{class}} {
            display: flex;
            align-items: flex-start;
            gap: 0;
            padding: 0;
            border-radius: 10px;
            min-width: 280px;
            max-width: 480px;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
            animation: toastSlideIn 0.25s ease-out;
            overflow: hidden;
        }

        @keyframes toastSlideIn {
            from { opacity: 0; transform: translateX(-16px); }
            to { opacity: 1; transform: translateX(0); }
        }
    "#;

    const CLASS_NAME: &'static str = "notification-root";
}

pub struct NotificationBodyStyle;

impl Style for NotificationBodyStyle {
    const CSS: &'static str = r#"
        {{class}} {
            flex: 1;
            padding: 14px 12px;
            min-width: 0;
        }
    "#;

    const CLASS_NAME: &'static str = "notification-body";
}

pub struct NotificationTextStyle;

impl Style for NotificationTextStyle {
    const CSS: &'static str = r#"
        {{class}} {
            font-size: 13px;
            font-weight: 500;
            line-height: 1.5;
            word-break: break-word;
        }
    "#;

    const CLASS_NAME: &'static str = "notification-text";
}

pub struct NotificationCloseStyle;

impl Style for NotificationCloseStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: none;
            border: none;
            font-size: 18px;
            cursor: pointer;
            padding: 12px 14px;
            line-height: 1;
            opacity: 0.7;
            transition: opacity 0.15s;
            flex-shrink: 0;
        }

        {{class}}:hover {
            opacity: 1;
        }
    "#;

    const CLASS_NAME: &'static str = "notification-close";
}

// ── Kind variants ────────────────────────────────────────────────

pub struct NotificationSuccessStyle;

impl Style for NotificationSuccessStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #065f46;
            border: 1px solid #34d399;
        }

        {{class}} .notification-text {
            color: #d1fae5;
        }

        {{class}} .notification-close {
            color: #6ee7b7;
        }

        {{class}} .notification-close:hover {
            color: #d1fae5;
        }
    "#;

    const CLASS_NAME: &'static str = "notification-success";
}

pub struct NotificationErrorStyle;

impl Style for NotificationErrorStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #7f1d1d;
            border: 1px solid #f87171;
        }

        {{class}} .notification-text {
            color: #fecaca;
        }

        {{class}} .notification-close {
            color: #fca5a5;
        }

        {{class}} .notification-close:hover {
            color: #fecaca;
        }
    "#;

    const CLASS_NAME: &'static str = "notification-error";
}

pub struct NotificationInfoStyle;

impl Style for NotificationInfoStyle {
    const CSS: &'static str = r#"
        {{class}} {
            background: #1e3a5f;
            border: 1px solid #60a5fa;
        }

        {{class}} .notification-text {
            color: #bfdbfe;
        }

        {{class}} .notification-close {
            color: #93c5fd;
        }

        {{class}} .notification-close:hover {
            color: #bfdbfe;
        }
    "#;

    const CLASS_NAME: &'static str = "notification-info";
}

inventory::submit! { StyleDefinition { css: ToastOverlayStyle::CSS, selector_type: ToastOverlayStyle::SELECTOR_TYPE, class_name: ToastOverlayStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: NotificationRootStyle::CSS, selector_type: NotificationRootStyle::SELECTOR_TYPE, class_name: NotificationRootStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: NotificationBodyStyle::CSS, selector_type: NotificationBodyStyle::SELECTOR_TYPE, class_name: NotificationBodyStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: NotificationTextStyle::CSS, selector_type: NotificationTextStyle::SELECTOR_TYPE, class_name: NotificationTextStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: NotificationCloseStyle::CSS, selector_type: NotificationCloseStyle::SELECTOR_TYPE, class_name: NotificationCloseStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: NotificationSuccessStyle::CSS, selector_type: NotificationSuccessStyle::SELECTOR_TYPE, class_name: NotificationSuccessStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: NotificationErrorStyle::CSS, selector_type: NotificationErrorStyle::SELECTOR_TYPE, class_name: NotificationErrorStyle::CLASS_NAME } }
inventory::submit! { StyleDefinition { css: NotificationInfoStyle::CSS, selector_type: NotificationInfoStyle::SELECTOR_TYPE, class_name: NotificationInfoStyle::CLASS_NAME } }
