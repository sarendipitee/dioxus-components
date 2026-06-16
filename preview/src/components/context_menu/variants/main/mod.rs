use dioxus::prelude::*;
use dioxus_components::context_menu::*;
use dioxus_components::menu::*;

#[component]
pub fn Demo() -> Element {
    let mut selected_item = use_signal(|| "Edit".to_string());
    let mut show_line_numbers = use_signal(|| true);
    let mut panel = use_signal(|| Some("preview".to_string()));

    rsx! {
        ContextMenu {
            ContextMenuTrigger { "right click here" }
            Menu {
                MenuLabel { "Canvas" }
                MenuGroup {
                    MenuItem::<String> {
                        value: "edit".to_string(),
                        index: 0usize,
                        on_select: move |_| selected_item.set("Edit".to_string()),
                        "Edit"
                        MenuItemSection { "⌘E" }
                    }
                    MenuSub {
                        MenuSubTrigger::<String> {
                            value: "arrange".to_string(),
                            index: 1usize,
                            "Arrange"
                        }
                        MenuSubContent {
                            MenuItem::<String> {
                                value: "front".to_string(),
                                index: 0usize,
                                on_select: move |_| selected_item.set("Bring to front".to_string()),
                                "Bring to front"
                            }
                            MenuItem::<String> {
                                value: "back".to_string(),
                                index: 1usize,
                                on_select: move |_| selected_item.set("Send to back".to_string()),
                                "Send to back"
                            }
                        }
                    }
                }
                MenuSeparator {}
                MenuCheckboxItem::<String> {
                    value: "line_numbers".to_string(),
                    index: 2usize,
                    checked: show_line_numbers,
                    on_checked_change: move |checked| show_line_numbers.set(checked),
                    "Show line numbers"
                    MenuItemIndicator { visible: show_line_numbers(), "✓" }
                }
                MenuSeparator {}
                MenuLabel { "Panel" }
                MenuRadioGroup {
                    value: panel,
                    on_value_change: move |value| panel.set(Some(value)),
                    MenuRadioItem::<String> {
                        value: "preview".to_string(),
                        index: 3usize,
                        "Preview"
                        MenuItemIndicator { visible: panel() == Some("preview".to_string()), "•" }
                    }
                    MenuRadioItem::<String> {
                        value: "code".to_string(),
                        index: 4usize,
                        "Code"
                        MenuItemIndicator { visible: panel() == Some("code".to_string()), "•" }
                    }
                }
            }
        }

        span { margin_left: "10px", "Selected: {selected_item}" }
        span { margin_left: "10px", "Line numbers: {show_line_numbers}" }
        span { margin_left: "10px", "Panel: {panel().unwrap_or(\"none\".to_string())}" }
    }
}
