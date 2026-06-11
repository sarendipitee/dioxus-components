use dioxus::prelude::*;
use dioxus_components::time_picker::*;
use dioxus_primitives::time_picker::{
    TimePickerFormat, TimePickerLabels, TimePickerSteps, TimePickerType, TimePickerValue,
};
use time::{macros::time, Time};

#[component]
fn DemoCard(title: &'static str, description: &'static str, children: Element) -> Element {
    rsx! {
        section {
            style: "display: grid; gap: 0.5rem; padding: 1rem; border: 1px solid color-mix(in srgb, currentColor 12%, transparent); border-radius: 0.75rem;",
            h3 {
                style: "margin: 0; font-size: 1rem;",
                {title}
            }
            p {
                style: "margin: 0; color: color-mix(in srgb, currentColor 68%, transparent); font-size: 0.95rem;",
                {description}
            }
            {children}
        }
    }
}

#[component]
pub fn Demo() -> Element {
    let mut basic_time = use_signal(|| Some(time!(14:45)));
    let mut clearable_time = use_signal(|| None::<Time>);
    let mut detailed_time = use_signal(|| Some(time!(09:30:15)));
    let mut duration = use_signal(|| {
        Some(TimePickerValue::Duration {
            hours: 36,
            minutes: 15,
            seconds: 30,
        })
    });

    rsx! {
        div {
            style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(18rem, 1fr)); gap: 1rem; max-width: 56rem;",

            DemoCard {
                title: "Basic time",
                description: "24-hour clock time with minute precision.",
                TimePicker {
                    selected_time: basic_time(),
                    on_value_change: move |value| basic_time.set(value),
                    TimePickerInput {}
                }
            }

            DemoCard {
                title: "Clearable",
                description: "Uses the primitive clear affordance and custom accessibility labels.",
                TimePicker {
                    selected_time: clearable_time(),
                    on_value_change: move |value| clearable_time.set(value),
                    clearable: true,
                    labels: TimePickerLabels {
                        group: "Reminder time".to_string(),
                        clear: "Clear reminder time".to_string(),
                        ..Default::default()
                    },
                    TimePickerInput {}
                }
            }

            DemoCard {
                title: "Seconds and 12-hour",
                description: "Second precision, 12-hour formatting, and custom step sizes.",
                TimePicker {
                    selected_time: detailed_time(),
                    on_value_change: move |value| detailed_time.set(value),
                    with_seconds: true,
                    format: TimePickerFormat::TwelveHour,
                    am_pm_labels: ("am".to_string(), "pm".to_string()),
                    min_time: time!(08:00:00),
                    max_time: time!(18:00:00),
                    steps: TimePickerSteps {
                        hours: 1,
                        minutes: 15,
                        seconds: 15,
                    },
                    TimePickerInput {}
                }
            }

            DemoCard {
                title: "Duration",
                description: "Duration mode switches to the value-based API for hour counts beyond 23.",
                TimePicker {
                    selected_value: duration(),
                    on_picker_value_change: move |value| duration.set(value),
                    picker_type: TimePickerType::Duration,
                    with_seconds: true,
                    min_hours_digits: 3,
                    clearable: true,
                    TimePickerInput {}
                }
            }
        }
    }
}
