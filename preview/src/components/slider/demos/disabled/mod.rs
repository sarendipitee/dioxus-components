use dioxus::prelude::*;
use dioxus_components::slider::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "display: grid; gap: 1rem;",
            Slider {
                label: "Disabled Slider",
                disabled: true,
                min: 0.0,
                max: 100.0,
                step: 1.0,
                default_value: 35.0,
            }
        }
    }
}
