use crate::component_styles;
use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use dioxus_primitives::tooltip::{self, TooltipContentProps, TooltipTriggerProps};
use dioxus_primitives::{ContentAlign, ContentSide, TextOrElement};

#[component_styles("./style.css")]
struct Styles;

#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    /// The controlled `open` state.
    pub open: ReadSignal<Option<bool>>,

    /// Default open state when uncontrolled.
    #[props(default)]
    pub default_open: bool,

    /// Callback when open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Whether the tooltip is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Shorthand content. When provided, children become the trigger and this
    /// content renders inside TooltipContent automatically.
    /// Omit or pass `""` for the default children-based API
    /// (children must contain explicit TooltipTrigger + TooltipContent).
    #[props(default, into)]
    pub content: TextOrElement<()>,

    /// Side of the trigger to place the tooltip content. Only used with `content` shorthand.
    #[props(default = ContentSide::Top)]
    pub side: ContentSide,

    /// Alignment of the tooltip content relative to the trigger. Only used with `content` shorthand.
    #[props(default = ContentAlign::Center)]
    pub align: ContentAlign,

    /// Additional attributes for the tooltip wrapper element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the tooltip. When `content` is provided, children become the trigger.
    pub children: Element,
}

#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_tooltip.to_string(),
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    if !props.content.is_empty() {
        rsx! {
            tooltip::Tooltip {
                disabled: props.disabled,
                open: props.open,
                default_open: props.default_open,
                on_open_change: props.on_open_change,
                attributes: merged,
                TooltipTrigger {
                    {props.children}
                }
                TooltipContent {
                    side: props.side,
                    align: props.align,
                    {props.content.into_element()}
                }
            }
        }
    } else {
        rsx! {
            tooltip::Tooltip {
                disabled: props.disabled,
                open: props.open,
                default_open: props.default_open,
                on_open_change: props.on_open_change,
                attributes: merged,
                {props.children}
            }
        }
    }
}

#[component]
pub fn TooltipTrigger(props: TooltipTriggerProps) -> Element {
    let base = attributes!(button {
        class: Styles::dx_tooltip_trigger.to_string(),
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        tooltip::TooltipTrigger {
            id: props.id,
            as: props.r#as,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn TooltipContent(props: TooltipContentProps) -> Element {
    let base = attributes!(div {
        class: format!("{} dx_dropdown", Styles::dx_tooltip_content),
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        tooltip::TooltipContent {
            id: props.id,
            side: props.side,
            align: props.align,
            attributes: merged,
            {props.children}
        }
    }
}
