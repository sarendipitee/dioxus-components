use dioxus::prelude::*;
use dioxus_components::input::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            Input {
                input {
                    style: "width: 100%; border: 0; background: transparent; outline: none;",
                    value: "release-notes",
                    placeholder: "release-notes",
                }
            }
            InputBase {
                label: rsx! { "Labeled shell" },
                description: rsx! { "InputBase adds wrapper metadata around same shell." },
                left_section: rsx! { span { "#" } },
                input {
                    style: "width: 100%; border: 0; background: transparent; outline: none;",
                    placeholder: "project-slug",
                }
            }
        }
    }
}
