use dioxus::prelude::*;
use dioxus_components::button::Button;
use dioxus_components::popover::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        Popover { open: open(), on_open_change: move |v| open.set(v),
            PopoverTrigger {
                Button { r#type: "button", "Open popover" }
            }
            PopoverContent {
                PopoverContentTitle { "Details" }
                PopoverContentDescription { "This is the popover content." }
            }
        }
    }
}
