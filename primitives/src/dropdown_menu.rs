//! Defines the [`DropdownMenu`] component and its subcomponents.

use std::rc::Rc;

use crate::{
    floating::{style_prop, use_position},
    focus::{use_deferred_focus, FocusPlacement},
    menu::{self, MenuContext},
    merge_attributes, use_controlled, use_unique_id, ContentAlign, ContentSide,
};
use dioxus::prelude::*;
use dioxus_attributes::attributes;

#[derive(Clone, Copy)]
struct DropdownMenuContext {
    initial_focus: Signal<Option<FocusPlacement>>,
}

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
    let mut initial_focus = ctx.initial_focus;

    use_context_provider(|| DropdownMenuContext { initial_focus });

    // Outside-click / Escape dismissal is now owned by the overlay manager's central
    // dismiss stack (the menu panel registers as a Floating entry via MenuContent),
    // so the per-component `use_menu_outside_dismiss` is no longer needed.

    let handle_keydown = move |event: Event<KeyboardData>| {
        if (props.disabled)() {
            return;
        }

        match event.key() {
            Key::Enter => ctx.set_open.call(!open()),
            Key::Escape => ctx.set_open.call(false),
            Key::ArrowDown => {
                if open() {
                    if ctx.focus.any_focused() {
                        ctx.focus.focus_next();
                    } else {
                        initial_focus.set(Some(FocusPlacement::First));
                    }
                } else {
                    initial_focus.set(Some(FocusPlacement::First));
                    ctx.set_open.call(true);
                }
            }
            Key::ArrowUp => {
                if open() {
                    if ctx.focus.any_focused() {
                        ctx.focus.focus_prev();
                    } else {
                        initial_focus.set(Some(FocusPlacement::Last));
                    }
                } else {
                    initial_focus.set(Some(FocusPlacement::Last));
                    ctx.set_open.call(true);
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
    let mut dropdown_ctx: DropdownMenuContext = use_context();
    let open = ctx.open;
    let disabled = ctx.disabled;

    let mut trigger_ref = ctx.trigger_ref;
    let base = attributes!(span {
        id: ctx.trigger_id,
        "data-state": if open() { "open" } else { "closed" },
        "data-disabled": disabled,
        aria_disabled: disabled,
        aria_expanded: open,
        aria_haspopup: "menu",
        onmounted: move |evt: MountedEvent| trigger_ref.set(Some(evt.data())),
        onclick: move |_| {
            if !disabled() {
                ctx.set_open.call(!open());
            }
        },
        onkeydown: move |event: Event<KeyboardData>| {
            if disabled() {
                return;
            }

            match event.key() {
                Key::ArrowDown => {
                    if !open() {
                        dropdown_ctx.initial_focus.set(Some(FocusPlacement::First));
                        ctx.set_open.call(true);
                    } else if ctx.focus.any_focused() {
                        ctx.focus.focus_next();
                    } else {
                        dropdown_ctx.initial_focus.set(Some(FocusPlacement::First));
                    }
                }
                Key::ArrowUp => {
                    if !open() {
                        dropdown_ctx.initial_focus.set(Some(FocusPlacement::Last));
                        ctx.set_open.call(true);
                    } else if ctx.focus.any_focused() {
                        ctx.focus.focus_prev();
                    } else {
                        dropdown_ctx.initial_focus.set(Some(FocusPlacement::Last));
                    }
                }
                _ => return,
            }

            event.prevent_default();
            event.stop_propagation();
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
    let dropdown_ctx: DropdownMenuContext = use_context();
    let mut menu_ctx: MenuContext = use_context();
    let open = menu_ctx.open;
    let mut menu_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let focused = move || open() && !menu_ctx.focus.any_focused();

    use_deferred_focus(menu_ctx.focus, dropdown_ctx.initial_focus, move || {
        (menu_ctx.open)()
    });
    use_effect(move || {
        let Some(menu) = menu_ref() else {
            return;
        };
        if focused() {
            spawn(async move {
                _ = menu.set_focus(true).await;
            });
        }
    });

    // Floating-element positioning. The trigger ref is shared via the menu context;
    // the content ref (`menu_ref`, also used for focus) doubles as the floating
    // element. A dropdown naturally opens below its trigger, aligned to the left edge
    // (matching the legacy `top:100% left:0` CSS); flip()/shift() handle viewport
    // edges. On native the hook is inert and the `:not([data-floating])` CSS fallback
    // provides the static placement.
    let pos = use_position(
        menu_ctx.trigger_ref,
        menu_ref,
        ContentSide::Bottom,
        ContentAlign::Start,
    );

    let style = pos.style;
    let is_positioned = pos.is_positioned;
    let resolved_side = pos.side;
    let resolved_align = pos.align;
    let floating_active = pos.floating_active;

    let position = use_memo(move || style_prop(&style.read(), "position"));
    let top = use_memo(move || style_prop(&style.read(), "top"));
    let left = use_memo(move || style_prop(&style.read(), "left"));
    let visibility = use_memo(move || if is_positioned() { "visible" } else { "hidden" });

    let base = attributes!(div {
        tabindex: if focused() { "0" } else { "-1" },
        onblur: move |_| {
            if focused() {
                menu_ctx.focus.blur();
            }
        },
        position: position(),
        top: top(),
        left: left(),
        visibility: visibility(),
        "data-side": resolved_side.read().as_str(),
        "data-align": resolved_align.read().as_str(),
        "data-floating": floating_active.then_some("true"),
        onmounted: move |evt| menu_ref.set(Some(evt.data())),
    });
    // Floating props must win over user-forwarded coords → place `base` last.
    let attributes = merge_attributes(vec![props.attributes, base]);

    rsx! {
        menu::MenuContent {
            id: props.id,
            role: "menu",
            attributes,
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
            role: "menuitem",
            on_select: props.on_select,
            close_on_select: props.close_on_select,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DropdownMenuLabel`] component.
pub type DropdownMenuLabelProps = menu::MenuLabelProps;

/// A non-interactive label within a [`DropdownMenuContent`].
#[component]
pub fn DropdownMenuLabel(props: DropdownMenuLabelProps) -> Element {
    rsx! {
        menu::MenuLabel {
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DropdownMenuSeparator`] component.
pub type DropdownMenuSeparatorProps = menu::MenuSeparatorProps;

/// A separator between groups of dropdown menu items.
#[component]
pub fn DropdownMenuSeparator(props: DropdownMenuSeparatorProps) -> Element {
    rsx! {
        menu::MenuSeparator {
            attributes: props.attributes,
        }
    }
}

/// The props for the [`DropdownMenuGroup`] component.
pub type DropdownMenuGroupProps = menu::MenuGroupProps;

/// A semantic group of related dropdown menu items.
#[component]
pub fn DropdownMenuGroup(props: DropdownMenuGroupProps) -> Element {
    rsx! {
        menu::MenuGroup {
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DropdownMenuItemIndicator`] component.
pub type DropdownMenuItemIndicatorProps = menu::MenuItemIndicatorProps;

/// A presentational indicator for checked dropdown menu items.
#[component]
pub fn DropdownMenuItemIndicator(props: DropdownMenuItemIndicatorProps) -> Element {
    rsx! {
        menu::MenuItemIndicator {
            visible: props.visible,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DropdownMenuItemSection`] component.
pub type DropdownMenuItemSectionProps = menu::MenuItemSectionProps;

/// A presentational section inside a dropdown menu item.
#[component]
pub fn DropdownMenuItemSection(props: DropdownMenuItemSectionProps) -> Element {
    rsx! {
        menu::MenuItemSection {
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DropdownMenuCheckboxItem`] component.
pub type DropdownMenuCheckboxItemProps<T> = menu::MenuCheckboxItemProps<T>;

/// A checkbox-style dropdown menu item.
#[component]
pub fn DropdownMenuCheckboxItem<T: Clone + PartialEq + 'static>(
    props: DropdownMenuCheckboxItemProps<T>,
) -> Element {
    rsx! {
        menu::MenuCheckboxItem {
            value: props.value,
            index: props.index,
            checked: props.checked,
            disabled: props.disabled,
            on_checked_change: props.on_checked_change,
            on_select: props.on_select,
            close_on_select: props.close_on_select,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DropdownMenuRadioGroup`] component.
pub type DropdownMenuRadioGroupProps<T> = menu::MenuRadioGroupProps<T>;

/// A group that coordinates related dropdown radio items.
#[component]
pub fn DropdownMenuRadioGroup<T: Clone + PartialEq + 'static>(
    props: DropdownMenuRadioGroupProps<T>,
) -> Element {
    rsx! {
        menu::MenuRadioGroup {
            value: props.value,
            on_value_change: props.on_value_change,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DropdownMenuRadioItem`] component.
pub type DropdownMenuRadioItemProps<T> = menu::MenuRadioItemProps<T>;

/// A radio-style dropdown menu item.
#[component]
pub fn DropdownMenuRadioItem<T: Clone + PartialEq + 'static>(
    props: DropdownMenuRadioItemProps<T>,
) -> Element {
    rsx! {
        menu::MenuRadioItem {
            value: props.value,
            index: props.index,
            disabled: props.disabled,
            on_select: props.on_select,
            close_on_select: props.close_on_select,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DropdownMenuSub`] component.
pub type DropdownMenuSubProps = menu::MenuSubProps;

/// A nested dropdown submenu root.
#[component]
pub fn DropdownMenuSub(props: DropdownMenuSubProps) -> Element {
    rsx! {
        menu::MenuSub {
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DropdownMenuSubTrigger`] component.
pub type DropdownMenuSubTriggerProps<T> = menu::MenuSubTriggerProps<T>;

/// A dropdown menu item that opens a nested submenu.
#[component]
pub fn DropdownMenuSubTrigger<T: Clone + PartialEq + 'static>(
    props: DropdownMenuSubTriggerProps<T>,
) -> Element {
    rsx! {
        menu::MenuSubTrigger {
            value: props.value,
            index: props.index,
            disabled: props.disabled,
            on_select: props.on_select,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DropdownMenuSubContent`] component.
pub type DropdownMenuSubContentProps = menu::MenuSubContentProps;

/// The popup content for a nested dropdown submenu.
#[component]
pub fn DropdownMenuSubContent(props: DropdownMenuSubContentProps) -> Element {
    rsx! {
        menu::MenuSubContent {
            id: props.id,
            role: props.role,
            attributes: props.attributes,
            {props.children}
        }
    }
}
