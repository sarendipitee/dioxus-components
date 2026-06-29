//! Defines the [`Tooltip`] component and its sub-components, which provide contextual information when hovering or focusing on elements.

use std::rc::Rc;

use crate::overlay::{use_overlay_registration, OverlayKind, OverlayRegistration, RegisterArgs};
use crate::portal::{use_portal, PortalIn};
use crate::{
    floating::{style_prop, use_position},
    merge_attributes, use_animated_open, use_controlled, use_id_or, use_unique_id, ContentAlign,
    ContentSide,
};
use dioxus::prelude::*;
use dioxus_attributes::attributes;

#[derive(Clone, Copy, PartialEq)]
struct TooltipCtx {
    // State
    open: Memo<bool>,
    set_open: Callback<bool>,
    disabled: ReadSignal<bool>,

    // ARIA attributes
    tooltip_id: Signal<String>,

    /// Reference (trigger) element shared with the content so the floating-ui hook can
    /// position the content relative to the trigger. Set by [`TooltipTrigger`] via
    /// `onmounted`.
    trigger_ref: Signal<Option<Rc<MountedData>>>,
}

/// The props for the [`Tooltip`] component
#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    /// Whether the tooltip is open
    pub open: ReadSignal<Option<bool>>,

    /// Default open state when uncontrolled
    #[props(default)]
    pub default_open: bool,

    /// Callback when open state changes
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Whether the tooltip is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Additional attributes for the tooltip
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the tooltip component, which should include a [`TooltipTrigger`] and a [`TooltipContent`].
    pub children: Element,
}

/// # Tooltip
///
/// The `Tooltip` component provides contextual information when users hover or focus on an
/// element. It consists of a [`TooltipTrigger`] that activates the tooltip and a [`TooltipContent`]
/// that displays the message.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::{tooltip::{Tooltip, TooltipContent, TooltipTrigger}, ContentSide};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Tooltip {
///             TooltipTrigger {
///                 "Rich content"
///             }
///             TooltipContent {
///                 side: ContentSide::Left,
///                 style: "width: 200px;",
///                 h4 { style: "margin-top: 0; margin-bottom: 8px;", "Tooltip title" }
///                 p { style: "margin: 0;", "This tooltip contains rich HTML content with styling." }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Tooltip`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the current state of the tooltip. Values are `open` or `closed`.
/// - `data-disabled`: Indicates if the tooltip is disabled. Values are `true` or `false`.
#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);
    let tooltip_id = use_unique_id();
    let trigger_ref = use_signal(|| None);

    let _ctx = use_context_provider(|| TooltipCtx {
        open,
        set_open,
        disabled: props.disabled,
        tooltip_id,
        trigger_ref,
    });

    rsx! {
        div {
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": (props.disabled)(),
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`TooltipTrigger`] component
#[derive(Props, Clone, PartialEq)]
pub struct TooltipTriggerProps {
    /// Optional ID for the trigger element
    #[props(default)]
    pub id: Option<String>,

    /// Render the trigger element as a custom component/element.
    #[props(default)]
    pub r#as: Option<Callback<Vec<Attribute>, Element>>,

    /// Additional attributes for the trigger element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the trigger element
    pub children: Element,
}

/// # TooltipTrigger
///
/// The trigger element for the [`Tooltip`] component. When users hover over or focus on this element, the tooltip content will be displayed.
///
/// This must be used inside a [`Tooltip`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::{tooltip::{Tooltip, TooltipContent, TooltipTrigger}, ContentSide};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Tooltip {
///             TooltipTrigger {
///                 "Rich content"
///             }
///             TooltipContent {
///                 side: ContentSide::Left,
///                 style: "width: 200px;",
///                 h4 { style: "margin-top: 0; margin-bottom: 8px;", "Tooltip title" }
///                 p { style: "margin: 0;", "This tooltip contains rich HTML content with styling." }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn TooltipTrigger(props: TooltipTriggerProps) -> Element {
    let ctx: TooltipCtx = use_context();
    let mut trigger_ref = ctx.trigger_ref;

    // Handle mouse events
    let handle_mouse_enter = move |_: Event<MouseData>| {
        if !(ctx.disabled)() {
            ctx.set_open.call(true);
        }
    };

    let handle_mouse_leave = move |_: Event<MouseData>| {
        if !(ctx.disabled)() {
            ctx.set_open.call(false);
        }
    };

    // Handle focus events
    let handle_focus = move |_: Event<FocusData>| {
        if !(ctx.disabled)() {
            ctx.set_open.call(true);
        }
    };

    let handle_blur = move |_: Event<FocusData>| {
        if !(ctx.disabled)() {
            ctx.set_open.call(false);
        }
    };

    // Handle keyboard events
    let handle_keydown = move |event: Event<KeyboardData>| {
        if event.key() == Key::Escape && (ctx.open)() {
            event.prevent_default();
            ctx.set_open.call(false);
        }
    };

    let base = attributes!(div {
        id: props.id.clone(),
        tabindex: "0",
        "aria-describedby": ctx.tooltip_id.cloned(),
        onmounted: move |evt: Event<MountedData>| trigger_ref.set(Some(evt.data())),
        onmouseenter: handle_mouse_enter,
        onmouseleave: handle_mouse_leave,
        onfocus: handle_focus,
        onblur: handle_blur,
        onkeydown: handle_keydown,
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    if let Some(dynamic) = props.r#as {
        dynamic.call(merged)
    } else {
        rsx! {
            div {
                ..merged,
                {props.children}
            }
        }
    }
}

/// The props for the [`TooltipContent`] component
#[derive(Props, Clone, PartialEq)]
pub struct TooltipContentProps {
    /// Optional ID for the tooltip content
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Side of the trigger to place the tooltip
    #[props(default = ContentSide::Top)]
    pub side: ContentSide,

    /// Alignment of the tooltip relative to the trigger
    #[props(default = ContentAlign::Center)]
    pub align: ContentAlign,

    /// Additional attributes for the tooltip content element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the tooltip content
    pub children: Element,
}

/// # TooltipContent
///
/// The content component for the [`Tooltip`] that displays the actual tooltip message. The content will only be
/// rendered when the tooltip is open (as controlled by the [`TooltipTrigger`] component).
///
/// This must be used inside a [`Tooltip`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::{tooltip::{Tooltip, TooltipContent, TooltipTrigger}, ContentSide};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Tooltip {
///             TooltipTrigger {
///                 "Rich content"
///             }
///             TooltipContent {
///                 side: ContentSide::Left,
///                 style: "width: 200px;",
///                 h4 { style: "margin-top: 0; margin-bottom: 8px;", "Tooltip title" }
///                 p { style: "margin: 0;", "This tooltip contains rich HTML content with styling." }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`TooltipContent`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the current state of the tooltip. Values are `open` or `closed`.
/// - `data-side`: Indicates which side of the trigger the tooltip is positioned. Values are `top`, `right`, `bottom`, or `left`.
/// - `data-align`: Indicates the alignment of the tooltip. Values are `start`, `center`, or `end`.
#[component]
pub fn TooltipContent(props: TooltipContentProps) -> Element {
    let mut ctx: TooltipCtx = use_context();

    let unique_id = use_unique_id();
    let id = use_id_or(unique_id, props.id);

    use_effect(move || {
        ctx.tooltip_id.set(id());
    });

    // Only render if the tooltip is open
    let render = use_animated_open(id, ctx.open);

    rsx! {
        if render() {
            TooltipPortaled {
                ctx,
                id,
                side: props.side,
                align: props.align,
                attributes: props.attributes.clone(),
                children: props.children.clone(),
            }
        }
    }
}

/// Props for [`TooltipPortaled`], the in-portal half of [`TooltipContent`].
#[derive(Props, Clone, PartialEq)]
struct TooltipPortaledProps {
    ctx: TooltipCtx,
    id: Memo<String>,
    side: ContentSide,
    align: ContentAlign,
    attributes: Vec<Attribute>,
    children: Element,
}

/// The portaled half of a tooltip: registers the overlay entry as
/// [`OverlayKind::Hint`] (`modal: false`, `dismissable: false` — hints never
/// participate in the manager's dismiss stack), renders the panel through the
/// shared [`OverlayOutlet`], and re-provides [`TooltipCtx`] inside the portal.
///
/// The tooltip keeps its own hover / focus / Escape open-close logic on the
/// trigger (which stays in the main tree); it does NOT route through the manager
/// dismiss stack.
#[component]
fn TooltipPortaled(props: TooltipPortaledProps) -> Element {
    let ctx = props.ctx;
    let id = props.id;

    let portal = use_portal();
    let on_dismiss = use_callback(|_| {});

    let reg: OverlayRegistration = use_overlay_registration(move || RegisterArgs {
        kind: OverlayKind::Hint,
        portal,
        modal: false,
        dismissable: false,
        on_dismiss,
        parent: None,
        trigger_id: None,
        content_root_id: Some(id.peek().clone()),
        stack_key: None,
    });

    // Subscribe to `open` HERE, in the non-portaled (Root-descendant) scope, and
    // forward the snapshot into the portaled body as a plain bool so the body
    // never reads the Root-owned `open` Memo across the portal boundary.
    let is_open = ctx.open.cloned();
    let rendered_id = id.cloned();
    let overlay_z = reg.z().map(|z| format!("--overlay-z: {z};"));
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

    // The body is a CHILD of `PortalIn` so the re-provide lands on the portaled
    // render chain.
    rsx! {
        PortalIn { portal,
            TooltipContentRendered {
                ctx,
                is_open,
                id: rendered_id,
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

/// Props for [`TooltipContentRendered`], the portaled DOM body.
#[derive(Props, Clone, PartialEq)]
struct TooltipContentRenderedProps {
    ctx: TooltipCtx,
    /// Open snapshot threaded from the non-portaled parent — see the matching
    /// note on `DialogPortalBodyProps::is_open`.
    is_open: bool,
    id: String,
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

/// The rendered tooltip content, rendered as a child of `PortalIn` (so this is
/// where [`TooltipCtx`] is re-provided). Floating layout is snapshotted in the
/// non-portaled parent and forwarded here as plain values.
#[component]
fn TooltipContentRendered(props: TooltipContentRenderedProps) -> Element {
    let ctx = props.ctx;
    let is_open = props.is_open;
    let id = props.id;
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

    // Re-provide a CLONE of the Root's ctx at the top of the portaled subtree.
    use_context_provider(|| ctx);

    // z-index assigned by the overlay manager via open order.
    let floating_attrs = attributes!(div {
        position: floating_position,
        top: floating_top,
        left: floating_left,
        visibility: floating_visibility,
        style: overlay_z,
        "data-floating": floating_active.then_some("true"),
        onmounted: move |evt| on_floating_mounted.call(evt.data()),
    });
    let attributes = merge_attributes(vec![attributes, floating_attrs]);

    rsx! {
        div {
            id,
            role: "tooltip",
            "data-state": if is_open { "open" } else { "closed" },
            "data-side": floating_side.as_str(),
            "data-align": floating_align.as_str(),
            ..attributes,
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    //! Proves the §4.2 re-provide is correctly wired for Tooltip: an open tooltip
    //! portals its panel through the overlay outlet, an in-panel consumer resolves
    //! `TooltipCtx` up the *portaled* render chain, and the panel carries the
    //! manager-assigned `--overlay-z`.
    use super::*;
    use crate::overlay::OverlayProvider;

    /// A test-only consumer that resolves `TooltipCtx` from inside the portaled
    /// panel. If the re-provide were on the wrong scope, `use_context` would panic.
    #[component]
    fn TooltipCtxProbe() -> Element {
        let ctx: TooltipCtx = use_context();
        let open = (ctx.open)();
        rsx! {
            span { class: "tooltip-ctx-probe", "open={open}" }
        }
    }

    #[component]
    fn OpenTooltipApp() -> Element {
        rsx! {
            OverlayProvider {
                Tooltip {
                    open: Some(true),
                    TooltipTrigger { "trigger" }
                    TooltipContent {
                        TooltipCtxProbe {}
                        "panel-marker"
                    }
                }
            }
        }
    }

    #[test]
    fn open_tooltip_portals_and_resolves_tooltip_ctx_inside_portal() {
        let mut dom = VirtualDom::new(OpenTooltipApp);
        dom.rebuild_in_place();
        for _ in 0..8 {
            let _ = dom.render_immediate_to_vec();
        }
        let html = dioxus_ssr::render(&dom);

        assert!(
            html.contains("panel-marker"),
            "tooltip panel not rendered via outlet: {html}"
        );
        assert!(
            html.contains("tooltip-ctx-probe"),
            "in-panel consumer did not resolve TooltipCtx through the portal: {html}"
        );
        assert!(
            html.contains("--overlay-z: calc(var(--z-overlay-base)"),
            "tooltip panel did not receive manager --overlay-z: {html}"
        );
    }
}
