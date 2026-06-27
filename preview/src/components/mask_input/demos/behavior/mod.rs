use dioxus::prelude::*;
use dioxus_components::mask_input::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            MaskInput {
                label: "Always show mask",
                mask: "9999-9999",
                always_show_mask: true,
            }
            MaskInput {
                label: "Custom slot character",
                mask: "99/99/9999",
                slot_char: "·",
                always_show_mask: true,
            }
            MaskInput {
                label: "Auto clear when incomplete",
                description: "Blur while incomplete to clear the field.",
                mask: "(999) 999-9999",
                auto_clear: true,
                placeholder: "(___) ___-____",
            }
        }
    }
}
