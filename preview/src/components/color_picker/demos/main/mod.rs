use dioxus::prelude::*;

use dioxus_components::color_picker::*;
use dioxus_primitives::color_picker::Color;
use palette::{encoding, Hsv, IntoColor};

#[component]
pub fn Demo() -> Element {
    let mut color = use_signal(|| -> Hsv<encoding::Srgb, f64> {
        Color::new(155, 128, 255).into_format::<f64>().into_color()
    });
    let mut disabled = use_signal(|| false);

    rsx! {
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
        ColorPicker {
            color: color(),
            disabled: disabled(),
            on_color_change: move |c| {
                tracing::info!("Color changed: {:?}", c);
                color.set(c);
            },
        }
    }
}
