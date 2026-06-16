use dioxus::prelude::*;
use dioxus_components::{Badge, checkbox::*};

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; gap: 1rem; max-width: 26rem;",
            Checkbox {
                name: "advanced-security",
                label: rsx! {
                    span { style: "display: inline-flex; align-items: center; gap: 0.5rem;",
                        "Advanced security"
                        Badge { "Pro" }
                    }
                },
                description: "Require additional verification for sensitive account changes.",
            }
        }
    }
}
