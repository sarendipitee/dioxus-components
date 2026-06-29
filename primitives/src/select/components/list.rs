//! SelectList component implementation.

use std::rc::Rc;

use crate::{
    dioxus_attributes::attributes,
    floating::{style_prop, use_position},
    listbox::{use_listbox_container, ListboxContext},
    merge_attributes,
    overlay::{use_overlay_registration, OverlayKind, OverlayRegistration, RegisterArgs},
    portal::{use_portal, PortalIn},
    use_effect, ContentAlign, ContentSide,
};
use dioxus::prelude::*;

use super::super::context::{SelectContext, SelectPortalContext};
use crate::selectable::RcPartialEqValue;

/// The props for the [`SelectList`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectListProps {
    /// The ID of the list for ARIA attributes
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes for the list
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the list
    pub children: Element,
}

/// # SelectList
///
/// The dropdown list container for the [`Select`](super::select::Select) component that contains the
/// [`SelectOption`](super::option::SelectOption)s. The list will only be rendered when the select is open.
///
/// This must be used inside a [`Select`](super::select::Select) component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::select::{
///     Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
///     SelectTrigger, SelectValue,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Select::<String> {
///             SelectTrigger {
///                 aria_label: "Select Trigger",
///                 width: "12rem",
///                 SelectValue { placeholder: "Select a fruit..." }
///             }
///             SelectList {
///                 aria_label: "Select Demo",
///                 SelectGroup {
///                     SelectGroupLabel { "Fruits" }
///                     SelectOption::<String> {
///                         index: 0usize,
///                         value: "apple",
///                         "Apple"
///                         SelectItemIndicator { "✔️" }
///                     }
///                     SelectOption::<String> {
///                         index: 1usize,
///                         value: "banana",
///                         "Banana"
///                         SelectItemIndicator { "✔️" }
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn SelectList(props: SelectListProps) -> Element {
    let ctx = use_context::<SelectContext>();

    // `use_listbox_container` provides `ListboxContext` to THIS (Root-tree) scope so
    // the closed-branch children resolve `render == false` and skip rendering while
    // still registering their option state. It must stay here, not in the portaled
    // body. The listbox id is also generated here.
    let listbox = use_listbox_container(props.id, ctx.selectable);
    let render = listbox.render;
    let listbox_ctx: ListboxContext = use_context();

    rsx! {
        if render() {
            SelectListRegistration {
                children: props.children.clone(),
            }
            SelectListPortaled {
                ctx,
                id: listbox.id,
                should_render: (listbox_ctx.render)(),
                attributes: props.attributes.clone(),
                children: props.children,
            }
        } else {
            // If not rendering, return children directly so we can populate the
            // selected list, but they should choose to not render themselves.
            {props.children}
        }
    }
}

/// Keeps root-tree option registration alive while the visible listbox is portaled.
#[component]
fn SelectListRegistration(children: Element) -> Element {
    let render = use_signal(|| false);
    use_context_provider(|| ListboxContext {
        render: render.into(),
    });

    rsx! {
        {children}
    }
}

/// Props for [`SelectListPortaled`], the in-portal half of [`SelectList`].
#[derive(Props, Clone, PartialEq)]
struct SelectListPortaledProps {
    ctx: SelectContext,
    id: Memo<String>,
    should_render: bool,
    attributes: Vec<Attribute>,
    children: Element,
}

/// Registers the listbox as an [`OverlayKind::Floating`] entry and renders it
/// through the shared [`crate::overlay::OverlayOutlet`]. Registers the trigger id
/// (the select trigger via `aria-controls`/the trigger ref) and the listbox
/// content root id so the manager's union "inside" predicate treats clicks on
/// either subtree as inside, and routes Escape / outside-click through the manager.
#[component]
fn SelectListPortaled(props: SelectListPortaledProps) -> Element {
    let mut ctx = props.ctx;
    let id = props.id;

    let portal = use_portal();

    let on_dismiss = use_callback(move |_| {
        ctx.set_open(false);
    });

    let reg: OverlayRegistration = use_overlay_registration(move || RegisterArgs {
        kind: OverlayKind::Floating,
        portal,
        modal: false,
        dismissable: true,
        on_dismiss,
        parent: None,
        trigger_id: None,
        content_root_id: Some(id.peek().clone()),
        stack_key: None,
    });

    // Keep the manager's "inside" predicate pointed at the live listbox content id.
    use_effect(move || {
        reg.set_dom_ids(None, Some(id()));
    });

    let open = ctx.selectable.open;
    // Exit-phase exclusion: mark `closing` while the list is animating out.
    use_effect(move || {
        reg.set_closing(!open());
    });

    // Subscribe to `open` HERE, in the non-portaled (Root-descendant) scope, and
    // forward the snapshot into the portaled body as a plain bool so the body
    // never reads the Root-owned `open` Memo across the portal boundary.
    let is_open = open();
    let is_multi = ctx.multi();
    let should_render = props.should_render;
    let root_disabled = ctx.selectable.disabled.cloned();
    let selected_values = ctx.selectable.values.read().clone();
    let focused_index = ctx.selectable.focus_state.current_focus();
    let any_focused = ctx.selectable.focus_state.any_focused();
    let options = ctx.selectable.options.read().clone();
    let content_id = id.cloned();
    let overlay_z = reg.z();
    let mut floating_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let on_floating_mounted = use_callback(move |mounted: Rc<MountedData>| {
        floating_ref.set(Some(mounted));
    });
    let pos = use_position(
        ctx.trigger_ref,
        floating_ref,
        ContentSide::Bottom,
        ContentAlign::Start,
    );
    let floating_style = pos.style.read().clone();
    let floating_position = style_prop(&floating_style, "position");
    let floating_top = style_prop(&floating_style, "top");
    let floating_left = style_prop(&floating_style, "left");
    let floating_visibility = if (pos.is_positioned)() {
        "visible".to_string()
    } else {
        "hidden".to_string()
    };
    let floating_side = *pos.side.read();
    let floating_align = *pos.align.read();
    let floating_active = pos.floating_active;
    let mut select_value_ctx = ctx.selectable;
    let select_value = use_callback(move |value: RcPartialEqValue| {
        select_value_ctx.select_value(value);
    });
    let mut set_open_ctx = ctx;
    let set_open = use_callback(move |open: bool| {
        set_open_ctx.set_open(open);
    });
    let mut blur_focus_state = ctx.selectable.focus_state;
    let blur_focus = use_callback(move |_| {
        blur_focus_state.blur();
    });
    let mut learn_ctx = ctx;
    let learn_key = use_callback(move |(code, actual_char): (String, char)| {
        learn_ctx.learn_from_keyboard_event(&code, actual_char);
    });
    let mut typeahead_buffer = ctx.typeahead_buffer;
    let clear_typeahead = use_callback(move |_| {
        typeahead_buffer.take();
    });
    let mut select_current_ctx = ctx;
    let select_current_item = use_callback(move |_| {
        select_current_ctx.select_current_item();
    });
    let mut add_typeahead_ctx = ctx;
    let add_to_typeahead_buffer = use_callback(move |text: String| {
        add_typeahead_ctx.add_to_typeahead_buffer(&text);
    });
    let mut focus_prev_state = ctx.selectable.focus_state;
    let focus_prev = use_callback(move |_| {
        focus_prev_state.focus_prev();
    });
    let mut focus_last_state = ctx.selectable.focus_state;
    let focus_last = use_callback(move |_| {
        focus_last_state.focus_last();
    });
    let mut focus_next_state = ctx.selectable.focus_state;
    let focus_next = use_callback(move |_| {
        focus_next_state.focus_next();
    });
    let mut focus_first_state = ctx.selectable.focus_state;
    let focus_first = use_callback(move |_| {
        focus_first_state.focus_first();
    });

    rsx! {
        PortalIn { portal,
            SelectListRendered {
                is_open,
                is_multi,
                should_render,
                focused: is_open && !any_focused,
                id: content_id,
                overlay_z,
                portal_ctx: SelectPortalContext {
                    root_disabled,
                    selected_values,
                    focused_index,
                    options,
                    select_value,
                    set_open,
                    blur_focus,
                },
                learn_key,
                clear_typeahead,
                select_current_item,
                add_to_typeahead_buffer,
                focus_prev,
                focus_last,
                focus_next,
                focus_first,
                floating_position,
                floating_top,
                floating_left,
                floating_visibility,
                floating_side,
                floating_align,
                floating_active,
                on_floating_mounted,
                attributes: props.attributes.clone(),
                children: props.children,
            }
        }
    }
}

/// Props for [`SelectListRendered`], the portaled DOM body.
#[derive(Props, Clone, PartialEq)]
struct SelectListRenderedProps {
    /// Open snapshot threaded from the non-portaled parent — see the matching
    /// note on `DialogPortalBodyProps::is_open`.
    is_open: bool,
    is_multi: bool,
    should_render: bool,
    focused: bool,
    id: String,
    overlay_z: Option<String>,
    portal_ctx: SelectPortalContext,
    learn_key: Callback<(String, char)>,
    clear_typeahead: Callback<()>,
    select_current_item: Callback<()>,
    add_to_typeahead_buffer: Callback<String>,
    focus_prev: Callback<()>,
    focus_last: Callback<()>,
    focus_next: Callback<()>,
    focus_first: Callback<()>,
    floating_position: String,
    floating_top: String,
    floating_left: String,
    floating_visibility: String,
    floating_side: ContentSide,
    floating_align: ContentAlign,
    floating_active: bool,
    on_floating_mounted: Callback<Rc<MountedData>>,
    attributes: Vec<Attribute>,
    children: Element,
}

/// The rendered listbox, a direct child of `PortalIn`. Re-provides `SelectContext`
/// and `ListboxContext` (context does NOT inherit through the portal) so the
/// portaled options resolve their context and render. Floating layout is
/// snapshotted in the non-portaled parent and forwarded here as plain values.
#[component]
fn SelectListRendered(props: SelectListRenderedProps) -> Element {
    let is_open = props.is_open;
    let id = props.id.clone();
    let is_multi = props.is_multi;
    let should_render = props.should_render;
    let focused = props.focused;
    let overlay_z = props.overlay_z;
    let portal_ctx = props.portal_ctx;
    let learn_key = props.learn_key;
    let clear_typeahead = props.clear_typeahead;
    let select_current_item = props.select_current_item;
    let add_to_typeahead_buffer = props.add_to_typeahead_buffer;
    let focus_prev = props.focus_prev;
    let focus_last = props.focus_last;
    let focus_next = props.focus_next;
    let focus_first = props.focus_first;
    let floating_position = props.floating_position;
    let floating_top = props.floating_top;
    let floating_left = props.floating_left;
    let floating_visibility = props.floating_visibility;
    let floating_side = props.floating_side;
    let floating_align = props.floating_align;
    let floating_active = props.floating_active;
    let on_floating_mounted = props.on_floating_mounted;
    let attributes = props.attributes;
    let children = props.children;
    let mut render_signal = use_signal(|| should_render);

    use_effect(use_reactive(&should_render, move |should_render| {
        render_signal.set(should_render);
    }));
    let portal_ctx_for_provider = portal_ctx.clone();
    let portal_set_open = portal_ctx.set_open;
    use_context_provider(|| portal_ctx_for_provider);
    use_context_provider(|| ListboxContext {
        render: render_signal.into(),
    });
    let mut listbox_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    use_effect(move || {
        let Some(listbox_ref) = listbox_ref() else {
            return;
        };
        if focused {
            spawn(async move {
                _ = listbox_ref.set_focus(true);
            });
        }
    });

    let onkeydown = move |event: KeyboardEvent| {
        let key = event.key();
        let code = event.code();

        // Learn from keyboard events for adaptive matching
        if let Key::Character(actual_char) = &key {
            if let Some(actual_char) = actual_char.chars().next() {
                learn_key.call((code.to_string(), actual_char));
            }
        }

        let arrow_key_navigation = |event: KeyboardEvent| {
            // Clear the typeahead buffer
            clear_typeahead.call(());
            event.prevent_default();
            event.stop_propagation();
        };

        match key {
            Key::Character(new_text) => {
                if new_text == " " {
                    select_current_item.call(());
                    event.prevent_default();
                    event.stop_propagation();
                    return;
                }

                add_to_typeahead_buffer.call(new_text);
            }
            Key::ArrowUp => {
                arrow_key_navigation(event);
                focus_prev.call(());
            }
            Key::End => {
                arrow_key_navigation(event);
                focus_last.call(());
            }
            Key::ArrowDown => {
                arrow_key_navigation(event);
                focus_next.call(());
            }
            Key::Home => {
                arrow_key_navigation(event);
                focus_first.call(());
            }
            Key::Enter => {
                select_current_item.call(());
                event.prevent_default();
                event.stop_propagation();
            }
            Key::Escape => {
                portal_set_open.call(false);
                event.prevent_default();
                event.stop_propagation();
            }
            _ => {}
        }
    };

    // Floating coordinates are split into individual `style:` props (position/top/left)
    // and merged LAST so the computed coordinates win over any user-forwarded style,
    // while leaving every other forwarded style intact (see popover for the rationale).
    // The `data-floating` marker keeps the native `:not([data-floating])` CSS fallback
    // inert on web.
    let floating_attrs = attributes!(div {
        position: floating_position,
        top: floating_top,
        left: floating_left,
        visibility: floating_visibility,
        style: overlay_z.as_ref().map(|z| format!("--overlay-z: {z};")),
        "data-floating": floating_active.then_some("true"),
        "data-side": floating_side.as_str(),
        "data-align": floating_align.as_str(),
    });
    let attributes = merge_attributes(vec![attributes, floating_attrs]);

    rsx! {
        div {
            id,
            role: "listbox",
            tabindex: if focused { "0" } else { "-1" },
            aria_multiselectable: is_multi,

            // Data attributes
            "data-state": if is_open { "open" } else { "closed" },

            onmounted: move |evt| {
                let mounted = evt.data();
                listbox_ref.set(Some(mounted.clone()));
                on_floating_mounted.call(mounted);
            },
            onkeydown,
            onblur: move |_| {
                if focused {
                    portal_set_open.call(false);
                }
            },

            ..attributes,
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    //! Overlay-manager migration proof for the select listbox (plan §4.2): an open
    //! select portals its listbox through the shared outlet, an in-panel consumer
    //! resolves the portal-local select read model up the *portaled* render chain,
    //! and the panel carries the manager-assigned `--overlay-z`. Mirrors
    //! `popover::tests::open_popover_portals_and_resolves_popover_ctx_inside_portal`.
    use super::super::super::context::SelectPortalContext;
    use super::super::{Select, SelectList, SelectTrigger};
    use crate::listbox::ListboxContext;
    use crate::overlay::OverlayProvider;
    use dioxus::prelude::*;

    /// Resolves `SelectPortalContext` from inside the portaled listbox. If the
    /// re-provide were on the wrong scope, `use_context` would panic during render.
    #[component]
    fn SelectCtxProbe() -> Element {
        let render = use_context::<ListboxContext>().render;
        if !render() {
            return rsx! {};
        }
        let ctx = use_context::<SelectPortalContext>();
        let selected = ctx.selected_values.len();
        rsx! {
            span { class: "select-ctx-probe", "selected={selected}" }
        }
    }

    #[component]
    fn OpenSelectApp() -> Element {
        rsx! {
            OverlayProvider {
                Select::<String> {
                    default_open: ReadSignal::new(Signal::new(true)),
                    SelectTrigger { "trigger" }
                    SelectList {
                        SelectCtxProbe {}
                        "panel-marker"
                    }
                }
            }
        }
    }

    #[test]
    fn open_select_portals_and_resolves_select_ctx_inside_portal() {
        let mut dom = VirtualDom::new(OpenSelectApp);
        dom.rebuild_in_place();
        // `use_animated_open` flips `show_in_dom` in an effect, so the portaled
        // content mounts on a subsequent flush. Drain pending effect work.
        for _ in 0..8 {
            let _ = dom.render_immediate_to_vec();
        }
        let html = dioxus_ssr::render(&dom);

        assert!(
            html.contains("panel-marker"),
            "select listbox not rendered via outlet: {html}"
        );
        assert!(
            html.contains("select-ctx-probe"),
            "in-panel consumer did not resolve SelectPortalContext through the portal: {html}"
        );
        assert!(
            html.contains("--overlay-z: calc(var(--z-overlay-base)"),
            "select listbox did not receive manager --overlay-z: {html}"
        );
    }
}
