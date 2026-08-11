use crate::component_styles;
use dioxus::prelude::*;
use dioxus_icons::lucide::ChevronDown;
use dioxus_primitives::accordion::{self, AccordionContentProps, AccordionItemProps};
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[component_styles("./style.css")]
struct Styles;

/// Position of the expand indicator inside an [`AccordionTrigger`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum AccordionChevronPosition {
    /// Render the indicator after the trigger label.
    #[default]
    Right,
    /// Render the indicator before the trigger label and optional icon.
    Left,
}

impl AccordionChevronPosition {
    fn as_str(self) -> &'static str {
        match self {
            Self::Right => "right",
            Self::Left => "left",
        }
    }
}

#[derive(Clone, Copy)]
struct StyledAccordionContext {
    chevron_position: ReadSignal<AccordionChevronPosition>,
    chevron_icon_size: ReadSignal<u32>,
    disable_chevron_rotation: ReadSignal<bool>,
}

/// Props for the styled [`Accordion`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AccordionProps {
    /// Id of the accordion root element.
    pub id: Option<String>,
    /// Whether multiple items can be open at once.
    #[props(default)]
    pub allow_multiple_open: ReadSignal<bool>,
    /// Whether the entire accordion is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,
    /// Whether every item can be collapsed.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub collapsible: ReadSignal<bool>,
    /// Whether arrow-key navigation uses the horizontal axis.
    #[props(default)]
    pub horizontal: ReadSignal<bool>,
    /// Position of trigger expand indicators.
    #[props(default)]
    pub chevron_position: ReadSignal<AccordionChevronPosition>,
    /// Size of the built-in Lucide chevron, in pixels.
    #[props(default = ReadSignal::new(Signal::new(16)))]
    pub chevron_icon_size: ReadSignal<u32>,
    /// Prevents expand indicators from rotating when their item opens.
    #[props(default)]
    pub disable_chevron_rotation: ReadSignal<bool>,
    /// Attributes applied to the accordion root.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// Accordion items.
    pub children: Element,
}

/// Props for the styled [`AccordionTrigger`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AccordionTriggerProps {
    /// Id of the trigger button.
    pub id: ReadSignal<Option<String>>,
    /// Optional leading content rendered next to the label.
    #[props(default)]
    pub icon: Option<Element>,
    /// Custom expand indicator. Pass an empty element to hide the indicator.
    #[props(default)]
    pub chevron: Option<Element>,
    /// Additional attributes applied to the trigger button.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// Trigger label content.
    pub children: Element,
}

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    use_context_provider(|| StyledAccordionContext {
        chevron_position: props.chevron_position,
        chevron_icon_size: props.chevron_icon_size,
        disable_chevron_rotation: props.disable_chevron_rotation,
    });

    let base = attributes!(div {
        class: Styles::dx_accordion,
        "data-chevron-position": (props.chevron_position)().as_str(),
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        accordion::Accordion {
            id: props.id,
            allow_multiple_open: props.allow_multiple_open,
            disabled: props.disabled,
            collapsible: props.collapsible,
            horizontal: props.horizontal,
            attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    rsx! {
        accordion::AccordionItem {
            class: Styles::dx_accordion_item,
            disabled: props.disabled,
            default_open: props.default_open,
            on_change: props.on_change,
            on_trigger_click: props.on_trigger_click,
            index: props.index,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AccordionTrigger(props: AccordionTriggerProps) -> Element {
    let context: StyledAccordionContext = use_context();
    let position = (context.chevron_position)().as_str();
    let rotates = !(context.disable_chevron_rotation)();
    let chevron = props.chevron.unwrap_or_else(|| {
        rsx! {
            ChevronDown {
                size: "{(context.chevron_icon_size)()}px",
                stroke: "currentColor",
                "aria-hidden": "true",
            }
        }
    });

    let base = attributes!(button {
        class: Styles::dx_accordion_trigger,
        "data-chevron-position": position,
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        accordion::AccordionTrigger {
            id: props.id,
            attributes,
            if position == "left" {
                span {
                    class: Styles::dx_accordion_chevron,
                    "data-rotate": rotates,
                    "aria-hidden": "true",
                    {chevron.clone()}
                }
            }
            if let Some(icon) = props.icon {
                span { class: Styles::dx_accordion_icon, "aria-hidden": "true", {icon} }
            }
            span { class: Styles::dx_accordion_label, {props.children} }
            if position == "right" {
                span {
                    class: Styles::dx_accordion_chevron,
                    "data-rotate": rotates,
                    "aria-hidden": "true",
                    {chevron}
                }
            }
        }
    }
}

/// Styled accordion panel. Root attributes, including CSS custom properties, are inherited by the
/// animated inner content container.
#[component]
pub fn AccordionContent(props: AccordionContentProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_accordion_content,
        "data-slot": "accordion-content",
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        accordion::AccordionContent {
            id: props.id,
            attributes,
            div {
                class: Styles::dx_accordion_content_inner,
                "data-slot": "accordion-content-inner",
                div {
                    class: Styles::dx_accordion_content_body,
                    "data-slot": "accordion-content-body",
                    style: "padding: var(--accordion-content-padding, var(--surface-padding));",
                    {props.children}
                }
            }
        }
    }
}
