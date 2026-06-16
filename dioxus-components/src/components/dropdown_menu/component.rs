use crate::component_styles;
use crate::components::menu::{provide_styled_menu_surface, StyledMenuSurface};
use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::dropdown_menu::{self, DropdownMenuProps, DropdownMenuTriggerProps};
use dioxus_primitives::merge_attributes;

#[component_styles("./style.css")]
pub(crate) struct Styles;

fn merge_with_class(tag: &str, class_name: String, attributes: Vec<Attribute>) -> Vec<Attribute> {
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

    let attributes = merge_with_class(
        "div",
        Styles::dx_dropdown_menu.to_string(),
        props.attributes,
    );

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

/// Styled wrapper for the dropdown menu trigger.
#[component]
pub fn DropdownMenuTrigger(props: DropdownMenuTriggerProps) -> Element {
    rsx! {
        dropdown_menu::DropdownMenuTrigger {
            as: props.r#as,
            attributes: props.attributes,
            {props.children}
        }
    }
}
