use dioxus_components::checkbox::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Checkbox { name: "tos-check", aria_label: "Demo Checkbox" }
    }
}
