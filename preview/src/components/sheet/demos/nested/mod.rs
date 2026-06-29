use dioxus::prelude::*;
use dioxus_components::button::Button;
use dioxus_components::sheet::{Sheet, SheetSide};

const MAX_DEPTH: usize = 10;

#[component]
fn SheetLevel(level: usize, open: bool, on_open_change: Callback<bool>) -> Element {
    let mut child_open = use_signal(|| false);

    rsx! {
        Sheet {
            open,
            on_open_change,
            side: SheetSide::Right,
            title: "Sheet {level}",
            description: "This is sheet level {level}.",
            Button {
                onclick: move |_| child_open.set(true),
                "Open Sheet {level + 1}"
            }
            if level < MAX_DEPTH {
                SheetLevel {
                    level: level + 1,
                    open: child_open(),
                    on_open_change: move |open| child_open.set(open),
                }
            }
        }
    }
}

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        Button { onclick: move |_| open.set(true), "Open Sheet" }
        SheetLevel {
            level: 1,
            open: open(),
            on_open_change: move |next| open.set(next),
        }
    }
}
