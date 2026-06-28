//! SelectList component implementation.

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

use super::super::context::SelectContext;

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
            SelectListPortaled {
                ctx,
                listbox_ctx,
                id: listbox.id,
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

/// Props for [`SelectListPortaled`], the in-portal half of [`SelectList`].
#[derive(Props, Clone, PartialEq)]
struct SelectListPortaledProps {
    ctx: SelectContext,
    listbox_ctx: ListboxContext,
    id: Memo<String>,
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

    rsx! {
        PortalIn { portal,
            SelectListRendered {
                ctx,
                listbox_ctx: props.listbox_ctx,
                reg,
                is_open,
                id,
                attributes: props.attributes.clone(),
                children: props.children,
            }
        }
    }
}

/// Props for [`SelectListRendered`], the portaled DOM body.
#[derive(Props, Clone, PartialEq)]
struct SelectListRenderedProps {
    ctx: SelectContext,
    listbox_ctx: ListboxContext,
    reg: OverlayRegistration,
    /// Open snapshot threaded from the non-portaled parent — see the matching
    /// note on `DialogPortalBodyProps::is_open`.
    is_open: bool,
    id: Memo<String>,
    attributes: Vec<Attribute>,
    children: Element,
}

/// The rendered listbox, a direct child of `PortalIn`. Re-provides `SelectContext`
/// and `ListboxContext` (context does NOT inherit through the portal) so the
/// portaled options resolve their context and render. Floating positioning runs
/// here off the trigger ref shared through the ctx; keeps the `visibility:hidden
/// until is_positioned` gate verbatim. Emits `--overlay-z` from the manager.
#[component]
fn SelectListRendered(props: SelectListRenderedProps) -> Element {
    let mut ctx = props.ctx;
    let reg = props.reg;
    let id = props.id;

    // Re-provide both contexts INSIDE the portal (the load-bearing rule).
    use_context_provider(|| ctx);
    use_context_provider(|| props.listbox_ctx);

    let is_open = props.is_open;
    let mut listbox_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);
    let focused = move || is_open && !ctx.selectable.focus_state.any_focused();

    // Floating-element positioning. The list drops below the trigger (side=Bottom,
    // align=Start); flip handles upward placement near the bottom viewport edge and
    // shift slides it back into view. `use_position` is called unconditionally; the
    // listbox only mounts while open so both refs settle on first open.
    let pos = use_position(
        ctx.trigger_ref,
        listbox_ref,
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

    use_effect(move || {
        let Some(listbox_ref) = listbox_ref() else {
            return;
        };
        if focused() {
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
                ctx.learn_from_keyboard_event(&code.to_string(), actual_char);
            }
        }

        let mut arrow_key_navigation = |event: KeyboardEvent| {
            // Clear the typeahead buffer
            ctx.typeahead_buffer.take();
            event.prevent_default();
            event.stop_propagation();
        };

        match key {
            Key::Character(new_text) => {
                if new_text == " " {
                    ctx.select_current_item();
                    event.prevent_default();
                    event.stop_propagation();
                    return;
                }

                ctx.add_to_typeahead_buffer(&new_text);
            }
            Key::ArrowUp => {
                arrow_key_navigation(event);
                ctx.selectable.focus_state.focus_prev();
            }
            Key::End => {
                arrow_key_navigation(event);
                ctx.selectable.focus_state.focus_last();
            }
            Key::ArrowDown => {
                arrow_key_navigation(event);
                ctx.selectable.focus_state.focus_next();
            }
            Key::Home => {
                arrow_key_navigation(event);
                ctx.selectable.focus_state.focus_first();
            }
            Key::Enter => {
                ctx.select_current_item();
                event.prevent_default();
                event.stop_propagation();
            }
            Key::Escape => {
                ctx.set_open(false);
                event.prevent_default();
                event.stop_propagation();
            }
            _ => {}
        }
    };

    // z-index assigned by the overlay manager via open order.
    let z_style = reg.z().map(|z| format!("--overlay-z: {z};"));

    // Floating coordinates are split into individual `style:` props (position/top/left)
    // and merged LAST so the computed coordinates win over any user-forwarded style,
    // while leaving every other forwarded style intact (see popover for the rationale).
    // The `data-floating` marker keeps the native `:not([data-floating])` CSS fallback
    // inert on web.
    let floating_attrs = attributes!(div {
        position: position(),
        top: top(),
        left: left(),
        visibility: visibility(),
        style: z_style,
        "data-floating": floating_active.then_some("true"),
        "data-side": resolved_side.read().as_str(),
        "data-align": resolved_align.read().as_str(),
    });
    let attributes = merge_attributes(vec![props.attributes.clone(), floating_attrs]);

    rsx! {
        div {
            id,
            role: "listbox",
            tabindex: if focused() { "0" } else { "-1" },
            aria_multiselectable: ctx.multi(),

            // Data attributes
            "data-state": if is_open { "open" } else { "closed" },

            onmounted: move |evt| listbox_ref.set(Some(evt.data())),
            onkeydown,
            onblur: move |_| {
                if focused() {
                    ctx.set_open(false);
                }
            },

            ..attributes,
            {props.children}
        }
    }
}

#[cfg(test)]
mod tests {
    //! Overlay-manager migration proof for the select listbox (plan §4.2): an open
    //! select portals its listbox through the shared outlet, an in-panel consumer
    //! resolves the re-provided `SelectContext` up the *portaled* render chain, and
    //! the panel carries the manager-assigned `--overlay-z`. Mirrors
    //! `popover::tests::open_popover_portals_and_resolves_popover_ctx_inside_portal`.
    use super::super::super::context::SelectContext;
    use super::super::{Select, SelectList, SelectTrigger};
    use crate::overlay::OverlayProvider;
    use dioxus::prelude::*;

    /// Resolves `SelectContext` from inside the portaled listbox. If the re-provide
    /// were on the wrong scope, `use_context` would panic during render.
    #[component]
    fn SelectCtxProbe() -> Element {
        let ctx = use_context::<SelectContext>();
        let open = (ctx.selectable.open)();
        rsx! {
            span { class: "select-ctx-probe", "open={open}" }
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
            "in-panel consumer did not resolve SelectContext through the portal: {html}"
        );
        assert!(
            html.contains("--overlay-z: calc(var(--z-overlay-base)"),
            "select listbox did not receive manager --overlay-z: {html}"
        );
    }
}
