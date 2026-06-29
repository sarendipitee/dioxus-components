//! Overlay-manager nesting-matrix demo (dev/test route).
//!
//! Exercises the hard overlay-nesting cases the unified overlay manager must
//! survive, all rendered under a CSS-`transform`ed ancestor that establishes a
//! new stacking context and containing block (the GPU-compositing trap from
//! `docs/plans/overlay-manager.md` §1). Before the portal-based manager, a
//! `position: fixed` overlay nested under such an ancestor is trapped, mis-sized,
//! and can paint behind sibling shells. This page proves overlays portal out to
//! the root `OverlayOutlet` and land at the correct viewport position, above the
//! transformed shell, and dismiss independently.
//!
//! Every trigger and key overlay surface carries a stable `data-testid` so the
//! Wave 4b Playwright suite can drive and assert against it.

use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonVariant};
use dioxus_components::dialog::Dialog;
use dioxus_components::dropdown_menu::*;
use dioxus_components::menu::*;
use dioxus_components::popover::*;
use dioxus_components::select::*;
use dioxus_components::sheet::{Sheet, SheetSide};
use dioxus_components::toast::ToastProvider;
use dioxus_primitives::toast::{use_toast, ToastOptions};

/// Top-level page component for the `/overlay-nesting` route.
///
/// Wraps the matrix in a local [`ToastProvider`] (so the toast-over-modal case
/// has a toast queue + outlet) and a `transform: translateZ(0)` container that
/// activates the stacking-context trap the overlay manager must defeat.
#[component]
pub fn OverlayNestingDemo() -> Element {
    rsx! {
        ToastProvider {
            // The transform here establishes a new stacking context AND a new
            // containing block. Any `position: fixed` overlay rendered inline
            // under this node would be trapped relative to this box rather than
            // the viewport. The overlay manager portals content to the document
            // root outlet, escaping this trap.
            div {
                "data-testid": "overlay-nesting-transform-root",
                style: "transform: translateZ(0); will-change: transform; filter: saturate(1); \
                        margin: 2rem auto; max-width: 48rem; padding: 1.5rem; \
                        border: 1px dashed var(--border, #888); border-radius: 0.5rem; \
                        display: flex; flex-direction: column; gap: 1rem;",

                h1 { style: "font-size: var(--text-lg); font-weight: 600;", "Overlay nesting matrix" }
                p {
                    style: "opacity: 0.7; font-size: var(--text-sm);",
                    "All overlays below render inside a CSS-transformed ancestor. \
                     They must portal to the root outlet and land above this shell."
                }

                DialogInDialog {}
                SheetStacks {}
                FloatingInsideModal {}
                SubmenuInsideDialog {}
                ToastOverModal {}
            }
        }
    }
}

/// Case 1: a Dialog whose body contains another Dialog (dialog-in-dialog).
#[component]
fn DialogInDialog() -> Element {
    let mut outer = use_signal(|| false);
    let mut inner = use_signal(|| false);

    rsx! {
        Button {
            "data-testid": "open-dialog-1",
            onclick: move |_| outer.set(true),
            "Dialog in Dialog",
        }
        Dialog {
            open: outer(),
            on_open_change: move |v| outer.set(v),
            id: "dialog-outer",
            "data-testid": "dialog-outer",
            title: "Outer dialog",
            description: "Open a second dialog stacked on top of this one.",
            footer: rsx! {
                Button {
                    variant: ButtonVariant::Outline,
                    "data-testid": "dialog-outer-close",
                    onclick: move |_| outer.set(false),
                    "Close outer",
                }
            },
            Button {
                "data-testid": "open-dialog-2",
                onclick: move |_| inner.set(true),
                "Open inner dialog",
            }
            Dialog {
                open: inner(),
                on_open_change: move |v| inner.set(v),
                id: "dialog-inner",
                "data-testid": "dialog-inner",
                title: "Inner dialog",
                description: "This dialog is nested inside the outer dialog.",
                footer: rsx! {
                    Button {
                        variant: ButtonVariant::Outline,
                        "data-testid": "dialog-inner-close",
                        onclick: move |_| inner.set(false),
                        "Close inner",
                    }
                },
                p { "Innermost content." }
            }
        }
    }
}

/// Case 2: Sheet-on-Sheet, plus a Dialog opened on top of a Sheet.
#[component]
fn SheetStacks() -> Element {
    let mut sheet_a = use_signal(|| false);
    let mut sheet_b = use_signal(|| false);
    let mut dialog_on_sheet = use_signal(|| false);

    rsx! {
        Button {
            "data-testid": "open-sheet-1",
            onclick: move |_| sheet_a.set(true),
            "Sheet on Sheet + Dialog on Sheet",
        }
        Sheet {
            open: sheet_a(),
            on_open_change: move |v| sheet_a.set(v),
            side: SheetSide::Right,
            "data-testid": "sheet-outer",
            title: "Outer sheet",
            description: "Stack a second sheet, or open a dialog on top.",
            footer: rsx! {
                Button {
                    variant: ButtonVariant::Outline,
                    "data-testid": "sheet-outer-close",
                    onclick: move |_| sheet_a.set(false),
                    "Close outer sheet",
                }
            },
            div {
                style: "display: flex; flex-direction: column; gap: 0.5rem;",
                Button {
                    "data-testid": "open-sheet-2",
                    onclick: move |_| sheet_b.set(true),
                    "Open second sheet",
                }
                Button {
                    "data-testid": "open-dialog-on-sheet",
                    onclick: move |_| dialog_on_sheet.set(true),
                    "Open dialog on this sheet",
                }
            }

            Sheet {
                open: sheet_b(),
                on_open_change: move |v| sheet_b.set(v),
                side: SheetSide::Left,
                "data-testid": "sheet-inner",
                title: "Inner sheet",
                description: "Second sheet docked on the opposite edge.",
                footer: rsx! {
                    Button {
                        variant: ButtonVariant::Outline,
                        "data-testid": "sheet-inner-close",
                        onclick: move |_| sheet_b.set(false),
                        "Close inner sheet",
                    }
                },
                p { "Second sheet content." }
            }

            Dialog {
                open: dialog_on_sheet(),
                on_open_change: move |v| dialog_on_sheet.set(v),
                id: "dialog-on-sheet",
                "data-testid": "dialog-on-sheet",
                title: "Dialog on sheet",
                description: "This modal dialog opened from inside a sheet.",
                footer: rsx! {
                    Button {
                        variant: ButtonVariant::Outline,
                        "data-testid": "dialog-on-sheet-close",
                        onclick: move |_| dialog_on_sheet.set(false),
                        "Close dialog",
                    }
                },
                p { "Dialog content above the sheet." }
            }
        }
    }
}

/// Case 3: a Select, a DropdownMenu, and a Popover opened INSIDE a Dialog, and
/// the same floating panels opened inside a second-level Dialog (proving the
/// floating panels land above the modal and dismiss independently).
#[component]
fn FloatingInsideModal() -> Element {
    let mut dialog = use_signal(|| false);
    let mut nested_dialog = use_signal(|| false);
    let mut popover = use_signal(|| false);
    let mut nested_popover = use_signal(|| false);

    rsx! {
        Button {
            "data-testid": "open-floating-host-dialog",
            onclick: move |_| dialog.set(true),
            "Floating panels inside Dialog",
        }
        Dialog {
            open: dialog(),
            on_open_change: move |v| dialog.set(v),
            id: "floating-host-dialog",
            "data-testid": "floating-host-dialog",
            title: "Floating inside a modal",
            description: "Select, dropdown menu, and popover must layer above this dialog.",
            footer: rsx! {
                Button {
                    variant: ButtonVariant::Outline,
                    "data-testid": "floating-host-dialog-close",
                    onclick: move |_| dialog.set(false),
                    "Close",
                }
            },
            div {
                style: "display: flex; flex-direction: column; gap: 0.75rem;",

                FruitSelect { testid: "select-in-dialog" }

                DropdownMenu {
                    DropdownMenuTrigger {
                        Button {
                            variant: ButtonVariant::Outline,
                            "data-testid": "dropdown-in-dialog-trigger",
                            "Dropdown in dialog",
                        }
                    }
                    Menu {
                        "data-testid": "dropdown-in-dialog-menu",
                        MenuItem::<String> {
                            value: "one".to_string(),
                            index: 0usize,
                            "data-testid": "dropdown-in-dialog-item-1",
                            "Item one",
                        }
                        MenuItem::<String> {
                            value: "two".to_string(),
                            index: 1usize,
                            "Item two",
                        }
                    }
                }

                Popover {
                    open: popover(),
                    on_open_change: move |v| popover.set(v),
                    PopoverTrigger {
                        Button {
                            r#type: "button",
                            variant: ButtonVariant::Outline,
                            "data-testid": "popover-in-dialog-trigger",
                            "Popover in dialog",
                        }
                    }
                    PopoverContent {
                        "data-testid": "popover-in-dialog-content",
                        p { "Popover content above the modal." }
                        Button {
                            r#type: "button",
                            "data-style": "outline",
                            "data-testid": "popover-in-dialog-close",
                            onclick: move |_| popover.set(false),
                            "Close popover",
                        }
                    }
                }

                Button {
                    "data-testid": "open-nested-dialog",
                    onclick: move |_| nested_dialog.set(true),
                    "Open 2nd-level dialog",
                }
            }

            Dialog {
                open: nested_dialog(),
                on_open_change: move |v| nested_dialog.set(v),
                id: "floating-nested-dialog",
                "data-testid": "floating-nested-dialog",
                title: "Second-level dialog",
                description: "Floating panels must still land above this deeper modal.",
                footer: rsx! {
                    Button {
                        variant: ButtonVariant::Outline,
                        "data-testid": "floating-nested-dialog-close",
                        onclick: move |_| nested_dialog.set(false),
                        "Close",
                    }
                },
                div {
                    style: "display: flex; flex-direction: column; gap: 0.75rem;",
                    FruitSelect { testid: "select-in-nested-dialog" }
                    Popover {
                        open: nested_popover(),
                        on_open_change: move |v| nested_popover.set(v),
                        PopoverTrigger {
                            Button {
                                r#type: "button",
                                variant: ButtonVariant::Outline,
                                "data-testid": "popover-in-nested-dialog-trigger",
                                "Popover in nested dialog",
                            }
                        }
                        PopoverContent {
                            "data-testid": "popover-in-nested-dialog-content",
                            p { "Deep popover content." }
                            Button {
                                r#type: "button",
                                "data-style": "outline",
                                "data-testid": "popover-in-nested-dialog-close",
                                onclick: move |_| nested_popover.set(false),
                                "Close popover",
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Case 4: a DropdownMenu with a submenu, opened inside a Dialog (triple-nest:
/// Dialog -> portaled menu -> portaled submenu). Validates the §4.2 context
/// re-provide rule across two portal boundaries.
#[component]
fn SubmenuInsideDialog() -> Element {
    let mut dialog = use_signal(|| false);
    let mut picked = use_signal(|| "none".to_string());

    rsx! {
        Button {
            "data-testid": "open-submenu-host-dialog",
            onclick: move |_| dialog.set(true),
            "Submenu inside Dialog (triple nest)",
        }
        Dialog {
            open: dialog(),
            on_open_change: move |v| dialog.set(v),
            id: "submenu-host-dialog",
            "data-testid": "submenu-host-dialog",
            title: "Submenu inside a modal",
            description: "Dialog -> portaled menu -> portaled submenu.",
            footer: rsx! {
                Button {
                    variant: ButtonVariant::Outline,
                    "data-testid": "submenu-host-dialog-close",
                    onclick: move |_| dialog.set(false),
                    "Close",
                }
            },
            DropdownMenu {
                DropdownMenuTrigger {
                    Button {
                        variant: ButtonVariant::Outline,
                        "data-testid": "submenu-trigger",
                        "Open menu",
                    }
                }
                Menu {
                    "data-testid": "submenu-menu",
                    MenuItem::<String> {
                        value: "direct".to_string(),
                        index: 0usize,
                        "data-testid": "submenu-direct-item",
                        on_select: move |_| picked.set("direct".to_string()),
                        "Direct item",
                    }
                    MenuSub {
                        MenuSubTrigger::<String> {
                            value: "more".to_string(),
                            index: 1usize,
                            "data-testid": "submenu-subtrigger",
                            "More",
                        }
                        MenuSubContent {
                            "data-testid": "submenu-subcontent",
                            MenuItem::<String> {
                                value: "nested".to_string(),
                                index: 0usize,
                                "data-testid": "submenu-nested-item",
                                on_select: move |_| picked.set("nested".to_string()),
                                "Nested item",
                            }
                        }
                    }
                }
            }
            p { "data-testid": "submenu-picked", "Picked: {picked}" }
        }
    }
}

/// Case 5: fire a Toast while a Dialog is open (toast over modal).
#[component]
fn ToastOverModal() -> Element {
    let mut dialog = use_signal(|| false);
    let toast = use_toast();

    rsx! {
        Button {
            "data-testid": "open-toast-host-dialog",
            onclick: move |_| dialog.set(true),
            "Toast over modal",
        }
        Dialog {
            open: dialog(),
            on_open_change: move |v| dialog.set(v),
            id: "toast-host-dialog",
            "data-testid": "toast-host-dialog",
            title: "Toast over a modal",
            description: "Fire a toast; it must paint above the dialog and not block it.",
            footer: rsx! {
                Button {
                    variant: ButtonVariant::Outline,
                    "data-testid": "toast-host-dialog-close",
                    onclick: move |_| dialog.set(false),
                    "Close",
                }
            },
            Button {
                "data-testid": "fire-toast",
                onclick: move |_| {
                    toast.info("Toast fired over the modal", ToastOptions::new());
                },
                "Fire toast",
            }
        }
    }
}

/// A reusable Select with stable, parameterized test ids.
#[component]
fn FruitSelect(testid: &'static str) -> Element {
    rsx! {
        Select::<Option<String>> {
            width: "12rem",
            "data-testid": testid,
            SelectGroup {
                SelectGroupLabel { "Fruits" }
                SelectOption::<Option<String>> {
                    index: 0usize,
                    value: Some("apple".to_string()),
                    text_value: "Apple",
                    "Apple"
                }
                SelectOption::<Option<String>> {
                    index: 1usize,
                    value: Some("banana".to_string()),
                    text_value: "Banana",
                    "Banana"
                }
                SelectOption::<Option<String>> {
                    index: 2usize,
                    value: Some("cherry".to_string()),
                    text_value: "Cherry",
                    "Cherry"
                }
            }
        }
    }
}
