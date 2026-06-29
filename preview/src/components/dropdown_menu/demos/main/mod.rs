use dioxus::prelude::*;
use dioxus_components::ButtonVariant;
use dioxus_components::button::Button;
use dioxus_components::dropdown_menu::*;
use dioxus_components::menu::*;

#[derive(Clone, Copy, PartialEq)]
enum ThemeMode {
    System,
    Light,
    Dark,
}

impl ThemeMode {
    fn label(self) -> &'static str {
        match self {
            Self::System => "System",
            Self::Light => "Light",
            Self::Dark => "Dark",
        }
    }
}

#[component]
pub fn Demo() -> Element {
    let mut selected_operation = use_signal(|| "Edit");
    let mut show_toolbar = use_signal(|| true);
    let mut active_theme = use_signal(|| Some(ThemeMode::System));

    rsx! {
        DropdownMenu {
            DropdownMenuTrigger {
                Button {
                    variant: ButtonVariant::Outline,
                    "Open Menu"
                }
            }
            Menu {
                MenuLabel { "Actions" }
                MenuGroup {
                    MenuItem::<String> {
                        value: "edit",
                        index: 0usize,
                        on_select: move |_| selected_operation.set("Edit"),
                        "Edit"
                        MenuItemSection { "⌘E" }
                    }
                    MenuItem::<String> {
                        value: "undo",
                        index: 1usize,
                        disabled: true,
                        on_select: move |_| selected_operation.set("Undo"),
                        "Undo"
                        MenuItemSection { "⌘Z" }
                    }
                    MenuSub {
                        MenuSubTrigger::<String> {
                            value: "share",
                            index: 2usize,
                            "Share"
                        }
                        MenuSubContent {
                            MenuItem::<String> {
                                value: "link",
                                index: 0usize,
                                on_select: move |_| selected_operation.set("Copy link"),
                                "Copy link"
                            }
                            MenuItem::<String> {
                                value: "invite",
                                index: 1usize,
                                on_select: move |_| selected_operation.set("Invite teammate"),
                                "Invite teammate"
                            }
                        }
                    }
                }
                MenuSeparator {}
                MenuCheckboxItem::<String> {
                    value: "toolbar",
                    index: 3usize,
                    checked: show_toolbar(),
                    on_checked_change: move |checked| show_toolbar.set(checked),
                    "Show Toolbar"
                    MenuItemIndicator { visible: show_toolbar(), "✓" }
                }
                MenuSeparator {}
                MenuLabel { "Theme" }
                MenuRadioGroup {
                    value: active_theme(),
                    on_value_change: move |value| active_theme.set(Some(value)),
                    MenuRadioItem::<ThemeMode> {
                        value: ThemeMode::System,
                        index: 4usize,
                        "System"
                        MenuItemIndicator {
                            visible: active_theme() == Some(ThemeMode::System),
                            "•"
                        }
                    }
                    MenuRadioItem::<ThemeMode> {
                        value: ThemeMode::Light,
                        index: 5usize,
                        "Light"
                        MenuItemIndicator {
                            visible: active_theme() == Some(ThemeMode::Light),
                            "•"
                        }
                    }
                    MenuRadioItem::<ThemeMode> {
                        value: ThemeMode::Dark,
                        index: 6usize,
                        "Dark"
                        MenuItemIndicator {
                            visible: active_theme() == Some(ThemeMode::Dark),
                            "•"
                        }
                    }
                }
            }
        }
        p { "Selected action: {selected_operation}" }
        p { "Toolbar visible: {show_toolbar}" }
        p { "Theme: {active_theme().map(ThemeMode::label).unwrap_or(\"None\")}" }
    }
}
