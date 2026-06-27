use dioxus::prelude::*;
use dioxus_components::number_input::*;

#[component]
pub fn Demo() -> Element {
    let mut value = use_signal(|| Some(42.0_f64));

    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",

            NumberInput {
                label: "Quantity",
                description: "Use the steppers or arrow keys to adjust.",
                value: value(),
                on_change: move |v| value.set(v),
                min: 0.0,
                max: 100.0,
            }

            p { id: "number-value", "Value: {value().map(|v| v.to_string()).unwrap_or_default()}" }
        }
    }
}
