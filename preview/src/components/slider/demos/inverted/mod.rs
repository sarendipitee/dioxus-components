use dioxus::prelude::*;
use dioxus_components::slider::*;

#[component]
pub fn Demo() -> Element {
    let mut inverted_value = use_signal(|| Some(70.0));

    rsx! {
        div {
            style: "display: grid; gap: 1rem;",
            output {
                aria_live: "polite",
                "{inverted_value().unwrap_or_default():.0}"
            }
            Slider {
                label: "Inverted Slider",
                horizontal: true,
                inverted: true,
                min: 0.0,
                max: 100.0,
                step: 1.0,
                value: inverted_value,
                on_value_change: move |value: f64| inverted_value.set(Some(value)),
            }
        }
    }
}
