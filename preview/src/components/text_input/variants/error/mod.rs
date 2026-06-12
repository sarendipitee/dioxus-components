use dioxus::prelude::*;
use dioxus_components::input::*;

#[component]
pub fn Demo() -> Element {
    let mut username = use_signal(|| "taken-name".to_string());

    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            TextInput {
                label: rsx! { "Username" },
                description: rsx! { "Use 3-24 lowercase letters, numbers, or hyphens." },
                error: rsx! { "That username is already in use." },
                value: username,
                oninput: move |event: FormEvent| username.set(event.value()),
            }
        }
    }
}
