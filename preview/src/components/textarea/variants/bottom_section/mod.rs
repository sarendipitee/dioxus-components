use super::super::component::*;
use crate::components::label::Label;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut description = use_signal(|| "sample text".to_string());
    let character_count = use_memo(move || description.read().chars().count());

    rsx! {
        div { display: "flex", flex_direction: "column", gap: "1.5rem",

            div {
                display: "flex",
                flex_direction: "column",
                gap: ".5rem",
                justify_content: "center",

                Label { html_for: "bottom-section", "Bottom section" }
                Textarea {
                    id: "bottom-section",
                    variant: TextareaVariant::Default,
                    placeholder: "Enter your description",
                    value: description,
                    oninput: move |e: FormEvent| description.set(e.value()),
                    bottom_section: rsx! {
                        span { "Helper text" }
                        span { "{character_count} / 280" }
                    },
                }
            }
        }
    }
}
