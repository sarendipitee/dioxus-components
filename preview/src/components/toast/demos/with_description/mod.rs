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
                onclick: move |_| {
                    toast.success(
                        "Profile updated".to_string(),
                        ToastOptions::new().description("Your changes have been saved."),
                    );
                },
                "Success"
            }
            Button {
                r#type: "button",
                variant: ButtonVariant::Destructive,
                onclick: move |_| {
                    toast.error(
                        "Payment failed".to_string(),
                        ToastOptions::new()
                            .description("Your card was declined. Try a different payment method."),
                    );
                },
                "Error"
            }
            Button {
                r#type: "button",
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    toast.warning(
                        "Session expiring".to_string(),
                        ToastOptions::new().description("You'll be signed out in 5 minutes."),
                    );
                },
                "Warning"
            }
            Button {
                r#type: "button",
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    toast.info(
                        "New features available".to_string(),
                        ToastOptions::new()
                            .description("Dark mode and keyboard shortcuts are now supported."),
                    );
                },
                "Info"
            }
        }
    }
}
