use dioxus::prelude::*;
use dioxus_components::tooltip::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Tooltip {
            content: "This tooltip uses the shorthand content prop. Children become the trigger.",
            "Hover me"
        }
    }
}
