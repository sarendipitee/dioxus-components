use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonVariant};
use dioxus_components::sheet::{Sheet, SheetSide};

const MAX_DEPTH: usize = 10;

#[component]
fn SheetLevel(level: usize, depth: Signal<usize>) -> Element {
    rsx! {
        Sheet {
            open: depth() >= level,
            on_open_change: move |v: bool| { if !v { depth.set(level - 1); } },
            side: SheetSide::Right,
            title: "Sheet {level}",
            description: "This is sheet level {level}. Open another to go deeper.",
            footer: rsx! {
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| depth.set(level - 1),
                    "Close"
                }
            },
            Button {
                onclick: move |_| depth.set(level + 1),
                "Open Sheet {level + 1}"
            }
            if level < MAX_DEPTH {
                SheetLevel { level: level + 1, depth }
            }
        }
    }
}

#[component]
pub fn Demo() -> Element {
    let mut depth = use_signal(|| 0usize);

    rsx! {
        Button { onclick: move |_| depth.set(1), "Open Sheet" }
        SheetLevel { level: 1, depth }
    }
}
