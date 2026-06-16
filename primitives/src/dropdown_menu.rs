//! Defines the [`DropdownMenu`] component and its subcomponents.

use crate::{
    menu::{self, MenuContext},
    merge_attributes, use_controlled, use_unique_id,
};
use dioxus::prelude::*;
use dioxus_attributes::attributes;

/// The props for the [`DropdownMenu`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuProps {
    /// Whether the dropdown menu is open. If not provided, the component will be uncontrolled and use `default_open`.
    pub open: ReadSignal<Option<bool>>,
    /// Default open state if the component is not controlled.
    #[props(default)]
    pub default_open: bool,
    /// Callback when the open state changes. This is called when the dropdown menu is opened or closed.
    #[props(default)]
    pub on_open_change: Callback<bool>,
    /// Whether the dropdown menu is disabled. If true, the menu will not open and items will not be selectable.
    #[props(default)]
    pub disabled: ReadSignal<bool>,
    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,
    /// Additional attributes to apply to the dropdown menu element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the dropdown menu, which should include a [`DropdownMenuTrigger`] and a [`DropdownMenuContent`].
    pub children: Element,
}

/// A menu opened from a trigger element.
#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);
    let mut ctx = menu::use_menu_provider(open, set_open, props.disabled, props.roving_loop);
    let root_id = use_unique_id();

    menu::use_menu_outside_dismiss(root_id, ctx, true);

    let handle_keydown = move |event: Event<KeyboardData>| {
        if (props.disabled)() {
            return;
        }

        match event.key() {
            Key::Enter => ctx.set_open.call(!open()),
            Key::Escape => ctx.set_open.call(false),
            Key::ArrowDown => ctx.focus.focus_next(),
            Key::ArrowUp => {
                if open() {
                    ctx.focus.focus_prev();
                }
            }
            Key::Home => ctx.focus.focus_first(),
            Key::End => ctx.focus.focus_last(),
            Key::Tab => {
                ctx.focus.blur();
                ctx.set_open.call(false);
            }
            _ => return,
        }
        event.prevent_default();
    };

    let base = attributes!(div {
        id: root_id,
        onkeydown: handle_keydown,
    });
    let attributes = merge_attributes(vec![base, props.attributes]);
    rsx! {
        div {
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": (props.disabled)(),
            ..attributes,
            {props.children}
        }
    }
}

/// The props for the [`DropdownMenuTrigger`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuTriggerProps {
    /// Render the trigger element as a custom component/element.
    #[props(default)]
    pub r#as: Option<Callback<Vec<Attribute>, Element>>,
    /// Additional attributes to apply to the trigger element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the trigger.
    pub children: Element,
}

/// The trigger wrapper for the parent [`DropdownMenu`].
#[component]
pub fn DropdownMenuTrigger(props: DropdownMenuTriggerProps) -> Element {
    let mut ctx: MenuContext = use_context();
    let open = ctx.open;
    let disabled = ctx.disabled;

    let base = attributes!(span {
        id: ctx.trigger_id,
        "data-state": if open() { "open" } else { "closed" },
        "data-disabled": disabled,
        aria_disabled: disabled,
        aria_expanded: open,
        aria_haspopup: "listbox",
        onclick: move |_| {
            if !disabled() {
                ctx.set_open.call(!open());
            }
        },
        onblur: move |_| {
            if !ctx.focus.any_focused() {
                ctx.focus.blur();
                ctx.set_open.call(false);
            }
        },
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    if let Some(dynamic) = props.r#as {
        dynamic.call(attributes)
    } else {
        rsx! {
            span {
                ..attributes,
                {props.children}
            }
        }
    }
}

/// The props for the [`DropdownMenuContent`] component.
pub type DropdownMenuContentProps = menu::MenuContentProps;

/// The contents of a [`DropdownMenu`].
#[component]
pub fn DropdownMenuContent(props: DropdownMenuContentProps) -> Element {
    rsx! {
        menu::MenuContent {
            id: props.id,
            role: "listbox",
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DropdownMenuItem`] component.
pub type DropdownMenuItemProps<T> = menu::MenuItemProps<T>;

/// An item within a [`DropdownMenuContent`].
#[component]
pub fn DropdownMenuItem<T: Clone + PartialEq + 'static>(
    props: DropdownMenuItemProps<T>,
) -> Element {
    rsx! {
        menu::MenuItem {
            value: props.value,
            index: props.index,
            disabled: props.disabled,
            role: "option",
            on_select: props.on_select,
            attributes: props.attributes,
            {props.children}
        }
    }
}
