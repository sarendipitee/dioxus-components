//! Shared menu primitives used by dropdown menus, context menus, and menubars.

use crate::{
    focus::{
        use_deferred_focus, use_focus_controlled_item_disabled, use_focus_provider, FocusPlacement,
        FocusState,
    },
    merge_attributes,
    selectable::{pointer_select_cancel, pointer_select_commit, pointer_select_start},
    use_animated_open, use_effect_with_cleanup, use_id_or, use_unique_id,
};
use dioxus::prelude::*;
use dioxus_attributes::attributes;

/// Shared menu state provided to menu triggers, content, and items.
#[derive(Clone, Copy)]
pub struct MenuContext {
    /// Whether this menu is open.
    pub(crate) open: Memo<bool>,
    /// Sets the menu open state.
    pub(crate) set_open: Callback<bool>,
    /// Whether this menu and its items are disabled.
    pub(crate) disabled: Memo<bool>,
    /// Roving focus state for items in this menu.
    pub(crate) focus: FocusState,
    /// Unique id for the trigger that labels this menu.
    pub(crate) trigger_id: Signal<String>,
}

/// Provides shared menu state to descendants in the current component scope.
#[allow(clippy::redundant_closure)]
pub(crate) fn use_menu_provider(
    open: Memo<bool>,
    set_open: Callback<bool>,
    disabled: ReadSignal<bool>,
    roving_loop: ReadSignal<bool>,
) -> MenuContext {
    let focus = use_focus_provider(roving_loop);
    let trigger_id = use_unique_id();
    let disabled = use_memo(move || disabled());
    let ctx = use_context_provider(|| MenuContext {
        open,
        set_open,
        disabled,
        focus,
        trigger_id,
    });

    use_effect(move || {
        let focused = focus.any_focused();
        if *ctx.open.peek() != focused {
            (ctx.set_open)(focused);
        }
    });

    ctx
}

/// Light-dismisses an open menu when pointerdown or focus moves outside `id`.
pub(crate) fn use_menu_outside_dismiss(
    id: impl Readable<Target = String> + Copy + 'static,
    mut ctx: MenuContext,
    prevent_pointer_default: bool,
) {
    use_effect_with_cleanup(move || {
        let mut eval = if (ctx.open)() {
            let mut eval = document::eval(
                "const id = await dioxus.recv();
                const preventPointerDefault = await dioxus.recv();
                const onPointer = e => {
                    const root = document.getElementById(id);
                    if (root && !root.contains(e.target)) {
                        if (preventPointerDefault) {
                            e.preventDefault();
                            e.stopPropagation();
                            e.stopImmediatePropagation();
                        }
                        dioxus.send(true);
                    }
                };
                const onFocus = e => {
                    const root = document.getElementById(id);
                    if (root && !root.contains(e.target) && !e.target.contains(root)) dioxus.send(true);
                };
                document.addEventListener('pointerdown', onPointer, true);
                document.addEventListener('focusin', onFocus, true);
                await dioxus.recv();
                document.removeEventListener('pointerdown', onPointer, true);
                document.removeEventListener('focusin', onFocus, true);",
            );
            let _ = eval.send(id.cloned());
            let _ = eval.send(prevent_pointer_default);
            spawn(async move {
                while let Ok(true) = eval.recv().await {
                    ctx.focus.blur();
                    ctx.set_open.call(false);
                }
            });
            Some(eval)
        } else {
            None
        };

        move || {
            if let Some(eval) = eval.as_mut() {
                let _ = eval.send(true);
            }
        }
    });
}

/// Props for [`MenuRoot`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuRootProps {
    /// Whether the menu is open.
    pub open: Memo<bool>,
    /// Callback to set the open state.
    pub set_open: Callback<bool>,
    /// Whether the menu is disabled.
    pub disabled: ReadSignal<bool>,
    /// Whether focus should loop around when reaching the end.
    pub roving_loop: ReadSignal<bool>,
    /// Additional attributes for the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the menu root.
    pub children: Element,
}

/// Shared root for a single menu surface.
#[component]
pub fn MenuRoot(props: MenuRootProps) -> Element {
    let mut ctx = use_menu_provider(
        props.open,
        props.set_open,
        props.disabled,
        props.roving_loop,
    );

    let handle_keydown = move |event: Event<KeyboardData>| {
        if (props.disabled)() {
            return;
        }

        match event.key() {
            Key::Enter => ctx.set_open.call(!(ctx.open)()),
            Key::Escape => ctx.set_open.call(false),
            Key::ArrowDown => ctx.focus.focus_next(),
            Key::ArrowUp => {
                if (ctx.open)() {
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

    rsx! {
        div {
            "data-state": if (props.open)() { "open" } else { "closed" },
            "data-disabled": (props.disabled)(),
            onkeydown: handle_keydown,
            ..props.attributes,
            {props.children}
        }
    }
}

/// Props for [`MenuContent`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuContentProps {
    /// The ID of the menu content element. If not provided, a unique ID will be generated.
    pub id: ReadSignal<Option<String>>,
    /// The ARIA role for the menu content.
    #[props(default = "menu")]
    pub role: &'static str,
    /// Additional attributes to apply to the menu content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the menu content.
    pub children: Element,
}

/// Shared popup content for a menu.
#[component]
pub fn MenuContent(props: MenuContentProps) -> Element {
    let mut ctx: MenuContext = use_context();
    let unique_id = use_unique_id();
    let id = use_id_or(unique_id, props.id);
    let render = use_animated_open(id, ctx.open);

    let onkeydown = move |event: Event<KeyboardData>| {
        match event.key() {
            Key::Escape => {
                ctx.focus.blur();
                ctx.set_open.call(false);
            }
            Key::ArrowDown => ctx.focus.focus_next(),
            Key::ArrowUp => {
                if (ctx.open)() {
                    ctx.focus.focus_prev();
                }
            }
            Key::Home => ctx.focus.focus_first(),
            Key::End => ctx.focus.focus_last(),
            _ => return,
        }
        event.prevent_default();
        event.stop_propagation();
    };

    rsx! {
        if render() {
            div {
                id,
                role: props.role,
                aria_orientation: "vertical",
                aria_labelledby: "{ctx.trigger_id}",
                "data-state": if (ctx.open)() { "open" } else { "closed" },
                onkeydown,
                onpointerdown: move |event| {
                    event.prevent_default();
                    event.stop_propagation();
                },
                ..props.attributes,
                {props.children}
            }
        }
    }
}

fn use_submenu_position(
    content_id: impl Readable<Target = String> + Copy + 'static,
    trigger_id: impl Readable<Target = String> + Copy + 'static,
    open: impl Fn() -> bool + Copy + 'static,
) {
    use_effect_with_cleanup(move || {
        let mut eval = if open() {
            let eval = document::eval(
                "const contentId = await dioxus.recv();
                const triggerId = await dioxus.recv();
                const margin = 4;
                const position = () => {
                    const content = document.getElementById(contentId);
                    const trigger = document.getElementById(triggerId);
                    if (!content || !trigger) return;

                    content.dataset.side = 'right';
                    const triggerRect = trigger.getBoundingClientRect();
                    const contentRect = content.getBoundingClientRect();
                    const gap = parseFloat(getComputedStyle(content).marginLeft) || 0;
                    const hasRoomRight = triggerRect.right + gap + contentRect.width <= window.innerWidth - margin;
                    const hasRoomLeft = triggerRect.left - gap - contentRect.width >= margin;
                    content.dataset.side = !hasRoomRight && hasRoomLeft ? 'left' : 'right';
                };

                position();
                requestAnimationFrame(position);
                window.addEventListener('resize', position);
                await dioxus.recv();
                window.removeEventListener('resize', position);",
            );
            let _ = eval.send(content_id.cloned());
            let _ = eval.send(trigger_id.cloned());
            Some(eval)
        } else {
            None
        };

        move || {
            if let Some(eval) = eval.as_mut() {
                let _ = eval.send(true);
            }
        }
    });
}

/// Props for [`MenuItem`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuItemProps<T: Clone + PartialEq + 'static> {
    /// The value passed to `on_select` when the item is selected.
    #[props(into)]
    pub value: T,
    /// The index of the item within the menu content.
    pub index: ReadSignal<usize>,
    /// Whether this item is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,
    /// The ARIA role for the menu item.
    #[props(default = "menuitem")]
    pub role: &'static str,
    /// Callback fired when the item is selected.
    #[props(default)]
    pub on_select: Callback<T>,
    /// Whether the menu should close after the item is selected.
    #[props(default = true)]
    pub close_on_select: bool,
    /// Additional attributes for the item element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the item.
    pub children: Element,
}

/// Shared selectable item for a menu.
#[component]
pub fn MenuItem<T: Clone + PartialEq + 'static>(props: MenuItemProps<T>) -> Element {
    let mut ctx: MenuContext = use_context();
    let disabled = move || (ctx.disabled)() || (props.disabled)();
    let focused = move || ctx.focus.is_focused((props.index)());
    let onmounted = use_focus_controlled_item_disabled(props.index, disabled);
    let down_pos: Signal<Option<(f64, f64)>> = use_signal(|| None);
    let tab_index = use_memo(move || if focused() { "0" } else { "-1" });

    let mut select = move |value: T| {
        if !disabled() {
            props.on_select.call(value);
            if props.close_on_select {
                ctx.focus.blur();
                ctx.set_open.call(false);
            }
        }
    };

    let keydown_value = props.value.clone();

    rsx! {
        div {
            role: props.role,
            "data-disabled": disabled(),
            tabindex: tab_index,
            onpointerdown: move |event| {
                pointer_select_start(&event, disabled(), down_pos);
            },
            onpointerup: move |event| {
                if pointer_select_commit(&event, disabled(), down_pos) {
                    select(props.value.clone());
                    event.prevent_default();
                    event.stop_propagation();
                }
            },
            onpointercancel: move |_| {
                pointer_select_cancel(down_pos);
            },
            onkeydown: move |event: Event<KeyboardData>| {
                if event.key() == Key::Enter || event.key() == Key::Character(" ".to_string()) {
                    select(keydown_value.clone());
                    event.prevent_default();
                    event.stop_propagation();
                }
            },
            onblur: move |_| {
                if focused() {
                    ctx.focus.blur();
                }
            },
            onmounted,
            aria_disabled: disabled(),
            ..props.attributes,
            {props.children}
        }
    }
}

/// Props for [`MenuLabel`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuLabelProps {
    /// Additional attributes for the label element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the label.
    pub children: Element,
}

/// A non-interactive label for a menu section.
#[component]
pub fn MenuLabel(props: MenuLabelProps) -> Element {
    rsx! {
        div {
            role: "presentation",
            ..props.attributes,
            {props.children}
        }
    }
}

/// Props for [`MenuSeparator`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuSeparatorProps {
    /// Additional attributes for the separator element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A separator between groups of menu items.
#[component]
pub fn MenuSeparator(props: MenuSeparatorProps) -> Element {
    rsx! {
        div {
            role: "separator",
            aria_orientation: "horizontal",
            ..props.attributes,
        }
    }
}

/// Props for [`MenuGroup`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuGroupProps {
    /// Additional attributes for the group element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the group.
    pub children: Element,
}

/// A semantic group of related menu items.
#[component]
pub fn MenuGroup(props: MenuGroupProps) -> Element {
    rsx! {
        div {
            role: "group",
            ..props.attributes,
            {props.children}
        }
    }
}

/// Props for [`MenuItemIndicator`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuItemIndicatorProps {
    /// Whether the indicator should be rendered.
    #[props(default = true)]
    pub visible: ReadSignal<bool>,
    /// Additional attributes for the indicator element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the indicator.
    pub children: Element,
}

/// A presentational indicator for checked menu items.
#[component]
pub fn MenuItemIndicator(props: MenuItemIndicatorProps) -> Element {
    rsx! {
        if (props.visible)() {
            span {
                role: "presentation",
                ..props.attributes,
                {props.children}
            }
        }
    }
}

/// Props for [`MenuItemSection`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuItemSectionProps {
    /// Additional attributes for the item section element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the item section.
    pub children: Element,
}

/// A presentational section inside a menu item, useful for shortcuts or right-aligned content.
#[component]
pub fn MenuItemSection(props: MenuItemSectionProps) -> Element {
    rsx! {
        span {
            role: "presentation",
            ..props.attributes,
            {props.children}
        }
    }
}

/// Props for [`MenuCheckboxItem`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuCheckboxItemProps<T: Clone + PartialEq + 'static> {
    /// The value passed to `on_select` when the item is selected.
    #[props(into)]
    pub value: T,
    /// The index of the item within the menu content.
    pub index: ReadSignal<usize>,
    /// Whether this item is checked.
    pub checked: ReadSignal<bool>,
    /// Whether this item is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,
    /// Callback fired with the next checked state when the item is selected.
    #[props(default)]
    pub on_checked_change: Callback<bool>,
    /// Callback fired when the item is selected.
    #[props(default)]
    pub on_select: Callback<T>,
    /// Whether the menu should close after the item is selected.
    #[props(default)]
    pub close_on_select: bool,
    /// Additional attributes for the item element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the item.
    pub children: Element,
}

/// A menu item that toggles a checked state.
#[component]
pub fn MenuCheckboxItem<T: Clone + PartialEq + 'static>(
    props: MenuCheckboxItemProps<T>,
) -> Element {
    let checked = props.checked;
    let on_checked_change = props.on_checked_change;
    let on_select = props.on_select;

    rsx! {
        MenuItem {
            value: props.value,
            index: props.index,
            disabled: props.disabled,
            role: "menuitemcheckbox",
            close_on_select: props.close_on_select,
            on_select: move |value| {
                on_checked_change.call(!checked());
                on_select.call(value);
            },
            aria_checked: checked(),
            "data-state": if checked() { "checked" } else { "unchecked" },
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[derive(Clone, Copy)]
struct MenuRadioGroupContext<T: Clone + PartialEq + 'static> {
    value: ReadSignal<Option<T>>,
    on_value_change: Callback<T>,
}

/// Props for [`MenuRadioGroup`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuRadioGroupProps<T: Clone + PartialEq + 'static> {
    /// The currently selected radio value.
    pub value: ReadSignal<Option<T>>,
    /// Callback fired when the selected value changes.
    #[props(default)]
    pub on_value_change: Callback<T>,
    /// Additional attributes for the radio group element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the radio group.
    pub children: Element,
}

/// A group that coordinates related radio menu items.
#[component]
pub fn MenuRadioGroup<T: Clone + PartialEq + 'static>(props: MenuRadioGroupProps<T>) -> Element {
    use_context_provider(|| MenuRadioGroupContext {
        value: props.value,
        on_value_change: props.on_value_change,
    });

    rsx! {
        div {
            role: "group",
            ..props.attributes,
            {props.children}
        }
    }
}

/// Props for [`MenuRadioItem`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuRadioItemProps<T: Clone + PartialEq + 'static> {
    /// The value represented by this radio item.
    #[props(into)]
    pub value: T,
    /// The index of the item within the menu content.
    pub index: ReadSignal<usize>,
    /// Whether this item is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,
    /// Callback fired when the item is selected.
    #[props(default)]
    pub on_select: Callback<T>,
    /// Whether the menu should close after the item is selected.
    #[props(default)]
    pub close_on_select: bool,
    /// Additional attributes for the item element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the item.
    pub children: Element,
}

/// A radio-style menu item coordinated by the nearest [`MenuRadioGroup`].
#[component]
pub fn MenuRadioItem<T: Clone + PartialEq + 'static>(props: MenuRadioItemProps<T>) -> Element {
    let group: MenuRadioGroupContext<T> = use_context();
    let value = props.value.clone();
    let checked = use_memo(move || group.value.cloned().is_some_and(|current| current == value));
    let on_select = props.on_select;

    rsx! {
        MenuItem {
            value: props.value,
            index: props.index,
            disabled: props.disabled,
            role: "menuitemradio",
            close_on_select: props.close_on_select,
            on_select: move |value: T| {
                group.on_value_change.call(value.clone());
                on_select.call(value);
            },
            aria_checked: checked(),
            "data-state": if checked() { "checked" } else { "unchecked" },
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[derive(Clone, Copy)]
struct MenuSubContext {
    open: Memo<bool>,
    set_open: Callback<bool>,
    close_parent_all: Callback<()>,
    disabled: Memo<bool>,
    focus: FocusState,
    initial_focus: Signal<Option<FocusPlacement>>,
    trigger_id: Signal<String>,
}

/// Props for [`MenuSub`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuSubProps {
    /// Whether the submenu is open. If not provided, the submenu is uncontrolled and uses `default_open`.
    pub open: ReadSignal<Option<bool>>,
    /// Default open state if the submenu is not controlled.
    #[props(default)]
    pub default_open: bool,
    /// Callback fired when the submenu open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,
    /// Whether the submenu is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,
    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,
    /// Additional attributes for the submenu root.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the submenu root.
    pub children: Element,
}

/// A nested menu root for submenu trigger/content pairs.
#[component]
pub fn MenuSub(props: MenuSubProps) -> Element {
    let mut internal_open = use_signal(|| props.open.cloned().unwrap_or(props.default_open));
    let open = use_memo(move || props.open.cloned().unwrap_or_else(&*internal_open));
    let set_open = use_callback(move |next| {
        internal_open.set(next);
        props.on_open_change.call(next);
    });
    let parent_ctx: MenuContext = use_context();
    let disabled = use_memo(move || (parent_ctx.disabled)() || (props.disabled)());
    let mut focus = FocusState::new(props.roving_loop);
    let initial_focus = use_signal(|| None);
    let close_parent_all = use_callback(move |_| parent_ctx.set_open.call(false));
    let trigger_id = use_unique_id();
    let sub_id = use_unique_id();

    use_effect_with_cleanup(move || {
        let mut eval = document::eval(
            "const id = await dioxus.recv();
            const pointInRect = (x, y, rect, padding) =>
                x >= rect.left - padding &&
                x <= rect.right + padding &&
                y >= rect.top - padding &&
                y <= rect.bottom + padding;
            const onMove = e => {
                const root = document.getElementById(id);
                if (!root) return;
                const trigger = root.querySelector('[aria-haspopup=\"menu\"]');
                const content = root.querySelector('[role=\"menu\"]');
                const padding = 8;
                const pointTarget = document.elementFromPoint(e.clientX, e.clientY) ?? e.target;
                const insideTrigger = trigger && (
                    trigger.contains(pointTarget) ||
                    pointInRect(e.clientX, e.clientY, trigger.getBoundingClientRect(), padding)
                );
                const insideContent = content && (
                    content.contains(pointTarget) ||
                    pointInRect(e.clientX, e.clientY, content.getBoundingClientRect(), padding)
                );
                if (!insideTrigger && !insideContent) dioxus.send(true);
            };
            document.addEventListener('pointermove', onMove, true);
            document.addEventListener('mousemove', onMove, true);
            await dioxus.recv();
            document.removeEventListener('pointermove', onMove, true);
            document.removeEventListener('mousemove', onMove, true);",
        );
        let _ = eval.send(sub_id.cloned());
        spawn(async move {
            while let Ok(true) = eval.recv().await {
                if open() {
                    focus.blur();
                    set_open.call(false);
                }
            }
        });
        move || {
            let _ = eval.send(true);
        }
    });

    use_context_provider(|| MenuSubContext {
        open,
        set_open,
        close_parent_all,
        disabled,
        focus,
        initial_focus,
        trigger_id,
    });

    rsx! {
        div {
            id: sub_id,
            role: "presentation",
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": disabled(),
            ..props.attributes,
            {props.children}
        }
    }
}

/// Props for [`MenuSubTrigger`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuSubTriggerProps<T: Clone + PartialEq + 'static> {
    /// The value passed to `on_select` when the trigger is selected.
    #[props(into)]
    pub value: T,
    /// The index of the trigger within the parent menu content.
    pub index: ReadSignal<usize>,
    /// Whether this trigger is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,
    /// Callback fired when the trigger is selected.
    #[props(default)]
    pub on_select: Callback<T>,
    /// Additional attributes for the trigger element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the trigger.
    pub children: Element,
}

/// A menu item that opens the submenu provided by [`MenuSub`].
#[component]
pub fn MenuSubTrigger<T: Clone + PartialEq + 'static>(props: MenuSubTriggerProps<T>) -> Element {
    let mut parent_ctx: MenuContext = use_context();
    let mut sub_ctx: MenuSubContext = use_context();
    let open = sub_ctx.open;
    let on_select = props.on_select;
    let open_submenu = use_callback(move |_| {
        if !open() {
            sub_ctx.initial_focus.set(Some(FocusPlacement::First));
        }
        sub_ctx.set_open.call(true);
    });
    let base = attributes!(div {
        onpointermove: move |_| {
            if !(sub_ctx.disabled)() {
                open_submenu.call(());
            }
        },
        onblur: move |_| {
            if !open() {
                parent_ctx.focus.blur();
            }
        },
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        div {
            role: "presentation",
            style: "display: contents;",
            onkeydown: move |event: Event<KeyboardData>| {
                if event.key() == Key::ArrowRight && !(sub_ctx.disabled)() {
                    open_submenu.call(());
                    sub_ctx.focus.focus_first();
                    event.prevent_default();
                    event.stop_propagation();
                }
            },
            MenuItem {
                value: props.value,
                index: props.index,
                disabled: props.disabled,
                close_on_select: false,
                on_select: move |value: T| {
                    open_submenu.call(());
                    on_select.call(value);
                },
                id: sub_ctx.trigger_id,
                aria_haspopup: "menu",
                aria_expanded: open(),
                "data-state": if open() { "open" } else { "closed" },
                attributes,
                {props.children}
            }
        }
    }
}

/// The props for [`MenuSubContent`].
pub type MenuSubContentProps = MenuContentProps;

/// The popup content for a nested submenu.
#[component]
pub fn MenuSubContent(props: MenuSubContentProps) -> Element {
    let sub_ctx: MenuSubContext = use_context();
    let unique_id = use_unique_id();
    let id = use_id_or(unique_id, props.id);
    let set_open = use_callback(move |next: bool| {
        sub_ctx.set_open.call(next);
        if !next {
            sub_ctx.close_parent_all.call(());
        }
    });
    use_deferred_focus(sub_ctx.focus, sub_ctx.initial_focus, move || {
        (sub_ctx.open)()
    });
    use_submenu_position(id, sub_ctx.trigger_id, move || (sub_ctx.open)());
    use_context_provider(|| sub_ctx.focus);
    use_context_provider(|| MenuContext {
        open: sub_ctx.open,
        set_open,
        disabled: sub_ctx.disabled,
        focus: sub_ctx.focus,
        trigger_id: sub_ctx.trigger_id,
    });

    rsx! {
        MenuContent {
            id: Some(id.cloned()),
            role: props.role,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[component]
    fn MenuStaticMarkupFixture() -> Element {
        rsx! {
            MenuLabel { "Actions" }
            MenuSeparator {}
            MenuGroup {
                MenuItemSection { "Shortcut" }
                MenuItemIndicator { visible: true, "Checked" }
            }
        }
    }

    #[component]
    fn StringCheckboxItem(checked: ReadSignal<bool>) -> Element {
        rsx! {
            MenuCheckboxItem {
                value: "checked".to_string(),
                index: 0usize,
                checked,
                on_select: move |_value: String| {},
                "Checked item"
            }
        }
    }

    #[component]
    fn StringRadioItems(value: ReadSignal<Option<String>>) -> Element {
        rsx! {
            MenuRadioGroup {
                value,
                on_value_change: move |_value: String| {},
                MenuRadioItem {
                    value: "one".to_string(),
                    index: 1usize,
                    on_select: move |_value: String| {},
                    "Radio item"
                }
            }
        }
    }

    #[component]
    fn MenuCheckedItemsFixture() -> Element {
        let open = use_memo(|| true);
        let set_open = use_callback(|_| {});
        let disabled = use_memo(|| false);
        let focus = use_focus_provider(ReadSignal::new(Signal::new(true)));
        let trigger_id = use_signal(|| "test-trigger".to_string());
        let checked = use_signal(|| true);
        let radio_value = use_signal(|| Some("one".to_string()));

        use_context_provider(|| MenuContext {
            open,
            set_open,
            disabled,
            focus,
            trigger_id,
        });

        rsx! {
            StringCheckboxItem { checked }
            StringRadioItems { value: radio_value }
        }
    }

    #[test]
    fn menu_static_parts_render_expected_roles() {
        let mut dom = VirtualDom::new(MenuStaticMarkupFixture);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("role=\"presentation\""));
        assert!(html.contains("role=\"separator\""));
        assert!(html.contains("aria-orientation=\"horizontal\""));
        assert!(html.contains("role=\"group\""));
    }

    #[test]
    fn menu_checked_items_render_aria_checked_roles() {
        let mut dom = VirtualDom::new(MenuCheckedItemsFixture);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("role=\"menuitemcheckbox\""));
        assert!(html.contains("role=\"menuitemradio\""));
        assert!(html.contains("data-state=\"checked\""));
    }
}
