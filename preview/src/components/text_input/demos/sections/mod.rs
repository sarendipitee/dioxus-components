use dioxus::prelude::*;
use dioxus_components::input::*;

#[component]
pub fn Demo() -> Element {
    let mut search = use_signal(|| "Release notes".to_string());

    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            TextInput {
                label: rsx! { "Search docs" },
                description: rsx! { "Sections share the same shell used by picker and combobox fields." },
                value: search,
                oninput: move |event: FormEvent| search.set(event.value()),
                left_section: rsx! { span { "@" } },
                right_section: rsx! {
                    InputClearButton {
                        aria_label: "Clear search",
                        disabled: search.read().is_empty(),
                        onclick: move |_| search.set(String::new()),
                    }
                },
            }
        }
    }
}
