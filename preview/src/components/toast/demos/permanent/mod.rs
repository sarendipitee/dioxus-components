use dioxus_components::button::{Button, ButtonVariant};
use dioxus_components::toast::*;
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};

#[component]
pub fn Demo() -> Element {
    rsx! {
        ToastProvider { ToastButton {} }
    }
}

#[component]
fn ToastButton() -> Element {
    let toast = use_toast();

    rsx! {
        Button {
            r#type: "button",
            variant: ButtonVariant::Outline,
            onclick: move |_| {
                toast.warning(
                    "You are in offline mode".to_string(),
                    ToastOptions::new()
                        .description("Changes will sync when your connection is restored.")
                        .permanent(true),
                );
            },
            "Show permanent"
        }
    }
}
