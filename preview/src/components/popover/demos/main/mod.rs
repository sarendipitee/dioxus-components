use dioxus::prelude::*;
use dioxus_components::button::Button;
use dioxus_components::popover::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    let mut non_modal_open = use_signal(|| false);

    rsx! {
        div { display: "flex", flex_direction: "column", gap: "1rem",
            Popover { open: open(), on_open_change: move |v| open.set(v),
                PopoverTrigger {
                    "data-testid": "popover-trigger",
                    Button { r#type: "button", "Open popover" }
                }
                PopoverContent {
                    "data-testid": "popover-content",
                    PopoverContentTitle { "Details" }
                    PopoverContentDescription { "This is the popover content." }
                    Button { r#type: "button", "First action" }
                    Button { r#type: "button", "Second action" }
                }
            }
            output {
                "data-testid": "popover-state",
                "Popover is "
                if open() { "open" } else { "closed" }
            }

            Popover {
                is_modal: false,
                open: non_modal_open(),
                on_open_change: move |v| non_modal_open.set(v),
                PopoverTrigger {
                    "data-testid": "non-modal-popover-trigger",
                    Button { r#type: "button", "Open non-modal popover" }
                }
                PopoverContent {
                    "data-testid": "non-modal-popover-content",
                    PopoverContentTitle { "Non-modal details" }
                    PopoverContentDescription { "Focus remains free to leave this popover." }
                    Button { r#type: "button", "Non-modal action" }
                }
            }
        }
    }
}
