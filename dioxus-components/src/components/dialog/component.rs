use crate::component_styles;
use crate::components::typography::{
    TextAlign, TextWrap, TypographySize, TypographyTone, TypographyWeight,
};
use dioxus::prelude::*;
use dioxus_primitives::dialog::{self};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes, TextOrElement};

#[component_styles("./style.css")]
pub(crate) struct Styles;

/// Props for the [`Dialog`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
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

    /// Whether clicking outside the dialog (on the backdrop) closes it. Defaults to `true`.
    #[props(default = true)]
    pub close_on_backdrop_click: bool,

    /// Whether pressing Escape closes the dialog. Defaults to `true`.
    #[props(default = true)]
    pub close_on_escape: bool,

    /// The title of the dialog. Rendered as an `<h2>` with `aria-labelledby` binding.
    /// Omit or pass `""` to hide.
    #[props(default, into)]
    pub title: TextOrElement<()>,

    /// The description of the dialog. Rendered as a `<p>` with `aria-describedby` binding.
    /// Omit or pass `""` to hide.
    #[props(default, into)]
    pub description: TextOrElement<()>,

    /// Whether to render a close button with `aria-label="Close"`. Defaults to `true`.
    #[props(default = true)]
    pub with_close_button: bool,

    /// Optional footer content, typically action buttons.
    /// Omit or pass `""` to hide.
    #[props(default, into)]
    pub footer: TextOrElement<()>,

    /// Additional attributes applied to the dialog content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The body content of the dialog.
    pub children: Element,
}

/// A dialog panel with optional title, description, close button, and footer.
///
/// Manages open state, scroll lock, focus trap, and ARIA attributes
/// (`role="dialog"`, `aria-modal`, `aria-labelledby`, `aria-describedby`).
///
/// ## Usage
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_components::dialog::Dialog;
///
/// #[component]
/// fn Demo() -> Element {
///     let mut open = use_signal(|| false);
///     rsx! {
///         button { onclick: move |_| open.set(true), "Open" }
///         Dialog {
///             open: open(),
///             on_open_change: move |v| open.set(v),
///             title: "Edit profile",
///             description: "Make changes below.",
///             div { "form content" }
///         }
///     }
/// }
/// ```
#[component]
pub fn Dialog(props: DialogProps) -> Element {
    let title = render_dialog_title(props.title);
    let description = render_dialog_description(props.description);
    let footer_has = !props.footer.is_empty();
    let footer_el = footer_has.then(|| props.footer.into_element());

    let content_attributes = merge_attributes(vec![
        attributes!(div {
            class: Styles::dx_dialog
        }),
        props.attributes,
    ]);

    rsx! {
        dialog::Dialog {
            id: props.id,
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,

            dialog::DialogContent {
                close_on_backdrop_click: props.close_on_backdrop_click,
                close_on_escape: props.close_on_escape,
                backdrop_class: Styles::dx_dialog_backdrop,
                attributes: content_attributes,

                if props.with_close_button {
                    dialog::DialogClose {
                        class: Styles::dx_dialog_close,
                    }
                }

                if title.is_some() || description.is_some() {
                    header { class: Styles::dx_dialog_header,
                        if let Some(title) = title {
                            {title}
                        }
                        if let Some(description) = description {
                            {description}
                        }
                    }
                }

                {props.children}

                if let Some(f) = footer_el {
                    div { class: Styles::dx_dialog_footer, {f} }
                }
            }
        }
    }
}

fn render_dialog_title(title: TextOrElement<()>) -> Option<Element> {
    if title.is_empty() {
        return None;
    }

    let content = title.into_element();
    let attributes = typography_slot_attributes(
        format!("{} dx_heading", Styles::dx_dialog_title),
        "dialog-title",
        TypographySize::Lg,
        TypographyTone::Default,
        TypographyWeight::Bold,
    );
    Some(rsx! {
        dialog::DialogTitle {
            attributes,
            {content}
        }
    })
}

fn render_dialog_description(description: TextOrElement<()>) -> Option<Element> {
    if description.is_empty() {
        return None;
    }

    let content = description.into_element();
    let attributes = typography_slot_attributes(
        format!("{} dx_text", Styles::dx_dialog_description),
        "dialog-description",
        TypographySize::Md,
        TypographyTone::Muted,
        TypographyWeight::Inherit,
    );
    Some(rsx! {
        dialog::DialogDescription {
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
