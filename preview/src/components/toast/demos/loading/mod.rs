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
                onclick: move |_| {
                    toast.promise(
                        async {
                            gloo_timers::future::sleep(Duration::from_secs(2)).await;
                            Ok::<_, String>(())
                        },
                        "Uploading file\u{2026}",
                        "Upload complete",
                        "Upload failed",
                        ToastOptions::new(),
                    );
                },
                "Promise (success)"
            }
            Button {
                r#type: "button",
                variant: ButtonVariant::Destructive,
                onclick: move |_| {
                    toast.promise(
                        async {
                            gloo_timers::future::sleep(Duration::from_secs(2)).await;
                            Err::<(), _>("Network error".to_string())
                        },
                        "Connecting\u{2026}",
                        "Connected",
                        "Connection failed",
                        ToastOptions::new(),
                    );
                },
                "Promise (error)"
            }
        }
    }
}
