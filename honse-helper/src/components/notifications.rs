use crate::styles::{
    notifications::{
        NotificationBodyStyle, NotificationCloseStyle, NotificationErrorStyle,
        NotificationInfoStyle, NotificationRootStyle, NotificationSuccessStyle,
        NotificationTextStyle, ToastOverlayStyle,
    },
    Style,
};
use gloo_timers::callback::Timeout;
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};
use yew::prelude::*;

static NEXT_NOTIF_ID: AtomicU32 = AtomicU32::new(0);

fn next_id() -> u32 {
    NEXT_NOTIF_ID.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone, PartialEq)]
pub enum NotificationKind {
    Success,
    Error,
    Info,
}

#[derive(Clone, PartialEq)]
pub struct Notification {
    pub id: u32,
    pub message: String,
    pub kind: NotificationKind,
}

impl Notification {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            id: 0,
            message: message.into(),
            kind: NotificationKind::Success,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            id: 0,
            message: message.into(),
            kind: NotificationKind::Error,
        }
    }

    pub fn info(message: impl Into<String>) -> Self {
        Self {
            id: 0,
            message: message.into(),
            kind: NotificationKind::Info,
        }
    }
}

// ── Reducer ───────────────────────────────────────────────────────

pub enum NotificationAction {
    Push(Notification),
    Remove(u32),
}

/// Newtype wrapper so we can implement `Reducible` on `Vec<Notification>`.
pub struct Notifications(pub Vec<Notification>);

impl Reducible for Notifications {
    type Action = NotificationAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut items = self.0.clone();

        match action {
            NotificationAction::Push(n) => {
                items.push(n);
            }
            NotificationAction::Remove(id) => {
                items.retain(|x| x.id != id);
            }
        }

        Rc::new(Notifications(items))
    }
}

// ── Overlay component ────────────────────────────────────────────

#[derive(Properties, PartialEq)]
pub struct NotificationOverlayProps {
    pub notifications: Vec<Notification>,
    pub on_close: Callback<u32>,
}

#[function_component(NotificationOverlay)]
pub fn notification_overlay(props: &NotificationOverlayProps) -> Html {
    if props.notifications.is_empty() {
        return Html::default();
    }

    let toasts: Html = props
        .notifications
        .iter()
        .map(|n| {
            let kind_class = match n.kind {
                NotificationKind::Success => NotificationSuccessStyle::CLASS_NAME,
                NotificationKind::Error => NotificationErrorStyle::CLASS_NAME,
                NotificationKind::Info => NotificationInfoStyle::CLASS_NAME,
            };
            let id = n.id;
            let on_close = props.on_close.clone();
            html! {
                <div class={classes!(NotificationRootStyle::CLASS_NAME, kind_class)} key={id}>
                    <div class={NotificationBodyStyle::CLASS_NAME}>
                        <span class={NotificationTextStyle::CLASS_NAME}>
                            { &n.message }
                        </span>
                    </div>
                    <button class={NotificationCloseStyle::CLASS_NAME} onclick={move |_| on_close.emit(id)}>
                        {"\u{00D7}"}
                    </button>
                </div>
            }
        })
        .collect();

    html! {
        <div class={ToastOverlayStyle::CLASS_NAME}>
            { toasts }
        </div>
    }
}

// ── Hook ─────────────────────────────────────────────────────────

/// Returns `(notifications, push, remove)`.
///
/// - `notifications` — `UseReducerHandle<Vec<Notification>>`, use `.clone()` for the overlay
/// - `push` — call `push(N::success("msg"))` to add a notification (ID + auto-dismiss timer managed internally)
/// - `remove` — call `remove(id)` to dismiss manually
///
/// Uses `use_reducer` so Push/Remove actions are processed atomically.
/// IDs are pre-assigned via a static atomic counter; each notification spawns its own
/// `Timeout` for auto-dismissal.
#[hook]
pub fn use_timed_notification(
    timeout_ms: u32,
) -> (
    UseReducerHandle<Notifications>,
    Rc<dyn Fn(Notification)>,
    Rc<dyn Fn(u32)>,
) {
    let items = use_reducer(|| Notifications(Vec::new()));

    let push = {
        let dispatcher = items.dispatcher();
        Rc::new(move |mut n: Notification| {
            n.id = next_id();
            let id = n.id;
            dispatcher.dispatch(NotificationAction::Push(n));
            let dispatcher = dispatcher.clone();
            Timeout::new(timeout_ms, move || {
                dispatcher.dispatch(NotificationAction::Remove(id));
            })
            .forget();
        }) as Rc<dyn Fn(Notification)>
    };

    let remove = {
        let dispatcher = items.dispatcher();
        Rc::new(move |id: u32| {
            dispatcher.dispatch(NotificationAction::Remove(id));
        }) as Rc<dyn Fn(u32)>
    };

    (items, push, remove)
}
