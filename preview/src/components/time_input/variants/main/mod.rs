use dioxus::prelude::*;
use dioxus_components::time_input::*;
use dioxus_components::time_picker::TimePickerFormat;
use time::{macros::time, Time};

#[component]
pub fn Demo() -> Element {
    let mut t24 = use_signal(|| Some(time!(14:45)));
    let mut t12 = use_signal(|| None::<Time>);

    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            TimeInput {
                label: rsx! { "Start time (24h)" },
                description: rsx! { "Opens a column picker when focused." },
                selected_time: t24(),
                on_value_change: move |value| t24.set(value),
            }
            TimeInput {
                label: rsx! { "Start time (12h)" },
                format: TimePickerFormat::TwelveHour,
                selected_time: t12(),
                on_value_change: move |value| t12.set(value),
                clearable: true,
            }
        }
    }
}
