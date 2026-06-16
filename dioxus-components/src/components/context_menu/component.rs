use crate::component_styles;
use crate::components::menu::{provide_styled_menu_surface, StyledMenuSurface};
use dioxus::prelude::*;
use dioxus_primitives::context_menu::{self, ContextMenuProps, ContextMenuTriggerProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[component_styles("./style.css")]
pub(crate) struct Styles;

fn merge_with_class(tag: &str, class_name: String, attributes: Vec<Attribute>) -> Vec<Attribute> {
    let base = match tag {
        "button" => attributes!(button { class: class_name }),
        _ => attributes!(div { class: class_name }),
    };

    merge_attributes(vec![base, attributes])
}

/// Styled wrapper for the context menu root.
#[component]
pub fn ContextMenu(props: ContextMenuProps) -> Element {
    provide_styled_menu_surface(StyledMenuSurface::Context);

    let attributes = merge_with_class("div", Styles::dx_context_menu.to_string(), props.attributes);

    rsx! {
        context_menu::ContextMenu {
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            roving_loop: props.roving_loop,
            attributes,
            {props.children}
        }
    }
}

/// Styled wrapper for the context menu trigger.
#[component]
pub fn ContextMenuTrigger(props: ContextMenuTriggerProps) -> Element {
    let attributes = merge_with_class(
        "button",
        Styles::dx_context_menu_trigger.to_string(),
        props.attributes,
    );

    rsx! {
        context_menu::ContextMenuTrigger {
            attributes,
            {props.children}
        }
    }
}
