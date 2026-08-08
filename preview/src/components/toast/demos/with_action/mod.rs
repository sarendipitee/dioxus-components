use dioxus_components::button::{Button, ButtonVariant};
use dioxus_components::toast::*;
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions, ToastPosition};

#[component]
pub fn Demo() -> Element {
    let status = use_signal(|| "No action selected".to_string());

    rsx! {
        ToastProvider {
            id: "toast-audit-region",
            "data-testid": "toast-audit-region",
            max_toasts: 2usize,
            position: ToastPosition::TopLeft,
            output {
                "data-testid": "toast-action-result",
                aria_live: "polite",
                "{status}"
            }
            ToastButtons { status }
        }
    }
}

#[component]
fn ToastButtons(mut status: Signal<String>) -> Element {
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
                    let undo_status = status;
                    toast.success(
                        "Event has been created",
                        ToastOptions::new()
                            .description("Monday, January 3rd at 6:00pm")
                            .permanent(true)
                            .action("Undo", move |_| {
                                let mut status = undo_status;
                                status.set("Undo selected".to_string());
                            }),
                    );
                },
                "Action"
            }
            Button {
                r#type: "button",
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    let save_status = status;
                    let discard_status = status;
                    toast.warning(
                        "You have unsaved changes",
                        ToastOptions::new()
                            .description("Navigating away will discard your edits.")
                            .permanent(true)
                            .action("Save now", move |_| {
                                let mut status = save_status;
                                status.set("Save selected".to_string());
                            })
                            .cancel("Discard", move |_| {
                                let mut status = discard_status;
                                status.set("Discard selected".to_string());
                            }),
                    );
                },
                "Action + cancel"
            }
        }
    }
}
