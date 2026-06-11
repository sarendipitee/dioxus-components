use dioxus::prelude::*;
use dioxus_components::checkbox::{Checkbox, CheckboxIndicator};

#[component]
pub fn Demo() -> Element {
    rsx! {
        form {
            onsubmit: move |e| {
                tracing::info!("{:?}", e.values());
            },
            Checkbox { id: "tos-check", name: "tos-check",
                CheckboxIndicator { "+" }
            }
            label { r#for: "tos-check", "I agree to the terms presented." }
            br {}
            button {
                r#type: "submit",
                padding: "8px 16px",
                border: "1px solid var(--surface-border)",
                border_radius: "4px",
                background_color: "var(--surface-muted)",
                color: "var(--fg-muted)",
                cursor: "pointer",
                font_size: "14px",
                "Submit"
            }
        }
    }
}
