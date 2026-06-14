use dioxus::prelude::*;
use dioxus_components::checkbox::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            Checkbox {
                name: "tos-check",
                label: "Accept terms and conditions",
                aria_label: "Accept terms and conditions",
            }
        }
    }
}
