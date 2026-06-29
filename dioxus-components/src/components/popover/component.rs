use crate::component_styles;
use crate::components::typography::{
    Heading, HeadingLevel, Text, TextElement, TypographySize, TypographyTone, TypographyWeight,
};
use dioxus::prelude::*;
use dioxus_primitives::popover::{self, PopoverContentProps, PopoverProps, PopoverTriggerProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[component_styles("./style.css")]
pub(crate) struct Styles;

#[component]
pub fn Popover(props: PopoverProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_popover.to_string()
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        popover::Popover {
            id: props.id,
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            attributes: merged,
            {props.children}
        }
    }
}

/// Renders a popover content heading with shared typography styling.
#[component]
pub fn PopoverContentTitle(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(h3 {
        class: Styles::dx_popover_content_title,
        "data-slot": "popover-content-title",
    });
    let attributes = merge_attributes(vec![base, attributes]);

    rsx! {
        Heading {
            size: TypographySize::Lg,
            weight: TypographyWeight::Bold,
            level: HeadingLevel::H3,
            attributes,
            {children}
        }
    }
}

/// Renders supporting popover content text with shared typography styling.
#[component]
pub fn PopoverContentDescription(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(p {
        class: Styles::dx_popover_content_description,
        "data-slot": "popover-content-description",
    });
    let attributes = merge_attributes(vec![base, attributes]);

    rsx! {
        Text {
            size: TypographySize::Md,
            tone: TypographyTone::Muted,
            element: TextElement::P,
            attributes,
            {children}
        }
    }
}

#[component]
pub fn PopoverTrigger(props: PopoverTriggerProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_popover_trigger.to_string()
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        popover::PopoverTrigger { attributes: merged, {props.children} }
    }
}

/// Styled popover trigger that opens the popover without toggling it closed.
#[component]
pub fn PopoverOpenTrigger(props: PopoverTriggerProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_popover_trigger.to_string()
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        popover::PopoverOpenTrigger { attributes: merged, {props.children} }
    }
}

#[component]
pub fn PopoverContent(props: PopoverContentProps) -> Element {
    let base = attributes!(div {
        class: format!("{} dx_dropdown", Styles::dx_popover_content)
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        popover::PopoverContent {
            id: props.id,
            side: props.side,
            align: props.align,
            attributes,
            {props.children}
        }
    }
}
