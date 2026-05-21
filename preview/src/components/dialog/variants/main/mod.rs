use dioxus_components::button::Button;

use dioxus_components::dialog::{Dialog, DialogClose, DialogDescription, DialogTitle};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        Button {
            r#type: "button",
            "data-style": "outline",
            onclick: move |_| open.set(true),
            "Show Dialog"
        }
        Dialog { open: open(), on_open_change: move |v| open.set(v),
            DialogClose {
                aria_label: "Close",
                tabindex: if open() { "0" } else { "-1" },
                onclick: move |_| open.set(false),
                "×"
            }
            DialogTitle { "Item information" }
            DialogDescription { "Here is some additional information about the item." }
        }
    }
}
