use dioxus::prelude::*;
use dioxus_components::ButtonVariant;
use dioxus_components::button::Button;
use dioxus_components::dropdown_menu::*;
use dioxus_components::menu::*;

#[component]
pub fn Demo() -> Element {
    let mut selected_action = use_signal(|| "None".to_string());

    rsx! {
        DropdownMenu {
            DropdownMenuTrigger {
                Button {
                    variant: ButtonVariant::Outline,
                    "Review actions"
                }
            }
            Menu {
                MenuLabel { "Standard" }
                MenuItem::<String> {
                    value: "open",
                    index: 0usize,
                    on_select: move |_| selected_action.set("Open review".to_string()),
                    "Open review"
                }
                MenuItem::<String> {
                    value: "duplicate",
                    index: 1usize,
                    on_select: move |_| selected_action.set("Duplicate review".to_string()),
                    style: "padding-left: var(--surface-padding-lg);",
                    "Inset duplicate review"
                    MenuItemSection { "Inset" }
                }
                MenuSeparator {}
                MenuLabel { "Danger zone" }
                MenuItem::<String> {
                    value: "remove",
                    index: 2usize,
                    on_select: move |_| selected_action.set("Remove reviewer".to_string()),
                    style: "color: var(--danger);",
                    "Remove reviewer"
                    MenuItemSection { "Destructive" }
                }
                MenuItem::<String> {
                    value: "archive",
                    index: 3usize,
                    on_select: move |_| selected_action.set("Archive project".to_string()),
                    style: "padding-left: var(--surface-padding-lg); color: var(--danger);",
                    "Inset destructive archive"
                    MenuItemSection { "Inset + destructive" }
                }
            }
        }

        p { "Last action: {selected_action}" }
    }
}
