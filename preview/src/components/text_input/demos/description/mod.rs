use dioxus::prelude::*;
use dioxus_components::input::*;
use dioxus_primitives::dioxus_attributes::attributes;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            TextInput {
                id: "text-input-required",
                label: rsx! { "Display name" },
                description: rsx! { "Shown across comments, mentions, and your public profile." },
                placeholder: "Enter a name",
                required: true,
                "data-testid": "text-input-required",
            }
            TextInput {
                id: "text-input-asterisk",
                label: rsx! { "Organization" },
                description: rsx! { "Optional field for workspaces and client directories." },
                with_asterisk: true,
                placeholder: "Acme Studio",
                "data-testid": "text-input-asterisk",
            }
            TextInput {
                id: "text-input-disabled",
                label: rsx! { "Disabled field" },
                disabled: true,
                "data-testid": "text-input-disabled",
            }
            TextInput {
                id: "text-input-read-only",
                label: rsx! { "Read-only field" },
                readonly: true,
                value: "Immutable value",
                "data-testid": "text-input-read-only",
            }
            TextInput {
                id: "text-input-loading",
                label: rsx! { "Loading field" },
                loading: true,
                input_attributes: attributes!(input {
                    "data-testid": "text-input-loading-shell",
                }),
                "data-testid": "text-input-loading",
            }
        }
    }
}
