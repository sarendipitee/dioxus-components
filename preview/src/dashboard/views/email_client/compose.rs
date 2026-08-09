use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};

use crate::components::button::{Button, ButtonVariant};
use crate::components::dialog::Dialog;
use crate::components::input::TextInput;
use crate::components::textarea::{Textarea, TextareaVariant};
use crate::dashboard::common::{IconKind, LucideIcon};

use super::state::{EmailClientState, EmailClientStateStoreExt, EmailClientStateStoreImplExt};

#[component]
pub(super) fn ComposeModal(mut state: Store<EmailClientState>) -> Element {
    let toasts = use_toast();
    let open = state.compose_open().cloned();
    let to = state.compose_to().cloned();
    let subject = state.compose_subject().cloned();
    let body = state.compose_body().cloned();
    let recipient = to.clone();

    let send = move |evt: FormEvent| {
        evt.prevent_default();
        let description = if recipient.trim().is_empty() {
            "Your message is on its way.".to_string()
        } else {
            format!("Delivered to {}.", recipient.trim())
        };
        state.discard_compose();
        toasts.info(
            "Email sent".to_string(),
            ToastOptions::new().description(description),
        );
    };

    rsx! {
        Dialog {
            open: Some(open),
            on_open_change: move |v: bool| state.set_compose_open(v),
            class: "ec-compose-dialog",
            title: "New message",
            description: "Send a message to your team or contacts.",
            with_close_button: false,
            footer: rsx! {
                Button {
                    variant: ButtonVariant::Ghost,
                    r#type: "button",
                    onclick: move |_| state.discard_compose(),
                    "Discard"
                }
                Button { r#type: "submit", form: "ec-compose-form",
                    LucideIcon { kind: IconKind::Send, size: 16 }
                    "Send"
                }
            },
            form { id: "ec-compose-form", class: "ec-compose-form", onsubmit: send,
                TextInput {
                    id: "ec-compose-to",
                    r#type: "email",
                    required: true,
                    label: "To",
                    error: "Enter a valid email address.",
                    value: to.clone(),
                    placeholder: "name@company.com",
                    oninput: move |e: FormEvent| state.set_compose_to(e.value()),
                }

                TextInput {
                    id: "ec-compose-subject",
                    label: "Subject",
                    value: subject.clone(),
                    placeholder: "What is this about?",
                    oninput: move |e: FormEvent| state.set_compose_subject(e.value()),
                }

                Textarea {
                    id: "ec-compose-body",
                    label: "Message",
                    value: body.clone(),
                    placeholder: "Write your message…",
                    variant: TextareaVariant::Outline,
                    oninput: move |e: FormEvent| state.set_compose_body(e.value()),
                }
            }
        }
    }
}
