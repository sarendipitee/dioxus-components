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
                    toast.promise(
                        async {
                            gloo_timers::future::sleep(Duration::from_secs(2)).await;
                            Ok::<_, String>(())
                        },
                        "Saving your changes\u{2026}",
                        "Changes saved",
                        "Failed to save",
                        ToastOptions::new(),
                    );
                },
                "Save (success)"
            }
            Button {
                r#type: "button",
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    toast.promise(
                        async {
                            gloo_timers::future::sleep(Duration::from_secs(2)).await;
                            Err::<(), _>("Request timed out".to_string())
                        },
                        "Publishing\u{2026}",
                        "Published",
                        "Publish failed",
                        ToastOptions::new(),
                    );
                },
                "Publish (error)"
            }
        }
    }
}
