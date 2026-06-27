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
                    toast.success(
                        "Copied to clipboard",
                        ToastOptions::new().duration(Duration::from_secs(2)),
                    );
                },
                "2s — quick confirm"
            }
            Button {
                r#type: "button",
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    toast.info(
                        "Your report is being generated",
                        ToastOptions::new()
                            .description("This may take a moment.")
                            .duration(Duration::from_secs(5)),
                    );
                },
                "5s — default"
            }
            Button {
                r#type: "button",
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    toast.warning(
                        "Scheduled maintenance tonight",
                        ToastOptions::new()
                            .description("The service will be unavailable from 2–4am UTC.")
                            .duration(Duration::from_secs(30)),
                    );
                },
                "30s — important notice"
            }
        }
    }
}
