use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonVariant};
use dioxus_components::collapsible::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| Some(false));
    let state_label = if open().unwrap_or(false) {
        "open"
    } else {
        "closed"
    };

    rsx! {
        div { style: "display: grid; gap: 0.75rem; width: 100%; max-width: 22rem;",
            Button {
                variant: ButtonVariant::Outline,
                style: "width: fit-content;",
                onclick: move |_| open.set(Some(!open().unwrap_or(false))),
                if open().unwrap_or(false) {
                    "Hide release notes"
                } else {
                    "Show release notes"
                }
            }
            output { "data-testid": "controlled-state", "Release notes are {state_label}." }
            Collapsible {
                id: "controlled-collapsible",
                open,
                on_open_change: move |next_open| open.set(Some(next_open)),
                CollapsibleTrigger { "Version 0.3.1" }
                CollapsibleContent {
                    "Improved focus handling and disclosure animations."
                }
            }
        }
    }
}
