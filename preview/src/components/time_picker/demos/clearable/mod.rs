use dioxus::prelude::*;
use dioxus_components::time_picker::*;
use time::Time;

fn state_value(value: Option<Time>) -> String {
    value
        .map(|value| {
            format!(
                "{:02}:{:02}:{:02}",
                value.hour(),
                value.minute(),
                value.second()
            )
        })
        .unwrap_or_else(|| "none".to_string())
}

#[component]
pub fn Demo() -> Element {
    let mut selected_time = use_signal(|| Some(Time::MIDNIGHT));

    rsx! {
        div { style: "display: grid; gap: 0.75rem;",
            TimePicker {
                selected_time: selected_time(),
                on_value_change: move |value| selected_time.set(value),
                clearable: true,
                "data-testid": "time-picker-clearable",
            }
            output { "data-testid": "time-picker-clearable-state", "{state_value(selected_time())}" }
        }
    }
}
