//! Alert dialog built on top of [`crate::dialog`].
//!
//! AlertDialog reuses all of Dialog's state, scroll-lock, focus-trap, and animation
//! infrastructure. The only differences are:
//!
//! - `role="alertdialog"` on the inner element
//! - Backdrop click and Escape key do NOT dismiss (user must use a button)
//! - Dedicated `AlertDialogAction` and `AlertDialogCancel` buttons that auto-close

use dioxus::prelude::*;

use crate::dialog::{self, DialogCtx};

// Re-export root/trigger/title/description props so callers don't need to import both modules.
pub use dialog::{
    DialogDescriptionProps as AlertDialogDescriptionProps, DialogRootProps as AlertDialogRootProps,
    DialogTitleProps as AlertDialogTitleProps, DialogTriggerProps as AlertDialogTriggerProps,
};

/// The props for the [`AlertDialogContent`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogContentProps {
    /// The id of the alert dialog content element. If not provided, a unique id will be generated.
    pub id: ReadSignal<Option<String>>,

    /// CSS class name to apply to the backdrop overlay element.
    /// When using the styled component layer, pass the hashed class from the CSS module
    /// so that scoped CSS rules match.
    #[props(default, into)]
    pub backdrop_class: String,

    /// Additional attributes to extend the content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the alert dialog content element.
    pub children: Element,
}

/// The props for the [`AlertDialogActions`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogActionsProps {
    /// Additional attributes to extend the actions container element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the actions element.
    pub children: Element,
}

/// The props for the [`AlertDialogAction`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogActionProps {
    /// The click event handler for the action button.
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,
    /// Additional attributes to extend the action button element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the action button.
    pub children: Element,
}

/// The props for the [`AlertDialogCancel`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogCancelProps {
    /// The click event handler for the cancel button.
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,
    /// Additional attributes to extend the cancel button element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the cancel button.
    pub children: Element,
}

// ─── Components ──────────────────────────────────────────────────────────────

/// The root alert dialog component. Thin alias of [`dialog::DialogRoot`].
///
/// Provides state, scroll-lock, and focus-trap context to child components.
/// Place [`AlertDialogContent`] as a child.
#[component]
pub fn AlertDialogRoot(props: AlertDialogRootProps) -> Element {
    rsx! { dialog::DialogRoot { ..props } }
}

/// A button that opens the alert dialog. Thin alias of [`dialog::DialogTrigger`].
///
/// Must be used inside an [`AlertDialogRoot`].
#[component]
pub fn AlertDialogTrigger(props: AlertDialogTriggerProps) -> Element {
    rsx! { dialog::DialogTrigger { ..props } }
}

/// The styled alert dialog panel with animated backdrop.
///
/// Renders as [`dialog::DialogContent`] with:
/// - `close_on_backdrop_click: false`
/// - `close_on_escape: false`
/// - `role="alertdialog"`
///
/// Must be used inside an [`AlertDialogRoot`].
#[component]
pub fn AlertDialogContent(props: AlertDialogContentProps) -> Element {
    let backdrop_class = if props.backdrop_class.is_empty() {
        None
    } else {
        Some(props.backdrop_class)
    };

    rsx! {
        dialog::DialogContent {
            id: props.id,
            backdrop_class,
            close_on_backdrop_click: false,
            close_on_escape: false,
            dialog_role: "alertdialog",
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// Groups title and description inside an alert dialog.
///
/// Thin alias of [`dialog::DialogTitle`]. Must be inside an [`AlertDialogRoot`].
#[component]
pub fn AlertDialogTitle(props: AlertDialogTitleProps) -> Element {
    rsx! { dialog::DialogTitle { ..props } }
}

/// Describes the alert dialog content for accessibility.
///
/// Thin alias of [`dialog::DialogDescription`]. Must be inside an [`AlertDialogRoot`].
#[component]
pub fn AlertDialogDescription(props: AlertDialogDescriptionProps) -> Element {
    rsx! { dialog::DialogDescription { ..props } }
}

/// Container for action and cancel buttons.
#[component]
pub fn AlertDialogActions(props: AlertDialogActionsProps) -> Element {
    rsx! {
        div { ..props.attributes, {props.children} }
    }
}

/// A confirm button that fires `on_click` then closes the alert dialog.
///
/// Must be used inside an [`AlertDialogRoot`].
#[component]
pub fn AlertDialogAction(props: AlertDialogActionProps) -> Element {
    let ctx: DialogCtx = use_context();
    let open = ctx.open_memo();
    let user_on_click = props.on_click;
    let on_click = use_callback(move |evt: MouseEvent| {
        ctx.set_open(false);
        if let Some(cb) = &user_on_click {
            cb.call(evt.clone());
        }
    });

    rsx! {
        button {
            r#type: "button",
            tabindex: if open() { "0" } else { "-1" },
            onclick: on_click,
            ..props.attributes,
            {props.children}
        }
    }
}

/// A cancel button that fires `on_click` then closes the alert dialog.
///
/// Must be used inside an [`AlertDialogRoot`].
#[component]
pub fn AlertDialogCancel(props: AlertDialogCancelProps) -> Element {
    let ctx: DialogCtx = use_context();
    let open = ctx.open_memo();
    let user_on_click = props.on_click;
    let on_click = use_callback(move |evt: MouseEvent| {
        ctx.set_open(false);
        if let Some(cb) = &user_on_click {
            cb.call(evt.clone());
        }
    });

    rsx! {
        button {
            r#type: "button",
            tabindex: if open() { "0" } else { "-1" },
            onclick: on_click,
            ..props.attributes,
            {props.children}
        }
    }
}
