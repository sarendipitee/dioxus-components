use dioxus::prelude::*;
use dioxus_components::slider::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "display: grid; gap: 1rem;",
            Slider {
                id: "slider-field-control",
                label: "Field Slider",
                field_label: "Field Slider",
                description: "Choose the preferred alert threshold.",
                error: "A threshold is required.",
                required: true,
                with_asterisk: true,
                min: 0.0,
                max: 100.0,
                step: 1.0,
                default_value: 50.0,
            }
        }
    }
}
