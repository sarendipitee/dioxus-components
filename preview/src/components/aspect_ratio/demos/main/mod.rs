use dioxus::prelude::*;
use dioxus_components::aspect_ratio::AspectRatio;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            overflow: "hidden",
            box_sizing: "border-box",
            padding: "1rem",
            border_radius: ".5rem",
            width: "20rem",
            max_width: "30vw",
            AspectRatio {
                ratio: 4.0 / 3.0,
                id: "aspect-ratio-demo",
                aria_label: "4 by 3 landscape preview",
                "data-testid": "aspect-ratio",
                "data-ratio": "4:3",
                div {
                    "data-testid": "aspect-ratio-content",
                    background: "linear-gradient(to bottom right, var(--surface-selected), var(--surface-muted))",
                    width: "100%",
                    height: "100%",
                    display: "flex",
                    align_items: "center",
                    justify_content: "center",
                    "4:3 landscape preview"
                }
            }
        }
    }
}
