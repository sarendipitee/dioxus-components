use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        "One thing"
        Separator {
            style: "margin: 25px 10px; width: 50%;",
            horizontal: true,
            decorative: true,
        }
        "Another thing"
    }
}
