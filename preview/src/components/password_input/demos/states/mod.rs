use dioxus::prelude::*;
use dioxus_components::password_input::*;

#[component]
pub fn Demo() -> Element {
    let mut callback_password = use_signal(String::new);

    rsx! {
        form {
            id: "password-input-form",
            "data-testid": "password-input-form",
            style: "display: grid; gap: 1rem; max-width: 24rem;",

            PasswordInput {
                id: "required-password",
                label: "Required",
                required: true,
                with_asterisk: true,
                placeholder: "Enter your password",
            }

            PasswordInput {
                id: "error-password",
                label: "With error",
                error: "Password is too short.",
                default_visible: true,
                placeholder: "Enter your password",
            }

            PasswordInput {
                label: "Loading",
                loading: true,
                placeholder: "Checking…",
            }

            PasswordInput {
                label: "Disabled",
                disabled: true,
                name: "disabled_password",
                value: "hunter2",
            }

            PasswordInput {
                label: "Read-only password",
                value: "immutable",
                name: "readonly_password",
                readonly: true,
            }

            PasswordInput {
                id: "callback-password",
                label: "Callback password",
                description: "Updates are reported below.",
                name: "account_password",
                form: "password-input-form",
                placeholder: "Type a password",
                "data-password-field": "callback",
                show_label: "Reveal account password",
                hide_label: "Conceal account password",
                value: callback_password,
                oninput: move |event: FormEvent| callback_password.set(event.value()),
            }

            output { "data-testid": "password-value", "{callback_password}" }
        }
    }
}
