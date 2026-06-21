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
                    toast.info(
                        "File deleted".to_string(),
                        ToastOptions::new()
                            .permanent(true)
                            .action("Undo", move |_| {}),
                    );
                },
                "With action"
            }
            Button {
                r#type: "button",
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    toast.warning(
                        "Unsaved changes".to_string(),
                        ToastOptions::new()
                            .description("Your edits will be lost.")
                            .permanent(true)
                            .action("Save now", move |_| {})
                            .cancel("Discard", move |_| {}),
                    );
                },
                "With action + cancel"
            }
        }
    }
}
