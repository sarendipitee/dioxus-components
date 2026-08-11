use dioxus::prelude::*;
use dioxus_components::collapsible::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Collapsible {
            id: "disabled-collapsible",
            disabled: true,
            style: "width: 100%; max-width: 22rem;",
            CollapsibleTrigger { "Archived project" }
            CollapsibleContent {
                "Archived project settings are unavailable."
            }
        }
    }
}
