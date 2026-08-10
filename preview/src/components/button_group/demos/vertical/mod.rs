use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonSize, ButtonVariant};
use dioxus_components::components::button_group::{ButtonGroup, ButtonGroupOrientation};
use dioxus_icons::lucide::{Minus, Plus};

#[component]
pub fn Demo() -> Element {
    rsx! {
        ButtonGroup {
            orientation: ButtonGroupOrientation::Vertical,
            aria_label: "Adjust value",
            Button {
                variant: ButtonVariant::Outline,
                size: ButtonSize::Icon,
                aria_label: "Increase value",
                Plus {}
            }
            Button {
                variant: ButtonVariant::Outline,
                size: ButtonSize::Icon,
                aria_label: "Decrease value",
                Minus {}
            }
        }
    }
}
