use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonVariant};
use dioxus_components::input::TextInput;
use dioxus_components::label::Label;
use dioxus_components::sheet::{Sheet, SheetSide};

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    let mut side = use_signal(|| SheetSide::Right);

    let open_sheet = move |s: SheetSide| {
        move |_| {
            side.set(s);
            open.set(true);
        }
    };

    rsx! {
        div { display: "flex", gap: "0.5rem",
            Button { variant: ButtonVariant::Outline, onclick: open_sheet(SheetSide::Top), "Top" }
            Button { variant: ButtonVariant::Outline, onclick: open_sheet(SheetSide::Right), "Right" }
            Button { variant: ButtonVariant::Outline, onclick: open_sheet(SheetSide::Bottom), "Bottom" }
            Button { variant: ButtonVariant::Outline, onclick: open_sheet(SheetSide::Left), "Left" }
        }
        Sheet {
            open: open(),
            on_open_change: move |v| open.set(v),
            side: side(),
            title: "Sheet Title",
            description: "Sheet description goes here.",
            footer: rsx! {
                Button { "Save changes" }
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| open.set(false),
                    "Cancel"
                }
            },
            div {
                display: "grid",
                flex: "1 1 0%",
                grid_auto_rows: "min-content",
                gap: "1.5rem",
                div { display: "grid", gap: "0.75rem",
                    Label { html_for: "sheet-demo-name", "Name" }
                    TextInput {
                        id: "sheet-demo-name",
                        initial_value: "Dioxus",
                    }
                }
                div { display: "grid", gap: "0.75rem",
                    Label { html_for: "sheet-demo-username", "Username" }
                    TextInput {
                        id: "sheet-demo-username",
                        initial_value: "@dioxus",
                    }
                }
            }
        }
    }
}
