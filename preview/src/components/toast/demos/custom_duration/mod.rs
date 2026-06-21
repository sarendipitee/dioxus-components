use dioxus_components::button::{Button, ButtonVariant};
use dioxus_components::toast::*;
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};
use std::time::Duration;

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
                        "Quick note".to_string(),
                        ToastOptions::new()
                            .description("Disappears in 2 seconds.")
                            .duration(Duration::from_secs(2)),
                    );
                },
                "2 seconds"
            }
            Button {
                r#type: "button",
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    toast.info(
                        "Standard notice".to_string(),
                        ToastOptions::new().description("Disappears in 5 seconds (default)."),
                    );
                },
                "5 seconds (default)"
            }
            Button {
                r#type: "button",
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    toast.info(
                        "Extended notice".to_string(),
                        ToastOptions::new()
                            .description("Disappears in 30 seconds.")
                            .duration(Duration::from_secs(30)),
                    );
                },
                "30 seconds"
            }
        }
    }
}
