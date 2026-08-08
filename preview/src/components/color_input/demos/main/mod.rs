use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_components::color_input::*;
use dioxus_primitives::color_picker::Color;
use palette::{encoding, FromColor, Hsv, IntoColor, Srgb};

fn format_color(color: Hsv<encoding::Srgb, f64>) -> String {
    let rgb: Color = Srgb::<f64>::from_color(color).into_format();
    format!("#{rgb:X}")
}

#[component]
pub fn Demo() -> Element {
    let mut color = use_signal(|| -> Hsv<encoding::Srgb, f64> {
        Color::new(155, 128, 255).into_format::<f64>().into_color()
    });
    let disabled_color = use_signal(|| -> Hsv<encoding::Srgb, f64> {
        Color::new(128, 128, 128).into_format::<f64>().into_color()
    });
    let read_only_color = use_signal(|| -> Hsv<encoding::Srgb, f64> {
        Color::new(34, 139, 34).into_format::<f64>().into_color()
    });
    let mut change_count = use_signal(|| 0_u32);

    rsx! {
        form {
            "data-testid": "color-input-form",
            id: "color-input-form",
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            ColorInput {
                label: rsx! { "Accent color" },
                description: rsx! { "Shared input shell with ColorPicker in the popover." },
                color: color(),
                name: "accent",
                form: "color-input-form",
                input_attributes: attributes!(input {
                    "data-testid": "accent-color-input",
                    "data-color-field": "accent",
                }),
                on_color_change: move |value| {
                    color.set(value);
                    change_count += 1;
                },
            }
            output { "data-testid": "accent-value", "{format_color(color())}" }
            output { "data-testid": "accent-callback-count", "{change_count}" }
            ColorInput {
                label: rsx! { "Disabled color" },
                color: disabled_color(),
                disabled: true,
                name: "disabled-color",
                input_attributes: attributes!(input {
                    "data-testid": "color-input-disabled",
                }),
            }
            ColorInput {
                label: rsx! { "Read-only color" },
                color: read_only_color(),
                read_only: true,
                name: "readonly-color",
                input_attributes: attributes!(input {
                    "data-testid": "color-input-read-only",
                }),
            }
        }
    }
}
