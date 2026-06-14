use dioxus::prelude::*;
use dioxus_components::checkbox::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            Checkbox {
                name: "notifications",
                label: "Email notifications",
                description: "Receive product updates, billing notices, and security alerts.",
            }
            Checkbox {
                name: "workspace-sync",
                label: "Workspace sync",
                description: "Keep settings and preferences synchronized across devices.",
                error: "Sync requires an active workspace subscription.",
            }
        }
    }
}
