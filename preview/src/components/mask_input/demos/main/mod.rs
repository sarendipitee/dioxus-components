use dioxus::prelude::*;
use dioxus_components::mask_input::*;

#[component]
pub fn Demo() -> Element {
    let mut raw = use_signal(String::new);
    let mut masked = use_signal(String::new);
    let mut complete = use_signal(|| false);

    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            MaskInput {
                label: "Phone number",
                description: "Type digits — formatting is applied automatically.",
                mask: "(999) 999-9999",
                placeholder: "(___) ___-____",
                on_change_raw: move |(r, m): (String, String)| {
                    raw.set(r);
                    masked.set(m);
                    complete.set(false);
                },
                on_complete: move |_: (String, String)| complete.set(true),
            }
            p { id: "mask-raw", "Raw: {raw}" }
            p { id: "mask-masked", "Masked: {masked}" }
            p { id: "mask-complete", "Complete: {complete}" }
        }
    }
}
