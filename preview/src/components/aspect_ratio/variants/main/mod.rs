use dioxus_components::aspect_ratio::AspectRatio;
use dioxus::prelude::*;

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
            AspectRatio { ratio: 4.0 / 3.0,
                div {
                    background: "linear-gradient(to bottom right, var(--surface-selected), var(--surface-muted))",
                    width: "100%",
                    height: "100%",
                }
            }
        }
    }
}
