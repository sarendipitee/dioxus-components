// demo.rs
use dioxus_components::button::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { display: "flex", gap: "1rem",
            Button { "Default" }

            Button { variant: ButtonVariant::Secondary, "Secondary" }

            Button { variant: ButtonVariant::Destructive, "Destructive" }

            Button { variant: ButtonVariant::Outline, "Outline" }

            Button { variant: ButtonVariant::Ghost, "Ghost" }

            Button { variant: ButtonVariant::Link, "Link" }
        }
    }
}
