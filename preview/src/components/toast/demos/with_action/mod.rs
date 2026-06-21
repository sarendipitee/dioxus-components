use dioxus_components::button::{Button, ButtonVariant};
use dioxus_components::toast::*;
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};

#[component]
pub fn Demo() -> Element {
    rsx! {
        ToastProvider { ToastButtons {} }
    }
}

#[component]
fn ToastButtons() -> Element {
    let toast = use_toast();

    rsx! {
        div {
            display: "flex",
            flex_wrap: "wrap",
            gap: "0.5rem",
            Button {
                r#type: "button",
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    toast.success(
                        "Event has been created".to_string(),
                        ToastOptions::new()
                            .description("Monday, January 3rd at 6:00pm")
                            .permanent(true)
                            .action("Undo", move |_| {}),
                    );
                },
                "Action"
            }
            Button {
                r#type: "button",
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    toast.warning(
                        "You have unsaved changes".to_string(),
                        ToastOptions::new()
                            .description("Navigating away will discard your edits.")
                            .permanent(true)
                            .action("Save now", move |_| {})
                            .cancel("Discard", move |_| {}),
                    );
                },
                "Action + cancel"
            }
        }
    }
}
