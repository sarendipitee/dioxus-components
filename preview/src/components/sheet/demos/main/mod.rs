use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonVariant};
use dioxus_components::input::TextInput;
use dioxus_components::label::Label;
use dioxus_components::sheet::{Sheet, SheetSide};

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    let mut side = use_signal(|| SheetSide::Right);
    let mut audit_open = use_signal(|| false);
    let mut audit_change_count = use_signal(|| 0_u32);
    let mut audit_last_change = use_signal(|| "none");
    let mut nonmodal_open = use_signal(|| false);

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
        div { display: "flex", gap: "0.5rem", margin_top: "1rem",
            Button {
                "data-testid": "sheet-audit-open",
                onclick: move |_| audit_open.set(true),
                "Open audit sheet"
            }
            output {
                "data-testid": "sheet-audit-status",
                "{audit_open()}:{audit_change_count()}:{audit_last_change()}"
            }
        }
        Sheet {
            open: audit_open(),
            on_open_change: move |next| {
                audit_open.set(next);
                audit_change_count += 1;
                audit_last_change.set(if next { "open" } else { "closed" });
            },
            close_on_escape: false,
            close_on_backdrop_click: false,
            with_close_button: false,
            title: "Audit sheet",
            description: "Dismissal is disabled for this audit sheet.",
            id: "sheet-audit-content",
            "data-testid": "sheet-audit-content",
            "data-audit-scope": "controlled",
            aria_label: "Audit sheet",
            Button {
                onclick: move |_| {
                    audit_open.set(false);
                    audit_change_count += 1;
                    audit_last_change.set("closed");
                },
                "Close audit sheet programmatically"
            }
        }

        div { display: "flex", gap: "0.5rem", margin_top: "1rem",
            Button {
                "data-testid": "sheet-nonmodal-open",
                onclick: move |_| nonmodal_open.set(true),
                "Open nonmodal sheet"
            }
            Button { "data-testid": "sheet-nonmodal-outside", "Outside action" }
        }
        Sheet {
            open: nonmodal_open(),
            on_open_change: move |next| nonmodal_open.set(next),
            is_modal: false,
            title: "Nonmodal sheet title",
            id: "sheet-nonmodal-content",
            "data-testid": "sheet-nonmodal-content",
            dioxus_primitives::dialog::DialogClose { "Close nonmodal sheet" }
        }
    }
}
