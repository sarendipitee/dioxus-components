use dioxus::prelude::*;
use dioxus_components::scroll_area::*;
use dioxus_primitives::scroll_area::{ScrollDirection, ScrollType};

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            display: "grid",
            gap: "1rem",

            ScrollArea {
                id: "scroll-area-forwarded-id",
                "data-testid": "scroll-area-vertical-auto",
                role: "region",
                aria_label: "Vertical auto scroll area",
                width: "12em",
                height: "10em",
                border: "1px solid var(--surface-border)",
                border_radius: "0.5em",
                padding: "0 1em 1em 1em",
                direction: ScrollDirection::Vertical,
                scroll_type: ScrollType::Auto,
                tabindex: "0",
                div {
                    for i in 1..=20 {
                        p { "Scrollable content item {i}" }
                    }
                }
            }

            ScrollArea {
                "data-testid": "scroll-area-horizontal-always",
                role: "region",
                aria_label: "Horizontal always scroll area",
                width: "12em",
                height: "6em",
                border: "1px solid var(--surface-border)",
                border_radius: "0.5em",
                padding: "1em",
                direction: ScrollDirection::Horizontal,
                scroll_type: ScrollType::Always,
                tabindex: "0",
                div {
                    width: "32em",
                    "Horizontal content extends beyond the viewport so the area can scroll."
                }
            }

            ScrollArea {
                "data-testid": "scroll-area-both-hidden",
                role: "region",
                aria_label: "Both-axis hidden scroll area",
                width: "12em",
                height: "10em",
                border: "1px solid var(--surface-border)",
                border_radius: "0.5em",
                padding: "1em",
                direction: ScrollDirection::Both,
                scroll_type: ScrollType::Hidden,
                tabindex: "0",
                div {
                    width: "32em",
                    height: "24em",
                    "Content extends beyond the viewport in both directions while the scrollbars remain hidden."
                }
            }
        }
    }
}
