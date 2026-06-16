use dioxus::prelude::*;
use dioxus_components::input::*;

#[component]
pub fn Demo() -> Element {
    let mut email = use_signal(String::new);

    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            TextInput {
                label: rsx! { "Email" },
                placeholder: "name@example.com",
                value: email,
                oninput: move |event: FormEvent| email.set(event.value()),
            }
            p { id: "text-input-value", "Value: {email}" }
        }
    }
}
