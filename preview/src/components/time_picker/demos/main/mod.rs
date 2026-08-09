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
    let mut selected_time = use_signal(|| Some(time!(09:30:15)));
    let mut bounded_time = use_signal(|| Some(time!(09:30:00)));
    let disabled = use_signal(|| true);
    let read_only = use_signal(|| true);
    let labels = use_signal(|| TimePickerLabels {
        group: "Appointment time".to_string(),
        hour: "Appointment hour".to_string(),
        minute: "Appointment minute".to_string(),
        second: "Appointment second".to_string(),
        am_pm: "Appointment period".to_string(),
        clear: "Clear appointment".to_string(),
    });

    rsx! {
        div { style: "display: grid; gap: 1rem;",
            div {
                TimePicker {
                    selected_time: selected_time(),
                    on_value_change: move |value| selected_time.set(value),
                    with_seconds: true,
                    format: TimePickerFormat::TwelveHour,
                    am_pm_labels: ("am".to_string(), "pm".to_string()),
                    "data-testid": "time-picker-default",
                }
                output { "data-testid": "time-picker-default-state", "{state_value(selected_time())}" }
            }
            div {
                TimePicker {
                    selected_time: bounded_time(),
                    on_value_change: move |value| bounded_time.set(value),
                    min_time: time!(09:00),
                    max_time: time!(10:30),
                    steps: TimePickerSteps { hours: 1, minutes: 15, seconds: 1 },
                    "data-testid": "time-picker-bounded-steps",
                }
                output { "data-testid": "time-picker-bounded-steps-state", "{state_value(bounded_time())}" }
            }
            div {
                TimePicker {
                    selected_time: selected_time(),
                    on_value_change: move |value| selected_time.set(value),
                    with_seconds: true,
                    format: TimePickerFormat::TwelveHour,
                    labels,
                    clearable: true,
                    "data-testid": "time-picker-custom-labels",
                    "data-time-picker": "custom-attributes",
                }
            }
            div {
                TimePicker {
                    selected_time: selected_time(),
                    disabled,
                    "data-testid": "time-picker-disabled",
                }
                TimePicker {
                    selected_time: selected_time(),
                    read_only,
                    "data-testid": "time-picker-read-only",
                }
            }
        }
    }
}
