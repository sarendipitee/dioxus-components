use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonSize, ButtonVariant};
use dioxus_components::collapsible::*;
use dioxus_icons::lucide::{Ellipsis, Plus};

#[component]
pub fn Demo() -> Element {
    rsx! {
        Collapsible {
            id: "inline-actions-collapsible",
            style: "width: 100%; max-width: 22rem;",
            CollapsibleTrigger {
                variant: CollapsibleTriggerVariant::InlineActions,
                "data-testid": "inline-actions-trigger",
                actions: rsx! {
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::IconSm,
                        aria_label: "More recent actions",
                        Ellipsis { size: "1rem", "aria-hidden": "true" }
                    }
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::IconSm,
                        aria_label: "Add recent item",
                        Plus { size: "1rem", "aria-hidden": "true" }
                    }
                },
                "Recents"
            }
            CollapsibleContent {
                id: "inline-actions-content",
                "data-testid": "inline-actions-content",
                CollapsibleList {
                    CollapsibleItem { "Design system notes" }
                    CollapsibleItem { "Quarterly planning" }
                    CollapsibleItem { "Release checklist" }
                }
            }
        }
    }
}
