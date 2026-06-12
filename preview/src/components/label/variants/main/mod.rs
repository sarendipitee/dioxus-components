use dioxus_components::input::TextInput;

use dioxus::prelude::*;
use dioxus_components::label::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { display: "flex", flex_direction: "column", gap: ".5rem",
            Label { html_for: "name", "Name" }

            TextInput { id: "name", placeholder: "Enter your name" }
        }

    }
}
