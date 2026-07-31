use chrono::{Locale, NaiveDate};
use shared::date_time::{normalize_bound, DateTimeRange};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_datepicker::Datepicker;

use crate::styles::{
    date_time_selector::{DateCalendarStyle, DateTimeClearStyle, DateTimeSelectorStyle},
    detail_modal::{
        ModalBodyStyle, ModalCloseStyle, ModalContentStyle, ModalHeaderStyle, ModalOverlayStyle,
    },
    filter_panel::{FilterInputStyle, FilterRangeStyle, FilterSectionStyle},
    Style,
};

#[derive(Properties, PartialEq)]
pub struct DateTimeRangeSelectorProps {
    pub value: DateTimeRange,
    pub on_change: Callback<DateTimeRange>,
    #[prop_or(true)]
    pub show_time: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Bound {
    After,
    Before,
}

fn split_bound(v: &Option<String>) -> (String, String) {
    match v {
        Some(s) => {
            let (d, t) = s
                .split_once(' ')
                .or_else(|| s.split_once('T'))
                .unwrap_or((s, ""));
            let t = t.get(..5).unwrap_or(t);
            (d.to_string(), t.to_string())
        }
        None => (String::new(), String::new()),
    }
}

#[function_component]
pub fn DateTimeRangeSelector(props: &DateTimeRangeSelectorProps) -> Html {
    let (initial_after_date, initial_after_time) = split_bound(&props.value.after);
    let (initial_before_date, initial_before_time) = split_bound(&props.value.before);

    let after_date = use_state(|| initial_after_date);
    let after_time = use_state(|| initial_after_time);
    let before_date = use_state(|| initial_before_date);
    let before_time = use_state(|| initial_before_time);

    let open = use_state(|| None as Option<Bound>);
    let after_reset = use_state(|| 0u32);
    let before_reset = use_state(|| 0u32);

    let show_time = props.show_time;

    let build_range = {
        let cb = props.on_change.clone();
        let show_time = show_time;
        move |after_date: &str, after_time: &str, before_date: &str, before_time: &str| {
            let raw_after = if after_date.is_empty() {
                String::new()
            } else if show_time {
                format!("{} {}", after_date, after_time)
            } else {
                after_date.to_string()
            };
            let raw_before = if before_date.is_empty() {
                String::new()
            } else if show_time {
                format!("{} {}", before_date, before_time)
            } else {
                before_date.to_string()
            };
            cb.emit(DateTimeRange {
                after: normalize_bound(&raw_after, true),
                before: normalize_bound(&raw_before, false),
            });
        }
    };

    let on_after_time = {
        let after_date = after_date.clone();
        let after_time = after_time.clone();
        let before_date = before_date.clone();
        let before_time = before_time.clone();
        let build = build_range.clone();
        Callback::from(move |e: InputEvent| {
            let t = e.target_unchecked_into::<HtmlInputElement>().value();
            after_time.set(t.clone());
            build(&after_date, &t, &before_date, &before_time);
        })
    };
    let on_before_time = {
        let after_date = after_date.clone();
        let after_time = after_time.clone();
        let before_date = before_date.clone();
        let before_time = before_time.clone();
        let build = build_range.clone();
        Callback::from(move |e: InputEvent| {
            let t = e.target_unchecked_into::<HtmlInputElement>().value();
            before_time.set(t.clone());
            build(&after_date, &after_time, &before_date, &t);
        })
    };

    let select_after_date = {
        let after_date = after_date.clone();
        let after_time = after_time.clone();
        let before_date = before_date.clone();
        let before_time = before_time.clone();
        let open = open.clone();
        let build = build_range.clone();
        Callback::from(move |d: NaiveDate| {
            let d = d.format("%Y-%m-%d").to_string();
            after_date.set(d.clone());
            open.set(None);
            build(&d, &after_time, &before_date, &before_time);
        })
    };
    let select_before_date = {
        let after_date = after_date.clone();
        let after_time = after_time.clone();
        let before_date = before_date.clone();
        let before_time = before_time.clone();
        let open = open.clone();
        let build = build_range.clone();
        Callback::from(move |d: NaiveDate| {
            let d = d.format("%Y-%m-%d").to_string();
            before_date.set(d.clone());
            open.set(None);
            build(&after_date, &after_time, &d, &before_time);
        })
    };

    let open_after = {
        let open = open.clone();
        Callback::from(move |_| open.set(Some(Bound::After)))
    };
    let open_before = {
        let open = open.clone();
        Callback::from(move |_| open.set(Some(Bound::Before)))
    };

    let close_modal = {
        let open = open.clone();
        Callback::from(move |_| open.set(None))
    };

    let clear_after = {
        let after_date = after_date.clone();
        let after_time = after_time.clone();
        let before_date = before_date.clone();
        let before_time = before_time.clone();
        let open = open.clone();
        let after_reset = after_reset.clone();
        let build = build_range.clone();
        Callback::from(move |_| {
            after_date.set(String::new());
            after_time.set(String::new());
            open.set(None);
            after_reset.set(*after_reset + 1);
            build("", "", &before_date, &before_time);
        })
    };
    let clear_before = {
        let after_date = after_date.clone();
        let after_time = after_time.clone();
        let before_date = before_date.clone();
        let before_time = before_time.clone();
        let open = open.clone();
        let before_reset = before_reset.clone();
        let build = build_range.clone();
        Callback::from(move |_| {
            before_date.set(String::new());
            before_time.set(String::new());
            open.set(None);
            before_reset.set(*before_reset + 1);
            build(&after_date, &after_time, "", "");
        })
    };

    let modal_title = match *open {
        Some(Bound::After) => "Select start date",
        Some(Bound::Before) => "Select end date",
        None => "Select date",
    };

    let bound_row = |label: &'static str,
                     date_value: String,
                     time_value: String,
                     on_date: Callback<MouseEvent>,
                     on_time: Callback<InputEvent>,
                     on_clear: Callback<MouseEvent>| {
        html! {
            <div class={FilterSectionStyle::CLASS_NAME}>
                <label>{label}</label>
                <div class={FilterRangeStyle::CLASS_NAME}>
                    <input class={FilterInputStyle::CLASS_NAME} type="text" readonly={true} value={date_value} placeholder={"Select date…".to_string()} onclick={on_date}/>
                    if show_time {
                        <input class={FilterInputStyle::CLASS_NAME} type="text" value={time_value} placeholder={"HH:MM (optional)".to_string()} maxlength="5" inputmode="numeric" oninput={on_time}/>
                    }
                    <button class={DateTimeClearStyle::CLASS_NAME} title="Clear" onclick={on_clear}>{"✕"}</button>
                </div>
            </div>
        }
    };

    html! {
        <div class={DateTimeSelectorStyle::CLASS_NAME}>
            {bound_row("After", (*after_date).clone(), (*after_time).clone(), open_after, on_after_time, clear_after)}
            {bound_row("Before", (*before_date).clone(), (*before_time).clone(), open_before, on_before_time, clear_before)}

            <div class={ModalOverlayStyle::CLASS_NAME}
                 style={if (*open).is_some() { "" } else { "display:none;" }}
                 onclick={close_modal.clone()}>
                <div class={ModalContentStyle::CLASS_NAME}
                     style="max-width: 320px; width: 100%;"
                     onclick={|e: yew::MouseEvent| e.stop_propagation()}>
                    <div class={ModalHeaderStyle::CLASS_NAME}>
                        <h2>{modal_title}</h2>
                        <button class={ModalCloseStyle::CLASS_NAME} onclick={close_modal}>
                            {"\u{00D7}"}
                        </button>
                    </div>
                    <div class={ModalBodyStyle::CLASS_NAME}>
                        <div class={DateCalendarStyle::CLASS_NAME}
                             style={if *open == Some(Bound::After) { "" } else { "display:none;" }}>
                            <Datepicker
                                key={format!("after:{}", *after_reset)}
                                on_select={select_after_date}
                                locale={Some(Locale::en_US)}
                            />
                        </div>
                        <div class={DateCalendarStyle::CLASS_NAME}
                             style={if *open == Some(Bound::Before) { "" } else { "display:none;" }}>
                            <Datepicker
                                key={format!("before:{}", *before_reset)}
                                on_select={select_before_date}
                                locale={Some(Locale::en_US)}
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
