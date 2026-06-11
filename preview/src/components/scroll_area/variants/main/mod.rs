use dioxus_components::scroll_area::*;
use dioxus::prelude::*;
use dioxus_primitives::scroll_area::ScrollDirection;

#[component]
pub fn Demo() -> Element {
    rsx! {

        ScrollArea {
            width: "10em",
            height: "10em",
            border: "1px solid var(--surface-border)",
            border_radius: "0.5em",
            padding: "0 1em 1em 1em",
            direction: ScrollDirection::Vertical,
            tabindex: "0",
            div {
                for i in 1..=20 {
                    p { "Scrollable content item {i}" }
                }
            }
        }
    }
}
