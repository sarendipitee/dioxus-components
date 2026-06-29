use dioxus::prelude::*;
use dioxus_components::menu::*;
use dioxus_components::menubar::*;

#[component]
pub fn Demo() -> Element {
    let mut selected_item = use_signal(|| "New".to_string());
    let mut status_bar = use_signal(|| true);
    let mut sort_order = use_signal(|| Some("name".to_string()));

    rsx! {
        div {
            Menubar {
                MenubarMenu { index: 0usize,
                    MenubarTrigger { "File" }
                    Menu {
                        MenuLabel { "File" }
                        MenuGroup {
                            MenuItem::<String> {
                                index: 0usize,
                                value: "new".to_string(),
                                on_select: move |_| selected_item.set("New".to_string()),
                                "New"
                                MenuItemSection { "⌘N" }
                            }
                            MenuItem::<String> {
                                index: 1usize,
                                value: "open".to_string(),
                                disabled: true,
                                on_select: move |_| selected_item.set("Open".to_string()),
                                "Open"
                                MenuItemSection { "⌘O" }
                            }
                            MenuSub {
                                MenuSubTrigger::<String> {
                                    value: "share".to_string(),
                                    index: 2usize,
                                    "Share"
                                }
                                MenuSubContent {
                                    MenuItem::<String> {
                                        index: 0usize,
                                        value: "link".to_string(),
                                        on_select: move |_| selected_item.set("Copy link".to_string()),
                                        "Copy link"
                                    }
                                    MenuItem::<String> {
                                        index: 1usize,
                                        value: "invite".to_string(),
                                        on_select: move |_| selected_item.set("Invite".to_string()),
                                        "Invite"
                                    }
                                }
                            }
                        }
                        MenuSeparator {}
                        MenuCheckboxItem::<String> {
                            value: "status_bar".to_string(),
                            index: 3usize,
                            checked: status_bar(),
                            on_checked_change: move |checked| status_bar.set(checked),
                            "Status bar"
                            MenuItemIndicator { visible: status_bar(), "✓" }
                        }
                    }
                }
                MenubarMenu { index: 1usize,
                    MenubarTrigger { "View" }
                    Menu {
                        MenuLabel { "Sort by" }
                        MenuRadioGroup {
                            value: sort_order(),
                            on_value_change: move |value| sort_order.set(Some(value)),
                            MenuRadioItem::<String> {
                                value: "name".to_string(),
                                index: 0usize,
                                "Name"
                                MenuItemIndicator { visible: sort_order() == Some("name".to_string()), "•" }
                            }
                            MenuRadioItem::<String> {
                                value: "date".to_string(),
                                index: 1usize,
                                "Date modified"
                                MenuItemIndicator { visible: sort_order() == Some("date".to_string()), "•" }
                            }
                        }
                    }
                }
            }
        }
        div {
            p { "Selected: {selected_item}" }
            p { "Status bar: {status_bar}" }
            p { "Sort: {sort_order().unwrap_or(\"none\".to_string())}" }
        }
    }
}
