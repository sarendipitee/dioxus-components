use crate::components::dialog::*;
use dioxus::prelude::*;
use dioxus_components::button::Button;

const PARAGRAPHS: &[&str] = &[
    "Dioxus is a Rust framework for building user interfaces across web, desktop, and mobile platforms.",
    "Components are the building blocks of Dioxus applications. Each component is a function that takes props and returns an Element.",
    "State management in Dioxus uses signals, which are reactive values that automatically re-render components when they change.",
    "The virtual DOM diffing algorithm in Dioxus ensures that only the minimal set of changes are applied to the real DOM.",
    "Dioxus supports server-side rendering, enabling fast initial page loads and SEO-friendly web applications.",
    "You can use standard Rust tooling — cargo, rust-analyzer, clippy — throughout the entire Dioxus development workflow.",
    "Platform-specific APIs can be accessed through Dioxus desktop and mobile crates, giving you native capabilities alongside your UI code.",
    "The RSX macro provides a JSX-like syntax for describing UI trees in Rust, with full type safety and IDE support.",
];

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        Button { onclick: move |_| open.set(true), "Open Scrollable Dialog" }
        Dialog {
            open: open(),
            on_open_change: move |v| open.set(v),
            DialogContent {
                style: "max-height: 400px;",
                DialogClose { "×" }
                DialogHeader {
                    DialogTitle { "About Dioxus" }
                    DialogDescription { "Scroll through the content below to learn more." }
                }
                DialogBody {
                    for para in PARAGRAPHS {
                        p { "{para}" }
                    }
                }
                DialogFooter {
                    Button {
                        onclick: move |_| open.set(false),
                        "Close"
                    }
                }
            }
        }
    }
}
