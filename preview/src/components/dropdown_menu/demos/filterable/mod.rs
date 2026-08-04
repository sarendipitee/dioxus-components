use dioxus::prelude::*;
use dioxus_components::button::Button;
use dioxus_components::dropdown_menu::*;
use dioxus_components::menu::*;
use dioxus_components::ButtonVariant;
use dioxus_primitives::dioxus_attributes::attributes;

#[component]
pub fn Demo() -> Element {
    let mut selected_action = use_signal(|| "No action selected");

    rsx! {
        DropdownMenu {
            DropdownMenuTrigger {
                Button {
                    variant: ButtonVariant::Outline,
                    "Actions"
                }
            }
            Menu {
                FilterableMenuContent {
                    filter_input_props: FilterableMenuInputProps {
                        attributes: attributes!(input {
                            placeholder: "Filter actions...",
                            aria_label: "Filter actions",
                        }),
                        ..Default::default()
                    },
                    MenuItem::<String> {
                        value: "create_issue",
                        search_text: "Create issue",
                        index: 0usize,
                        on_select: move |_| selected_action.set("Created issue"),
                        "Create issue"
                    }
                    MenuItem::<String> {
                        value: "assign_reviewer",
                        search_text: "Assign reviewer",
                        index: 1usize,
                        on_select: move |_| selected_action.set("Assigned reviewer"),
                        "Assign reviewer"
                    }
                    MenuItem::<String> {
                        value: "copy_link",
                        search_text: "Copy review link",
                        index: 2usize,
                        on_select: move |_| selected_action.set("Copied review link"),
                        "Copy review link"
                    }
                    MenuItem::<String> {
                        value: "archive",
                        search_text: "Archive project",
                        index: 3usize,
                        on_select: move |_| selected_action.set("Archived project"),
                        "Archive project"
                    }
                }
            }
        }
        p { "{selected_action}" }
    }
}
