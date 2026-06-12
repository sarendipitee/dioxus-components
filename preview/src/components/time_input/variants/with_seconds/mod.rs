use dioxus::prelude::*;
use dioxus_components::time_input::*;
use dioxus_components::time_picker::TimePickerFormat;
use time::macros::time;

#[component]
pub fn Demo() -> Element {
    let mut t = use_signal(|| Some(time!(09:30:15)));

    rsx! {
        div { style: "display: grid; gap: 1rem; max-width: 24rem;",
            TimeInput {
                label: rsx! { "Precise time" },
                description: rsx! { "Shows hour, minute, and second columns." },
                with_seconds: true,
                selected_time: t(),
                on_value_change: move |v| t.set(v),
            }
            TimeInput {
                label: rsx! { "Precise time (12h)" },
                with_seconds: true,
                format: TimePickerFormat::TwelveHour,
                selected_time: t(),
                on_value_change: move |v| t.set(v),
            }
        }
    }
}
