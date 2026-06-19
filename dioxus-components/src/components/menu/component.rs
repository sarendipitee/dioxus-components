use crate::component_styles;
use crate::components::context_menu::ContextMenuStyles;
use crate::components::dropdown_menu::DropdownMenuStyles;
use crate::components::menubar::MenubarStyles;
use crate::components::popover::PopoverStyles;
use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::menu::{
    self, MenuCheckboxItemProps, MenuGroupProps, MenuItemIndicatorProps, MenuItemProps,
    MenuItemSectionProps, MenuLabelProps, MenuRadioGroupProps, MenuRadioItemProps,
    MenuSeparatorProps, MenuSubContentProps, MenuSubProps, MenuSubTriggerProps,
};
use dioxus_primitives::{context_menu, dropdown_menu, menubar, merge_attributes};

#[component_styles("./style.css")]
struct Styles;

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum StyledMenuSurface {
    Dropdown,
    Context,
    Menubar,
}

pub(crate) fn provide_styled_menu_surface(surface: StyledMenuSurface) {
    use_context_provider(|| surface);
}

#[derive(Props, Clone, PartialEq)]
pub struct MenuProps {
    /// The ID of the menu content element. If not provided, a unique ID will be generated.
    pub id: ReadSignal<Option<String>>,
    /// Additional attributes to apply to the menu content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the menu content.
    pub children: Element,
}

fn merge_with_class(tag: &str, class_name: String, attributes: Vec<Attribute>) -> Vec<Attribute> {
    let base = match tag {
        "button" => attributes!(button { class: class_name }),
        _ => attributes!(div { class: class_name }),
    };

    merge_attributes(vec![base, attributes])
}

fn merge_with_surface_class(
    tag: &str,
    shared_class: String,
    surface_slot: MenuSurfaceSlot,
    attributes: Vec<Attribute>,
) -> Vec<Attribute> {
    let class_name = match current_surface_slot_class(surface_slot) {
        Some(surface_class) => format!("{shared_class} {surface_class}"),
        None => shared_class,
    };

    merge_with_class(tag, class_name, attributes)
}

fn current_surface_slot_class(slot: MenuSurfaceSlot) -> Option<String> {
    try_use_context::<StyledMenuSurface>().map(|surface| surface_slot_class(surface, slot))
}

fn surface_slot_class(surface: StyledMenuSurface, slot: MenuSurfaceSlot) -> String {
    match surface {
        StyledMenuSurface::Dropdown => match slot {
            MenuSurfaceSlot::Content => DropdownMenuStyles::dx_dropdown_menu_content.to_string(),
            MenuSurfaceSlot::Item => DropdownMenuStyles::dx_dropdown_menu_item.to_string(),
            MenuSurfaceSlot::Label => DropdownMenuStyles::dx_dropdown_menu_label.to_string(),
            MenuSurfaceSlot::Separator => {
                DropdownMenuStyles::dx_dropdown_menu_separator.to_string()
            }
            MenuSurfaceSlot::Indicator => {
                DropdownMenuStyles::dx_dropdown_menu_item_indicator.to_string()
            }
            MenuSurfaceSlot::ItemSection => {
                DropdownMenuStyles::dx_dropdown_menu_item_section.to_string()
            }
            MenuSurfaceSlot::CheckableItem => {
                DropdownMenuStyles::dx_dropdown_menu_checkable_item.to_string()
            }
            MenuSurfaceSlot::Sub => DropdownMenuStyles::dx_dropdown_menu_sub.to_string(),
            MenuSurfaceSlot::SubTrigger => {
                DropdownMenuStyles::dx_dropdown_menu_sub_trigger.to_string()
            }
            MenuSurfaceSlot::SubContent => {
                DropdownMenuStyles::dx_dropdown_menu_sub_content.to_string()
            }
        },
        StyledMenuSurface::Context => match slot {
            MenuSurfaceSlot::Content => ContextMenuStyles::dx_context_menu_content.to_string(),
            MenuSurfaceSlot::Item => ContextMenuStyles::dx_context_menu_item.to_string(),
            MenuSurfaceSlot::Label => ContextMenuStyles::dx_context_menu_label.to_string(),
            MenuSurfaceSlot::Separator => ContextMenuStyles::dx_context_menu_separator.to_string(),
            MenuSurfaceSlot::Indicator => {
                ContextMenuStyles::dx_context_menu_item_indicator.to_string()
            }
            MenuSurfaceSlot::ItemSection => {
                ContextMenuStyles::dx_context_menu_item_section.to_string()
            }
            MenuSurfaceSlot::CheckableItem => {
                ContextMenuStyles::dx_context_menu_checkable_item.to_string()
            }
            MenuSurfaceSlot::Sub => ContextMenuStyles::dx_context_menu_sub.to_string(),
            MenuSurfaceSlot::SubTrigger => {
                ContextMenuStyles::dx_context_menu_sub_trigger.to_string()
            }
            MenuSurfaceSlot::SubContent => {
                ContextMenuStyles::dx_context_menu_sub_content.to_string()
            }
        },
        StyledMenuSurface::Menubar => match slot {
            MenuSurfaceSlot::Content => MenubarStyles::dx_menubar_content.to_string(),
            MenuSurfaceSlot::Item => MenubarStyles::dx_menubar_item.to_string(),
            MenuSurfaceSlot::Label => MenubarStyles::dx_menubar_label.to_string(),
            MenuSurfaceSlot::Separator => MenubarStyles::dx_menubar_separator.to_string(),
            MenuSurfaceSlot::Indicator => MenubarStyles::dx_menubar_item_indicator.to_string(),
            MenuSurfaceSlot::ItemSection => MenubarStyles::dx_menubar_item_section.to_string(),
            MenuSurfaceSlot::CheckableItem => MenubarStyles::dx_menubar_checkable_item.to_string(),
            MenuSurfaceSlot::Sub => MenubarStyles::dx_menubar_sub.to_string(),
            MenuSurfaceSlot::SubTrigger => MenubarStyles::dx_menubar_sub_trigger.to_string(),
            MenuSurfaceSlot::SubContent => MenubarStyles::dx_menubar_sub_content.to_string(),
        },
    }
}

#[derive(Clone, Copy)]
enum MenuSurfaceSlot {
    Content,
    Item,
    Label,
    Separator,
    Indicator,
    ItemSection,
    CheckableItem,
    Sub,
    SubTrigger,
    SubContent,
}

/// Styled shared popup content for dropdown menus, context menus, and menubars.
#[component]
pub fn Menu(props: MenuProps) -> Element {
    let attributes = merge_with_surface_class(
        "div",
        format!(
            "{} {}",
            Styles::dx_menu_content,
            PopoverStyles::dx_popover_surface
        ),
        MenuSurfaceSlot::Content,
        props.attributes,
    );

    match try_use_context::<StyledMenuSurface>() {
        Some(StyledMenuSurface::Context) => rsx! {
            context_menu::ContextMenuContent {
                id: props.id,
                attributes,
                {props.children}
            }
        },
        Some(StyledMenuSurface::Menubar) => rsx! {
            menubar::MenubarContent {
                id: props.id,
                attributes,
                {props.children}
            }
        },
        Some(StyledMenuSurface::Dropdown) | None => rsx! {
            dropdown_menu::DropdownMenuContent {
                id: props.id,
                attributes,
                {props.children}
            }
        },
    }
}

/// Styled shared selectable menu item.
#[component]
pub fn MenuItem<T: Clone + PartialEq + 'static>(props: MenuItemProps<T>) -> Element {
    let attributes = merge_with_surface_class(
        "div",
        Styles::dx_menu_item.to_string(),
        MenuSurfaceSlot::Item,
        props.attributes,
    );

    rsx! {
        menu::MenuItem {
            value: props.value,
            index: props.index,
            disabled: props.disabled,
            role: props.role,
            on_select: props.on_select,
            close_on_select: props.close_on_select,
            attributes,
            {props.children}
        }
    }
}

/// Styled shared non-interactive menu label.
#[component]
pub fn MenuLabel(props: MenuLabelProps) -> Element {
    let attributes = merge_with_surface_class(
        "div",
        Styles::dx_menu_label.to_string(),
        MenuSurfaceSlot::Label,
        props.attributes,
    );

    rsx! {
        menu::MenuLabel {
            attributes,
            {props.children}
        }
    }
}

/// Styled shared menu separator.
#[component]
pub fn MenuSeparator(props: MenuSeparatorProps) -> Element {
    let attributes = merge_with_surface_class(
        "div",
        Styles::dx_menu_separator.to_string(),
        MenuSurfaceSlot::Separator,
        props.attributes,
    );

    rsx! {
        menu::MenuSeparator { attributes }
    }
}

/// Styled shared semantic menu group.
#[component]
pub fn MenuGroup(props: MenuGroupProps) -> Element {
    rsx! {
        menu::MenuGroup {
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// Styled shared presentational indicator for checked menu items.
#[component]
pub fn MenuItemIndicator(props: MenuItemIndicatorProps) -> Element {
    let attributes = merge_with_surface_class(
        "div",
        Styles::dx_menu_item_indicator.to_string(),
        MenuSurfaceSlot::Indicator,
        props.attributes,
    );

    rsx! {
        menu::MenuItemIndicator {
            visible: props.visible,
            attributes,
            {props.children}
        }
    }
}

/// Styled shared right-aligned section inside a menu item.
#[component]
pub fn MenuItemSection(props: MenuItemSectionProps) -> Element {
    let attributes = merge_with_surface_class(
        "div",
        Styles::dx_menu_item_section.to_string(),
        MenuSurfaceSlot::ItemSection,
        props.attributes,
    );

    rsx! {
        menu::MenuItemSection {
            attributes,
            {props.children}
        }
    }
}

/// Styled shared checkbox-style menu item.
#[component]
pub fn MenuCheckboxItem<T: Clone + PartialEq + 'static>(
    props: MenuCheckboxItemProps<T>,
) -> Element {
    let attributes = merge_with_class(
        "div",
        format!(
            "{} {}{}{}",
            Styles::dx_menu_item,
            Styles::dx_menu_checkable_item,
            current_surface_slot_class(MenuSurfaceSlot::Item)
                .map(|class_name| format!(" {class_name}"))
                .unwrap_or_default(),
            current_surface_slot_class(MenuSurfaceSlot::CheckableItem)
                .map(|class_name| format!(" {class_name}"))
                .unwrap_or_default(),
        ),
        props.attributes,
    );

    rsx! {
        menu::MenuCheckboxItem {
            value: props.value,
            index: props.index,
            checked: props.checked,
            disabled: props.disabled,
            on_checked_change: props.on_checked_change,
            on_select: props.on_select,
            close_on_select: props.close_on_select,
            attributes,
            {props.children}
        }
    }
}

/// Styled shared radio group for menu radio items.
#[component]
pub fn MenuRadioGroup<T: Clone + PartialEq + 'static>(props: MenuRadioGroupProps<T>) -> Element {
    rsx! {
        menu::MenuRadioGroup {
            value: props.value,
            on_value_change: props.on_value_change,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// Styled shared radio-style menu item.
#[component]
pub fn MenuRadioItem<T: Clone + PartialEq + 'static>(props: MenuRadioItemProps<T>) -> Element {
    let attributes = merge_with_class(
        "div",
        format!(
            "{} {}{}{}",
            Styles::dx_menu_item,
            Styles::dx_menu_checkable_item,
            current_surface_slot_class(MenuSurfaceSlot::Item)
                .map(|class_name| format!(" {class_name}"))
                .unwrap_or_default(),
            current_surface_slot_class(MenuSurfaceSlot::CheckableItem)
                .map(|class_name| format!(" {class_name}"))
                .unwrap_or_default(),
        ),
        props.attributes,
    );

    rsx! {
        menu::MenuRadioItem {
            value: props.value,
            index: props.index,
            disabled: props.disabled,
            on_select: props.on_select,
            close_on_select: props.close_on_select,
            attributes,
            {props.children}
        }
    }
}

/// Styled shared submenu root.
#[component]
pub fn MenuSub(props: MenuSubProps) -> Element {
    let attributes = merge_with_surface_class(
        "div",
        Styles::dx_menu_sub.to_string(),
        MenuSurfaceSlot::Sub,
        props.attributes,
    );

    rsx! {
        menu::MenuSub {
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

/// Styled shared menu item that opens a submenu.
#[component]
pub fn MenuSubTrigger<T: Clone + PartialEq + 'static>(props: MenuSubTriggerProps<T>) -> Element {
    let attributes = merge_with_class(
        "div",
        format!(
            "{} {}{}{}",
            Styles::dx_menu_item,
            Styles::dx_menu_sub_trigger,
            current_surface_slot_class(MenuSurfaceSlot::Item)
                .map(|class_name| format!(" {class_name}"))
                .unwrap_or_default(),
            current_surface_slot_class(MenuSurfaceSlot::SubTrigger)
                .map(|class_name| format!(" {class_name}"))
                .unwrap_or_default(),
        ),
        props.attributes,
    );

    rsx! {
        menu::MenuSubTrigger {
            value: props.value,
            index: props.index,
            disabled: props.disabled,
            on_select: props.on_select,
            attributes,
            {props.children}
        }
    }
}

/// Styled shared popup content for a nested submenu.
#[component]
pub fn MenuSubContent(props: MenuSubContentProps) -> Element {
    let attributes = merge_with_class(
        "div",
        format!(
            "{} {} {}{}",
            Styles::dx_menu_content,
            PopoverStyles::dx_popover_surface,
            Styles::dx_menu_sub_content,
            current_surface_slot_class(MenuSurfaceSlot::SubContent)
                .map(|class_name| format!(" {class_name}"))
                .unwrap_or_default(),
        ),
        props.attributes,
    );

    rsx! {
        menu::MenuSubContent {
            id: props.id,
            role: props.role,
            attributes,
            {props.children}
        }
    }
}
