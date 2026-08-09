use dioxus::prelude::*;
use dioxus_components::time_picker::*;
use time::{macros::time, Time};

fn state_value(value: Option<Time>) -> String {
    value
        .map(|value| {
            format!(
                "{:02}:{:02}:{:02}",
                value.hour(),
                value.minute(),
                value.second()
            )
        })
        .unwrap_or_else(|| "none".to_string())
}

#[component]
pub fn Demo() -> Element {
    let mut selected_time = use_signal(|| Some(time!(14:30:45)));

    rsx! {
        div { style: "display: grid; gap: 0.5rem;",
            TimePicker {
                selected_time: selected_time(),
                on_value_change: move |value| selected_time.set(value),
                with_seconds: true,
                format: TimePickerFormat::TwelveHour,
                am_pm_labels: ("morning".to_string(), "afternoon".to_string()),
                "data-testid": "time-picker-12-hour-seconds",
            }
            output { "data-testid": "time-picker-12-hour-seconds-state", "{state_value(selected_time())}" }
        }
    }
}
