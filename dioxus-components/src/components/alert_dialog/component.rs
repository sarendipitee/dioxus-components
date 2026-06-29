use crate::components::button::{Button, ButtonVariant};
use crate::components::dialog::DialogStyles;
use crate::components::typography::{
    TextAlign, TextWrap, TypographySize, TypographyTone, TypographyWeight,
};
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
    let title = render_alert_dialog_title(props.title);
    let description = render_alert_dialog_description(props.description);
    let confirm_has = !props.confirm.is_empty();
    let confirm_el = confirm_has.then(|| props.confirm.into_element());
    let cancel_has = !props.cancel.is_empty();
    let cancel_el = cancel_has.then(|| props.cancel.into_element());

    let content_attributes = merge_attributes(vec![
        attributes!(div {
            class: DialogStyles::dx_dialog.to_string(),
            "data-slot": "alert-dialog-content",
        }),
        props.attributes,
    ]);

    rsx! {
        alert_dialog::AlertDialog {
            id: props.id,
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,

            alert_dialog::AlertDialogContent {
                backdrop_class: DialogStyles::dx_dialog_backdrop.to_string(),
                attributes: content_attributes,

                if title.is_some() || description.is_some() {
                    div { class: DialogStyles::dx_dialog_header.to_string(),
                        if let Some(title) = title {
                            {title}
                        }
                        if let Some(description) = description {
                            {description}
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

fn render_alert_dialog_title(title: TextOrElement<()>) -> Option<Element> {
    if title.is_empty() {
        return None;
    }

    let content = title.into_element();
    let attributes = typography_slot_attributes(
        format!("{} dx_heading", DialogStyles::dx_dialog_title),
        "alert-dialog-title",
        TypographySize::Lg,
        TypographyTone::Default,
        TypographyWeight::Bold,
    );
    Some(rsx! {
        alert_dialog::AlertDialogTitle {
            attributes,
            {content}
        }
    })
}

fn render_alert_dialog_description(description: TextOrElement<()>) -> Option<Element> {
    if description.is_empty() {
        return None;
    }

    let content = description.into_element();
    let attributes = typography_slot_attributes(
        format!("{} dx_text", DialogStyles::dx_dialog_description),
        "alert-dialog-description",
        TypographySize::Md,
        TypographyTone::Default,
        TypographyWeight::Inherit,
    );
    Some(rsx! {
        alert_dialog::AlertDialogDescription {
            attributes,
            {content}
        }
    })
}

fn typography_slot_attributes(
    class: String,
    slot: &'static str,
    size: TypographySize,
    tone: TypographyTone,
    weight: TypographyWeight,
) -> Vec<Attribute> {
    attributes!(div {
        class,
        "data-slot": slot,
        "data-size": size.as_str(),
        "data-tone": tone.as_str(),
        "data-weight": weight.as_str(),
        "data-align": TextAlign::Inherit.as_str(),
        "data-wrap": TextWrap::Wrap.as_str(),
        "data-truncate": "false",
    })
}

/// Internal button that closes the alert dialog and fires an optional callback.
#[component]
fn AlertDialogButton(
    variant: ButtonVariant,
    on_click: Option<EventHandler<MouseEvent>>,
    content: Element,
) -> Element {
    let ctx: DialogCtx = use_context();
    let handler = use_callback(move |evt: MouseEvent| {
        if let Some(cb) = &on_click {
            cb.call(evt);
        }
        ctx.set_open(false);
    });
    let attrs = attributes!(button {
        r#type: "button",
        tabindex: "0",
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
