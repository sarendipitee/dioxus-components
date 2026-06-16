use dioxus::prelude::*;
use dioxus_components::time_picker::*;
use time::Time;

#[component]
pub fn Demo() -> Element {
    let mut selected_time = use_signal(|| None::<Time>);

    rsx! {
        div { style: "display: grid; gap: 0.75rem;",
            TimePicker {
                selected_time: selected_time(),
                on_value_change: move |v| selected_time.set(v),
            }
            if selected_time().is_some() {
                button {
                    onclick: move |_| selected_time.set(None),
                    "Clear"
                }
            }
        }
    }
}
