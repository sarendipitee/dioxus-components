use dioxus::prelude::*;
use dioxus_components::slider::*;

#[component]
pub fn Demo() -> Element {
    let mut vertical_value = use_signal(|| Some(30.0));
    let mut inverted_value = use_signal(|| Some(70.0));

    rsx! {
        div {
            style: "display: grid; gap: 2rem;",

            section {
                "data-testid": "vertical-slider-fixture",
                h3 { "Vertical Slider" }
                output {
                    "data-testid": "vertical-slider-value",
                    aria_live: "polite",
                    "{vertical_value().unwrap_or_default():.0}"
                }
                Slider {
                    label: "Vertical Slider",
                    horizontal: false,
                    min: 0.0,
                    max: 100.0,
                    step: 1.0,
                    value: vertical_value,
                    style: "height: 12rem;",
                    on_value_change: move |value: f64| vertical_value.set(Some(value)),
                }
            }

            section {
                "data-testid": "inverted-slider-fixture",
                h3 { "Inverted Slider" }
                output {
                    "data-testid": "inverted-slider-value",
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

            section {
                "data-testid": "disabled-slider-fixture",
                h3 { "Disabled Slider" }
                output { "data-testid": "disabled-slider-value", "35" }
                Slider {
                    label: "Disabled Slider",
                    disabled: true,
                    min: 0.0,
                    max: 100.0,
                    step: 1.0,
                    default_value: 35.0,
                }
            }

            section {
                "data-testid": "field-slider-fixture",
                h3 { "Field Slider" }
                Slider {
                    id: "slider-field-control",
                    class: "slider-audit",
                    title: "Slider audit field",
                    "data-audit": "slider",
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
}
