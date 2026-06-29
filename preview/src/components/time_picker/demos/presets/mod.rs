use dioxus::prelude::*;
use dioxus_components::time_picker::*;
use time::{macros::time, Duration, Time};

#[component]
pub fn Demo() -> Element {
    let mut selected_time_a = use_signal(|| None::<Time>);
    let mut selected_time_b = use_signal(|| None::<Time>);

    rsx! {
        div { style: "display: flex; gap: 2rem; flex-wrap: wrap;",
            div {
                p { style: "font-size: var(--text-xs); color: var(--muted-fg); margin-block-end: 0.5rem;",
                    "Manual presets"
                }
                TimePicker {
                    selected_time: selected_time_a(),
                    on_value_change: move |v| selected_time_a.set(v),
                    format: TimePickerFormat::TwelveHour,
                    presets: vec![time!(09:00), time!(12:00), time!(14:30), time!(17:00), time!(20:00)],
                }
            }
            div {
                p { style: "font-size: var(--text-xs); color: var(--muted-fg); margin-block-end: 0.5rem;",
                    "Generated range (every 1.5h)"
                }
                TimePicker {
                    selected_time: selected_time_b(),
                    on_value_change: move |v| selected_time_b.set(v),
                    presets: time_range(time!(06:00), time!(22:00), Duration::hours(1) + Duration::minutes(30)),
                }
            }
        }
    }
}
