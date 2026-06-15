use crate::components::alert_dialog::*;
use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonVariant};

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    let mut navigated = use_signal(|| false);

    rsx! {
        Button {
            variant: ButtonVariant::Outline,
            onclick: move |_| { open.set(true); navigated.set(false); },
            "Leave page"
        }
        AlertDialog {
            open: open(),
            on_open_change: move |v| open.set(v),
            AlertDialogContent {
                AlertDialogTitle { "Unsaved changes" }
                AlertDialogDescription {
                    "You have unsaved changes that will be lost. Are you sure you want to leave this page?"
                }
                AlertDialogActions {
                    AlertDialogCancel { "Stay" }
                    AlertDialogAction {
                        on_click: move |_| navigated.set(true),
                        "Leave"
                    }
                }
            }
        }
        if navigated() {
            p {
                style: "margin-top: 1rem; color: var(--fg-muted);",
                "You left the page."
            }
        }
    }
}
