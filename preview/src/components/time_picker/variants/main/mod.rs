use dioxus::prelude::*;
use dioxus_components::time_picker::*;
use time::macros::time;

#[component]
pub fn Demo() -> Element {
    let mut selected_time = use_signal(|| Some(time!(14:30)));

    rsx! {
        div { style: "display: flex; gap: 1.5rem; flex-wrap: wrap;",
            div {
                p { style: "font-size: 0.75rem; color: var(--muted-fg); margin-block-end: 0.5rem;",
                    "24-hour"
                }
                TimePicker {
                    selected_time: selected_time(),
                    on_value_change: move |v| selected_time.set(v),
                }
            }
            div {
                p { style: "font-size: 0.75rem; color: var(--muted-fg); margin-block-end: 0.5rem;",
                    "12-hour"
                }
                TimePicker {
                    selected_time: selected_time(),
                    on_value_change: move |v| selected_time.set(v),
                    format: TimePickerFormat::TwelveHour,
                }
            }
        }
    }
}
