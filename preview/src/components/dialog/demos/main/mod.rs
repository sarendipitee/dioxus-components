use crate::components::dialog::*;
use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonVariant};

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        Button { onclick: move |_| open.set(true), "Open Dialog" }
        Dialog {
            open: open(),
            on_open_change: move |v| open.set(v),
            DialogContent {
                DialogClose { "×" }
                DialogHeader {
                    DialogTitle { "Item information" }
                    DialogDescription {
                        "Here is some additional information about this item. Review the details before proceeding."
                    }
                }
                DialogFooter {
                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| open.set(false),
                        "Cancel"
                    }
                    Button { "Continue" }
                }
            }
        }
    }
}
