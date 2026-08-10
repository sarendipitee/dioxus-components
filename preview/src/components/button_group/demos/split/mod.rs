use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonSize, ButtonVariant};
use dioxus_components::components::button_group::{ButtonGroup, ButtonGroupSeparator};
use dioxus_components::dropdown_menu::*;
use dioxus_components::menu::*;
use dioxus_icons::lucide::ChevronDown;

#[component]
pub fn Demo() -> Element {
    let mut selected_option = use_signal(|| "split-button".to_string());

    rsx! {
        ButtonGroup {
            Button { variant: ButtonVariant::Outline, "Split button" }
            DropdownMenu {
                DropdownMenuTrigger {
                    Button {
                        size: ButtonSize::Icon,
                        variant: ButtonVariant::Outline,
                        aria_label: "Open options",
                        ChevronDown {}
                    }
                }
                Menu {
                    MenuItem::<String> {
                        value: "option-1",
                        index: 0usize,
                        on_select: move |_| selected_option.set("option-1".to_string()),
                        "Option 1"
                    }
                    MenuItem::<String> {
                        value: "option-2",
                        index: 1usize,
                        on_select: move |_| selected_option.set("option-2".to_string()),
                        "Option 2"
                    }
                }
            }
        }
        ButtonGroup {
            Button {
                size: ButtonSize::Icon,
                variant: ButtonVariant::Secondary,
                aria_label: "Copy",
                "⌘"
            }
            ButtonGroupSeparator {}
            Button { variant: ButtonVariant::Secondary, "Paste" }
        }
        p { "Selected option: {selected_option}" }
    }
}
