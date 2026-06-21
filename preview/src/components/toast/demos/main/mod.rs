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
                    toast.success("Event has been created".to_string(), ToastOptions::new());
                },
                "Success"
            }
            Button {
                r#type: "button",
                variant: ButtonVariant::Destructive,
                onclick: move |_| {
                    toast.error("Event has not been created".to_string(), ToastOptions::new());
                },
                "Error"
            }
            Button {
                r#type: "button",
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    toast.warning(
                        "Event start time cannot be earlier than 8am".to_string(),
                        ToastOptions::new(),
                    );
                },
                "Warning"
            }
            Button {
                r#type: "button",
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    toast.info(
                        "Be at the area 10 minutes before the event time".to_string(),
                        ToastOptions::new(),
                    );
                },
                "Info"
            }
        }
    }
}
