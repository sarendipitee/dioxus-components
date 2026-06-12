use dioxus::prelude::*;
use dioxus_components::input::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            TextInput {
                label: rsx! { "Small" },
                size: InputSize::Sm,
                placeholder: "Compact field",
            }
            TextInput {
                label: rsx! { "Medium" },
                size: InputSize::Md,
                placeholder: "Default field",
            }
            TextInput {
                label: rsx! { "Large" },
                size: InputSize::Lg,
                placeholder: "Large field",
            }
        }
    }
}
