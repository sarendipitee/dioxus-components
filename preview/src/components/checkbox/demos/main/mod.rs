use dioxus::prelude::*;
use dioxus_components::checkbox::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        form {
            "data-testid": "checkbox-form",
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            Checkbox {
                name: "tos-check",
                value: "accepted",
                label: "Accept terms and conditions",
                aria_label: "Accept terms and conditions",
            }
            Checkbox {
                name: "managed-setting",
                value: "locked",
                label: "Managed setting",
                default_checked: dioxus_primitives::checkbox::CheckboxState::Checked,
                read_only: true,
            }
        }
    }
}
