use crate::components::alert_dialog::*;
use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonVariant};

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    let mut deleted = use_signal(|| false);

    rsx! {
        Button {
            variant: ButtonVariant::Destructive,
            onclick: move |_| { open.set(true); deleted.set(false); },
            "Delete Account"
        }
        AlertDialog {
            open: open(),
            on_open_change: move |v| open.set(v),
            AlertDialogContent {
                AlertDialogTitle { "Are you absolutely sure?" }
                AlertDialogDescription {
                    "This action cannot be undone. This will permanently delete your account and remove all of your data from our servers."
                }
                AlertDialogActions {
                    AlertDialogCancel { "Cancel" }
                    AlertDialogAction {
                        on_click: move |_| deleted.set(true),
                        "Yes, delete account"
                    }
                }
            }
        }
        if deleted() {
            p {
                style: "margin-top: 1rem; color: var(--danger-subtle-fg);",
                "Your account has been permanently deleted."
            }
        }
    }
}
