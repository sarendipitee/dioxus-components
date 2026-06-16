use crate::component_styles;
use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[component_styles("./style.css")]
struct Styles;

/// Visual variants for the Alert component.
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum AlertVariant {
    /// Neutral alert styling.
    #[default]
    Default,
    /// Destructive or error alert styling.
    Destructive,
    /// Informational alert styling.
    Info,
    /// Success alert styling.
    Success,
}

impl AlertVariant {
    fn class(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Destructive => "destructive",
            Self::Info => "info",
            Self::Success => "success",
        }
    }
}

/// Renders a styled alert container with optional icon, title, description, and action slots.
#[component]
pub fn Alert(
    #[props(default)] variant: AlertVariant,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_alert,
        "data-slot": "alert",
        "data-variant": variant.class(),
        role: "alert",
    });
    let attributes = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..attributes, {children} }
    }
}

/// Renders the leading visual slot for an alert.
#[component]
pub fn AlertIcon(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_alert_icon,
        "data-slot": "alert-icon",
        "aria-hidden": "true",
    });
    let attributes = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..attributes, {children} }
    }
}

/// Renders the alert heading.
#[component]
pub fn AlertTitle(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(h5 {
        class: Styles::dx_alert_title,
        "data-slot": "alert-title",
    });
    let attributes = merge_attributes(vec![base, attributes]);

    rsx! {
        h5 { ..attributes, {children} }
    }
}

/// Renders the supporting alert body content.
#[component]
pub fn AlertDescription(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_alert_description,
        "data-slot": "alert-description",
    });
    let attributes = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..attributes, {children} }
    }
}

/// Renders trailing actions aligned with the alert content.
#[component]
pub fn AlertAction(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_alert_action,
        "data-slot": "alert-action",
    });
    let attributes = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..attributes, {children} }
    }
}
