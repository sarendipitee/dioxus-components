use dioxus::prelude::*;
use dioxus_components::number_input::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",

            NumberInput {
                label: "Strict (1–10)",
                description: "Values outside the bounds are blocked as you type.",
                min: 1.0,
                max: 10.0,
                clamp_behavior: ClampBehavior::Strict,
                default_value: 5.0,
            }

            NumberInput {
                label: "Clamp on blur (0–100)",
                description: "Out-of-range values snap back when the field loses focus.",
                min: 0.0,
                max: 100.0,
                clamp_behavior: ClampBehavior::Blur,
                default_value: 50.0,
            }

            NumberInput {
                label: "Custom step",
                description: "Steppers and arrow keys move by 0.25.",
                step: 0.25,
                decimal_scale: 2,
                default_value: 1.0,
            }
        }
    }
}
