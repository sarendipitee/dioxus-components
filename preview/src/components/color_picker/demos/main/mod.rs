use dioxus::prelude::*;

use dioxus_components::color_picker::*;
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
    let mut disabled = use_signal(|| false);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 1rem; align-items: flex-start;",
            div {
                style: "display: flex; gap: 0.5rem; align-items: center;",
                button {
                    onclick: move |_| {
                        color.set(Color::new(255, 0, 0).into_format::<f64>().into_color());
                    },
                    "Set color to red"
                }
                button {
                    onclick: move |_| disabled.toggle(),
                    if disabled() {
                        "Enable color picker"
                    } else {
                        "Disable color picker"
                    }
                }
            }
            div {
                style: "display: flex; flex-direction: column; gap: 0.25rem;",
                span {
                    style: "font-size: var(--text-sm); font-weight: 500;",
                    "Selected Color"
                }
                div {
                    style: "display: flex; gap: 0.75rem; align-items: center;",
                    ColorSwatch {
                        color: color(),
                    }
                    span {
                        style: "font-size: var(--text-sm); font-family: monospace;",
                        "{format_color(color())}"
                    }
                }
            }
            ColorPicker {
                color: color(),
                disabled: disabled(),
                on_color_change: move |c| color.set(c),
            }
        }
    }
}
