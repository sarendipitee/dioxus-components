use dioxus::prelude::*;
use dioxus_components::time_picker::*;
use time::{macros::time, Duration, Time};

fn state_value(value: Option<Time>) -> String {
    value
        .map(|value| format!("{:02}:{:02}:{:02}", value.hour(), value.minute(), value.second()))
        .unwrap_or_else(|| "none".to_string())
}

#[component]
pub fn Demo() -> Element {
    let mut selected_time_a = use_signal(|| None::<Time>);
    let mut selected_time_b = use_signal(|| None::<Time>);

    rsx! {
        div { style: "display: flex; gap: 2rem; flex-wrap: wrap;",
            div {
                p { style: "font-size: var(--text-xs); color: var(--muted-fg); margin-block-end: 0.5rem;", "Manual presets" }
                TimePicker {
                    selected_time: selected_time_a(),
                    on_value_change: move |value| selected_time_a.set(value),
                    format: TimePickerFormat::TwelveHour,
                    presets: vec![time!(09:00), time!(12:00), time!(14:30), time!(17:00), time!(20:00)],
                    "data-testid": "time-picker-presets-manual",
                }
                output { "data-testid": "time-picker-presets-manual-state", "{state_value(selected_time_a())}" }
            }
            div {
                p { style: "font-size: var(--text-xs); color: var(--muted-fg); margin-block-end: 0.5rem;", "Generated range (every 1.5h)" }
                TimePicker {
                    selected_time: selected_time_b(),
                    on_value_change: move |value| selected_time_b.set(value),
                    presets: time_range(time!(06:00), time!(22:00), Duration::hours(1) + Duration::minutes(30)),
                    "data-testid": "time-picker-presets-generated",
                }
                output { "data-testid": "time-picker-presets-generated-state", "{state_value(selected_time_b())}" }
            }
        }
    }
}
