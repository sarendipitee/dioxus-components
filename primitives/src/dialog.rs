//! Defines the [`DialogRoot`] component and its sub-components.

use dioxus::prelude::*;
use dioxus_attributes::attributes;

use crate::overlay::{use_overlay_registration, OverlayKind, OverlayRegistration, RegisterArgs};
use crate::portal::{use_portal, PortalIn};
use crate::{merge_attributes, use_animated_open, use_controlled, use_id_or, use_unique_id};

/// Context for the [`DialogRoot`] component
#[derive(Clone, Copy, PartialEq)]
pub struct DialogCtx {
    #[allow(unused)]
    open: Memo<bool>,
    /// Callback to set the open state of the dialog.
    #[allow(unused)]
    set_open: Callback<bool>,

    // Whether the dialog is a modal and should capture focus.
    #[allow(unused)]
    is_modal: ReadSignal<bool>,
    dialog_labelledby: Signal<String>,
    dialog_describedby: Signal<String>,
}

impl DialogCtx {
    /// Returns whether the dialog is open.
    pub fn is_open(&self) -> bool {
        self.open.cloned()
    }

    /// Returns a reactive memo of the open state.
    pub fn open_memo(&self) -> Memo<bool> {
        self.open
    }

    /// Sets the open state of the dialog.
    pub fn set_open(&self, open: bool) {
        self.set_open.call(open);
    }
}

/// The props for the [`DialogRoot`] component
#[derive(Props, Clone, PartialEq)]
pub struct DialogRootProps {
    /// The ID of the dialog root element.
    pub id: ReadSignal<Option<String>>,

    /// Whether the dialog is modal. If true, it will trap focus within the dialog when open.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub is_modal: ReadSignal<bool>,

    /// The controlled `open` state of the dialog.
    pub open: ReadSignal<Option<bool>>,

    /// The default `open` state of the dialog if it is not controlled.
    #[props(default)]
    pub default_open: bool,

    /// A callback that is called when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Additional attributes to apply to the dialog root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the dialog root component.
    pub children: Element,
}

/// # DialogRoot
///
/// The entry point for the dialog. It manages the open state of the dialog and provides context to its children. You
/// can use it to create a backdrop for the dialog if needed. The contents will only be rendered when the dialog is open.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::dialog::{DialogContent, DialogDescription, DialogRoot, DialogTitle};
///
/// #[component]
/// fn Demo() -> Element {
///     let mut open = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| open.set(true),
///             "Show Dialog"
///         }
///         DialogRoot {
///             open: open(),
///             on_open_change: move |v| open.set(v),
///             DialogContent {
///                 button {
///                     aria_label: "Close",
///                     tabindex: if open() { "0" } else { "-1" },
///                     onclick: move |_| open.set(false),
///                     "×"
///                 }
///                 DialogTitle {
///                     "Item information"
///                 }
///                 DialogDescription {
///                     "Here is some additional information about the item."
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`DialogRoot`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates if the dialog is open or closed. It can be either "open" or "closed".
#[component]
pub fn DialogRoot(props: DialogRootProps) -> Element {
    let dialog_labelledby = use_unique_id();
    let dialog_describedby = use_unique_id();

    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    let is_modal = props.is_modal;

    // The Root keeps its own provider so the trigger (which stays in the main
    // tree, outside the portal) resolves `DialogCtx`. The portaled content
    // re-provides a clone of this same ctx inside the portal subtree
    // (see `DialogPortaled`) because context does not inherit through the portal.
    use_context_provider(|| DialogCtx {
        open,
        set_open,
        is_modal,
        dialog_labelledby,
        dialog_describedby,
    });

    // Scroll-lock, the Escape listener, outside-dismiss, and the focus trap are
    // now owned centrally by the overlay manager (`OverlayProvider`). The Root no
    // longer runs per-component versions of those.

    rsx! {
        div {
            "data-state": if open() { "open" } else { "closed" },
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DialogTrigger`] component
#[derive(Props, Clone, PartialEq)]
pub struct DialogTriggerProps {
    /// Additional attributes to apply to the trigger button element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the dialog trigger.
    pub children: Element,
}

/// # DialogTrigger
///
/// A button that opens the dialog when clicked. It reads the dialog open state from the
/// surrounding [`DialogRoot`] context.
///
/// This must be used inside an [`DialogRoot`] component.
#[component]
pub fn DialogTrigger(props: DialogTriggerProps) -> Element {
    let ctx: DialogCtx = use_context();
    let set_open = ctx.set_open;

    rsx! {
        button {
            r#type: "button",
            onclick: move |_| set_open.call(true),
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DialogClose`] component
#[derive(Props, Clone, PartialEq)]
pub struct DialogCloseProps {
    /// Override the rendered element (e.g. wrap in styled Button).
    /// Receives merged attributes; return the replacement element.
    pub r#as: Option<Callback<Vec<Attribute>, Element>>,
    /// Additional attributes to apply to the close button element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the dialog close button.
    pub children: Element,
}

/// # DialogClose
///
/// A button that closes the dialog when clicked. It reads the dialog open state from the
/// surrounding [`DialogRoot`] context.
///
/// This must be used inside an [`DialogRoot`] component and should be placed inside an
/// [`DialogContent`] component.
#[component]
pub fn DialogClose(props: DialogCloseProps) -> Element {
    let ctx: DialogCtx = use_context();
    let set_open = ctx.set_open;

    let base = attributes!(button {
        r#type: "button",
        aria_label: "Close",
        onclick: move |_| set_open.call(false),
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    if let Some(dynamic) = props.r#as {
        dynamic.call(attributes)
    } else {
        rsx! { button { ..attributes, {props.children} } }
    }
}

/// The props for the [`DialogContent`] component
#[derive(Props, Clone, PartialEq)]
pub struct DialogContentProps {
    /// The ID of the dialog content element.
    pub id: ReadSignal<Option<String>>,

    /// CSS class name to apply to the backdrop overlay element.
    /// When using the styled component layer, pass the hashed class from the CSS module
    /// so that scoped CSS rules match.
    #[props(default, into)]
    pub backdrop_class: String,

    /// Whether clicking outside the dialog (on the backdrop) closes it. Defaults to `true`.
    #[props(default = true)]
    pub close_on_backdrop_click: bool,

    /// Whether pressing Escape closes the dialog. Defaults to `true`.
    #[props(default = true)]
    pub close_on_escape: bool,

    /// The ARIA role for the inner dialog element. Defaults to `"dialog"`.
    /// Pass `"alertdialog"` when building an alert dialog.
    #[props(default = "dialog", into)]
    pub dialog_role: String,

    /// The overlay band this content registers under. Defaults to
    /// [`OverlayKind::Modal`]; the Sheet wrapper passes [`OverlayKind::Sheet`] so
    /// it gets edge-docked push-aside math instead of centered-modal depth.
    #[props(default = OverlayKind::Modal)]
    pub overlay_kind: OverlayKind,

    /// Additional attributes to apply to the dialog content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the dialog content.
    pub children: Element,
}

/// # DialogContent
///
/// The content of the dialog. Any interactive content in the dialog should be placed
/// inside this component. It will trap focus within the dialog while it is open
///
/// This must be used inside an [`DialogRoot`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::dialog::{DialogContent, DialogDescription, DialogRoot, DialogTitle};
///
/// #[component]
/// fn Demo() -> Element {
///     let mut open = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| open.set(true),
///             "Show Dialog"
///         }
///         DialogRoot {
///             open: open(),
///             on_open_change: move |v| open.set(v),
///             DialogContent {
///                 button {
///                     aria_label: "Close",
///                     tabindex: if open() { "0" } else { "-1" },
///                     onclick: move |_| open.set(false),
///                     "×"
///                 }
///                 DialogTitle {
///                     "Item information"
///                 }
///                 DialogDescription {
///                     "Here is some additional information about the item."
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`DialogRoot`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates if the dialog is open or closed. It can be either "open" or "closed".
#[component]
pub fn DialogContent(props: DialogContentProps) -> Element {
    let ctx: DialogCtx = use_context();
    let open = ctx.open;

    let gen_id = use_unique_id();
    let id = use_id_or(gen_id, props.id);

    // The backdrop is the element that carries the open/close CSS animation, so it
    // owns a dedicated id that drives `use_animated_open`. The inner dialog box
    // keeps `id`, which the manager's focus trap looks up. Both are computed in the
    // Root-tree component so they stay stable across the portal boundary.
    let backdrop_id = use_unique_id();

    // Keep the content mounted (and the overlay registered) until the exit
    // animation completes. `render()` stays true through the closing animation,
    // then flips false once `getAnimations()` settle — at which point the
    // portaled subtree (and its `OverlayEntry`) unmounts and auto-unregisters.
    let render = use_animated_open(backdrop_id, open);

    rsx! {
        if render() {
            DialogPortaled {
                ctx,
                content_id: id,
                backdrop_id,
                backdrop_class: props.backdrop_class.clone(),
                close_on_backdrop_click: props.close_on_backdrop_click,
                close_on_escape: props.close_on_escape,
                dialog_role: props.dialog_role.clone(),
                overlay_kind: props.overlay_kind,
                attributes: props.attributes.clone(),
                {props.children}
            }
        }
    }
}

/// Props for [`DialogPortaled`], the in-portal half of [`DialogContent`].
#[derive(Props, Clone, PartialEq)]
struct DialogPortaledProps {
    ctx: DialogCtx,
    content_id: Memo<String>,
    backdrop_id: Signal<String>,
    backdrop_class: String,
    close_on_backdrop_click: bool,
    close_on_escape: bool,
    dialog_role: String,
    overlay_kind: OverlayKind,
    attributes: Vec<Attribute>,
    children: Element,
}

/// The portaled half of a dialog: registers the overlay entry, renders the
/// backdrop + content through the shared [`OverlayOutlet`], and **re-provides
/// `DialogCtx` inside the portal** so `DialogClose`/`DialogTitle`/
/// `DialogDescription` (and `AlertDialogAction`/`AlertDialogCancel`) resolve
/// their context up the *portaled* render chain — context does not inherit
/// through the portal.
///
/// This component is only mounted while the dialog is rendering (open or
/// animating out), so its `use_overlay_registration` registers on open and
/// auto-unregisters once the exit animation settles and it unmounts.
#[component]
fn DialogPortaled(props: DialogPortaledProps) -> Element {
    let ctx = props.ctx;
    let open = ctx.open;
    let is_modal = ctx.is_modal;
    let set_open = ctx.set_open;

    let close_on_backdrop_click = props.close_on_backdrop_click;
    let close_on_escape = props.close_on_escape;

    let portal = use_portal();

    // The dismiss config: this entry participates in the central Escape /
    // outside-click stack only if either close path is enabled. `on_dismiss`
    // routes through the controlled state, identical to the old per-component
    // behavior, but selection of the *topmost* entry is now centralized.
    let dismissable = close_on_escape || close_on_backdrop_click;
    let modal = is_modal();
    let overlay_kind = props.overlay_kind;
    let on_dismiss = use_callback(move |_| {
        set_open.call(false);
    });

    let content_id = props.content_id;

    let reg: OverlayRegistration = use_overlay_registration(move || RegisterArgs {
        kind: overlay_kind,
        portal,
        modal,
        dismissable,
        on_dismiss,
        parent: None,
        trigger_id: None,
        content_root_id: Some(content_id.peek().clone()),
    });

    // Keep the manager's "inside" predicate + focus trap pointed at the live
    // content-root id (the unique id may resolve after first mount).
    use_effect(move || {
        reg.set_dom_ids(None, Some(content_id()));
    });

    // Drive exit-phase exclusion: while closing (open == false but still
    // rendering), mark the entry `closing` so depth/stack math drops it
    // immediately and siblings reflow without waiting for the animation.
    use_effect(move || {
        reg.set_closing(!open());
    });

    // The body is rendered as a CHILD of `PortalIn` so the re-provide
    // (`use_context_provider`) inside it lands on the *portaled* render chain —
    // the only place a portaled consumer resolves it. Re-providing here, in the
    // component that merely *contains* `PortalIn`, would NOT cover the portaled
    // children (context resolves up the render tree, where `PortalOut` sits).
    rsx! {
        PortalIn { portal,
            DialogPortalBody {
                ctx,
                reg,
                content_id,
                backdrop_id: props.backdrop_id,
                backdrop_class: props.backdrop_class.clone(),
                close_on_backdrop_click,
                dialog_role: props.dialog_role.clone(),
                attributes: props.attributes.clone(),
                {props.children}
            }
        }
    }
}

/// Props for [`DialogPortalBody`].
#[derive(Props, Clone, PartialEq)]
struct DialogPortalBodyProps {
    ctx: DialogCtx,
    reg: OverlayRegistration,
    content_id: Memo<String>,
    backdrop_id: Signal<String>,
    backdrop_class: String,
    close_on_backdrop_click: bool,
    dialog_role: String,
    attributes: Vec<Attribute>,
    children: Element,
}

/// The actual portaled DOM: rendered as a child of `PortalIn`, so this is where
/// `DialogCtx` is re-provided. Every in-portal consumer
/// (`DialogClose`/`DialogTitle`/`DialogDescription`,
/// `AlertDialogAction`/`AlertDialogCancel`) resolves the re-provided ctx up THIS
/// render chain (mirrors the Wave-0 R-CTX smoke test's `SmokePortaledRoot`).
#[component]
fn DialogPortalBody(props: DialogPortalBodyProps) -> Element {
    let ctx = props.ctx;
    let reg = props.reg;
    let open = ctx.open;
    let set_open = ctx.set_open;
    let close_on_backdrop_click = props.close_on_backdrop_click;
    let content_id = props.content_id;
    let backdrop_id = props.backdrop_id;

    // Re-provide a CLONE of the Root's ctx at the top of the portaled subtree.
    // Same signals as the Root provider (nearest-wins), so consumers inside the
    // portal stay in sync with the Root.
    use_context_provider(|| ctx);

    let base = attributes!(div { class: "dx-dialog" });
    let attributes = merge_attributes(vec![base, props.attributes.clone()]);

    let depth = reg.depth();
    let stack_size = reg.stack_size();

    rsx! {
        div {
            // Backdrop. z == --overlay-z (manager assigned by open order).
            div {
                id: backdrop_id,
                class: props.backdrop_class.clone(),
                style: reg.z().map(|z| format!("--overlay-z: {z};")),
                aria_hidden: (!open()).then_some("true"),
                "data-state": if open() { "open" } else { "closed" },
                onclick: move |_| {
                    if close_on_backdrop_click {
                        set_open.call(false);
                    }
                },
            }
            // Content. z == calc(--overlay-z + 1) so it paints above its own
            // backdrop. Carries push-aside depth metadata for CSS.
            div {
                id: content_id,
                role: props.dialog_role.clone(),
                aria_modal: "true",
                aria_labelledby: ctx.dialog_labelledby,
                aria_describedby: ctx.dialog_describedby,
                tabindex: "-1",
                style: reg.z().map(|z| format!("--overlay-z: {z}; --overlay-depth: {depth};")),
                "data-state": if open() { "open" } else { "closed" },
                "data-overlay-depth": "{depth}",
                "data-overlay-stack-size": "{stack_size}",
                onclick: move |e| e.stop_propagation(),
                ..attributes,
                {props.children}
            }
        }
    }
}

/// The props for the [`DialogTitle`] component
#[derive(Props, Clone, PartialEq)]
pub struct DialogTitleProps {
    /// The ID of the dialog title element. If not provided, uses the auto-generated aria ID.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,
    /// Additional attributes for the dialog title element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the dialog title.
    pub children: Element,
}

/// # DialogTitle
///
/// The title of the dialog. This will be used to label the dialog for accessibility purposes.
///
/// This must be used inside an [`DialogRoot`] component and should be placed inside an [`DialogContent`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::dialog::{DialogContent, DialogDescription, DialogRoot, DialogTitle};
///
/// #[component]
/// fn Demo() -> Element {
///     let mut open = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| open.set(true),
///             "Show Dialog"
///         }
///         DialogRoot {
///             open: open(),
///             on_open_change: move |v| open.set(v),
///             DialogContent {
///                 button {
///                     aria_label: "Close",
///                     tabindex: if open() { "0" } else { "-1" },
///                     onclick: move |_| open.set(false),
///                     "×"
///                 }
///                 DialogTitle {
///                     "Item information"
///                 }
///                 DialogDescription {
///                     "Here is some additional information about the item."
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    let ctx: DialogCtx = use_context();
    let id = use_id_or(ctx.dialog_labelledby, props.id);

    rsx! {
        h2 {
            id: id,
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DialogDescription`] component
#[derive(Props, Clone, PartialEq)]
pub struct DialogDescriptionProps {
    /// The ID of the dialog description element. If not provided, uses the auto-generated aria ID.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,
    /// Additional attributes for the dialog description element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the dialog description.
    pub children: Element,
}

/// # DialogDescription
///
/// The description of the dialog. This will be used to describe the dialog for accessibility purposes.
///
/// This must be used inside an [`DialogRoot`] component and should be placed inside an [`DialogContent`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::dialog::{DialogContent, DialogDescription, DialogRoot, DialogTitle};
///
/// #[component]
/// fn Demo() -> Element {
///     let mut open = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| open.set(true),
///             "Show Dialog"
///         }
///         DialogRoot {
///             open: open(),
///             on_open_change: move |v| open.set(v),
///             DialogContent {
///                 button {
///                     aria_label: "Close",
///                     tabindex: if open() { "0" } else { "-1" },
///                     onclick: move |_| open.set(false),
///                     "×"
///                 }
///                 DialogTitle {
///                     "Item information"
///                 }
///                 DialogDescription {
///                     "Here is some additional information about the item."
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element {
    let ctx: DialogCtx = use_context();
    let id = use_id_or(ctx.dialog_describedby, props.id);

    rsx! {
        p {
            id: id,
            ..props.attributes,
            {props.children}
        }
    }
}

#[cfg(test)]
mod tests {
    //! Proves the §4.2 re-provide is correctly wired for Dialog: an open dialog
    //! portals its content through the overlay outlet, and the in-portal
    //! consumers (`DialogClose`/`DialogTitle`/`DialogDescription`) resolve
    //! `DialogCtx` up the *portaled* render chain. If the re-provide were on the
    //! wrong scope, `use_context::<DialogCtx>()` inside those components would
    //! panic during render, failing this test.
    use super::*;
    use crate::overlay::OverlayProvider;

    #[component]
    fn OpenDialogApp() -> Element {
        rsx! {
            OverlayProvider {
                DialogRoot {
                    open: Some(true),
                    DialogContent {
                        DialogClose { "close-marker" }
                        DialogTitle { "title-marker" }
                        DialogDescription { "desc-marker" }
                    }
                }
            }
        }
    }

    #[test]
    fn open_dialog_portals_and_resolves_dialog_ctx_inside_portal() {
        let mut dom = VirtualDom::new(OpenDialogApp);
        dom.rebuild_in_place();
        // `use_animated_open` flips `show_in_dom` in an effect, so the portaled
        // content mounts on a *subsequent* flush, not the first rebuild. Drain
        // pending effect-driven work before snapshotting (no async executor
        // needed for the synchronous signal updates here).
        for _ in 0..8 {
            let _ = dom.render_immediate_to_vec();
        }
        let html = dioxus_ssr::render(&dom);

        // All three context-dependent consumers rendered — proving each resolved
        // the re-provided DialogCtx inside the portal (no panic, content present).
        assert!(
            html.contains("close-marker"),
            "DialogClose did not resolve DialogCtx through the portal: {html}"
        );
        assert!(
            html.contains("title-marker"),
            "DialogTitle did not resolve DialogCtx through the portal: {html}"
        );
        assert!(
            html.contains("desc-marker"),
            "DialogDescription did not resolve DialogCtx through the portal: {html}"
        );
        // The dialog content carries the manager-assigned z + depth metadata.
        assert!(
            html.contains("--overlay-z: calc(var(--z-overlay-base)"),
            "content did not receive manager --overlay-z: {html}"
        );
        assert!(
            html.contains("data-overlay-depth"),
            "content missing push-aside depth metadata: {html}"
        );
    }
}
