use dioxus::prelude::*;
use dioxus_components::checkbox::*;
use dioxus_primitives::checkbox::CheckboxState;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            Checkbox {
                name: "archived-projects",
                label: "Include archived projects",
                description: "This option is disabled for read-only workspaces.",
                disabled: true,
            }
            Checkbox {
                name: "locked-policy",
                label: "Enforce organization policy",
                description: "Managed by your organization administrator.",
                default_checked: CheckboxState::Checked,
                disabled: true,
            }
        }
    }
}
