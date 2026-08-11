use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonSize, ButtonVariant};
use dioxus_components::components::button_group::ButtonGroup;
use dioxus_components::dropdown_menu::*;
use dioxus_components::menu::*;
use dioxus_icons::lucide::{ChevronDown, UserRoundPen};

#[component]
pub fn Demo() -> Element {
    let mut selected_name = use_signal(|| "Follow".to_string());

    rsx! {
        ButtonGroup {
            Button { variant: ButtonVariant::Default, "{selected_name}" }
            DropdownMenu {
                DropdownMenuTrigger {
                    size: ButtonSize::Icon,
                    variant: ButtonVariant::Default,
                    aria_label: "Options",
                    ChevronDown {}
                }
                Menu {
                    MenuItem::<String> {
                        value: "follow",
                        index: 0usize,
                        on_select: move |_| selected_name.set("Following".to_string()),
                        "Follow"
                    }
                    MenuItem::<String> {
                        value: "unfollow",
                        index: 1usize,
                        on_select: move |_| selected_name.set("Follow".to_string()),
                        "Unfollow"
                    }
                }
            }
        }
        ButtonGroup {
            Button {
                size: ButtonSize::Icon,
                variant: ButtonVariant::Outline,
                aria_label: "Profile",
                UserRoundPen {}
            }
            Button {
                variant: ButtonVariant::Outline,
                "Profile"
            }
        }
        p { "Current state: {selected_name}" }
    }
}
