//! Defines the [`Tooltip`] component and its sub-components, which provide contextual information when hovering or focusing on elements.

use std::rc::Rc;

use crate::{
    floating::{style_prop, use_position}, merge_attributes, use_animated_open, use_controlled,
    use_id_or, use_unique_id, ContentAlign, ContentSide,
};
use dioxus::prelude::*;
use dioxus_attributes::attributes;

#[derive(Clone, Copy)]
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
            TooltipContentRendered {
                id: id(),
                side: props.side,
                align: props.align,
                attributes: props.attributes.clone(),
                children: props.children.clone(),
            }
        }
    }
}

/// The rendered tooltip content. Separated so floating-ui positioning hooks run only
/// while the tooltip is mounted (open), letting [`use_position`] be called
/// unconditionally with both refs settled on first mount.
#[component]
fn TooltipContentRendered(
    id: String,
    side: ContentSide,
    align: ContentAlign,
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx: TooltipCtx = use_context();

    // Floating-element positioning. The content ref is local; the trigger ref is shared
    // through the context. `use_position` is called unconditionally — this component only
    // renders while the tooltip is open, so both refs settle on first mount.
    let mut floating_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let pos = use_position(ctx.trigger_ref, floating_ref, side, align);

    // Split the floating-ui inline coordinates into individual `style:` props so
    // user-forwarded styles are preserved (see popover for the full rationale). The
    // floating props are merged LAST so the computed coordinates win.
    let style = pos.style;
    let is_positioned = pos.is_positioned;
    let resolved_side = pos.side;
    let resolved_align = pos.align;
    let floating_active = pos.floating_active;

    let position = use_memo(move || style_prop(&style.read(), "position"));
    let top = use_memo(move || style_prop(&style.read(), "top"));
    let left = use_memo(move || style_prop(&style.read(), "left"));
    // R4: keep the element `visibility: hidden` until first compute so it does not flash
    // at the origin. `visibility` does not suppress the opacity fade-in animation, and the
    // tooltip's `display:block` open state keeps the element laid out for measurement.
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
            "data-state": if ctx.open.cloned() { "open" } else { "closed" },
            "data-side": resolved_side.read().as_str(),
            "data-align": resolved_align.read().as_str(),
            ..attributes,
            {children}
        }
    }
}

