use dioxus::prelude::*;
use dioxus_components::collapsible::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Collapsible {
            id: "default-open-collapsible",
            default_open: true,
            style: "width: 100%; max-width: 22rem;",
            CollapsibleTrigger { "What is included?" }
            CollapsibleContent {
                CollapsibleItem {
                    "Accessible keyboard behavior, controlled and uncontrolled state, and reusable styling."
                }
            }
        }
    }
}
