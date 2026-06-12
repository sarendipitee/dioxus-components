use dioxus::prelude::*;
use dioxus_components::input::*;

#[component]
pub fn Demo() -> Element {
    let mut query = use_signal(|| "billing".to_string());

    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            Input {
                left_section: rsx! { span { "/" } },
                right_section: rsx! {
                    InputClearButton {
                        aria_label: "Clear query",
                        disabled: query.read().is_empty(),
                        onclick: move |_| query.set(String::new()),
                    }
                },
                input {
                    style: "width: 100%; border: 0; background: transparent; outline: none;",
                    value: query,
                    placeholder: "Search a route",
                    oninput: move |event: FormEvent| query.set(event.value()),
                }
            }
        }
    }
}
