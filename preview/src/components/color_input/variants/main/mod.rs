use dioxus::prelude::*;
use dioxus_components::color_input::*;
use dioxus_primitives::color_picker::Color;
use palette::{encoding, Hsv, IntoColor};

#[component]
pub fn Demo() -> Element {
    let mut color = use_signal(|| -> Hsv<encoding::Srgb, f64> {
        Color::new(155, 128, 255).into_format::<f64>().into_color()
    });

    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            ColorInput {
                label: rsx! { "Accent color" },
                description: rsx! { "Shared input shell with ColorPicker in the popover." },
                color: color(),
                on_color_change: move |value| color.set(value),
            }
        }
    }
}
