//! Combobox options components.

use std::rc::Rc;

use dioxus::prelude::*;

use super::super::{
    context::{ComboboxContext, ComboboxPortalContext},
    hook::ComboboxDropdownEventSource,
};
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
            ComboboxOptionsRegistration {
                children: props.children.clone(),
            }
            ComboboxOptionsPortaled {
                ctx,
                id: listbox.id,
                open,
                should_render: (listbox_ctx.render)(),
                attributes: props.attributes.clone(),
                children: props.children,
            }
        } else {
            {props.children}
        }
    }
}

/// Keeps root-tree option registration alive while the visible listbox is portaled.
#[component]
fn ComboboxOptionsRegistration(children: Element) -> Element {
    let render = use_signal(|| false);
    use_context_provider(|| ListboxContext {
        render: render.into(),
    });

    rsx! {
        {children}
    }
}

/// Props for [`ComboboxOptionsPortaled`], the in-portal half of [`ComboboxOptions`].
#[derive(Props, Clone, PartialEq)]
struct ComboboxOptionsPortaledProps {
    ctx: ComboboxContext,
    id: Memo<String>,
    open: Memo<bool>,
    should_render: bool,
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
        stack_key: None,
    });

    use_effect(move || {
        reg.set_dom_ids(None, Some(id()));
    });
    use_effect(move || {
        reg.set_closing(!open());
    });

    // Subscribe to `open` HERE, in the non-portaled (Root-descendant) scope, and
    // forward the snapshot into the portaled body as a plain bool so the body
    // never reads the Root-owned `open` Memo across the portal boundary.
    let is_open = open();
    let should_render = props.should_render;
    let content_id = id.cloned();
    let overlay_z = reg.z();
    let root_disabled = ctx.selectable.disabled.cloned();
    let selected_values = ctx.selectable.values.read().clone();
    let focused_index = ctx.selectable.focus_state.current_focus();
    let highlighted_index = ctx.store.highlighted_option_index();
    let options = ctx.selectable.options.read().clone();
    let visible_indices = options
        .iter()
        .filter(|option| ctx.is_visible_text(option.tab_index, option.text_value.clone()))
        .map(|option| option.tab_index)
        .collect::<Vec<_>>();
    let has_visible_options = !visible_indices.is_empty();

    // Snapshot every floating-layout value HERE so the portaled body never reads
    // Root-owned position memos or the combobox store's target ref.
    let mut floating_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let on_floating_mounted = use_callback(move |mounted: Rc<MountedData>| {
        floating_ref.set(Some(mounted));
    });
    let pos = use_position(
        ctx.store.target_mount_ref(),
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
    let mut submit_ctx = ctx;
    let submit_index_from_mouse = use_callback(move |index: usize| {
        submit_ctx.submit_index(index, ComboboxDropdownEventSource::Mouse);
    });

    rsx! {
        PortalIn { portal,
            ComboboxOptionsRendered {
                is_open,
                should_render,
                id: content_id,
                overlay_z,
                portal_ctx: ComboboxPortalContext {
                    selectable: ctx.selectable,
                    store: ctx.store,
                    root_disabled,
                    selected_values,
                    focused_index,
                    highlighted_index,
                    options,
                    visible_indices: Some(visible_indices),
                    has_visible_options,
                    register_options: false,
                    submit_index_from_mouse,
                },
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

/// Props for [`ComboboxOptionsRendered`], the portaled DOM body.
#[derive(Props, Clone, PartialEq)]
struct ComboboxOptionsRenderedProps {
    /// Open snapshot threaded from the non-portaled parent — see the matching
    /// note on `DialogPortalBodyProps::is_open`.
    is_open: bool,
    should_render: bool,
    /// Snapshotted in the non-portaled parent so the portaled body never reads
    /// the Root-owned listbox id memo across the portal boundary.
    id: String,
    /// Overlay z metadata, snapshotted in the non-portaled parent where the
    /// backing `OverlayRegistration` id signal is owned.
    overlay_z: Option<String>,
    portal_ctx: ComboboxPortalContext,
    /// Floating layout fields, snapshotted in the non-portaled parent so the
    /// portaled body never reads Root-owned positioning memos.
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

/// The rendered listbox, a direct child of `PortalIn`. Re-provides the portal
/// combobox read model and `ListboxContext` so portaled options resolve local
/// snapshots instead of root-owned reactive reads. Floating layout and
/// `--overlay-z` are snapshotted in the non-portaled parent and forwarded here
/// as plain values.
#[component]
fn ComboboxOptionsRendered(props: ComboboxOptionsRenderedProps) -> Element {
    let is_open = props.is_open;
    let should_render = props.should_render;

    let mut render_signal = use_signal(|| should_render);
    use_effect(use_reactive(&should_render, move |should_render| {
        render_signal.set(should_render);
    }));
    use_context_provider(|| props.portal_ctx);
    use_context_provider(|| ListboxContext {
        render: render_signal.into(),
    });
    let on_floating_mounted = props.on_floating_mounted;
    let z_style = props
        .overlay_z
        .as_ref()
        .map(|z| format!("--overlay-z: {z};"));

    // Floating coords merged LAST so they win over forwarded styles; `data-floating`
    // keeps the native `:not([data-floating])` CSS fallback inert on web.
    let floating_attrs = attributes!(div {
        position: props.floating_position.clone(),
        top: props.floating_top.clone(),
        left: props.floating_left.clone(),
        visibility: props.floating_visibility.clone(),
        style: z_style,
        "data-floating": props.floating_active.then_some("true"),
        "data-side": props.floating_side.as_str(),
        "data-align": props.floating_align.as_str(),
        onmounted: move |event| on_floating_mounted.call(event.data()),
    });
    let attributes = merge_attributes(vec![props.attributes.clone(), floating_attrs]);

    rsx! {
        div {
            id: props.id.clone(),
            role: "listbox",
            "data-state": if is_open { "open" } else { "closed" },
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
    //! consumer resolves the portal-local combobox read model up the *portaled*
    //! render chain, and the panel carries the manager-assigned `--overlay-z`.
    use super::super::super::context::ComboboxPortalContext;
    use super::super::{Combobox, ComboboxInput, ComboboxOptions};
    use crate::listbox::ListboxContext;
    use crate::overlay::OverlayProvider;
    use dioxus::prelude::*;

    /// Resolves `ComboboxPortalContext` from inside the portaled listbox. If the
    /// re-provide were on the wrong scope, `use_context` would panic during render.
    #[component]
    fn ComboboxCtxProbe() -> Element {
        let render = use_context::<ListboxContext>().render;
        if !render() {
            return rsx! {};
        }
        let ctx = use_context::<ComboboxPortalContext>();
        let selected = ctx.selected_values.len();
        rsx! {
            span { class: "combobox-ctx-probe", "selected={selected}" }
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
            "in-panel consumer did not resolve ComboboxPortalContext through the portal: {html}"
        );
        assert!(
            html.contains("--overlay-z: calc(var(--z-overlay-base)"),
            "combobox listbox did not receive manager --overlay-z: {html}"
        );
    }
}
