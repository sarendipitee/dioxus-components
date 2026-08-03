use dioxus::prelude::*;
use dioxus_components::menu::{FilterableMenu, FilterableMenuInputProps, MenuItem};
use dioxus_primitives::dioxus_attributes::attributes;

#[component]
pub fn Demo() -> Element {
    let open = use_memo(|| true);
    let set_open = use_callback(|_| {});
    let disabled = use_memo(|| false);
    let roving_loop = use_memo(|| true);
    let mut selected_action = use_signal(|| "No action selected");

    rsx! {
        FilterableMenu {
            open,
            set_open,
            disabled,
            roving_loop,
            filter_input_props: FilterableMenuInputProps {
                attributes: attributes!(input {
                    placeholder: "Filter actions...",
                    aria_label: "Filter actions",
                }),
                ..Default::default()
            },
            MenuItem::<String> {
                value: "create_issue",
                index: 0usize,
                on_select: move |_| selected_action.set("Created issue"),
                "Create issue"
            }
            MenuItem::<String> {
                value: "assign_reviewer",
                index: 1usize,
                on_select: move |_| selected_action.set("Assigned reviewer"),
                "Assign reviewer"
            }
            MenuItem::<String> {
                value: "copy_link",
                index: 2usize,
                on_select: move |_| selected_action.set("Copied review link"),
                "Copy review link"
            }
            MenuItem::<String> {
                value: "archive",
                index: 3usize,
                on_select: move |_| selected_action.set("Archived project"),
                "Archive project"
            }
        }
        p { "{selected_action}" }
    }
}
