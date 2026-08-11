use crate::component_styles;
use crate::components::{
    button::{Button, ButtonSize, ButtonVariant},
    menu::{provide_styled_menu_surface, Menu, StyledMenuSurface},
};
use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::dropdown_menu::{self, DropdownMenuContentProps, DropdownMenuProps};
use dioxus_primitives::merge_attributes;

#[component_styles("./style.css")]
pub(crate) struct Styles;

fn merge_with_class(tag: &str, class_name: &str, attributes: Vec<Attribute>) -> Vec<Attribute> {
    let base = match tag {
        "button" => attributes!(button { class: class_name }),
        _ => attributes!(div { class: class_name }),
    };

    merge_attributes(vec![base, attributes])
}

/// Styled wrapper for the dropdown menu root.
#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    provide_styled_menu_surface(StyledMenuSurface::Dropdown);

    let attributes = merge_with_class("div", Styles::dx_dropdown_menu, props.attributes);

    rsx! {
        dropdown_menu::DropdownMenu {
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes,
            {props.children}
        }
    }
}
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuTriggerProps {
    #[props(default)]
    pub variant: ButtonVariant,
    #[props(default)]
    pub size: ButtonSize,
    #[props(default)]
    pub r#as: Option<Callback<Vec<Attribute>, Element>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
}

/// Styled wrapper for the dropdown menu trigger.
#[component]
pub fn DropdownMenuTrigger(props: DropdownMenuTriggerProps) -> Element {
    let renderer = if let Some(renderer) = props.r#as {
        renderer
    } else {
        let renderer_children = props.children.clone();
        let variant = props.variant;
        let size = props.size;
        Callback::new(move |attributes| {
            let children = renderer_children.clone();
            rsx! {
                Button { variant, size, attributes, {children} }
            }
        })
    };

    rsx! {
        dropdown_menu::DropdownMenuTrigger {
            as: renderer,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// Styled wrapper for the dropdown menu content surface.
#[component]
pub fn DropdownMenuContent(props: DropdownMenuContentProps) -> Element {
    rsx! {
        Menu {
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}
