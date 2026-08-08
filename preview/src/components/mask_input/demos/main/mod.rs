use dioxus::prelude::*;
use dioxus_components::mask_input::*;

#[component]
pub fn Demo() -> Element {
    let mut raw = use_signal(String::new);
    let mut masked = use_signal(String::new);
    let mut complete = use_signal(|| false);
    let mut mask_handle = use_signal(|| None::<UseMask>);

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
            MaskInput {
                id: "mask-ref-controlled",
                label: "Mask ref controlled value",
                mask: "9999",
                default_value: "1234",
                mask_ref: move |mask| mask_handle.set(Some(mask)),
            }
            button {
                id: "mask-ref-reset",
                r#type: "button",
                onclick: move |_| {
                    if let Some(mask) = mask_handle() {
                        mask.reset.call(());
                    }
                },
                "Reset mask ref value"
            }
            form { id: "mask-fixture-form" }
            p { id: "mask-extra-description", "External account-code guidance." }
            MaskInput {
                id: "mask-field-id",
                label: "Account code",
                description: "Enter the required account code.",
                error: "Account code is invalid.",
                required: true,
                mask: "9999",
                name: "account-code",
                form: "mask-fixture-form",
                autocomplete: "one-time-code",
                described_by: "mask-extra-description",
            }
            MaskInput {
                id: "mask-disabled",
                label: "Disabled mask input",
                mask: "9999",
                disabled: true,
                default_value: "1234",
            }
            MaskInput {
                id: "mask-readonly",
                label: "Read-only mask input",
                mask: "9999",
                default_value: "1234",
                readonly: true,
            }
        }
    }
}
