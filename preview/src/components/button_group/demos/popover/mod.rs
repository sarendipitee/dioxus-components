use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonSize, ButtonVariant};
use dioxus_components::components::button_group::ButtonGroup;
use dioxus_components::popover::*;
use dioxus_components::typography::{
    Heading, HeadingLevel, Text, TypographySize, TypographyTone, TypographyWeight,
};
use dioxus_icons::lucide::{Bot, ChevronDown};

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        Popover {
            open: open(),
            on_open_change: move |next| open.set(next),
            PopoverTrigger {
                "data-testid": "popover-trigger",
                ButtonGroup {
                    Button { variant: ButtonVariant::Default, "Copilot" }
                    Button {
                        size: ButtonSize::Icon,
                        variant: ButtonVariant::Default,
                        aria_label: "Open copilot",
                        ChevronDown {}
                    }
                }
            }
            PopoverContent {
                "data-testid": "popover-content",
                Heading {
                    level: HeadingLevel::H3,
                    size: TypographySize::Lg,
                    weight: TypographyWeight::Bold,
                    "Copilot Summary"
                }
                Text {
                    size: TypographySize::Md,
                    tone: TypographyTone::Muted,
                    "Ask copilot to summarize this page into a note."
                }
                Button { r#type: "button", Bot {}, "Generate summary" }
            }
        }
    }
}
