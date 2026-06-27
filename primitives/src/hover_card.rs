//! Defines the [`HoverCard`] component and its subcomponents.

use std::rc::Rc;

use crate::{
    floating::{style_prop, use_position}, merge_attributes, use_animated_open, use_controlled,
    use_id_or, use_unique_id, ContentAlign, ContentSide,
};
use dioxus::prelude::*;
use dioxus_attributes::attributes;

#[derive(Clone, Copy)]
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
            HoverCardContentRendered {
                id: id(),
                side: props.side,
                align: props.align,
                attributes: props.attributes.clone(),
                children: props.children.clone(),
            }
        }
    }
}

/// The rendered hover card content. Separated so floating-ui positioning hooks run only
/// while the card is mounted (open), letting [`use_position`] be called unconditionally
/// with both refs settled on first mount.
#[component]
fn HoverCardContentRendered(
    id: String,
    side: ContentSide,
    align: ContentAlign,
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx: HoverCardCtx = use_context();

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

    let floating_attrs = attributes!(div {
        position: position(),
        top: top(),
        left: left(),
        visibility: visibility(),
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

