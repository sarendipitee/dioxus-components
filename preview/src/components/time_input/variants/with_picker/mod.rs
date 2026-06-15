use dioxus::prelude::*;
use dioxus_components::time_input::*;
use dioxus_components::time_picker::TimePickerFormat;
use time::Time;

#[component]
pub fn Demo() -> Element {
    let mut t = use_signal(|| None::<Time>);

    rsx! {
        div { style: "display: grid; gap: 1rem; max-width: 24rem;",
            TimeInput {
                label: rsx! { "Appointment time" },
                description: rsx! { "Click the field to open the column picker." },
                format: TimePickerFormat::TwelveHour,
                selected_time: t(),
                on_value_change: move |value| t.set(value),
                clearable: true,
            }
        }
    }
}
