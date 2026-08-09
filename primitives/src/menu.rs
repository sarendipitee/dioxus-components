//! Shared menu primitives used by dropdown menus, context menus, and menubars.

use std::rc::Rc;

use crate::{
    floating::{style_prop, use_position},
    focus::{
        use_deferred_focus, use_focus_controlled_item_disabled, use_focus_provider, FocusPlacement,
        FocusState,
    },
    merge_attributes,
    overlay::{
        use_overlay_registration, OverlayId, OverlayKind, OverlayRegistration, RegisterArgs,
    },
    portal::{use_portal, PortalIn},
    selectable::{pointer_select_cancel, pointer_select_commit, pointer_select_start},
    use_animated_open, use_effect_with_cleanup, use_id_or, use_unique_id, ContentAlign,
    ContentSide,
};
use dioxus::prelude::*;
use dioxus_attributes::attributes;

/// Shared state for a menu and its descendants.
#[derive(Clone, Copy, PartialEq)]
pub struct MenuContext {
    pub(crate) open: Memo<bool>,
    pub(crate) set_open: Callback<bool>,
    pub(crate) parent_set_open: Option<Callback<bool>>,
    pub(crate) disabled: Memo<bool>,
    pub(crate) focus: FocusState,
    pub(crate) initial_focus: Signal<Option<FocusPlacement>>,
    pub(crate) trigger_id: Signal<String>,
    pub(crate) trigger_ref: Signal<Option<Rc<MountedData>>>,
    pub(crate) overlay_id: Signal<Option<OverlayId>>,
    pub(crate) filter_query: Signal<String>,
    pub(crate) filter_input_id: Signal<Option<String>>,
    /// The currently active direct-child submenu. Sibling submenus use this
    /// coordination signal to close as soon as another trigger is entered.
    pub(crate) active_submenu: Signal<Option<String>>,
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
    let initial_focus = use_signal(|| None);
    let trigger_id = use_unique_id();
    let trigger_ref = use_signal(|| None);
    let overlay_id = use_signal(|| None);
    let filter_query = use_signal(String::new);
    let filter_input_id = use_signal(|| None);
    let active_submenu = use_signal(|| None);
    let disabled = use_memo(move || disabled());
    let ctx = use_context_provider(|| MenuContext {
        open,
        set_open,
        parent_set_open: None,
        disabled,
        focus,
        initial_focus,
        trigger_id,
        trigger_ref,
        overlay_id,
        filter_query,
        filter_input_id,
        active_submenu,
    });

    use_effect(move || {
        let focused = focus.any_focused();
        if focused && !*ctx.open.peek() {
            (ctx.set_open)(true);
        }
    });

    ctx
}

/// Props for [`Menu`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuProps {
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

/// Props forwarded to the unstyled text input rendered by [`FilterableMenu`].
#[derive(Props, Clone, PartialEq, Default)]
pub struct FilterableMenuInputProps {
    /// Callback fired after the menu items have been filtered.
    #[props(default)]
    pub oninput: Option<EventHandler<FormEvent>>,
    /// Callback fired after the filter input mounts and receives focus.
    #[props(default)]
    pub onmounted: Option<EventHandler<MountedEvent>>,
    /// Callback fired for keyboard events in the filter input.
    #[props(default)]
    pub onkeydown: Option<EventHandler<KeyboardEvent>>,
    /// Render the filter control as a custom component while preserving primitive attributes and events.
    #[props(default)]
    pub r#as: Option<Callback<Vec<Attribute>, Element>>,
    /// Additional attributes for the native text input.
    #[props(extends = GlobalAttributes)]
    #[props(extends = input)]
    pub attributes: Vec<Attribute>,
}

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

/// An unstyled menu with a focused text input that filters its menu items.
#[component]
pub fn FilterableMenu(props: FilterableMenuProps) -> Element {
    let filter_id = use_unique_id();
    let filter_input_id = format!("{filter_id}-input");
    rsx! {
        Menu {
            open: props.open,
            set_open: props.set_open,
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            div {
                "data-slot": "filterable-menu-input-section",
                FilterableMenuInputControl {
                    input_id: filter_input_id,
                    props: props.filter_input_props,
                }
            }
            MenuSeparator { "data-slot": "filterable-menu-separator" }
            {props.children}
        }
    }
}

#[component]
fn FilterableMenuInputControl(input_id: String, props: FilterableMenuInputProps) -> Element {
    let ctx = use_context::<MenuContext>();
    let mut query = ctx.filter_query;
    let mut filter_input_id = ctx.filter_input_id;
    let focus_on_mount = ctx.parent_set_open.is_none();
    let oninput = props.oninput;
    let onmounted = props.onmounted;
    let onkeydown = props.onkeydown;
    let dynamic = props.r#as;
    let mounted_input_id = input_id.clone();
    let registered_input_id = input_id.clone();
    let filter_surface_input_id = input_id.clone();
    let keyup_input_id = input_id.clone();
    let input_attributes = merge_attributes(vec![
        attributes!(input {
            id: input_id,
            r#type: "text",
            autofocus: focus_on_mount,
            value: (ctx.filter_query)(),
            onmounted: move |event| {
                filter_input_id.set(Some(registered_input_id.clone()));
                if focus_on_mount {
                    let input_id = mounted_input_id.clone();
                    spawn(async move {
                        let mut eval = document::eval("await new Promise(requestAnimationFrame); const input = document.getElementById(await dioxus.recv()); if (input) input.focus();");
                        let _ = eval.send(input_id);
                        let _ = eval.recv::<bool>().await;
                    });
                }
                if let Some(callback) = onmounted {
                    callback.call(event);
                }
            },
            oninput: move |event: Event<FormData>| {
                let input_id = filter_surface_input_id.clone();
                let value = event.value().trim().to_lowercase();
                spawn(async move {
                    let mut eval = document::eval("const [inputId, query] = await dioxus.recv(); const input = document.getElementById(inputId); const menu = input?.closest('[role=menu]'); if (!menu) return; for (const item of menu.querySelectorAll('[role^=menuitem]')) { if (item.closest('[role=menu]') !== menu) continue; const searchText = item.dataset.filterSearchText ?? ''; item.style.display = !query || searchText.includes(query) ? '' : 'none'; }");
                    let _ = eval.send((input_id, value));
                    let _ = eval.recv::<bool>().await;
                });
                query.set(event.value().trim().to_lowercase());
                if let Some(callback) = oninput {
                    callback.call(event);
                }
            },
            onkeydown: move |event: Event<KeyboardData>| {
                event.stop_propagation();
                if event.key() == Key::Backspace {
                    let current = (ctx.filter_query)();
                    let next = current
                        .char_indices()
                        .next_back()
                        .map(|(index, _)| current[..index].to_string())
                        .unwrap_or_default();
                    query.set(next);
                }
                if let Some(callback) = onkeydown {
                    callback.call(event);
                }
            },
            onkeyup: move |_| {
                let input_id = keyup_input_id.clone();
                spawn(async move {
                    let mut eval = document::eval("const input = document.getElementById(await dioxus.recv()); if (!input) return ''; const query = input.value.trim().toLowerCase(); const menu = input.closest('[role=menu]'); if (menu) { for (const item of menu.querySelectorAll('[role^=menuitem]')) { if (item.closest('[role=menu]') !== menu) continue; const searchText = item.dataset.filterSearchText ?? ''; item.style.display = !query || searchText.includes(query) ? '' : 'none'; } } return input.value;");
                    let _ = eval.send(input_id);
                    if let Ok(value) = eval.recv::<String>().await {
                        query.set(value.trim().to_lowercase());
                    }
                });
            },
        }),
        props.attributes,
    ]);
    if let Some(dynamic) = dynamic {
        dynamic.call(input_attributes)
    } else {
        rsx! { input { ..input_attributes } }
    }
}

/// Props for filterable content rendered inside an existing menu root.
#[derive(Props, Clone, PartialEq)]
pub struct FilterableMenuContentProps {
    /// Props forwarded to the filter text input.
    #[props(default)]
    pub filter_input_props: FilterableMenuInputProps,
    /// Menu items and other menu content.
    pub children: Element,
}

/// Filterable menu content for an existing menu root.
#[component]
pub fn FilterableMenuContent(props: FilterableMenuContentProps) -> Element {
    let filter_id = use_unique_id();
    let filter_input_id = format!("{filter_id}-input");

    rsx! {
        div {
            "data-slot": "filterable-menu-input-section",
            FilterableMenuInputControl {
                input_id: filter_input_id,
                props: props.filter_input_props,
            }
        }
        MenuSeparator { "data-slot": "filterable-menu-separator" }
        {props.children}
    }
}

/// Shared root for a single menu surface.
#[component]
pub fn Menu(props: MenuProps) -> Element {
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
                return;
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
    #[props(default)]
    pub id: Option<String>,
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
///
/// This is the Root-tree half: it keeps the component's own [`MenuContext`] (for
/// the trigger that stays in the tree), drives `use_animated_open`, and while open
/// renders the panel through the shared overlay outlet.
/// The panel DOM and the `MenuContext` re-provide live in the portaled body.
#[component]
pub fn MenuContent(props: MenuContentProps) -> Element {
    let ctx: MenuContext = use_context();
    let unique_id = use_unique_id();
    let id_value = props.id.clone();
    let id_signal = ReadSignal::new(use_memo(use_reactive(&id_value, |id_value| {
        id_value.clone()
    })));
    let id = use_id_or(unique_id, id_signal);
    let render = use_animated_open(id, ctx.open);

    rsx! {
        if render() {
            MenuContentPortaled {
                ctx,
                id,
                role: props.role,
                attributes: props.attributes,
                children: props.children,
            }
        }
    }
}

/// Props for [`MenuContentPortaled`], the in-portal half of [`MenuContent`].
#[derive(Props, Clone, PartialEq)]
struct MenuContentPortaledProps {
    ctx: MenuContext,
    id: Memo<String>,
    role: &'static str,
    attributes: Vec<Attribute>,
    children: Element,
}

/// Registers the menu panel as an [`OverlayKind::Floating`] entry and renders it
/// through the shared [`crate::overlay::OverlayOutlet`].
///
/// Parent linkage: the incoming [`MenuContext::overlay_id`] carries the *parent*
/// menu's overlay id when this content is a submenu (set by [`MenuSubContent`]),
/// or `None` for a top-level menu. We register with `parent` = that id so the
/// union dismiss predicate and z-stacking treat a submenu as inside its parent.
///
/// The trigger id (`ctx.trigger_id`) and the panel content root id (`id`) are
/// registered so the manager's union "inside" predicate treats clicks on either
/// subtree as inside — the trigger now lives in a different DOM subtree than the
/// portaled panel.
#[component]
fn MenuContentPortaled(props: MenuContentPortaledProps) -> Element {
    let mut ctx = props.ctx;
    let id = props.id;
    let trigger_id = ctx.trigger_id;
    // The parent menu's overlay id (if this content is a submenu).
    let parent = *ctx.overlay_id.peek();

    let portal = use_portal();

    let set_open = ctx.set_open;
    let mut focus = ctx.focus;
    let on_dismiss = use_callback(move |_| {
        focus.blur();
        set_open.call(false);
    });

    let reg: OverlayRegistration = use_overlay_registration(move || RegisterArgs {
        kind: OverlayKind::Floating,
        portal,
        modal: false,
        dismissable: true,
        on_dismiss,
        parent,
        trigger_id: Some(trigger_id.peek().clone()),
        content_root_id: Some(id.peek().clone()),
        stack_key: None,
    });

    // Expose this entry's id to descendants (a nested MenuSubContent reads it as
    // its `parent`). Stored on the ctx so the re-provided clone inside the portal
    // carries it.
    use_effect(move || {
        let next = reg.id();
        // Dropped-tolerant write: this can re-run during teardown.
        let _ = ctx.overlay_id.try_write().map(|mut w| *w = next);
    });

    // Keep the manager's "inside" predicate pointed at the live trigger + content
    // ids once the elements have mounted. Read with `try_peek` (non-subscribing):
    // a reactive read of these `use_unique_id`-backed signals leaves a
    // `SignalSubscriberDrop` guard whose `update_subscribers` drop runs during
    // teardown against the already-freed signal storage, panicking with
    // `ValueDroppedError` (signal.rs:540). The ids are assigned once at mount, so
    // a non-reactive read is sufficient; bail if either is already gone.
    use_effect(move || {
        // Subscribe to `open` only, so this effect re-runs while the menu is live.
        let _ = (ctx.open)();
        let (Ok(t), Ok(i)) = (trigger_id.try_peek(), id.try_peek()) else {
            return;
        };
        reg.set_dom_ids(Some(t.clone()), Some(i.clone()));
    });

    // Exit-phase exclusion: mark `closing` while the menu is animating out.
    let open = ctx.open;
    use_effect(move || {
        reg.set_closing(!open());
    });

    // Subscribe to `open` HERE, in the non-portaled (Root-descendant) scope, and
    // forward the snapshot into the portaled body as a plain bool so the body
    // never reads the Root-owned `open` Memo across the portal boundary.
    let is_open = open();
    // Snapshot every body-owned cross-scope value HERE so the portaled body never
    // reads `Memo`/`Signal`/`OverlayRegistration` handles owned by this scope.
    let panel_id = id.cloned();
    let aria_labelledby = trigger_id.cloned();
    let is_disabled = (ctx.disabled)();
    let filter_query = ctx.filter_query.cloned();
    let content_overlay_id = reg.id();
    let overlay_z = reg.z();

    rsx! {
        PortalIn { portal,
            MenuContentRendered {
                set_open,
                parent_set_open: ctx.parent_set_open,
                active_submenu: ctx.active_submenu,
                is_open,
                panel_id,
                aria_labelledby,
                is_disabled,
                focus: ctx.focus,
                initial_focus: ctx.initial_focus,
                content_overlay_id,
                filter_query,
                overlay_z,
                role: props.role,
                attributes: props.attributes.clone(),
                children: props.children,
            }
        }
    }
}

/// Props for [`MenuContentRendered`], the portaled DOM body.
#[derive(Props, Clone, PartialEq)]
struct MenuContentRenderedProps {
    set_open: Callback<bool>,
    parent_set_open: Option<Callback<bool>>,
    active_submenu: Signal<Option<String>>,
    /// note on `DialogPortalBodyProps::is_open`.
    is_open: bool,
    panel_id: String,
    aria_labelledby: String,
    is_disabled: bool,
    focus: FocusState,
    initial_focus: Signal<Option<FocusPlacement>>,
    content_overlay_id: Option<OverlayId>,
    overlay_z: Option<String>,
    filter_query: String,
    role: &'static str,
    attributes: Vec<Attribute>,
    children: Element,
}
/// (`MenuItem`, `MenuSub`, `MenuSubContent`, the focus state, …) resolve their
/// context up the *portaled* render chain. Emits `--overlay-z` from the manager.
#[component]
fn MenuContentRendered(props: MenuContentRenderedProps) -> Element {
    let is_open = props.is_open;
    let set_open = props.set_open;

    let open = use_memo(use_reactive(&props.is_open, |is_open| is_open));
    let disabled = use_memo(use_reactive(&props.is_disabled, |is_disabled| is_disabled));
    let filter_query = use_signal(|| props.filter_query.clone());
    let mut trigger_id = use_signal(|| props.aria_labelledby.clone());
    use_effect(use_reactive(
        &props.aria_labelledby,
        move |aria_labelledby| {
            trigger_id.set(aria_labelledby);
        },
    ));

    // The trigger and the portaled panel must share one focus controller. Recreating
    // it here disconnects trigger ArrowDown/ArrowUp from the mounted menu items.
    let mut focus = props.focus;
    let initial_focus = props.initial_focus;
    use_deferred_focus(focus, initial_focus, move || *open.read());

    let trigger_ref = use_signal(|| None);
    let mut overlay_id = use_signal(|| props.content_overlay_id);
    let filter_input_id = use_signal(|| None);
    use_effect(use_reactive(
        &props.content_overlay_id,
        move |content_overlay_id| {
            overlay_id.set(content_overlay_id);
        },
    ));

    let portal_ctx = MenuContext {
        open,
        set_open,
        parent_set_open: props.parent_set_open,
        disabled,
        focus,
        initial_focus,
        trigger_id,
        trigger_ref,
        overlay_id,
        filter_query,
        filter_input_id,
        active_submenu: props.active_submenu,
    };
    use_context_provider(|| portal_ctx);
    use_context_provider(|| focus);

    let base = attributes!(div {
        style: props
            .overlay_z
            .as_ref()
            .map(|z| format!("--overlay-z: {z};"))
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        div {
            id: props.panel_id.clone(),
            role: props.role,
            aria_orientation: "vertical",
            aria_labelledby: props.aria_labelledby.clone(),
            "data-state": if is_open { "open" } else { "closed" },
            onmouseenter: move |_| {
                let input_id = filter_input_id.peek().clone();
                spawn(async move {
                    let Some(input_id) = input_id else { return };
                    let mut eval = document::eval("await new Promise(requestAnimationFrame); const input = document.getElementById(await dioxus.recv()); if (input) input.focus();");
                    let _ = eval.send(input_id);
                    let _ = eval.recv::<bool>().await;
                });
            },
            onkeydown: move |event: Event<KeyboardData>| {
                match event.key() {
                    Key::Escape => {
                        focus.blur();
                        set_open.call(false);
                    }
                    Key::ArrowDown => focus.focus_next(),
                    Key::ArrowUp => {
                        if is_open {
                            focus.focus_prev();
                        }
                    }
                    Key::Home => focus.focus_first(),
                    Key::End => focus.focus_last(),
                    Key::Tab => {
                        focus.blur();
                        set_open.call(false);
                        return;
                    }
                    _ => return,
                }
                event.prevent_default();
                event.stop_propagation();
            },
            onpointerdown: move |event| {
                event.prevent_default();
                event.stop_propagation();
            },
            ..attributes,
            {props.children}
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
    pub index: usize,
    /// Whether this item is disabled.
    #[props(default)]
    pub disabled: bool,
    /// The ARIA role for the menu item.
    #[props(default = "menuitem")]
    pub role: &'static str,
    /// Callback fired when the item is selected.
    #[props(default)]
    pub on_select: Callback<T>,
    /// Whether the menu should close after the item is selected.
    #[props(default = true)]
    pub close_on_select: bool,
    /// Optional callback invoked with the item's mounted element data. Used by
    /// submenu triggers to capture the item as a floating-ui reference element
    /// without clobbering the internal roving-focus `onmounted`. Benign default for
    /// all other items.
    #[props(default)]
    pub on_mounted: Option<Callback<Rc<MountedData>>>,
    /// Whether this item owns an openable submenu.
    #[props(default)]
    pub is_submenu_trigger: bool,
    /// Text used for matching when rendered inside a filterable menu.
    #[props(default)]
    pub search_text: Option<String>,
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
    let index_value = props.index;
    let index = use_memo(move || index_value);
    let disabled_value = props.disabled;
    let has_search_text = props.search_text.is_some();
    let mut warned_missing_search_text = use_signal(|| false);
    use_effect(move || {
        let query = (ctx.filter_query)();
        if cfg!(debug_assertions)
            && !has_search_text
            && !query.is_empty()
            && !warned_missing_search_text()
        {
            tracing::warn!(
                "FilterableMenu item is missing `search_text`; it will not match filter queries"
            );
            warned_missing_search_text.set(true);
        }
    });
    let search_text = use_memo(move || props.search_text.clone());
    let visible = move || {
        let query = (ctx.filter_query)();
        query.is_empty()
            || search_text()
                .as_deref()
                .unwrap_or_default()
                .to_lowercase()
                .contains(&query)
    };
    let focused = move || ctx.focus.is_focused(index_value);
    let disabled = move || (ctx.disabled)() || disabled_value;
    let unavailable = move || disabled() || !visible();
    let forward_mounted = props.on_mounted;
    let mut focus_onmounted = use_focus_controlled_item_disabled(index, unavailable);
    use_effect(move || {
        let _ = ctx.focus.items_revision();
        let _ = ctx.focus.current_focus();
        if (ctx.open)() {
            if let Some(placement) = (ctx.initial_focus)() {
                if ctx.focus.try_focus_placement(placement) {
                    ctx.initial_focus.set(None);
                }
            }
        }
    });
    let onmounted = move |evt: MountedEvent| {
        let mounted = evt.data();
        if let Some(cb) = forward_mounted {
            cb.call(mounted);
        }
        focus_onmounted(evt);
    };
    let down_pos: Signal<Option<(f64, f64)>> = use_signal(|| None);
    let mut select = move |value: T| {
        if !disabled() {
            props.on_select.call(value);
            if props.close_on_select {
                ctx.focus.blur();
                ctx.set_open.call(false);
                if let Some(parent_set_open) = ctx.parent_set_open {
                    parent_set_open.call(false);
                }
            }
        }
    };
    let keydown_value = props.value.clone();
    rsx! {
        div {
            style: if !visible() { "display: none;" } else { "" },
            role: props.role,
            "data-filter-search-text": search_text().unwrap_or_default().to_lowercase(),
            onmouseenter: move |_| {
                if !props.is_submenu_trigger {
                    ctx.active_submenu.set(None);
                }
                ctx.focus.set_focus(Some(index_value));
            },
            onpointerdown: move |event| { pointer_select_start(&event, disabled(), down_pos); },
            onpointerup: move |event| {
                if pointer_select_commit(&event, disabled(), down_pos) {
                    select(props.value.clone());
                    event.prevent_default();
                    event.stop_propagation();
                }
            },
            onpointercancel: move |_| { pointer_select_cancel(down_pos); },
            onkeydown: move |event: Event<KeyboardData>| {
                if event.key() == Key::Enter || event.key() == Key::Character(" ".to_string()) {
                    select(keydown_value.clone());
                    event.prevent_default();
                    event.stop_propagation();
                }
            },
            // Roving focus is updated by the focus controller; clearing it from
            // blur races portaled submenu focus and resets the pending target.
            aria_disabled: disabled(),
            "data-disabled": disabled().then_some("true"),
            "data-focused": focused(),
            // MountedData::set_focus requires a focusable element. Keep only the
            // roving-focus item tabbable so native focus follows trigger arrows.
            tabindex: if focused() { "0" } else { "-1" },
            onmounted: onmounted,
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
    pub visible: bool,
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
        if props.visible {
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
    pub index: usize,
    /// Whether this item is checked.
    pub checked: bool,
    /// Whether this item is disabled.
    #[props(default)]
    pub disabled: bool,
    /// Callback fired with the next checked state when the item is selected.
    #[props(default)]
    pub on_checked_change: Callback<bool>,
    /// Callback fired when the item is selected.
    #[props(default)]
    pub on_select: Callback<T>,
    /// Text used for matching when rendered inside a filterable menu.
    #[props(default)]
    pub search_text: Option<String>,
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
            search_text: props.search_text,
            on_select: move |value| {
                on_checked_change.call(!checked);
                on_select.call(value);
            },
            aria_checked: checked,
            "data-state": if checked { "checked" } else { "unchecked" },
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
    #[props(default)]
    pub value: Option<T>,
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
    let mut value = use_signal(|| props.value.clone());
    use_effect(use_reactive(&props.value, move |current_value| {
        value.set(current_value);
    }));
    use_context_provider(|| MenuRadioGroupContext {
        value: value.into(),
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
    /// Value represented by this radio item.
    #[props(into)]
    pub value: T,
    /// Position of this item in the radio group.
    pub index: usize,
    /// Whether this item is disabled.
    #[props(default)]
    pub disabled: bool,
    /// Callback invoked when this item is selected.
    #[props(default)]
    pub on_select: Callback<T>,
    /// Whether selecting this item closes the menu.
    #[props(default)]
    pub close_on_select: bool,
    /// Text used when filtering menu items.
    #[props(default)]
    pub search_text: Option<String>,
    /// Attributes forwarded to the rendered element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// Content rendered inside this item.
    pub children: Element,
}
/// A radio-style menu item coordinated by the nearest [`MenuRadioGroup`].
#[component]
pub fn MenuRadioItem<T: Clone + PartialEq + 'static>(props: MenuRadioItemProps<T>) -> Element {
    let group: MenuRadioGroupContext<T> = use_context();
    let value = props.value.clone();
    let checked = (group.value)()
        .as_ref()
        .is_some_and(|current| current == &value);
    let on_select = props.on_select;

    rsx! {
        MenuItem {
            value: props.value,
            index: props.index,
            disabled: props.disabled,
            role: "menuitemradio",
            close_on_select: props.close_on_select,
            search_text: props.search_text,
            on_select: move |value: T| {
                group.on_value_change.call(value.clone());
                on_select.call(value);
            },
            aria_checked: checked,
            "data-state": if checked { "checked" } else { "unchecked" },
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[derive(Clone, Copy)]
struct MenuSubContext {
    open: Memo<bool>,
    set_open: Callback<bool>,
    disabled: Memo<bool>,
    focus: FocusState,
    initial_focus: Signal<Option<FocusPlacement>>,
    trigger_id: Signal<String>,
    /// Reference (submenu trigger) element used to position the submenu content.
    trigger_ref: Signal<Option<Rc<MountedData>>>,
    /// Id of the submenu's portaled content panel. The panel is portaled OUT of
    /// the `MenuSub` subtree (rendered through the shared overlay outlet), so the
    /// hover-leave detection in [`MenuSub`] can no longer find it by querying
    /// descendants of `sub_id`. [`MenuSubContent`] writes its content root id here
    /// so the hover predicate can resolve the portaled panel via `getElementById`.
    content_id: Signal<String>,
}

/// Props for [`MenuSub`].
#[derive(Props, Clone, PartialEq)]
pub struct MenuSubProps {
    /// Whether the submenu is open. If not provided, the submenu is uncontrolled and uses `default_open`.
    #[props(default)]
    pub open: Option<bool>,
    /// Default open state if the submenu is not controlled.
    #[props(default)]
    pub default_open: bool,
    /// Callback fired when the submenu open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,
    /// Whether the submenu is disabled.
    #[props(default)]
    pub disabled: bool,
    /// Whether focus should loop around when reaching the end.
    #[props(default = true)]
    pub roving_loop: bool,
    /// Additional attributes for the submenu root.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the submenu root.
    pub children: Element,
}

/// A nested menu root for submenu trigger/content pairs.
#[component]
pub fn MenuSub(props: MenuSubProps) -> Element {
    let open_value = props.open;
    let mut internal_open = use_signal(move || open_value.unwrap_or(props.default_open));
    let open = use_memo(use_reactive(&props.open, move |open_value| {
        open_value.unwrap_or_else(&*internal_open)
    }));
    let set_open = use_callback(move |next| {
        internal_open.set(next);
        props.on_open_change.call(next);
    });
    let parent_ctx: MenuContext = use_context();
    let disabled_value = props.disabled;
    let disabled = use_memo(use_reactive(&disabled_value, move |disabled_value| {
        (parent_ctx.disabled)() || disabled_value
    }));
    let roving_loop_value = props.roving_loop;
    let mut roving_loop = use_signal(move || roving_loop_value);
    use_effect(use_reactive(&roving_loop_value, move |next| {
        roving_loop.set(next);
    }));
    let mut focus = FocusState::new(ReadSignal::new(roving_loop));
    let initial_focus = use_signal(|| None);
    let trigger_id = use_unique_id();
    use_effect(move || {
        let active = (parent_ctx.active_submenu)();
        if open() && active.as_deref() != Some(&(trigger_id)()) {
            set_open.call(false);
        }
    });
    let trigger_ref = use_signal(|| None);
    let sub_id = use_unique_id();
    // The portaled submenu panel's content root id, written by `MenuSubContent`.
    let content_id = use_signal(String::new);

    use_effect_with_cleanup(move || {
        // The trigger still lives inside `sub_id`, but the submenu content panel is
        // portaled OUT to the shared overlay outlet.
        // The trigger still lives inside `sub_id`, but the submenu content panel is
        // portaled OUT to the shared overlay outlet — so it can no longer be found
        // by querying descendants of `sub_id`. Resolve the trigger from the sub
        // subtree and the content from the registered portaled panel id. Reading
        // `content_id` here makes this effect re-run (tearing down and recreating
        // the listener) once `MenuSubContent` publishes the panel id, so the
        // hover-leave predicate picks up the portaled panel.
        let panel_id = content_id();
        let Some(sub_id_value) = sub_id.try_peek().ok().map(|v| v.clone()) else {
            return Box::new(|| {}) as Box<dyn FnOnce()>;
        };
        let mut eval = document::eval(
            "const ids = await dioxus.recv();
            const subId = ids[0];
            const contentId = ids[1];
            const pointInRect = (x, y, rect, padding) =>
                x >= rect.left - padding &&
                x <= rect.right + padding &&
                y >= rect.top - padding &&
                y <= rect.bottom + padding;
            const pointInTriangle = (point, a, b, c) => {
                const sign = (p1, p2, p3) =>
                    (p1.x - p3.x) * (p2.y - p3.y) -
                    (p2.x - p3.x) * (p1.y - p3.y);
                const d1 = sign(point, a, b);
                const d2 = sign(point, b, c);
                const d3 = sign(point, c, a);
                const hasNegative = d1 < 0 || d2 < 0 || d3 < 0;
                const hasPositive = d1 > 0 || d2 > 0 || d3 > 0;
                return !(hasNegative && hasPositive);
            };
            const padding = 8;
            let triangleOrigin = null;
            const onMove = e => {
                const root = document.getElementById(subId);
                if (!root) return;
                const trigger = root.querySelector('[aria-haspopup=\"menu\"]');
                // The panel is portaled out of `subId`; resolve it by its own id.
                // Fall back to a descendant lookup for the inline/native path where
                // the panel is not portaled.
                const content = (contentId && document.getElementById(contentId))
                    ?? root.querySelector('[role=\"menu\"]');
                // The portaled panel may not exist during the first pointer event
                // that opens it. Do not interpret that mount gap as a pointer leave.
                if (!content) return;
                const pointTarget = document.elementFromPoint(e.clientX, e.clientY);
                // During portal mount/unmount the browser can briefly return no
                // hit-test target. Treat that transient state as indeterminate;
                // closing here races the submenu open transition.
                if (!pointTarget) return;
                const triggerRect = trigger?.getBoundingClientRect();
                const contentRect = content.getBoundingClientRect();
                const guardId = `${subId}-pointer-grace`;
                let guard = document.getElementById(guardId);
                if (!guard && triggerRect) {
                    const opensRight = contentRect.left >= triggerRect.right;
                    const edgeX = opensRight ? contentRect.left : contentRect.right;
                    const triangleWidth = Math.max(48, Math.abs(
                        edgeX - (opensRight ? triggerRect.right : triggerRect.left)
                    ));
                    guard = document.createElement('div');
                    guard.id = guardId;
                    guard.style.cssText = [
                        'position:fixed',
                        `left:${opensRight ? edgeX - triangleWidth : edgeX}px`,
                        `top:${contentRect.top - padding}px`,
                        `width:${triangleWidth}px`,
                        `height:${contentRect.height + padding * 2}px`,
                        'background:transparent',
                        'pointer-events:auto',
                        'z-index:2147483647',
                        `clip-path:polygon(${opensRight ? '100% 0,0 50%,100% 100%' : '0 0,100% 50%,0 100%'})`,
                    ].join(';');
                    document.body.appendChild(guard);
                }
                const insideTrigger = trigger && (
                    trigger.contains(pointTarget) ||
                    pointInRect(e.clientX, e.clientY, triggerRect, padding)
                );
                const insideContent = content.contains(pointTarget) ||
                    pointInRect(e.clientX, e.clientY, contentRect, padding);
                const insideNestedSubmenu = pointTarget.closest('[role=\"menu\"][data-side]');
                const insidePointerGrace = pointTarget.id === `${subId}-pointer-grace`;
                if (insideTrigger) {
                    triangleOrigin = { x: e.clientX, y: e.clientY };
                    return;
                }
                if (insideContent || insideNestedSubmenu || insidePointerGrace) {
                    triangleOrigin = null;
                    return;
                }
                if (triggerRect) {
                    const opensRight = contentRect.left >= triggerRect.right;
                    const edgeX = opensRight ? contentRect.left : contentRect.right;
                    const origin = triangleOrigin ?? {
                        x: opensRight ? triggerRect.right : triggerRect.left,
                        y: triggerRect.top + triggerRect.height / 2,
                    };
                    const point = { x: e.clientX, y: e.clientY };
                    const top = { x: edgeX, y: contentRect.top - padding };
                    const bottom = { x: edgeX, y: contentRect.bottom + padding };
                    if (pointInTriangle(point, origin, top, bottom)) return;
                }
                triangleOrigin = null;
                dioxus.send(true);
            };
            document.addEventListener('pointermove', onMove, true);
            await dioxus.recv();
            document.removeEventListener('pointermove', onMove, true);
            document.getElementById(`${subId}-pointer-grace`)?.remove();
            ",
        );
        let _ = eval.send(vec![sub_id_value, panel_id]);
        spawn(async move {
            while let Ok(true) = eval.recv().await {
                if open() {
                    focus.blur();
                    set_open.call(false);
                }
            }
        });
        Box::new(move || {
            let _ = eval.send(true);
        }) as Box<dyn FnOnce()>
    });

    use_context_provider(|| MenuSubContext {
        open,
        set_open,
        disabled,
        focus,
        initial_focus,
        trigger_id,
        trigger_ref,
        content_id,
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
    pub index: usize,
    /// Whether this trigger is disabled.
    #[props(default)]
    pub disabled: bool,
    /// Callback fired when the trigger is selected.
    #[props(default)]
    pub on_select: Callback<T>,
    /// Text used for matching when rendered inside a filterable menu.
    #[props(default)]
    pub search_text: Option<String>,
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
    let mut trigger_ref = sub_ctx.trigger_ref;
    let open = sub_ctx.open;
    let disabled_value = props.disabled;
    let on_select = props.on_select;
    let open_submenu = use_callback(move |_| {
        // Pointer-move fires continuously while the cursor is over a trigger.
        // Once this submenu is open, rewriting the coordination/open signals
        // needlessly reconciles its portaled content under the pointer and can
        // detach the trigger before Playwright (or a user) completes the hover.
        if !open() {
            sub_ctx.initial_focus.set(Some(FocusPlacement::First));
            (parent_ctx.active_submenu).set(Some((sub_ctx.trigger_id)()));
            sub_ctx.set_open.call(true);
        }
    });
    let base = attributes!(div {
        onmouseenter: move |_| {
            if !(sub_ctx.disabled)() && !disabled_value {
                open_submenu.call(());
            }
        },
        onpointermove: move |_| {
            if !(sub_ctx.disabled)() && !disabled_value {
                open_submenu.call(());
            }
        },
        // Opening a submenu transfers native focus into its portaled panel. The
        // trigger can blur while the open signal is propagating; do not clear the
        // shared parent focus state during that handoff.
    });
    // Focus is owned by the shared roving controller; item blur must not
    // clear it while a submenu transfers focus through the portal.
    let attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        div {
            role: "presentation",
            style: "display: contents;",
            onkeydown: move |event: Event<KeyboardData>| {
                if event.key() == Key::ArrowRight && !(sub_ctx.disabled)() && !disabled_value {
                    // `open_submenu` records a pending first-item placement. The
                    // submenu panel is portaled and its items mount afterward, so
                    // focusing synchronously here observes an empty item registry.
                    open_submenu.call(());
                    event.prevent_default();
                    event.stop_propagation();
                }
            },
            MenuItem {
                value: props.value,
                index: props.index,
                disabled: props.disabled,
                is_submenu_trigger: true,
                close_on_select: false,
                search_text: props.search_text,
                on_select: move |value: T| {
                    open_submenu.call(());
                    on_select.call(value);
                },
                id: sub_ctx.trigger_id,
                aria_haspopup: "menu",
                aria_expanded: open(),
                "data-state": if open() { "open" } else { "closed" },
                on_mounted: move |data| trigger_ref.set(Some(data)),
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
    let id_value = props.id.clone();
    let id_signal = ReadSignal::new(use_memo(use_reactive(&id_value, |id_value| {
        id_value.clone()
    })));
    let id = use_id_or(unique_id, id_signal);
    // Publish the portaled panel's root id so `MenuSub`'s hover-leave predicate can
    // resolve the panel via `getElementById` (it lives outside `sub_id` now).
    let mut content_id = sub_ctx.content_id;
    use_effect(move || {
        let next = id();
        // Dropped-tolerant: on submenu close the whole subtree unmounts and this
        // effect can re-run after `content_id`'s backing store is freed.
        if content_id.try_peek().map(|v| *v != next).unwrap_or(false) {
            let _ = content_id.try_write().map(|mut w| *w = next);
        }
    });
    // The first focus request may run before the portaled item registry exists;
    // retry from the submenu's context after its content has mounted.
    use_deferred_focus(sub_ctx.focus, sub_ctx.initial_focus, move || {
        (sub_ctx.open)()
    });
    use_context_provider(|| sub_ctx.focus);
    // `MenuContent` consumes `MenuContext`, not `MenuSubContext`. Re-provide the
    // submenu state only around its content so this panel follows this submenu's
    // open state while the trigger continues to use the parent menu context.
    let parent_ctx: MenuContext = use_context();
    let overlay_id = use_signal(|| *parent_ctx.overlay_id.peek());
    let filter_query = use_signal(String::new);
    let filter_input_id = use_signal(|| None);
    // Coordinate only submenus that are direct children of this content. A
    // nested trigger must not replace its ancestor's active sibling id.
    let active_submenu = use_signal(|| None);
    use_context_provider(|| MenuContext {
        open: sub_ctx.open,
        set_open: sub_ctx.set_open,
        parent_set_open: Some(parent_ctx.set_open),
        disabled: sub_ctx.disabled,
        focus: sub_ctx.focus,
        initial_focus: sub_ctx.initial_focus,
        trigger_id: sub_ctx.trigger_id,
        trigger_ref: sub_ctx.trigger_ref,
        overlay_id,
        filter_query,
        filter_input_id,
        active_submenu,
    });
    // Floating-element positioning for the submenu. A submenu naturally opens to the
    // left side when there is no room (mirroring the old JS `hasRoomRight ? right :
    // left` logic). The reference is the parent submenu trigger item; the floating
    // element is this sub-content. On native the hook is inert and the
    // `:not([data-floating])` CSS fallback provides the static placement.
    let mut floating_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let pos = use_position(
        sub_ctx.trigger_ref,
        floating_ref,
        ContentSide::Right,
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

    let floating_attrs = attributes!(div {
        position: position(),
        top: top(),
        left: left(),
        visibility: visibility(),
        "data-side": resolved_side.read().as_str(),
        "data-align": resolved_align.read().as_str(),
        "data-floating": floating_active.then_some("true"),
        onmounted: move |evt: MountedEvent| floating_ref.set(Some(evt.data())),
    });
    // Floating props must win over user-forwarded coords → place them last.
    let attributes = merge_attributes(vec![props.attributes, floating_attrs]);

    rsx! {
        MenuContent {
            id: Some(id.cloned()),
            role: props.role,
            attributes,
            {props.children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[component]
    fn ClosedSubmenuFixture() -> Element {
        let open = use_memo(|| true);
        let set_open = use_callback(|_| {});

        rsx! {
            Menu {
                open,
                set_open,
                disabled: false,
                roving_loop: true,
                MenuSub {
                    MenuSubTrigger::<String> {
                        value: "child",
                        index: 0usize,
                        "Child"
                    }
                    MenuSubContent {
                        MenuItem::<String> {
                            value: "grandchild",
                            index: 0usize,
                            "Grandchild"
                        }
                    }
                }
            }
        }
    }

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
                checked: checked(),
                on_select: move |_value: String| {},
                "Checked item"
            }
        }
    }

    #[component]
    fn StringRadioItems(value: ReadSignal<Option<String>>) -> Element {
        rsx! {
            MenuRadioGroup {
                value: value(),
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
        let trigger_ref = use_signal(|| None);
        let overlay_id = use_signal(|| None);
        let checked = use_signal(|| true);
        let initial_focus = use_signal(|| None);
        let radio_value = use_signal(|| Some("one".to_string()));
        let filter_query = use_signal(String::new);
        let filter_input_id = use_signal(|| None);
        let active_submenu = use_signal(|| None);

        use_context_provider(|| MenuContext {
            open,
            set_open,
            parent_set_open: None,
            disabled,
            focus,
            initial_focus,
            trigger_id,
            trigger_ref,
            overlay_id,
            filter_query,
            filter_input_id,
            active_submenu,
        });

        rsx! {
            StringCheckboxItem { checked }
            StringRadioItems { value: radio_value }
        }
    }

    #[component]
    fn FilterableMenuFixture() -> Element {
        let open = use_memo(|| true);
        let set_open = use_callback(|_| {});
        let disabled = use_memo(|| false);
        let roving_loop = use_memo(|| true);

        rsx! {
            FilterableMenu {
                open,
                set_open,
                disabled,
                roving_loop,
                filter_input_props: FilterableMenuInputProps {
                    attributes: attributes!(input { placeholder: "Filter actions" }),
                    ..Default::default()
                },
                MenuItem::<String> {
                    value: "copy".to_string(),
                    index: 0usize,
                    "Copy"
                }
            }
        }
    }

    #[test]
    fn closed_submenu_does_not_render_with_open_parent_menu() {
        let mut dom = VirtualDom::new(ClosedSubmenuFixture);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("Child"));
        assert!(!html.contains("Grandchild"));
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
    fn filterable_menu_renders_filter_input_and_menu_items() {
        let mut dom = VirtualDom::new(FilterableMenuFixture);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("type=\"text\""));
        assert!(html.contains("placeholder=\"Filter actions\""));
        assert!(html.contains("data-slot=\"filterable-menu-separator\""));
        assert!(html.contains("role=\"separator\""));
        assert!(html.contains("role=\"menuitem\""));
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

    /// Overlay-manager migration proof for the menu core (plan §4.2): an open
    /// menu portals its panel through the shared outlet, an in-panel consumer
    /// resolves the re-provided `MenuContext` up the *portaled* render chain, and
    /// the panel carries the manager-assigned `--overlay-z`. Mirrors
    /// `popover::tests::open_popover_portals_and_resolves_popover_ctx_inside_portal`.
    mod overlay {
        use super::super::*;
        use crate::overlay::OverlayProvider;

        /// Resolves `MenuContext` from inside the portaled panel. If the re-provide
        /// were on the wrong scope, `use_context` would panic during render.
        #[component]
        fn MenuCtxProbe() -> Element {
            let ctx: MenuContext = use_context();
            let open = (ctx.open)();
            rsx! {
                span { class: "menu-ctx-probe", "open={open}" }
            }
        }

        #[component]
        fn OpenMenuApp() -> Element {
            let open = use_memo(|| true);
            let set_open = use_callback(|_| {});
            rsx! {
                OverlayProvider {
                    Menu {
                        open,
                        set_open,
                        disabled: ReadSignal::new(Signal::new(false)),
                        roving_loop: ReadSignal::new(Signal::new(true)),
                        MenuContent {
                            MenuCtxProbe {}
                            "panel-marker"
                        }
                    }
                }
            }
        }

        #[test]
        fn open_menu_portals_and_resolves_menu_ctx_inside_portal() {
            let mut dom = VirtualDom::new(OpenMenuApp);
            dom.rebuild_in_place();
            // `use_animated_open` flips `show_in_dom` in an effect, so the portaled
            // content mounts on a subsequent flush. Drain pending effect work.
            for _ in 0..8 {
                let _ = dom.render_immediate_to_vec();
            }
            let html = dioxus_ssr::render(&dom);

            assert!(
                html.contains("panel-marker"),
                "menu panel not rendered via outlet: {html}"
            );
            assert!(
                html.contains("menu-ctx-probe"),
                "in-panel consumer did not resolve MenuContext through the portal: {html}"
            );
            assert!(
                html.contains("--overlay-z: calc(var(--z-overlay-base)"),
                "menu panel did not receive manager --overlay-z: {html}"
            );
        }

        /// A probe that reads the overlay manager's entries and renders a marker
        /// only when some Floating entry registered with a `parent` that points at
        /// another live entry (the parent menu) — i.e. a submenu CHILD entry.
        #[component]
        fn SubmenuParentProbe() -> Element {
            let ctx = crate::overlay::use_overlay();
            let entries = ctx.entries();
            let list = entries.read();
            let has_child = list
                .iter()
                .any(|e| e.parent.is_some() && list.iter().any(|p| Some(p.id) == e.parent));
            rsx! {
                if has_child {
                    span { class: "submenu-has-parent" }
                }
            }
        }

        /// A submenu registered inside an open parent menu must register as a CHILD
        /// overlay entry (`parent = Some(parent menu id)`). This proves the parent
        /// linkage is wired through `MenuContext::overlay_id`.
        #[component]
        fn OpenSubmenuApp() -> Element {
            let open = use_memo(|| true);
            let set_open = use_callback(|_| {});
            rsx! {
                OverlayProvider {
                    SubmenuParentProbe {}
                    Menu {
                        open,
                        set_open,
                        disabled: ReadSignal::new(Signal::new(false)),
                        roving_loop: ReadSignal::new(Signal::new(true)),
                        MenuContent {
                            MenuSub {
                                open: Some(true),
                                MenuSubTrigger::<String> {
                                    value: "sub".to_string(),
                                    index: 0usize,
                                    "Submenu"
                                }
                                MenuSubContent {
                                    "submenu-panel-marker"
                                }
                            }
                        }
                    }
                }
            }
        }

        #[test]
        fn submenu_registers_with_parent_overlay_entry() {
            let mut dom = VirtualDom::new(OpenSubmenuApp);
            dom.rebuild_in_place();
            for _ in 0..16 {
                let _ = dom.render_immediate_to_vec();
            }
            let html = dioxus_ssr::render(&dom);

            assert!(
                html.contains("submenu-panel-marker"),
                "submenu panel not rendered via outlet: {html}"
            );
            assert!(
                html.contains("submenu-has-parent"),
                "no submenu entry registered with a parent overlay id: {html}"
            );
        }
    }
}
