use crate::component_styles;
use crate::components::typography::{
    Heading, HeadingLevel, Text, TextElement, TypographySize, TypographyTone, TypographyWeight,
};
use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::{merge_attributes, TextOrElement};

#[component_styles("./style.css")]
struct Styles;

#[component]
pub fn Card(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_card,
        "data-slot": "card",
    });
    let attributes = merge_attributes(vec![base, attributes]);

    rsx! {
        div {
            ..attributes,
            {children}
        }
    }
}

/// Props for [`CardHeader`].
#[derive(Props, Clone, PartialEq)]
pub struct CardHeaderProps {
    /// Optional shorthand title rendered in the card header. Omit or pass `""` to hide.
    #[props(default, into)]
    pub title: TextOrElement<()>,
    /// Optional shorthand description rendered below the title. Omit or pass `""` to hide.
    #[props(default, into)]
    pub description: TextOrElement<()>,
    /// Global DOM attributes applied to the header element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// Additional header content, such as [`CardAction`].
    pub children: Element,
}

/// Renders the card header with optional title, description, and action slots.
#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    let title = render_card_header_title(props.title);
    let description = render_card_header_description(props.description);
    let base = attributes!(div {
        class: Styles::dx_card_header,
        "data-slot": "card-header",
    });
    let attributes = merge_attributes(vec![base, props.attributes]);
    let children = props.children;

    rsx! {
        div {
            ..attributes,
            if let Some(title) = title {
                {title}
            }
            if let Some(description) = description {
                {description}
            }
            {children}
        }
    }
}

/// Compatibility title slot for card headers.
#[component]
pub fn CardTitle(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let attributes =
        card_slot_attributes(Styles::dx_card_title.to_string(), "card-title", attributes);

    rsx! {
        Heading {
            size: TypographySize::Md,
            weight: TypographyWeight::Semibold,
            level: HeadingLevel::H3,
            attributes,
            {children}
        }
    }
}

/// Compatibility description slot for card headers.
#[component]
pub fn CardDescription(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let attributes = card_slot_attributes(
        Styles::dx_card_description.to_string(),
        "card-description",
        attributes,
    );

    rsx! {
        Text {
            size: TypographySize::Sm,
            tone: TypographyTone::SurfaceMuted,
            element: TextElement::Div,
            attributes,
            {children}
        }
    }
}

#[component]
pub fn CardAction(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_card_action,
        "data-slot": "card-action",
    });
    let attributes = merge_attributes(vec![base, attributes]);

    rsx! {
        div {
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CardContent(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_card_content,
        "data-slot": "card-content",
    });
    let attributes = merge_attributes(vec![base, attributes]);

    rsx! {
        div {
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CardFooter(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_card_footer,
        "data-slot": "card-footer",
    });
    let attributes = merge_attributes(vec![base, attributes]);

    rsx! {
        div {
            ..attributes,
            {children}
        }
    }
}

fn render_card_header_title(title: TextOrElement<()>) -> Option<Element> {
    if title.is_empty() {
        return None;
    }

    Some(match title {
        TextOrElement::Text(text) => rsx! {
            Heading {
                size: TypographySize::Md,
                weight: TypographyWeight::Semibold,
                level: HeadingLevel::H3,
                attributes: card_slot_attributes(
                    Styles::dx_card_title.to_string(),
                    "card-title",
                    Vec::new(),
                ),
                "{text}"
            }
        },
        TextOrElement::Element(element) => {
            render_card_header_slot(Styles::dx_card_title.to_string(), "card-title", element)
        }
        TextOrElement::Render(render) => render_card_header_slot(
            Styles::dx_card_title.to_string(),
            "card-title",
            render.call(()),
        ),
    })
}

fn render_card_header_description(description: TextOrElement<()>) -> Option<Element> {
    if description.is_empty() {
        return None;
    }

    Some(match description {
        TextOrElement::Text(text) => rsx! {
            Text {
                size: TypographySize::Sm,
                tone: TypographyTone::SurfaceMuted,
                attributes: card_slot_attributes(
                    Styles::dx_card_description.to_string(),
                    "card-description",
                    Vec::new(),
                ),
                "{text}"
            }
        },
        TextOrElement::Element(element) => render_card_header_slot(
            Styles::dx_card_description.to_string(),
            "card-description",
            element,
        ),
        TextOrElement::Render(render) => render_card_header_slot(
            Styles::dx_card_description.to_string(),
            "card-description",
            render.call(()),
        ),
    })
}

fn render_card_header_slot(class: String, slot: &'static str, element: Element) -> Element {
    let attributes = card_slot_attributes(class, slot, Vec::new());

    rsx! {
        div {
            ..attributes,
            {element}
        }
    }
}

fn card_slot_attributes(
    class: String,
    slot: &'static str,
    attributes: Vec<Attribute>,
) -> Vec<Attribute> {
    let base = attributes!(div {
        class,
        "data-slot": slot,
    });

    merge_attributes(vec![base, attributes])
}
