use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonVariant};
use dioxus_components::components::button_group::{ButtonGroup, ButtonGroupSeparator};

#[component]
pub fn Demo() -> Element {
    rsx! {
        ButtonGroup {
            "data-testid": "separator-group",
            Button { variant: ButtonVariant::Secondary, "Copy" }
            Button { variant: ButtonVariant::Secondary, "Paste" }
            ButtonGroupSeparator { "data-testid": "group-separator" }
            Button { variant: ButtonVariant::Secondary, "Archive" }
            Button { variant: ButtonVariant::Secondary, "Delete" }
        }
    }
}
