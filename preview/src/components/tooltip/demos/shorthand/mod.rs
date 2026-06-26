use dioxus_components::tooltip::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Tooltip {
            content: "This tooltip uses the shorthand content prop. Children become the trigger.",
            "Hover me"
        }
    }
}
