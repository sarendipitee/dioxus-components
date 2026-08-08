use dioxus::prelude::*;
use dioxus_components::time_input::*;
use dioxus_components::time_picker::TimePickerFormat;
use time::macros::time;

#[component]
pub fn Demo() -> Element {
    let mut default_time = use_signal(|| Some(time!(14:45)));
    let mut callback_count = use_signal(|| 0_u32);
    let mut twelve_hour_time = use_signal(|| Some(time!(23:58:30)));
    let mut bounded_time = use_signal(|| Some(time!(09:30)));
    let mut clearable_time = use_signal(|| Some(time!(08:15)));
    let mut reactive = use_signal(|| false);

    rsx! {
        div { style: "display: grid; gap: 1rem; max-width: 24rem;",
            section { "data-testid": "time-input-default",
                TimeInput {
                    label: "Start time",
                    description: "Enter the start time.",
                    selected_time: default_time(),
                    on_value_change: move |value| {
                        default_time.set(value);
                        callback_count += 1;
                    },
                }
                output { "data-testid": "time-input-value", "Value: {default_time():?}" }
                output { "data-testid": "time-input-change-count", "Changes: {callback_count}" }
            }

            section { "data-testid": "time-input-12h-seconds",
                TimeInput {
                    label: "Alarm time",
                    format: TimePickerFormat::TwelveHour,
                    with_seconds: true,
                    selected_time: twelve_hour_time(),
                    on_value_change: move |value| twelve_hour_time.set(value),
                }
            }

            section { "data-testid": "time-input-bounded",
                TimeInput {
                    label: "Office time",
                    selected_time: bounded_time(),
                    min_time: time!(09:00),
                    max_time: time!(17:00),
                    on_value_change: move |value| bounded_time.set(value),
                }
            }

            section { "data-testid": "time-input-disabled",
                TimeInput { label: "Disabled time", selected_time: Some(time!(10:20)), disabled: true }
            }

            section { "data-testid": "time-input-readonly",
                TimeInput { label: "Read only time", selected_time: Some(time!(11:25)), read_only: true }
            }

            section { "data-testid": "time-input-validation",
                TimeInput {
                    label: "Required time",
                    description: "Choose a time for the appointment.",
                    error: "A time is required.",
                    required: true,
                }
            }

            section { "data-testid": "time-input-clearable",
                TimeInput {
                    label: "Optional time",
                    selected_time: clearable_time(),
                    clearable: true,
                    on_value_change: move |value| clearable_time.set(value),
                }
                output { "data-testid": "time-input-clear-value", "Value: {clearable_time():?}" }
            }

            section { "data-testid": "time-input-reactive",
                TimeInput {
                    label: "Reactive time",
                    selected_time: Some(time!(12:00)),
                    class: if reactive() { "reactive-on" } else { "reactive-off" },
                    "data-state": if reactive() { "on" } else { "off" },
                }
                button {
                    "data-testid": "time-input-reactive-toggle",
                    onclick: move |_| reactive.toggle(),
                    "Toggle attributes"
                }
            }
        }
    }
}
