use crate::components::button::{Button, ButtonVariant};
use crate::components::dialog::DialogStyles;
use dioxus::prelude::*;
use dioxus_primitives::alert_dialog::{self};
use dioxus_primitives::dialog::DialogCtx;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes, TextOrElement};

/// Props for the [`AlertDialog`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogProps {
    /// The ID of the alert dialog root element.
    pub id: ReadSignal<Option<String>>,

    /// Whether the dialog is modal. Defaults to `true`.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub is_modal: ReadSignal<bool>,

    /// The controlled `open` state.
    pub open: ReadSignal<Option<bool>>,

    /// The default `open` state if not controlled.
    #[props(default)]
    pub default_open: bool,

    /// A callback that is called when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// The title of the alert dialog. Rendered as an `<h2>` with `aria-labelledby` binding.
    /// Omit or pass `""` to hide.
    #[props(default, into)]
    pub title: TextOrElement<()>,

    /// The description of the alert dialog. Rendered as a `<p>` with `aria-describedby` binding.
    /// Omit or pass `""` to hide.
    #[props(default, into)]
    pub description: TextOrElement<()>,

    /// Label for the confirm (action) button.
    /// Omit or pass `""` to hide.
    #[props(default, into)]
    pub confirm: TextOrElement<()>,

    /// Label for the cancel button.
    /// Omit or pass `""` to hide.
    #[props(default, into)]
    pub cancel: TextOrElement<()>,

    /// Called when the confirm button is clicked, before the dialog closes.
    #[props(default)]
    pub on_confirm: Option<EventHandler<MouseEvent>>,

    /// Called when the cancel button is clicked, before the dialog closes.
    #[props(default)]
    pub on_cancel: Option<EventHandler<MouseEvent>>,

    /// Additional attributes applied to the inner content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Optional body content displayed above the action buttons.
    pub children: Element,
}

/// An alert dialog for confirmations and destructive actions.
///
/// Uses `role="alertdialog"`, disables escape and backdrop dismiss,
/// and renders confirm/cancel action buttons.
///
/// ## Usage
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_components::alert_dialog::AlertDialog;
///
/// #[component]
/// fn Demo() -> Element {
///     let mut open = use_signal(|| false);
///     rsx! {
///         button { onclick: move |_| open.set(true), "Delete" }
///         AlertDialog {
///             open: open(),
///             on_open_change: move |v| open.set(v),
///             title: "Are you sure?",
///             description: "This cannot be undone.",
///             confirm: "Delete",
///             cancel: "Cancel",
///             on_confirm: move |_| { /* delete */ },
///         }
///     }
/// }
/// ```
#[component]
pub fn AlertDialog(props: AlertDialogProps) -> Element {
    let title_has = !props.title.is_empty();
    let title_el = title_has.then(|| props.title.into_element());
    let desc_has = !props.description.is_empty();
    let desc_el = desc_has.then(|| props.description.into_element());
    let confirm_has = !props.confirm.is_empty();
    let confirm_el = confirm_has.then(|| props.confirm.into_element());
    let cancel_has = !props.cancel.is_empty();
    let cancel_el = cancel_has.then(|| props.cancel.into_element());

    let content_attributes = merge_attributes(vec![
        attributes!(div { class: DialogStyles::dx_dialog.to_string() }),
        props.attributes,
    ]);

    rsx! {
        alert_dialog::AlertDialogRoot {
            id: props.id,
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,

            alert_dialog::AlertDialogContent {
                backdrop_class: DialogStyles::dx_dialog_backdrop.to_string(),
                attributes: content_attributes,

                if title_el.is_some() || desc_el.is_some() {
                    div { class: DialogStyles::dx_dialog_header.to_string(),
                        if let Some(t) = title_el {
                            alert_dialog::AlertDialogTitle { {t} }
                        }
                        if let Some(d) = desc_el {
                            alert_dialog::AlertDialogDescription { {d} }
                        }
                    }
                }

                {props.children}

                if cancel_el.is_some() || confirm_el.is_some() {
                    div { class: DialogStyles::dx_dialog_footer.to_string(),
                        if let Some(c) = cancel_el {
                            AlertDialogButton {
                                variant: ButtonVariant::Secondary,
                                on_click: props.on_cancel,
                                content: c,
                            }
                        }
                        if let Some(c) = confirm_el {
                            AlertDialogButton {
                                variant: ButtonVariant::Destructive,
                                on_click: props.on_confirm,
                                content: c,
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Internal button that closes the alert dialog and fires an optional callback.
#[component]
fn AlertDialogButton(
    variant: ButtonVariant,
    on_click: Option<EventHandler<MouseEvent>>,
    content: Element,
) -> Element {
    let ctx: DialogCtx = use_context();
    let open = ctx.open_memo();
    let tabindex = if open() { "0" } else { "-1" };
    let handler = use_callback(move |evt: MouseEvent| {
        ctx.set_open(false);
        if let Some(cb) = &on_click {
            cb.call(evt);
        }
    });
    let attrs = attributes!(button {
        tabindex: tabindex,
    });

    rsx! {
        Button {
            variant,
            onclick: handler,
            attributes: attrs,
            {content}
        }
    }
}
