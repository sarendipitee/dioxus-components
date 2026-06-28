//! Defines the [`HoverCard`] component and its subcomponents.

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
struct HoverCardCtx {
    // State
    open: Memo<bool>,
    set_open: Callback<bool>,
    disabled: ReadSignal<bool>,

    // ARIA attributes
    content_id: Signal<String>,

    /// Reference (trigger) element shared with the content so the floating-ui hook can
    /// position the content relative to the trigger. Set by [`HoverCardTrigger`] via
    /// `onmounted`.
    trigger_ref: Signal<Option<Rc<MountedData>>>,
}

/// The props for the [`HoverCard`] component
#[derive(Props, Clone, PartialEq)]
pub struct HoverCardProps {
    /// Whether the hover card is open
    pub open: ReadSignal<Option<bool>>,

    /// Default open state
    #[props(default)]
    pub default_open: bool,

    /// Callback when open state changes
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Whether the hover card is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Additional attributes for the hover card
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the hover card
    pub children: Element,
}

/// # HoverCard
///
/// The `HoverCard` component wraps a [`HoverCardTrigger`] and a [`HoverCardContent`]. It provides a way to show additional information when hovering over an element.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::{
///     ContentAlign, ContentSide,
///     hover_card::{
///         HoverCard, HoverCardContent, HoverCardTrigger,
///     }
/// };
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         HoverCard {
///             HoverCardTrigger {
///                 i { "Dioxus" }
///             }
///             HoverCardContent {
///                 side: ContentSide::Bottom,
///                 div {
///                     padding: "1rem",
///                     "Dioxus is"
///                     i { " the " }
///                     "Rust framework for building fullstack web, desktop, and mobile apps. Iterate with live hotreloading, add server functions, and deploy in record time."
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`HoverCard`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the current state of the hover card. Values are `open` or `closed`.
/// - `data-disabled`: Indicates whether the item is disabled. Values are `true` or `false`.
#[component]
pub fn HoverCard(props: HoverCardProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);
    // Generate a unique ID for the hover card content
    let content_id = use_unique_id();
    let trigger_ref = use_signal(|| None);

    use_context_provider(|| HoverCardCtx {
        open,
        set_open,
        disabled: props.disabled,
        content_id,
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

/// The props for the [`HoverCardTrigger`] component
#[derive(Props, Clone, PartialEq)]
pub struct HoverCardTriggerProps {
    /// Optional ID for the trigger element
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes for the hover card trigger
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the hover card trigger
    pub children: Element,
}

/// # HoverCardTrigger
///
/// The [`HoverCardTrigger`] component triggers the [`HoverCardContent`] to appear when hovered or focused.
///
/// This component must be used inside a [`HoverCard`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::{
///     ContentAlign, ContentSide,
///     hover_card::{
///         HoverCard, HoverCardContent, HoverCardTrigger,
///     }
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         HoverCard {
///             HoverCardTrigger {
///                 i { "Dioxus" }
///             }
///             HoverCardContent {
///                 side: ContentSide::Bottom,
///                 div {
///                     padding: "1rem",
///                     "Dioxus is"
///                     i { " the " }
///                     "Rust framework for building fullstack web, desktop, and mobile apps. Iterate with live hotreloading, add server functions, and deploy in record time."
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn HoverCardTrigger(props: HoverCardTriggerProps) -> Element {
    let ctx: HoverCardCtx = use_context();
    let mut trigger_ref = ctx.trigger_ref;

    // Generate a unique ID for the trigger
    let trigger_id = use_unique_id();

    // Use use_id_or to handle the ID
    let id = use_id_or(trigger_id, props.id);

    // Handle mouse events
    let open_event = move || {
        if !(ctx.disabled)() {
            ctx.set_open.call(true);
        }
    };

    let close_event = move || {
        if !(ctx.disabled)() {
            ctx.set_open.call(false);
        }
    };

    rsx! {
        div {
            id,
            tabindex: "0", // Make the trigger focusable
            onmounted: move |evt: Event<MountedData>| trigger_ref.set(Some(evt.data())),

            // Mouse events
            onmouseenter: move |_| open_event(),
            onmouseleave: move |_| close_event(),

            // Focus events
            onfocus: move |_| open_event(),
            onblur: move |_| close_event(),

            // ARIA attributes
            role: "button",
            aria_describedby: (ctx.open)().then(|| ctx.content_id.cloned()),

            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`HoverCardContent`] component
#[derive(Props, Clone, PartialEq)]
pub struct HoverCardContentProps {
    /// Optional ID for the hover card content
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Side of the trigger to place the hover card
    #[props(default = ContentSide::Top)]
    pub side: ContentSide,

    /// Alignment of the hover card relative to the trigger
    #[props(default = ContentAlign::Center)]
    pub align: ContentAlign,

    /// Whether to force the hover card to stay open when hovered
    #[props(default = true)]
    pub force_mount: bool,

    /// Additional attributes for the hover card content
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the hover card content
    pub children: Element,
}

/// # HoverCardContent
///
/// The [`HoverCardContent`] component defines the content of the parent [`HoverCard`]. It is only rendered when the hover card is open or if [`HoverCardContentProps::force_mount`] is set to true.
///
/// This component must be used inside a [`HoverCard`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::{
///     ContentAlign, ContentSide,
///     hover_card::{
///         HoverCard, HoverCardContent, HoverCardTrigger,
///     }
/// };
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         HoverCard {
///             HoverCardTrigger {
///                 i { "Dioxus" }
///             }
///             HoverCardContent {
///                 side: ContentSide::Bottom,
///                 div {
///                     padding: "1rem",
///                     "Dioxus is"
///                     i { " the " }
///                     "Rust framework for building fullstack web, desktop, and mobile apps. Iterate with live hotreloading, add server functions, and deploy in record time."
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`HoverCardContent`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the current state of the hover card. Values are `open` or `closed`.
/// - `data-side`: Indicates the side of the trigger where the hover card is placed. Values are `top`, `right`, `bottom`, or `left`.
/// - `data-align`: Indicates the alignment of the hover card relative to the trigger. Values are `start`, `center`, or `end`.
#[component]
pub fn HoverCardContent(props: HoverCardContentProps) -> Element {
    let ctx: HoverCardCtx = use_context();

    // HOOKS FIRST: all hooks run unconditionally before any early return so the hook
    // call order is stable across renders (the previous early-return-before-hooks was a
    // conditional-hook bug).
    let id = use_id_or(ctx.content_id, props.id);
    let render = use_animated_open(id, ctx.open);

    // Only render if the hover card is open or force_mount is true.
    let is_open = (ctx.open)();
    if !is_open && !props.force_mount {
        return rsx!({});
    }

    rsx! {
        if render() {
            HoverCardPortaled {
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

/// Props for [`HoverCardPortaled`], the in-portal half of [`HoverCardContent`].
#[derive(Props, Clone, PartialEq)]
struct HoverCardPortaledProps {
    ctx: HoverCardCtx,
    id: Memo<String>,
    side: ContentSide,
    align: ContentAlign,
    attributes: Vec<Attribute>,
    children: Element,
}

/// The portaled half of a hover card: registers the overlay entry as
/// [`OverlayKind::Hint`] (`modal: false`, `dismissable: false` — hints never
/// participate in the manager's dismiss stack), renders the panel through the
/// shared [`OverlayOutlet`], and re-provides [`HoverCardCtx`] inside the portal.
///
/// The hover card keeps its own hover / focus open-close logic (on the trigger,
/// which stays in the main tree, and on the panel's own mouse handlers); it does
/// NOT route through the manager dismiss stack.
#[component]
fn HoverCardPortaled(props: HoverCardPortaledProps) -> Element {
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
    });

    // The body is a CHILD of `PortalIn` so the re-provide lands on the portaled
    // render chain.
    rsx! {
        PortalIn { portal,
            HoverCardContentRendered {
                ctx,
                reg,
                id,
                side: props.side,
                align: props.align,
                attributes: props.attributes.clone(),
                children: props.children,
            }
        }
    }
}

/// Props for [`HoverCardContentRendered`], the portaled DOM body.
#[derive(Props, Clone, PartialEq)]
struct HoverCardContentRenderedProps {
    ctx: HoverCardCtx,
    reg: OverlayRegistration,
    id: Memo<String>,
    side: ContentSide,
    align: ContentAlign,
    attributes: Vec<Attribute>,
    children: Element,
}

/// The rendered hover card content, rendered as a child of `PortalIn` (so this is
/// where [`HoverCardCtx`] is re-provided). Floating positioning runs here off the
/// trigger ref shared through the ctx. Keeps the `visibility:hidden until
/// is_positioned` gate verbatim.
#[component]
fn HoverCardContentRendered(props: HoverCardContentRenderedProps) -> Element {
    let ctx = props.ctx;
    let reg = props.reg;
    let id = props.id;
    let side = props.side;
    let align = props.align;
    let attributes = props.attributes;
    let children = props.children;

    // Re-provide a CLONE of the Root's ctx at the top of the portaled subtree.
    use_context_provider(|| ctx);

    // Handle mouse events to keep the hover card open when hovered
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

    // Floating-element positioning. The content ref is local; the trigger ref is shared
    // through the context. `use_position` is called unconditionally — this component only
    // renders while the card is open, so both refs settle on first mount.
    let mut floating_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let pos = use_position(ctx.trigger_ref, floating_ref, side, align);

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

    let floating_attrs = attributes!(div {
        position: position(),
        top: top(),
        left: left(),
        visibility: visibility(),
        style: z_style,
        "data-floating": floating_active.then_some("true"),
        onmounted: move |evt| floating_ref.set(Some(evt.data())),
    });
    let attributes = merge_attributes(vec![attributes, floating_attrs]);

    rsx! {
        div {
            id,
            role: "tooltip",
            "data-state": if (ctx.open)() { "open" } else { "closed" },
            "data-side": resolved_side.read().as_str(),
            "data-align": resolved_align.read().as_str(),

            // Mouse events to keep the hover card open when hovered
            onmouseenter: handle_mouse_enter,
            onmouseleave: handle_mouse_leave,

            ..attributes,
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    //! Proves the §4.2 re-provide is correctly wired for HoverCard: an open hover
    //! card portals its panel through the overlay outlet, an in-panel consumer
    //! resolves `HoverCardCtx` up the *portaled* render chain, and the panel
    //! carries the manager-assigned `--overlay-z`.
    use super::*;
    use crate::overlay::OverlayProvider;

    /// A test-only consumer that resolves `HoverCardCtx` from inside the portaled
    /// panel. If the re-provide were on the wrong scope, `use_context` would panic.
    #[component]
    fn HoverCardCtxProbe() -> Element {
        let ctx: HoverCardCtx = use_context();
        let open = (ctx.open)();
        rsx! {
            span { class: "hover-card-ctx-probe", "open={open}" }
        }
    }

    #[component]
    fn OpenHoverCardApp() -> Element {
        rsx! {
            OverlayProvider {
                HoverCard {
                    open: Some(true),
                    HoverCardTrigger { "trigger" }
                    HoverCardContent {
                        HoverCardCtxProbe {}
                        "panel-marker"
                    }
                }
            }
        }
    }

    #[test]
    fn open_hover_card_portals_and_resolves_hover_card_ctx_inside_portal() {
        let mut dom = VirtualDom::new(OpenHoverCardApp);
        dom.rebuild_in_place();
        for _ in 0..8 {
            let _ = dom.render_immediate_to_vec();
        }
        let html = dioxus_ssr::render(&dom);

        assert!(
            html.contains("panel-marker"),
            "hover card panel not rendered via outlet: {html}"
        );
        assert!(
            html.contains("hover-card-ctx-probe"),
            "in-panel consumer did not resolve HoverCardCtx through the portal: {html}"
        );
        assert!(
            html.contains("--overlay-z: calc(var(--z-overlay-base)"),
            "hover card panel did not receive manager --overlay-z: {html}"
        );
    }
}

