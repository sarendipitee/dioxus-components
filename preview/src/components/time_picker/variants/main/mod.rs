use dioxus::prelude::*;
use dioxus_components::time_picker::*;
use time::macros::time;

#[component]
pub fn Demo() -> Element {
    let mut selected_time = use_signal(|| Some(time!(14:30)));

    rsx! {
        TimePicker {
            selected_time: selected_time(),
            on_value_change: move |v| selected_time.set(v),
        }
    }
}
