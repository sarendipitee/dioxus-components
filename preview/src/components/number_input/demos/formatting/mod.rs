use dioxus::prelude::*;
use dioxus_components::number_input::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",

            NumberInput {
                label: "Price",
                description: "Fixed to two decimals with a currency prefix.",
                prefix: "$",
                decimal_scale: 2,
                step: 0.01,
                default_value: 9.99,
            }

            NumberInput {
                label: "Population",
                description: "Thousands grouping is applied to the display value.",
                thousands_separator: ",",
                step: 1000.0,
                default_value: 1_250_000.0,
            }

            NumberInput {
                label: "Completion",
                description: "Suffix renders after the value.",
                suffix: "%",
                min: 0.0,
                max: 100.0,
                default_value: 75.0,
            }
        }
    }
}
