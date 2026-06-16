//! Shared menu primitives used by dropdown menus, context menus, and menubars.

use crate::{
    focus::{use_focus_controlled_item_disabled, use_focus_provider, FocusState},
    selectable::{pointer_select_cancel, pointer_select_commit, pointer_select_start},
    use_animated_open, use_effect_with_cleanup, use_id_or, use_unique_id,
};
use dioxus::prelude::*;

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
        move || {
            let _ = eval.send(true);
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
            ctx.focus.blur();
            ctx.set_open.call(false);
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
