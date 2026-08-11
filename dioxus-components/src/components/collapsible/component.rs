use crate::component_styles;
use dioxus::prelude::*;
use dioxus_icons::lucide::{ChevronRight, ChevronsUpDown};
use dioxus_primitives::collapsible::{self, CollapsibleContentProps, CollapsibleProps};
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[component_styles("./style.css")]
struct Styles;

/// Visual layouts available for a styled [`CollapsibleTrigger`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum CollapsibleTriggerVariant {
    /// Full-width disclosure trigger with a trailing up/down indicator.
    #[default]
    Default,
    /// Compact label and rotating chevron with trailing controls revealed on interaction.
    InlineActions,
}

impl CollapsibleTriggerVariant {
    fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::InlineActions => "inline-actions",
        }
    }
}

/// Props for the styled [`CollapsibleTrigger`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleTriggerProps {
    /// Trigger layout.
    #[props(default)]
    pub variant: ReadSignal<CollapsibleTriggerVariant>,
    /// Optional controls rendered beside the trigger button.
    #[props(default)]
    pub actions: Option<Element>,
    /// Render the trigger button as a custom component or element.
    #[props(default)]
    pub r#as: Option<Callback<Vec<Attribute>, Element>>,
    /// Additional attributes applied to the trigger button.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// Trigger label content.
    pub children: Element,
}

#[component]
pub fn Collapsible(props: CollapsibleProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_collapsible,
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        collapsible::Collapsible {
            keep_mounted: props.keep_mounted,
            default_open: props.default_open,
            disabled: props.disabled,
            open: props.open,
            on_open_change: props.on_open_change,
            as: props.r#as,
            attributes,
            {props.children}
        }
    }
}

#[component]
pub fn CollapsibleTrigger(props: CollapsibleTriggerProps) -> Element {
    let variant = (props.variant)();
    let base = attributes!(button {
        class: Styles::dx_collapsible_trigger,
        "data-variant": variant.as_str(),
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    match variant {
        CollapsibleTriggerVariant::Default => {
            let show_icon = props.r#as.is_none();

            rsx! {
                collapsible::CollapsibleTrigger { as: props.r#as, attributes: merged,
                    {props.children}
                    if show_icon {
                        ChevronsUpDown {
                            class: Styles::dx_collapsible_icon,
                            size: "1rem",
                            stroke: "currentColor",
                            "aria-hidden": "true",
                        }
                    }
                }
            }
        }
        CollapsibleTriggerVariant::InlineActions => rsx! {
            div {
                class: Styles::dx_collapsible_trigger_row,
                "data-variant": variant.as_str(),
                collapsible::CollapsibleTrigger { as: props.r#as, attributes: merged,
                    span { class: Styles::dx_collapsible_label, {props.children} }
                    span {
                        class: Styles::dx_collapsible_inline_icon,
                        "aria-hidden": "true",
                        ChevronRight { size: "1rem", stroke: "currentColor" }
                    }
                }
                if let Some(actions) = props.actions {
                    div {
                        class: Styles::dx_collapsible_actions,
                        "data-slot": "collapsible-actions",
                        {actions}
                    }
                }
            }
        },
    }
}

#[component]
pub fn CollapsibleContent(props: CollapsibleContentProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_collapsible_content,
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        collapsible::CollapsibleContent {
            id: props.id,
            attributes,
            {props.children}
        }
    }
}

/// A vertical group of rows displayed inside a [`Collapsible`].
#[component]
pub fn CollapsibleList(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_collapsible_list,
    });
    let attributes = merge_attributes(vec![base, attributes]);

    rsx! {
        div {
            ..attributes,
            {children}
        }
    }
}
