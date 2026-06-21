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
                        "Event has been created".to_string(),
                        ToastOptions::new().description("Monday, January 3rd at 6:00pm"),
                    );
                },
                "Success"
            }
            Button {
                r#type: "button",
                variant: ButtonVariant::Destructive,
                onclick: move |_| {
                    toast.error(
                        "Event has not been created".to_string(),
                        ToastOptions::new()
                            .description("Your session expired. Please sign in and try again."),
                    );
                },
                "Error"
            }
            Button {
                r#type: "button",
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    toast.warning(
                        "Event start time cannot be earlier than 8am".to_string(),
                        ToastOptions::new().description("Set a start time of 8:00am or later."),
                    );
                },
                "Warning"
            }
            Button {
                r#type: "button",
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    toast.info(
                        "App update available".to_string(),
                        ToastOptions::new()
                            .description("Version 2.0 includes performance improvements and new features."),
                    );
                },
                "Info"
            }
        }
    }
}
