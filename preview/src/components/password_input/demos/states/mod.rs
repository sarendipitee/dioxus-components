use dioxus::prelude::*;
use dioxus_components::password_input::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",

            PasswordInput {
                label: "Required",
                with_asterisk: true,
                placeholder: "Enter your password",
            }

            PasswordInput {
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
                value: "hunter2",
            }
        }
    }
}
