use dioxus::prelude::*;
use dioxus_components::input::*;

#[component]
pub fn Demo() -> Element {
    let mut slug = use_signal(|| "release-notes".to_string());

    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            Input {
                input {
                    style: "width: 100%; border: 0; background: transparent; outline: none;",
                    value: slug,
                    placeholder: "release-notes",
                    oninput: move |event: FormEvent| slug.set(event.value()),
                }
            }
            p { id: "input-shell-value", "Shell value: {slug}" }
            InputBase {
                label: rsx! { "Labeled shell" },
                description: rsx! { "InputBase adds wrapper metadata around the same shell." },
                left_section: rsx! { span { "#" } },
                input {
                    style: "width: 100%; border: 0; background: transparent; outline: none;",
                    placeholder: "project-slug",
                }
            }
        }
    }
}
