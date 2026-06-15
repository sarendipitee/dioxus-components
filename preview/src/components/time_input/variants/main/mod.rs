use dioxus::prelude::*;
use dioxus_components::time_input::*;
use time::macros::time;

#[component]
pub fn Demo() -> Element {
    let mut t = use_signal(|| Some(time!(14:45)));

    rsx! {
        div { style: "display: grid; gap: 1rem; max-width: 24rem;",
            TimeInput {
                label: rsx! { "Start time" },
                description: rsx! { "Opens a column picker when focused." },
                selected_time: t(),
                on_value_change: move |value| t.set(value),
            }
        }
    }
}
