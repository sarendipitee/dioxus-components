use crate::component_styles;
use crate::components::context_menu::ContextMenuStyles;
use crate::components::dropdown_menu::DropdownMenuStyles;
use crate::components::input::{InputVariant, TextInput};
use crate::components::menubar::MenubarStyles;
use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::menu::{
    self, FilterableMenuContentProps as PrimitiveFilterableMenuContentProps,
    FilterableMenuInputProps as PrimitiveFilterableMenuInputProps, MenuCheckboxItemProps,
    MenuGroupProps, MenuItemIndicatorProps, MenuItemProps, MenuItemSectionProps, MenuLabelProps,
    MenuRadioGroupProps, MenuRadioItemProps, MenuSeparatorProps, MenuSubContentProps, MenuSubProps,
    MenuSubTriggerProps,
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
    #[props(default)]
    pub id: Option<String>,
    /// Additional attributes to apply to the menu content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the menu content.
    pub children: Element,
}
/// Props forwarded to the styled filterable content rendered inside an existing menu.
pub type FilterableMenuContentProps = PrimitiveFilterableMenuContentProps;

/// Props forwarded to the styled filter input rendered by [`FilterableMenu`].
pub type FilterableMenuInputProps = PrimitiveFilterableMenuInputProps;

/// Props for [`FilterableMenu`].
#[derive(Props, Clone, PartialEq)]
pub struct FilterableMenuProps {
    /// Whether the menu is open.
    pub open: Memo<bool>,
    /// Callback to set the open state.
    pub set_open: Callback<bool>,
    /// Whether the menu and its items are disabled.
    pub disabled: ReadSignal<bool>,
    /// Whether focus should loop around when reaching the end.
    pub roving_loop: ReadSignal<bool>,
    /// Props forwarded to the filter text input.
    #[props(default)]
    pub filter_input_props: FilterableMenuInputProps,
    /// Additional attributes for the menu root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// Menu items and other menu content.
    pub children: Element,
}

/// A styled filterable menu with a text input above its menu items.
#[component]
pub fn FilterableMenu(props: FilterableMenuProps) -> Element {
    let filter_input_props = styled_filter_input_props(props.filter_input_props);

    rsx! {
        menu::FilterableMenu {
            open: props.open,
            set_open: props.set_open,
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            filter_input_props,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// Styled filterable content for an existing menu surface.
#[component]
pub fn FilterableMenuContent(props: FilterableMenuContentProps) -> Element {
    let filter_input_props = styled_filter_input_props(props.filter_input_props);

    rsx! {
        menu::FilterableMenuContent {
            filter_input_props,
            {props.children}
        }
    }
}

fn styled_filter_input_props(
    props: PrimitiveFilterableMenuInputProps,
) -> PrimitiveFilterableMenuInputProps {
    let attributes = merge_attributes(vec![
        attributes!(input {
            placeholder: "Filter...",
            aria_label: "Filter menu items",
        }),
        props.attributes,
    ]);

    PrimitiveFilterableMenuInputProps {
        oninput: props.oninput,
        onmounted: props.onmounted,
        onkeydown: props.onkeydown,
        r#as: Some(Callback::new(|attributes: Vec<Attribute>| {
            rsx! {
                TextInput {
                    variant: InputVariant::Unstyled,
                    attributes,
                }
            }
        })),
        attributes,
    }
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
    shared_class: impl Into<String>,
    surface_slot: MenuSurfaceSlot,
    attributes: Vec<Attribute>,
) -> Vec<Attribute> {
    let shared_class = shared_class.into();
    let class_name = match current_surface_slot_class(surface_slot) {
        Some(surface_class) => format!("{shared_class} {surface_class}"),
        None => shared_class,
    };

    merge_with_class(tag, class_name, attributes)
}

fn current_surface_slot_class(slot: MenuSurfaceSlot) -> Option<&'static str> {
    try_use_context::<StyledMenuSurface>().map(|surface| surface_slot_class(surface, slot))
}

fn surface_slot_class(surface: StyledMenuSurface, slot: MenuSurfaceSlot) -> &'static str {
    match surface {
        StyledMenuSurface::Dropdown => match slot {
            MenuSurfaceSlot::Content => DropdownMenuStyles::dx_dropdown_menu_content,
            MenuSurfaceSlot::Item => DropdownMenuStyles::dx_dropdown_menu_item,
            MenuSurfaceSlot::Label => DropdownMenuStyles::dx_dropdown_menu_label,
            MenuSurfaceSlot::Separator => DropdownMenuStyles::dx_dropdown_menu_separator,
            MenuSurfaceSlot::Indicator => DropdownMenuStyles::dx_dropdown_menu_item_indicator,
            MenuSurfaceSlot::ItemSection => DropdownMenuStyles::dx_dropdown_menu_item_section,
            MenuSurfaceSlot::CheckableItem => DropdownMenuStyles::dx_dropdown_menu_checkable_item,
            MenuSurfaceSlot::Sub => DropdownMenuStyles::dx_dropdown_menu_sub,
            MenuSurfaceSlot::SubTrigger => DropdownMenuStyles::dx_dropdown_menu_sub_trigger,
            MenuSurfaceSlot::SubContent => DropdownMenuStyles::dx_dropdown_menu_sub_content,
        },
        StyledMenuSurface::Context => match slot {
            MenuSurfaceSlot::Content => ContextMenuStyles::dx_context_menu_content,
            MenuSurfaceSlot::Item => ContextMenuStyles::dx_context_menu_item,
            MenuSurfaceSlot::Label => ContextMenuStyles::dx_context_menu_label,
            MenuSurfaceSlot::Separator => ContextMenuStyles::dx_context_menu_separator,
            MenuSurfaceSlot::Indicator => ContextMenuStyles::dx_context_menu_item_indicator,
            MenuSurfaceSlot::ItemSection => ContextMenuStyles::dx_context_menu_item_section,
            MenuSurfaceSlot::CheckableItem => ContextMenuStyles::dx_context_menu_checkable_item,
            MenuSurfaceSlot::Sub => ContextMenuStyles::dx_context_menu_sub,
            MenuSurfaceSlot::SubTrigger => ContextMenuStyles::dx_context_menu_sub_trigger,
            MenuSurfaceSlot::SubContent => ContextMenuStyles::dx_context_menu_sub_content,
        },
        StyledMenuSurface::Menubar => match slot {
            MenuSurfaceSlot::Content => MenubarStyles::dx_menubar_content,
            MenuSurfaceSlot::Item => MenubarStyles::dx_menubar_item,
            MenuSurfaceSlot::Label => MenubarStyles::dx_menubar_label,
            MenuSurfaceSlot::Separator => MenubarStyles::dx_menubar_separator,
            MenuSurfaceSlot::Indicator => MenubarStyles::dx_menubar_item_indicator,
            MenuSurfaceSlot::ItemSection => MenubarStyles::dx_menubar_item_section,
            MenuSurfaceSlot::CheckableItem => MenubarStyles::dx_menubar_checkable_item,
            MenuSurfaceSlot::Sub => MenubarStyles::dx_menubar_sub,
            MenuSurfaceSlot::SubTrigger => MenubarStyles::dx_menubar_sub_trigger,
            MenuSurfaceSlot::SubContent => MenubarStyles::dx_menubar_sub_content,
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
        format!("{} dx_dropdown", Styles::dx_menu_content),
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
        Styles::dx_menu_item,
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
            search_text: props.search_text,
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
        Styles::dx_menu_label,
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
        Styles::dx_menu_separator,
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
        Styles::dx_menu_item_indicator,
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
        Styles::dx_menu_item_section,
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
            search_text: props.search_text,
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
            search_text: props.search_text,
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
        Styles::dx_menu_sub,
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
            search_text: props.search_text,
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
            "{} dx_dropdown {}{}{}",
            Styles::dx_menu_content,
            current_surface_slot_class(MenuSurfaceSlot::Content)
                .map(|class_name| format!(" {class_name}"))
                .unwrap_or_default(),
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
