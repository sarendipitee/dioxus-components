// demo.rs
use dioxus::prelude::*;
use dioxus_components::button::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { display: "flex", flex_direction: "row", gap: "1rem",

            Button { "Default" }

            Button { variant: ButtonVariant::Outline, "Outline" }

            Button { variant: ButtonVariant::Destructive, "Destructive" }

            Button { variant: ButtonVariant::Ghost, "Ghost" }

            Button { variant: ButtonVariant::Link, "Link" }

        }
    }
}
