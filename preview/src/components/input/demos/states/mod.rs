use dioxus::prelude::*;
use dioxus_components::input::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            InputBase {
                label: rsx! { "Workspace name" },
                description: rsx! { "Shown in the sidebar and on billing receipts." },
                with_asterisk: true,
                input {
                    style: "width: 100%; border: 0; background: transparent; outline: none;",
                    placeholder: "Acme Studio",
                }
            }
            InputBase {
                label: rsx! { "Subdomain" },
                error: rsx! { "That subdomain is already taken." },
                input {
                    style: "width: 100%; border: 0; background: transparent; outline: none;",
                    placeholder: "acme",
                }
            }
            InputBase {
                label: rsx! { "Account id" },
                description: rsx! { "Managed by your administrator." },
                disabled: true,
                input {
                    style: "width: 100%; border: 0; background: transparent; outline: none;",
                    disabled: true,
                    value: "acct_01H8X9",
                }
            }
        }
    }
}
