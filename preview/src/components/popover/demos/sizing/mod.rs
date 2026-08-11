use dioxus::prelude::*;
use dioxus_components::button::Button;
use dioxus_components::popover::*;
use dioxus_components::typography::{Text, TypographySize, TypographyTone};
use dioxus_primitives::popover::PopoverWidth;

#[component]
pub fn Demo() -> Element {
    let mut explicit_width = use_signal(|| false);

    rsx! {
        div { style: "display: flex; flex-direction: column; align-items: center; gap: 1rem;",
            div { style: "display: flex; gap: 0.5rem;",
                button {
                    r#type: "button",
                    "data-testid": "intrinsic-size-option",
                    onclick: move |_| explicit_width.set(false),
                    "Intrinsic size"
                }
                button {
                    r#type: "button",
                    "data-testid": "extrinsic-size-option",
                    onclick: move |_| explicit_width.set(true),
                    "Extrinsic size"
                }
            }
            Popover {
                PopoverTrigger {
                    "data-testid": "sizing-popover-trigger",
                    Button { r#type: "button", "Open popover" }
                }
                PopoverContent {
                    "data-testid": "sizing-popover-content",
                    width: if explicit_width() {
                        Some(PopoverWidth::Css("320px".to_string()))
                    } else {
                        None
                    },
                    Text {
                        size: TypographySize::Md,
                        tone: TypographyTone::Muted,
                        if explicit_width() {
                            "This popover has an explicit 320px width."
                        } else {
                            "Content determines this popover's width."
                        }
                    }
                }
            }
        }
    }
}
