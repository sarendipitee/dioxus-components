use dioxus_components::collapsible::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut controlled_open = use_signal(|| Some(false));

    rsx! {
        Collapsible {
            id: "uncontrolled-collapsible",
            "data-testid": "collapsible-root",
            CollapsibleTrigger {
                "data-testid": "collapsible-trigger",
                b { "Recent Activity" }
            }
            CollapsibleList {
                CollapsibleItem { "Added a new feature to the collapsible component" }
                CollapsibleContent {
                    id: "recent-activity-content",
                    "data-testid": "collapsible-content",
                    CollapsibleItem { "Fixed a bug in the collapsible component" }
                    CollapsibleItem { "Updated the documentation for the collapsible component" }
                }
            }
        }

        Collapsible {
            id: "default-open-collapsible",
            default_open: true,
            CollapsibleTrigger { "Default open details" }
            CollapsibleContent { "This content starts open." }
        }

        Collapsible {
            id: "controlled-collapsible",
            open: controlled_open,
            on_open_change: move |open| controlled_open.set(Some(open)),
            CollapsibleTrigger { "Controlled details" }
            CollapsibleContent { "This content is controlled." }
        }
        button {
            onclick: move |_| controlled_open.set(Some(true)),
            "Set controlled open"
        }
        p {
            if controlled_open().unwrap_or(false) {
                "Controlled state: open"
            } else {
                "Controlled state: closed"
            }
        }

        Collapsible {
            id: "disabled-collapsible",
            disabled: true,
            CollapsibleTrigger { "Disabled details" }
            CollapsibleContent { "Disabled content should stay hidden." }
        }
    }
}
