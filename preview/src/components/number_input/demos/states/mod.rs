use dioxus::prelude::*;
use dioxus_components::number_input::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",

            NumberInput {
                label: "Seats",
                description: "Required field with the asterisk shown.",
                required: true,
                with_asterisk: true,
                min: 1.0,
                default_value: 1.0,
            }

            NumberInput {
                label: "Quantity",
                error: "Must be at least 1.",
                default_value: 0.0,
            }

            NumberInput {
                label: "Syncing balance",
                description: "Loading marks the field busy.",
                loading: true,
                prefix: "$",
                decimal_scale: 2,
                default_value: 128.5,
            }

            NumberInput {
                label: "Locked",
                disabled: true,
                default_value: 42.0,
            }
        }
    }
}
