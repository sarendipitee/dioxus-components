use dioxus::prelude::*;
use dioxus_components::time_picker::*;

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
            }
        }
    }
}
