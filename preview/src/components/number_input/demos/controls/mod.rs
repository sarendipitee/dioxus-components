use dioxus::prelude::*;
use dioxus_components::number_input::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",

            NumberInput {
                label: "No steppers",
                description: "Hide the increment/decrement buttons with `hide_controls`.",
                hide_controls: true,
                default_value: 20.0,
            }

            NumberInput {
                label: "Integers only",
                description: "`allow_decimal: false` rejects the decimal separator.",
                allow_decimal: false,
                default_value: 7.0,
            }

            NumberInput {
                label: "Non-negative",
                description: "`allow_negative: false` strips the minus sign.",
                allow_negative: false,
                min: 0.0,
                default_value: 3.0,
            }
        }
    }
}
