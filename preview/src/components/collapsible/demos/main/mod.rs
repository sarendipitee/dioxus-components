use dioxus::prelude::*;
use dioxus_components::collapsible::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Collapsible {
            id: "uncontrolled-collapsible",
            "data-testid": "collapsible-root",
            style: "width: 100%; max-width: 22rem;",
            CollapsibleTrigger {
                "data-testid": "collapsible-trigger",
                "3 repositories"
            }
            CollapsibleList {
                style: "max-width: none;",
                CollapsibleContent {
                    id: "repository-content",
                    "data-testid": "collapsible-content",
                    div { "dioxuslabs/components" }
                    div { "dioxuslabs/awesome-dioxus" }
                }
            }
        }
    }
}
