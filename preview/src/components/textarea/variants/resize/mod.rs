use super::super::component::*;
use crate::components::label::Label;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut none_value = use_signal(|| "Resize disabled".to_string());
    let mut vertical_value = use_signal(|| "Vertical resize".to_string());
    let mut both_value = use_signal(|| "Resize both directions".to_string());

    rsx! {
        div { display: "grid", gap: "1rem",
            div {
                display: "flex",
                flex_direction: "column",
                gap: ".5rem",

                Label { html_for: "resize-none", "None" }
                Textarea {
                    id: "resize-none",
                    variant: TextareaVariant::Default,
                    resize: TextareaResize::None,
                    value: none_value,
                    oninput: move |e: FormEvent| none_value.set(e.value()),
                }
            }

            div {
                display: "flex",
                flex_direction: "column",
                gap: ".5rem",

                Label { html_for: "resize-vertical", "Vertical" }
                Textarea {
                    id: "resize-vertical",
                    variant: TextareaVariant::Default,
                    resize: TextareaResize::Vertical,
                    value: vertical_value,
                    oninput: move |e: FormEvent| vertical_value.set(e.value()),
                }
            }

            div {
                display: "flex",
                flex_direction: "column",
                gap: ".5rem",

                Label { html_for: "resize-both", "Both" }
                Textarea {
                    id: "resize-both",
                    variant: TextareaVariant::Default,
                    resize: TextareaResize::Both,
                    value: both_value,
                    oninput: move |e: FormEvent| both_value.set(e.value()),
                }
            }
        }
    }
}
