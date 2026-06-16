use dioxus::prelude::*;
use dioxus_components::ButtonVariant;
use dioxus_components::button::Button;
use dioxus_components::dropdown_menu::*;
use dioxus_components::menu::*;

#[component]
pub fn Demo() -> Element {
    let mut selected_action = use_signal(|| "Open command palette");

    rsx! {
        DropdownMenu {
            DropdownMenuTrigger {
                Button {
                    variant: ButtonVariant::Outline,
                    "Project actions"
                }
            }
            Menu {
                MenuLabel { "Editing" }
                MenuGroup {
                    MenuItem::<String> {
                        value: "command_palette",
                        index: 0usize,
                        on_select: move |_| selected_action.set("Open command palette"),
                        "Open command palette"
                        MenuItemSection { "⌘K" }
                    }
                    MenuItem::<String> {
                        value: "rename",
                        index: 1usize,
                        on_select: move |_| selected_action.set("Rename project"),
                        "Rename project"
                        MenuItemSection { "⇧⌘R" }
                    }
                }
                MenuSeparator {}
                MenuLabel { "Sharing" }
                MenuGroup {
                    MenuItem::<String> {
                        value: "copy_link",
                        index: 2usize,
                        on_select: move |_| selected_action.set("Copy review link"),
                        "Copy review link"
                        MenuItemSection { "⌘⇧C" }
                    }
                    MenuItem::<String> {
                        value: "invite",
                        index: 3usize,
                        disabled: true,
                        on_select: move |_| selected_action.set("Invite teammate"),
                        "Invite teammate"
                        MenuItemSection { "Pro" }
                    }
                }
                MenuSeparator {}
                MenuLabel { "Automation" }
                MenuGroup {
                    MenuItem::<String> {
                        value: "run_checks",
                        index: 4usize,
                        on_select: move |_| selected_action.set("Run release checks"),
                        "Run release checks"
                        MenuItemSection { "⌃R" }
                    }
                }
            }
        }

        p { "Last action: {selected_action}" }
    }
}
