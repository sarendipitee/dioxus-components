//! Defines the [`Popover`] component and its sub-components.

use std::rc::Rc;

use dioxus::document;
use dioxus::prelude::*;
use dioxus_attributes::attributes;

use crate::overlay::{use_overlay_registration, OverlayKind, OverlayRegistration, RegisterArgs};
use crate::portal::{use_portal, PortalIn};
use crate::{
    floating::{style_prop, use_position},
    merge_attributes, use_animated_open, use_controlled, use_id_or, use_unique_id, ContentAlign,
    ContentSide, FOCUS_TRAP_JS,
};

#[derive(Clone, Copy, PartialEq)]
struct PopoverCtx {
    #[allow(unused)]
    open: Memo<bool>,
    #[allow(unused)]
    set_open: Callback<bool>,

    // Whether the dialog is a modal and should capture focus.
    #[allow(unused)]
    is_modal: ReadSignal<bool>,
    labelledby: Signal<String>,
    root_id: Memo<String>,

    /// Reference (trigger) element shared with the content so the floating-ui hook can
    /// position the content relative to the trigger. Set by [`PopoverTrigger`] /
    /// [`PopoverOpenTrigger`] via `onmounted`.
    trigger_ref: Signal<Option<Rc<MountedData>>>,
}

/// The props for the [`Popover`] component.
#[derive(Props, Clone, PartialEq)]
pub struct PopoverProps {
    /// Whether the popover is a modal and should capture focus.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub is_modal: ReadSignal<bool>,

    /// The controlled open state of the popover.
    pub open: ReadSignal<Option<bool>>,

    /// The default open state when uncontrolled.
    #[props(default)]
    pub default_open: bool,

    /// Callback fired when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// The id of the popover root element.
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes to apply to the popover root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the popover root component.
    pub children: Element,
}

/// # Popover
///
/// The `Popover` component wraps all the popover components and manages the state. You can define a
/// [`PopoverTrigger`] component to toggle the popover's open state, and a [`PopoverContent`] component
/// to define the content that appears when the popover is open under this component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::popover::*;
///
/// #[component]
/// fn Demo() -> Element {
///     let mut open = use_signal(|| false);
///
///     rsx! {
///         Popover {
///             open: open(),
///             on_open_change: move |v| open.set(v),
///             PopoverTrigger {
///                 "Show Popover"
///             }
///             PopoverContent {
///                 gap: "0.25rem",
///                 h3 {
///                     padding_top: "0.25rem",
///                     padding_bottom: "0.25rem",
///                     width: "100%",
///                     text_align: "center",
///                     margin: 0,
///                     "Delete Item?"
///                 }
///                 button {
///                     onclick: move |_| {
///                         open.set(false);;
///                     },
///                     "Yes!"
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Popover`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates if the popover is open or closed. Values are `open` or `closed`.
#[component]
pub fn Popover(props: PopoverProps) -> Element {
    let labelledby = use_unique_id();
    let gen_root_id = use_unique_id();
    let root_id = use_id_or(gen_root_id, props.id);

    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    let trigger_ref = use_signal(|| None);

    use_context_provider(|| PopoverCtx {
        open,
        set_open,
        is_modal: props.is_modal,
        labelledby,
        root_id,
        trigger_ref,
    });

    rsx! {
        div {
            id: root_id,
            "data-state": if open() { "open" } else { "closed" },
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`PopoverContent`] component.
#[derive(Props, Clone, PartialEq)]
pub struct PopoverContentProps {
    /// The id of the popover content element.
    pub id: ReadSignal<Option<String>>,

    /// Side of the trigger to place the popover.
    #[props(default = ContentSide::Bottom)]
    pub side: ContentSide,

    /// Alignment of the popover relative to the trigger.
    #[props(default = ContentAlign::Center)]
    pub align: ContentAlign,

    /// Additional attributes to apply to the content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the popover content component.
    pub children: Element,
}

/// # PopoverContent
///
/// The `PopoverContent` component defines the content of the popover. This component will
/// only be rendered if the popover is open, and it will handle focus trapping if the popover is modal.
///
/// This must be used inside a [`Popover`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::popover::*;
///
/// #[component]
/// fn Demo() -> Element {
///     let mut open = use_signal(|| false);
///
///     rsx! {
///         Popover {
///             open: open(),
///             on_open_change: move |v| open.set(v),
///             PopoverTrigger {
///                 "Show Popover"
///             }
///             PopoverContent {
///                 gap: "0.25rem",
///                 h3 {
///                     padding_top: "0.25rem",
///                     padding_bottom: "0.25rem",
///                     width: "100%",
///                     text_align: "center",
///                     margin: 0,
///                     "Delete Item?"
///                 }
///                 button {
///                     onclick: move |_| {
///                         open.set(false);;
///                     },
///                     "Yes!"
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`PopoverContent`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates if the popover is open or closed. Values are `open` or `closed`.
/// - `data-side`: Indicates the side where the popover is positioned relative to the trigger. Possible values are `top`, `right`, `bottom`, and `left`.
/// - `data-align`: Indicates the alignment of the popover relative to the trigger. Possible values are `start`, `center`, and `end`.
#[component]
pub fn PopoverContent(props: PopoverContentProps) -> Element {
    let ctx: PopoverCtx = use_context();
    let open = ctx.open;
    let is_modal = ctx.is_modal;
    let base = attributes!(div {
        class: "dx-popover-content"
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    let gen_id = use_unique_id();
    let id = use_id_or(gen_id, props.id);

    let render = use_animated_open(id, ctx.open);

    use_effect(move || {
        if !render() {
            return;
        }
        let is_modal = is_modal();
        if !is_modal {
            // If the dialog is not modal, we don't need to trap focus.
            return;
        }

        let eval = document::eval(
            r#"let id = await dioxus.recv();
            let is_open = await dioxus.recv();
            let dialog = document.getElementById(id);

            if (is_open) {
                dialog.trap = window.createFocusTrap(dialog);
            }
            if (!is_open && dialog.trap) {
                dialog.trap.remove();
                dialog.trap = null;
            }"#,
        );
        let _ = eval.send(id.to_string());
        let _ = eval.send(open.cloned());
    });

    rsx! {
        document::Script { src: FOCUS_TRAP_JS, defer: true }
        if render() {
            PopoverPortaled {
                ctx,
                id,
                side: props.side,
                align: props.align,
                attributes,
                children: props.children,
            }
        }
    }
}

/// Props for [`PopoverPortaled`], the in-portal half of [`PopoverContent`].
#[derive(Props, Clone, PartialEq)]
struct PopoverPortaledProps {
    ctx: PopoverCtx,
    id: Memo<String>,
    side: ContentSide,
    align: ContentAlign,
    attributes: Vec<Attribute>,
    children: Element,
}

/// The portaled half of a popover: registers the overlay entry as
/// [`OverlayKind::Floating`], renders the panel through the shared
/// [`OverlayOutlet`], and re-provides [`PopoverCtx`] inside the portal so any
/// in-panel consumers resolve their context up the *portaled* render chain
/// (context does not inherit through the portal).
///
/// The Escape + outside-click dismissal that the popover used to own
/// (`use_global_escape_listener` / `use_outside_dismiss`) is now handled by the
/// manager's central dismiss stack. This entry registers its trigger id
/// (`ctx.labelledby`, the trigger element's id) and content root id (the panel
/// `id`) so the union "inside" predicate treats clicks on the portaled panel —
/// which now lives in a different DOM subtree than the trigger — as inside.
#[component]
fn PopoverPortaled(props: PopoverPortaledProps) -> Element {
    let ctx = props.ctx;
    let set_open = ctx.set_open;
    let open = ctx.open;
    let id = props.id;
    // The trigger element carries `id = ctx.labelledby` (see PopoverTrigger), so
    // the union "inside" predicate can test the trigger subtree by that id.
    let trigger_id = ctx.labelledby;

    let portal = use_portal();

    let on_dismiss = use_callback(move |_| {
        set_open.call(false);
    });

    let reg: OverlayRegistration = use_overlay_registration(move || RegisterArgs {
        kind: OverlayKind::Floating,
        portal,
        modal: false,
        dismissable: true,
        on_dismiss,
        parent: None,
        trigger_id: Some(trigger_id.peek().clone()),
        content_root_id: Some(id.peek().clone()),
        stack_key: None,
    });

    // Keep the manager's "inside" predicate pointed at the live trigger + content
    // ids (the unique ids may resolve after first mount).
    use_effect(move || {
        reg.set_dom_ids(Some(trigger_id()), Some(id()));
    });

    // Exit-phase exclusion: mark `closing` while open == false but still
    // rendering (animating out).
    use_effect(move || {
        reg.set_closing(!open());
    });

    // Subscribe to `open` HERE, in the non-portaled (Root-descendant) scope, and
    // forward the snapshot into the portaled body as a plain bool so the body
    // never reads the Root-owned `open` Memo across the portal boundary. The
    // parent re-renders synchronously when `open` flips, keeping data-state in
    // lockstep with the open/close animation.
    let is_open = open();
    let rendered_id = id.cloned();
    let overlay_z = reg.z().map(|z| format!("--overlay-z: {z};"));
    let is_modal = (ctx.is_modal)();
    let labelledby = ctx.labelledby.cloned();
    let mut floating_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let on_floating_mounted = use_callback(move |mounted: Rc<MountedData>| {
        floating_ref.set(Some(mounted));
    });
    let pos = use_position(ctx.trigger_ref, floating_ref, props.side, props.align);
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

    // The body is a CHILD of `PortalIn` so the re-provide lands on the *portaled*
    // render chain — the only place a portaled consumer resolves it.
    rsx! {
        PortalIn { portal,
            PopoverContentRendered {
                ctx,
                is_open,
                id: rendered_id,
                is_modal,
                labelledby,
                overlay_z,
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

/// Props for [`PopoverContentRendered`], the portaled DOM body.
#[derive(Props, Clone, PartialEq)]
pub struct PopoverContentRenderedProps {
    ctx: PopoverCtx,
    /// Open snapshot threaded from the non-portaled parent — see the matching
    /// note on `DialogPortalBodyProps::is_open`. The body must not read the
    /// Root-owned `ctx.open` Memo across the portal boundary.
    is_open: bool,
    id: String,
    is_modal: bool,
    labelledby: String,
    overlay_z: Option<String>,
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

/// The rendered content of the popover, rendered as a child of `PortalIn` (so
/// this is where [`PopoverCtx`] is re-provided). Floating layout is snapshotted
/// in the non-portaled parent and forwarded here as plain values.
#[component]
pub fn PopoverContentRendered(props: PopoverContentRenderedProps) -> Element {
    let ctx = props.ctx;
    let is_open = props.is_open;
    let id = props.id;
    let is_modal = props.is_modal;
    let labelledby = props.labelledby;
    let overlay_z = props.overlay_z;
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

    // Re-provide a CLONE of the Root's ctx at the top of the portaled subtree so
    // in-panel consumers resolve PopoverCtx up THIS (portaled) render chain.
    use_context_provider(|| ctx);

    // z-index assigned by the overlay manager via open order. Emitted as a raw
    // `style` string (name=""), so it coexists with the per-property
    // position/top/left styles under `merge_attributes` (merged by name).
    let floating_attrs = attributes!(div {
        position: floating_position,
        top: floating_top,
        left: floating_left,
        visibility: floating_visibility,
        style: overlay_z,
        "data-floating": floating_active.then_some("true"),
        onmounted: move |evt| on_floating_mounted.call(evt.data()),
    });
    // Floating props must win over user-forwarded coords → place them in the last list.
    let attributes = merge_attributes(vec![attributes, floating_attrs]);

    rsx! {
        div {
            id,
            role: "dialog",
            aria_modal: is_modal.then_some("true"),
            aria_labelledby: labelledby,
            aria_hidden: (!is_open).then_some("true"),
            "data-state": if is_open { "open" } else { "closed" },
            "data-side": floating_side.as_str(),
            "data-align": floating_align.as_str(),
            ..attributes,
            {children}
        }
    }
}

/// The props for the [`PopoverTrigger`] component.
#[derive(Props, Clone, PartialEq)]
pub struct PopoverTriggerProps {
    /// Additional attributes to apply to the trigger element.
    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    pub attributes: Vec<Attribute>,

    /// The children of the trigger component.
    pub children: Element,
}

fn use_popover_trigger_labelledby(
    mut labelledby: Signal<String>,
    attributes: &[Attribute],
) -> Signal<String> {
    let id_attribute = attributes.iter().find(|attr| attr.name == "id").cloned();
    use_effect(use_reactive!(|id_attribute| {
        if let Some(id_attribute) = id_attribute {
            match &id_attribute.value {
                dioxus_core::AttributeValue::Text(val) => labelledby.set(val.to_string()),
                dioxus_core::AttributeValue::Float(val) => labelledby.set(val.to_string()),
                dioxus_core::AttributeValue::Int(val) => labelledby.set(val.to_string()),
                dioxus_core::AttributeValue::Bool(val) => labelledby.set(val.to_string()),
                _ => {}
            }
        }
    }));
    labelledby
}

/// # PopoverTrigger
///
/// The `PopoverTrigger` toggles the visibility of the [`PopoverContent`].
///
/// This must be used inside a [`Popover`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::popover::*;
///
/// #[component]
/// fn Demo() -> Element {
///     let mut open = use_signal(|| false);
///
///     rsx! {
///         Popover {
///             open: open(),
///             on_open_change: move |v| open.set(v),
///             PopoverTrigger {
///                 "Show Popover"
///             }
///             PopoverContent {
///                 gap: "0.25rem",
///                 h3 {
///                     padding_top: "0.25rem",
///                     padding_bottom: "0.25rem",
///                     width: "100%",
///                     text_align: "center",
///                     margin: 0,
///                     "Delete Item?"
///                 }
///                 button {
///                     onclick: move |_| {
///                         open.set(false);;
///                     },
///                     "Yes!"
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn PopoverTrigger(props: PopoverTriggerProps) -> Element {
    let ctx: PopoverCtx = use_context();
    let mut trigger_ref = ctx.trigger_ref;
    let id = use_popover_trigger_labelledby(ctx.labelledby, &props.attributes);

    rsx! {
        div {
            id,
            onmounted: move |evt| trigger_ref.set(Some(evt.data())),
            onclick: move |e| {
                e.stop_propagation();
                ctx.set_open.call(!(ctx.open)());
            },
            ..props.attributes,
            {props.children}
        }
    }
}

/// # PopoverOpenTrigger
///
/// Trigger that opens the [`PopoverContent`] without toggling it closed.
///
/// This is useful for input adornments where the associated field can also open the popover
/// and clicking the adornment should be an idempotent open action.
#[component]
pub fn PopoverOpenTrigger(props: PopoverTriggerProps) -> Element {
    let ctx: PopoverCtx = use_context();
    let mut trigger_ref = ctx.trigger_ref;
    let id = use_popover_trigger_labelledby(ctx.labelledby, &props.attributes);

    rsx! {
        div {
            id,
            onmounted: move |evt| trigger_ref.set(Some(evt.data())),
            onclick: move |e| {
                e.stop_propagation();
                ctx.set_open.call(true);
            },
            ..props.attributes,
            {props.children}
        }
    }
}

#[cfg(test)]
mod tests {
    //! Proves the §4.2 re-provide is correctly wired for Popover: an open popover
    //! portals its panel through the overlay outlet, an in-panel consumer resolves
    //! `PopoverCtx` up the *portaled* render chain, and the panel carries the
    //! manager-assigned `--overlay-z`.
    use super::*;
    use crate::overlay::OverlayProvider;

    /// A test-only consumer that resolves `PopoverCtx` from inside the portaled
    /// panel. If the re-provide were on the wrong scope, `use_context` would panic
    /// during render and fail this test.
    #[component]
    fn PopoverCtxProbe() -> Element {
        let ctx: PopoverCtx = use_context();
        let open = (ctx.open)();
        rsx! {
            span { class: "popover-ctx-probe", "open={open}" }
        }
    }

    #[component]
    fn OpenPopoverApp() -> Element {
        rsx! {
            OverlayProvider {
                Popover {
                    open: Some(true),
                    PopoverTrigger { "trigger" }
                    PopoverContent {
                        PopoverCtxProbe {}
                        "panel-marker"
                    }
                }
            }
        }
    }

    #[test]
    fn open_popover_portals_and_resolves_popover_ctx_inside_portal() {
        let mut dom = VirtualDom::new(OpenPopoverApp);
        dom.rebuild_in_place();
        // `use_animated_open` flips `show_in_dom` in an effect, so the portaled
        // content mounts on a subsequent flush. Drain pending effect-driven work.
        for _ in 0..8 {
            let _ = dom.render_immediate_to_vec();
        }
        let html = dioxus_ssr::render(&dom);

        // The panel rendered through the outlet.
        assert!(
            html.contains("panel-marker"),
            "popover panel not rendered via outlet: {html}"
        );
        // The in-panel consumer resolved the re-provided PopoverCtx in the portal.
        assert!(
            html.contains("popover-ctx-probe"),
            "in-panel consumer did not resolve PopoverCtx through the portal: {html}"
        );
        // The panel carries the manager-assigned --overlay-z.
        assert!(
            html.contains("--overlay-z: calc(var(--z-overlay-base)"),
            "popover panel did not receive manager --overlay-z: {html}"
        );
    }
}
