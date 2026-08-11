use dioxus::core::AttributeValue;
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

            div { display: "grid", gap: "1rem",
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

            div {
                id: "typography-semantic-fixture",
                "data-testid": "typography-semantic-fixture",
                display: "grid",
                gap: "1rem",

                Heading {
                    level: HeadingLevel::H1,
                    id: "typography-heading-h1",
                    class: "typography-semantic-heading",
                    aria_label: "Typography level one heading",
                    "data-typography-heading-level": "h1",
                    "Heading level one"
                }
                Heading {
                    level: HeadingLevel::H3,
                    id: "typography-heading-h3",
                    "data-typography-heading-level": "h3",
                    "Heading level three"
                }
                Heading {
                    level: HeadingLevel::H6,
                    id: "typography-heading-h6",
                    "data-typography-heading-level": "h6",
                    "Heading level six"
                }
                Text {
                    element: TextElement::Div,
                    id: "typography-semantic-div",
                    class: "typography-semantic-div",
                    aria_label: "Typography semantic division",
                    "data-typography-text-element": "div",
                    "Text rendered as a semantic division with forwarded global attributes."
                }
                Text {
                    element: TextElement::Label,
                    attributes: vec![Attribute::new(
                        "for",
                        AttributeValue::Text("typography-semantic-input".to_string()),
                        None,
                        false,
                    )],
                    id: "typography-semantic-label",
                    class: "typography-semantic-label",
                    "data-typography-text-element": "label",
                    "Fixture input label"
                }
                input {
                    id: "typography-semantic-input",
                    "data-typography-input": "associated",
                }
                Text {
                    truncate: true,
                    "data-typography-text-element": "paragraph",
                    "This paragraph demonstrates single-line truncation behavior."
                }
                Text {
                    element: TextElement::Span,
                    line_clamp: Some(2),
                    "data-typography-text-element": "span",
                    "This inline span demonstrates two-line clamping behavior."
                }
            }
        }
    }
}
