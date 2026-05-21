use dioxus_components::input::Input;

use dioxus_components::label::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { display: "flex", flex_direction: "column", gap: ".5rem",
            Label { html_for: "name", "Name" }

            Input { id: "name", placeholder: "Enter your name" }
        }

    }
}
