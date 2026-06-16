use crate::component_styles;
use dioxus::prelude::*;
use dioxus_primitives::menubar::{
    self, MenubarContentProps, MenubarItemProps, MenubarMenuProps, MenubarProps,
    MenubarTriggerProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
#[component_styles("./style.css")]
struct Styles;

#[component]
pub fn Menubar(props: MenubarProps) -> Element {
    rsx! {
        menubar::Menubar {
            class: Styles::dx_menubar.to_string(),
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn MenubarMenu(props: MenubarMenuProps) -> Element {
    rsx! {
        menubar::MenubarMenu {
            class: Styles::dx_menubar_menu.to_string(),
            index: props.index,
            disabled: props.disabled,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn MenubarTrigger(props: MenubarTriggerProps) -> Element {
    let base = attributes!(button {
        class: Styles::dx_menubar_trigger,
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        menubar::MenubarTrigger { attributes, {props.children} }
    }
}

#[component]
pub fn MenubarContent(props: MenubarContentProps) -> Element {
    rsx! {
        menubar::MenubarContent {
            class: Styles::dx_menubar_content.to_string(),
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn MenubarItem(props: MenubarItemProps) -> Element {
    rsx! {
        menubar::MenubarItem {
            class: Styles::dx_menubar_item.to_string(),
            index: props.index,
            value: props.value,
            disabled: props.disabled,
            on_select: props.on_select,
            attributes: props.attributes,
            {props.children}
        }
    }
}
