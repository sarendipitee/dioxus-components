use crate::component_styles;
use crate::components::menu::{provide_styled_menu_surface, Menu, StyledMenuSurface};
use dioxus::prelude::*;
use dioxus_primitives::menubar::{
    self, MenubarContentProps, MenubarMenuProps, MenubarProps, MenubarTriggerProps,
};
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

/// Styled wrapper for the menubar root.
#[component]
pub fn Menubar(props: MenubarProps) -> Element {
    let attributes = merge_with_class("div", Styles::dx_menubar.to_string(), props.attributes);

    rsx! {
        menubar::Menubar {
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes,
            {props.children}
        }
    }
}

/// Styled wrapper for an individual menubar menu root.
#[component]
pub fn MenubarMenu(props: MenubarMenuProps) -> Element {
    provide_styled_menu_surface(StyledMenuSurface::Menubar);

    let attributes = merge_with_class("div", Styles::dx_menubar_menu.to_string(), props.attributes);

    rsx! {
        menubar::MenubarMenu {
            index: props.index,
            disabled: props.disabled,
            attributes,
            {props.children}
        }
    }
}

/// Styled wrapper for a menubar trigger.
#[component]
pub fn MenubarTrigger(props: MenubarTriggerProps) -> Element {
    let attributes = merge_with_class(
        "button",
        Styles::dx_menubar_trigger.to_string(),
        props.attributes,
    );

    rsx! {
        menubar::MenubarTrigger { attributes, {props.children} }
    }
}

/// Styled wrapper for menubar popup content.
#[component]
pub fn MenubarContent(props: MenubarContentProps) -> Element {
    rsx! {
        Menu {
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}
