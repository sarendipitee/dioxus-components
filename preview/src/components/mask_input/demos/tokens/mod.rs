use dioxus::prelude::*;
use dioxus_components::mask_input::*;

/// Custom token: an uppercase letter slot, keyed to `c`.
fn upper(c: char) -> bool {
    c.is_ascii_uppercase()
}

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            MaskInput {
                label: "Credit card",
                mask: "9999 9999 9999 9999",
                placeholder: "1234 5678 9012 3456",
            }
            MaskInput {
                label: "Expiry",
                mask: "99 / 99",
                placeholder: "MM / YY",
            }
            MaskInput {
                label: "License plate",
                description: "Custom `c` token + transform: type lowercase, get uppercase.",
                mask: "cc-999",
                tokens: vec![('c', upper as CharPredicate)],
                transform: move |c: char| c.to_ascii_uppercase(),
                placeholder: "AB-123",
            }
        }
    }
}
