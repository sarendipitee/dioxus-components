use dioxus::prelude::*;
use dioxus_components::input::*;

#[component]
pub fn Demo() -> Element {
    let mut email = use_signal(String::new);

    rsx! {
        form {
            id: "text-input-form",
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            TextInput {
                id: "controlled-email",
                label: rsx! { "Email" },
                placeholder: "name@example.com",
                name: "email",
                r#type: "email",
                form: "text-input-form",
                clearable: true,
                "data-testid": "text-input-native",
                "data-input-purpose": "account-email",
                value: email,
                oninput: move |event: FormEvent| email.set(event.value()),
            }
            output { id: "text-input-value", "Value: {email}" }
        }
    }
}
