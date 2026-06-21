use dioxus_components::button::Button;
use dioxus_components::toast::*;
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};

#[component]
pub fn Demo() -> Element {
    rsx! {
        ToastProvider { ToastButton {} }
    }
}

#[component]
fn ToastButton() -> Element {
    let toast = use_toast();

    rsx! {
        Button {
            r#type: "button",
            onclick: move |_| {
                toast.warning(
                    "Action required".to_string(),
                    ToastOptions::new()
                        .description("This notification stays until you dismiss it.")
                        .permanent(true),
                );
            },
            "Show permanent toast"
        }
    }
}
