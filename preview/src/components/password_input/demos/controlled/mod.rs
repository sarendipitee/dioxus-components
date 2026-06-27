use dioxus::prelude::*;
use dioxus_components::button::*;
use dioxus_components::password_input::*;

#[component]
pub fn Demo() -> Element {
    let mut shown = use_signal(|| false);

    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",

            PasswordInput {
                label: "Password",
                placeholder: "Enter your password",
                visible: shown(),
                on_visibility_change: move |v| shown.set(v),
            }

            PasswordInput {
                label: "Confirm password",
                placeholder: "Repeat your password",
                visibility_toggle: false,
                visible: shown(),
            }

            Button {
                onclick: move |_| shown.toggle(),
                if shown() { "Hide both" } else { "Show both" }
            }
        }
    }
}
