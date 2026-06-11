use dioxus_components::textarea::*;
use dioxus_components::label::Label;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut description = use_signal(|| {
        "Write enough text to see the textarea grow, then delete it to see it shrink.".to_string()
    });

    rsx! {
        div { display: "flex", flex_direction: "column", gap: "1.5rem",

            div {
                display: "flex",
                flex_direction: "column",
                gap: ".5rem",
                justify_content: "center",

                Label { html_for: "autosize", "Autosize" }
                p {
                    color: "var(--fg-muted)",
                    font_size: ".875rem",
                    margin: "0",
                    "This demo grows from 3 rows up to a 6-row limit, then becomes scrollable."
                }
                Textarea {
                    id: "autosize",
                    variant: TextareaVariant::Default,
                    placeholder: "Enter your description",
                    autosize: true,
                    min_rows: 3usize,
                    max_rows: 6usize,
                    value: description,
                    oninput: move |e: FormEvent| description.set(e.value()),
                }
            }

            p { id: "textarea-message", "{description}" }
        }
    }
}
