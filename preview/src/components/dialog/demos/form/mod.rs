use crate::components::dialog::*;
use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonVariant};
use dioxus_components::input::TextInput;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        Button { onclick: move |_| open.set(true), "Edit Profile" }
        Dialog {
            open: open(),
            on_open_change: move |v| open.set(v),
            DialogContent {
                DialogClose { "×" }
                DialogHeader {
                    DialogTitle { "Edit profile" }
                    DialogDescription {
                        "Make changes to your profile here. Click save when you're done."
                    }
                }
                DialogBody {
                    TextInput { id: "dialog-name", label: "Name", value: "Pedro Duarte" }
                    TextInput { id: "dialog-username", label: "Username", value: "@peduarte" }
                }
                DialogFooter {
                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| open.set(false),
                        "Cancel"
                    }
                    Button { "Save changes" }
                }
            }
        }
    }
}
