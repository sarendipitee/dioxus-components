use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonSize, ButtonVariant};
use dioxus_components::popover::*;
use dioxus_components::typography::{
    Heading, HeadingLevel, Text, TypographySize, TypographyTone, TypographyWeight,
};

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        Popover { open: open(), on_open_change: move |v| open.set(v),
            PopoverTrigger {
                "data-testid": "popover-trigger",
                Button {
                    r#type: "button",
                    variant: ButtonVariant::Outline,
                    "Open popover"
                }
            }
            PopoverContent {
                "data-testid": "popover-content",
                div { style: "display: grid; gap: var(--surface-gap);",
                    div { style: "display: grid; gap: var(--space);",
                        Heading {
                            level: HeadingLevel::H3,
                            size: TypographySize::Md,
                            weight: TypographyWeight::Semibold,
                            "Details"
                        }
                        Text {
                            size: TypographySize::Sm,
                            tone: TypographyTone::Muted,
                            "This is the popover content."
                        }
                    }
                    div {
                        style: "display: flex; justify-content: flex-end; gap: var(--content-gap);",
                        Button {
                            r#type: "button",
                            size: ButtonSize::Sm,
                            variant: ButtonVariant::Outline,
                            onclick: move |_| open.set(false),
                            "Cancel"
                        }
                        Button {
                            r#type: "button",
                            size: ButtonSize::Sm,
                            onclick: move |_| open.set(false),
                            "Confirm"
                        }
                    }
                }
            }
        }
    }
}
