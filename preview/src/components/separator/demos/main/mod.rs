use dioxus_components::separator::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut horizontal = use_signal(|| true);
    let mut decorative = use_signal(|| false);
    let orientation = if horizontal() { "horizontal" } else { "vertical" };

    rsx! {
        div {
            style: "display: grid; gap: 1rem;",
            div {
                style: "display: flex; gap: 0.5rem;",
                button {
                    r#type: "button",
                    onclick: move |_| horizontal.set(!horizontal()),
                    "Toggle orientation"
                }
                button {
                    r#type: "button",
                    onclick: move |_| decorative.set(!decorative()),
                    "Toggle decorative"
                }
            }
            output {
                id: "separator-status",
                aria_live: "polite",
                {format!("orientation: {orientation}; decorative: {}", decorative())}
            }
            div {
                style: "display: flex; align-items: center; width: 320px; height: 120px;",
                "One thing"
                Separator {
                    id: "separator-fixture",
                    class: "separator-fixture",
                    title: "Reactive separator fixture",
                    "data-separator-fixture": "reactive",
                    horizontal: horizontal(),
                    decorative: decorative(),
                }
                "Another thing"
            }
        }
    }
}
