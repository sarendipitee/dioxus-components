use dioxus::prelude::*;
use dioxus_components::ButtonVariant;
use dioxus_components::button::Button;
use dioxus_components::dropdown_menu::*;
use dioxus_components::menu::*;

#[derive(Clone, Copy, PartialEq)]
enum SortMode {
    Priority,
    Updated,
    Assignee,
}

impl SortMode {
    fn label(self) -> &'static str {
        match self {
            Self::Priority => "Priority",
            Self::Updated => "Recently updated",
            Self::Assignee => "Assignee",
        }
    }
}

#[component]
pub fn Demo() -> Element {
    let mut show_completed = use_signal(|| true);
    let mut pin_critical = use_signal(|| false);
    let mut sort_mode = use_signal(|| Some(SortMode::Priority));

    rsx! {
        DropdownMenu {
            DropdownMenuTrigger {
                Button {
                    variant: ButtonVariant::Outline,
                    "View options"
                }
            }
            Menu {
                MenuLabel { "Visible items" }
                MenuCheckboxItem::<String> {
                    value: "completed",
                    index: 0usize,
                    checked: show_completed,
                    on_checked_change: move |checked| show_completed.set(checked),
                    "Show completed issues"
                    MenuItemIndicator { visible: show_completed(), "✓" }
                }
                MenuCheckboxItem::<String> {
                    value: "critical",
                    index: 1usize,
                    checked: pin_critical,
                    on_checked_change: move |checked| pin_critical.set(checked),
                    "Pin critical issues"
                    MenuItemIndicator { visible: pin_critical(), "✓" }
                }
                MenuSeparator {}
                MenuLabel { "Sort by" }
                MenuRadioGroup {
                    value: sort_mode,
                    on_value_change: move |value| sort_mode.set(Some(value)),
                    MenuRadioItem::<SortMode> {
                        value: SortMode::Priority,
                        index: 2usize,
                        "Priority"
                        MenuItemIndicator { visible: sort_mode() == Some(SortMode::Priority), "•" }
                    }
                    MenuRadioItem::<SortMode> {
                        value: SortMode::Updated,
                        index: 3usize,
                        "Recently updated"
                        MenuItemIndicator { visible: sort_mode() == Some(SortMode::Updated), "•" }
                    }
                    MenuRadioItem::<SortMode> {
                        value: SortMode::Assignee,
                        index: 4usize,
                        "Assignee"
                        MenuItemIndicator { visible: sort_mode() == Some(SortMode::Assignee), "•" }
                    }
                }
            }
        }

        p { "Show completed: {show_completed}" }
        p { "Pin critical: {pin_critical}" }
        p { "Sort mode: {sort_mode().map(SortMode::label).unwrap_or(\"None\")}" }
    }
}
