use dioxus::prelude::*;
use dioxus_components::input::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            TextInput {
                label: rsx! { "Display name" },
                description: rsx! { "Shown across comments, mentions, and your public profile." },
                placeholder: "Enter a name",
            }
            TextInput {
                label: rsx! { "Organization" },
                description: rsx! { "Optional field for workspaces and client directories." },
                with_asterisk: true,
                placeholder: "Acme Studio",
            }
        }
    }
}
