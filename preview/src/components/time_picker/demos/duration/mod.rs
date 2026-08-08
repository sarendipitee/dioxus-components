use dioxus::prelude::*;
use dioxus_components::time_picker::*;

fn state_value(value: Option<TimePickerValue>) -> String {
    match value {
        Some(TimePickerValue::Duration { hours, minutes, seconds }) => {
            format!("{hours}:{minutes:02}:{seconds:02}")
        }
        Some(TimePickerValue::Time(value)) => {
            format!("{:02}:{:02}:{:02}", value.hour(), value.minute(), value.second())
        }
        None => "none".to_string(),
    }
}

#[component]
pub fn Demo() -> Element {
    let mut selected_value = use_signal(|| {
        Some(TimePickerValue::Duration {
            hours: 36,
            minutes: 15,
            seconds: 30,
        })
    });

    rsx! {
        div { style: "display: grid; gap: 0.5rem; max-width: 20rem;",
            TimePicker {
                selected_value: selected_value(),
                on_picker_value_change: move |value| selected_value.set(value),
                picker_type: TimePickerType::Duration,
                with_seconds: true,
                min_hours_digits: 3,
                "data-testid": "time-picker-duration",
            }
            output { "data-testid": "time-picker-duration-state", "{state_value(selected_value())}" }
        }
    }
}
