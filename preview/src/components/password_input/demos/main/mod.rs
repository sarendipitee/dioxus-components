use dioxus::prelude::*;
use dioxus_components::password_input::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",

            PasswordInput {
                label: "Password",
                description: "Click the eye to reveal the value.",
                placeholder: "Enter your password",
            }
        }
    }
}
