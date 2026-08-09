// demo.rs
use dioxus::prelude::*;
use dioxus_components::button::*;

#[component]
pub fn Demo() -> Element {
    let mut activation_count = use_signal(|| 0u32);

    rsx! {
        div { display: "flex", flex_direction: "column", gap: "1rem",
            div { display: "flex", flex_wrap: "wrap", gap: "1rem",
                Button {
                    id: "button-activation",
                    "data-testid": "button-activation",
                    title: "Activates an observable counter",
                    onclick: move |_| activation_count += 1,
                    "Activate"
                }

                Button {
                    "data-testid": "button-disabled",
                    disabled: true,
                    onclick: move |_| activation_count += 1,
                    "Disabled"
                }

                Button { r#type: "submit", "Submit action" }
            }

            output {
                "data-testid": "button-activation-count",
                aria_live: "polite",
                "Activations: {activation_count}"
            }

            div { display: "flex", flex_wrap: "wrap", gap: "1rem",
                Button { "Default" }

                Button { variant: ButtonVariant::Secondary, "Secondary" }

                Button { variant: ButtonVariant::Destructive, "Destructive" }

                Button { variant: ButtonVariant::Outline, "Outline" }

                Button { variant: ButtonVariant::Ghost, "Ghost" }

                Button { variant: ButtonVariant::Link, "Link" }
            }
        }
    }
}
