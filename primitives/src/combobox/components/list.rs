//! Combobox options components.

use std::rc::Rc;

use dioxus::prelude::*;

use super::super::context::ComboboxContext;
use crate::{
    dioxus_attributes::attributes,
    floating::{style_prop, use_position},
    listbox::{use_listbox_container_with_open, use_listbox_id, ListboxContext},
    merge_attributes,
    overlay::{use_overlay_registration, OverlayKind, OverlayRegistration, RegisterArgs},
    portal::{use_portal, PortalIn},
    ContentAlign, ContentSide,
};

/// Props for [`ComboboxOptions`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxOptionsProps {
    /// Optional id for the list element.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children, typically [`ComboboxOption`](super::option::ComboboxOption)s
    /// and an optional [`ComboboxEmpty`](super::empty::ComboboxEmpty).
    pub children: Element,
}

/// Listbox that contains the visible options.
///
/// Root-tree half: provides `ListboxContext` to the closed-branch children (so
/// they register their option state without rendering) and portals the open
/// listbox through the overlay manager.
#[component]
pub fn ComboboxOptions(props: ComboboxOptionsProps) -> Element {
    let ctx = use_context::<ComboboxContext>();
    let open = use_memo(move || ctx.store.dropdown_opened());
    let id = use_listbox_id(props.id, ctx.selectable.list_id);
    let listbox = use_listbox_container_with_open(id, ctx.selectable, open);
    let render = listbox.render;
    let listbox_ctx: ListboxContext = use_context();

    rsx! {
        if render() {
            ComboboxOptionsPortaled {
                ctx,
                listbox_ctx,
                id: listbox.id,
                open,
                attributes: props.attributes.clone(),
                children: props.children,
            }
        } else {
            {props.children}
        }
    }
}

/// Props for [`ComboboxOptionsPortaled`], the in-portal half of [`ComboboxOptions`].
#[derive(Props, Clone, PartialEq)]
struct ComboboxOptionsPortaledProps {
    ctx: ComboboxContext,
    listbox_ctx: ListboxContext,
    id: Memo<String>,
    open: Memo<bool>,
    attributes: Vec<Attribute>,
    children: Element,
}

/// Registers the listbox as an [`OverlayKind::Floating`] entry and renders it
/// through the shared overlay outlet. Routes Escape / outside-click dismissal
/// through the manager.
#[component]
fn ComboboxOptionsPortaled(props: ComboboxOptionsPortaledProps) -> Element {
    let mut ctx = props.ctx;
    let id = props.id;
    let open = props.open;

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

    use_effect(move || {
        reg.set_dom_ids(None, Some(id()));
    });
    use_effect(move || {
        reg.set_closing(!open());
    });

    rsx! {
        PortalIn { portal,
            ComboboxOptionsRendered {
                ctx,
                listbox_ctx: props.listbox_ctx,
                reg,
                id,
                open,
                attributes: props.attributes.clone(),
                children: props.children,
            }
        }
    }
}

/// Props for [`ComboboxOptionsRendered`], the portaled DOM body.
#[derive(Props, Clone, PartialEq)]
struct ComboboxOptionsRenderedProps {
    ctx: ComboboxContext,
    listbox_ctx: ListboxContext,
    reg: OverlayRegistration,
    id: Memo<String>,
    open: Memo<bool>,
    attributes: Vec<Attribute>,
    children: Element,
}

/// The rendered listbox, a direct child of `PortalIn`. Re-provides
/// `ComboboxContext` and `ListboxContext` (context does NOT inherit through the
/// portal) so the portaled options resolve their context and render. Floating
/// positioning runs off the combobox target ref shared through the store; keeps
/// the `visibility:hidden until is_positioned` gate. Emits `--overlay-z`.
#[component]
fn ComboboxOptionsRendered(props: ComboboxOptionsRenderedProps) -> Element {
    let ctx = props.ctx;
    let reg = props.reg;
    let id = props.id;
    let open = props.open;

    // Re-provide both contexts INSIDE the portal (the load-bearing rule).
    use_context_provider(|| ctx);
    use_context_provider(|| props.listbox_ctx);

    // Floating-element positioning. The list drops below the combobox target
    // (side=Bottom, align=Start); flip handles upward placement near the bottom
    // viewport edge and shift slides it into view. The reference element is the
    // combobox target registered in the store (search input or custom target). The
    // listbox only mounts while open, so both refs settle on first open.
    let mut floating_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let pos = use_position(
        ctx.store.target_mount_ref(),
        floating_ref,
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

    // z-index assigned by the overlay manager via open order.
    let z_style = reg.z().map(|z| format!("--overlay-z: {z};"));

    // Floating coords merged LAST so they win over forwarded styles; `data-floating`
    // keeps the native `:not([data-floating])` CSS fallback inert on web.
    let floating_attrs = attributes!(div {
        position: position(),
        top: top(),
        left: left(),
        visibility: visibility(),
        style: z_style,
        "data-floating": floating_active.then_some("true"),
        "data-side": resolved_side.read().as_str(),
        "data-align": resolved_align.read().as_str(),
        onmounted: move |evt| floating_ref.set(Some(evt.data())),
    });
    let attributes = merge_attributes(vec![props.attributes.clone(), floating_attrs]);

    rsx! {
        div {
            id,
            role: "listbox",
            "data-state": if open() { "open" } else { "closed" },
            onpointerdown: move |event| {
                event.prevent_default();
            },
            ..attributes,
            {props.children}
        }
    }
}

/// Compatibility props alias for [`ComboboxOptions`].
pub type ComboboxListProps = ComboboxOptionsProps;

/// Compatibility alias for [`ComboboxOptions`].
#[component]
pub fn ComboboxList(props: ComboboxListProps) -> Element {
    rsx! {
        ComboboxOptions {
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[cfg(test)]
mod tests {
    //! Overlay-manager migration proof for the combobox listbox (plan §4.2): an
    //! open combobox portals its listbox through the shared outlet, an in-panel
    //! consumer resolves the re-provided `ComboboxContext` up the *portaled* render
    //! chain, and the panel carries the manager-assigned `--overlay-z`.
    use super::super::super::context::ComboboxContext;
    use super::super::{Combobox, ComboboxInput, ComboboxOptions};
    use crate::overlay::OverlayProvider;
    use dioxus::prelude::*;

    /// Resolves `ComboboxContext` from inside the portaled listbox. If the
    /// re-provide were on the wrong scope, `use_context` would panic during render.
    #[component]
    fn ComboboxCtxProbe() -> Element {
        let ctx = use_context::<ComboboxContext>();
        let open = ctx.store.dropdown_opened();
        rsx! {
            span { class: "combobox-ctx-probe", "open={open}" }
        }
    }

    #[component]
    fn OpenComboboxApp() -> Element {
        rsx! {
            OverlayProvider {
                Combobox::<String> {
                    default_open: ReadSignal::new(Signal::new(true)),
                    ComboboxInput {}
                    ComboboxOptions {
                        ComboboxCtxProbe {}
                        "panel-marker"
                    }
                }
            }
        }
    }

    #[test]
    fn open_combobox_portals_and_resolves_combobox_ctx_inside_portal() {
        let mut dom = VirtualDom::new(OpenComboboxApp);
        dom.rebuild_in_place();
        // `use_animated_open` flips `show_in_dom` in an effect, so the portaled
        // content mounts on a subsequent flush. Drain pending effect work.
        for _ in 0..8 {
            let _ = dom.render_immediate_to_vec();
        }
        let html = dioxus_ssr::render(&dom);

        assert!(
            html.contains("panel-marker"),
            "combobox listbox not rendered via outlet: {html}"
        );
        assert!(
            html.contains("combobox-ctx-probe"),
            "in-panel consumer did not resolve ComboboxContext through the portal: {html}"
        );
        assert!(
            html.contains("--overlay-z: calc(var(--z-overlay-base)"),
            "combobox listbox did not receive manager --overlay-z: {html}"
        );
    }
}
