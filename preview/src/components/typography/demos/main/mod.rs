use dioxus::prelude::*;

use dioxus_components::typography::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            display: "grid",
            gap: "1rem",
            max_width: "36rem",

            div {
                Heading {
                    size: TypographySize::Lg,
                    weight: TypographyWeight::Bold,
                    "Shared typography"
                }
                Text {
                    tone: TypographyTone::Muted,
                    "Use Text and Heading for reusable styled copy without moving accessibility behavior out of primitives."
                }
            }

            div { display: "grid", gap: "0.5rem",
                Heading {
                    level: HeadingLevel::H3,
                    size: TypographySize::Md,
                    weight: TypographyWeight::Semibold,
                    "Semantic h3 with medium visual size"
                }
                Text {
                    size: TypographySize::Sm,
                    tone: TypographyTone::Faint,
                    "Small faint text for lower-emphasis metadata."
                }
                Text {
                    element: TextElement::Span,
                    tone: TypographyTone::Accent,
                    weight: TypographyWeight::Medium,
                    "Inline accent span"
                }
            }

            Text {
                align: TextAlign::Center,
                wrap: TextWrap::Balance,
                "Centered balanced text keeps short headings and descriptions readable in compact layouts."
            }
        }
    }
}
