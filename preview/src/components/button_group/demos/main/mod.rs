use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonSize, ButtonVariant};
use dioxus_components::components::button_group::ButtonGroup;
use dioxus_components::dropdown_menu::*;
use dioxus_components::menu::*;
use dioxus_icons::lucide::{Ellipsis, Trash2, Undo2};

#[component]
pub fn Demo() -> Element {
    let mut last_action = use_signal(|| "None");
    let mut archive_checked = use_signal(|| false);

    rsx! {
        div {
            div { display: "flex", flex_direction: "row", gap: "1rem", align_items: "flex-start", margin_bottom: "2rem",
                ButtonGroup {
                    Button {
                        size: ButtonSize::Icon,
                        variant: ButtonVariant::Outline,
                        aria_label: "Go back",
                        Undo2 {}
                    }
                }
                ButtonGroup {
                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| last_action.set("Archive"),
                        "Archive"
                    }
                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| last_action.set("Report"),
                        "Report"
                    }
                }
                ButtonGroup {
                    Button { variant: ButtonVariant::Outline, "Snooze" }
                    DropdownMenu {
                        DropdownMenuTrigger {
                            size: ButtonSize::Icon,
                            variant: ButtonVariant::Outline,
                            aria_label: "More options",
                            Ellipsis {}
                        }
                        Menu {
                            MenuLabel { "Message actions" }
                            MenuGroup {
                                MenuItem::<String> {
                                    value: "read",
                                    index: 0usize,
                                    on_select: move |_| last_action.set("Mark as read"),
                                    "Mark as read"
                                }
                                MenuCheckboxItem::<String> {
                                    value: "archive",
                                    index: 1usize,
                                    checked: archive_checked(),
                                    on_checked_change: move |checked| archive_checked.set(checked),
                                    "Archive after reply"
                                    MenuItemIndicator {
                                        visible: archive_checked(),
                                        "✓"
                                    }
                                }
                            }
                            MenuSeparator {}
                            MenuItem::<String> {
                                value: "trash",
                                index: 2usize,
                                on_select: move |_| last_action.set("Move to trash"),
                                Trash2 { width: "16px", height: "16px" }
                                "Move to trash"
                            }
                        }
                    }
                }
            }
            output { "data-testid": "last-action", "Last action: {last_action}" }
            output { "data-testid": "archive-checked", "Archive after reply: {archive_checked}" }
        }
    }
}
