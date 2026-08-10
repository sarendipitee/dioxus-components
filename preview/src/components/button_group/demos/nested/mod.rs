use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonVariant};
use dioxus_components::components::button_group::ButtonGroup;

#[component]
pub fn Demo() -> Element {
    rsx! {
        ButtonGroup { "data-testid": "nested-group",
            ButtonGroup {
                Button { variant: ButtonVariant::Outline, "Back" }
            }
            ButtonGroup {
                Button { variant: ButtonVariant::Outline, "Next" }
                Button { variant: ButtonVariant::Outline, "More" }
            }
        }
    }
}
