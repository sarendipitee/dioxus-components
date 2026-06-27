use dioxus::prelude::*;
use dioxus_components::button::Button;
use dioxus_components::input::*;

#[component]
pub fn Demo() -> Element {
    let mut loading = use_signal(|| true);
    let mut value = use_signal(|| "release-notes".to_string());

    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            InputBase {
                label: rsx! { "Project slug" },
                description: rsx! { "We check availability while you type." },
                loading: loading(),
                input {
                    style: "width: 100%; border: 0; background: transparent; outline: none;",
                    value,
                    placeholder: "project-slug",
                    oninput: move |event: FormEvent| value.set(event.value()),
                }
            }
            Button {
                onclick: move |_| loading.toggle(),
                if loading() { "Stop loading" } else { "Start loading" }
            }
        }
    }
}
