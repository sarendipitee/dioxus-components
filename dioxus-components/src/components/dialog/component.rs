use crate::component_styles;
use dioxus::prelude::*;
use dioxus_primitives::dialog::{
    self, DialogCloseProps, DialogContentProps, DialogDescriptionProps, DialogRootProps,
    DialogTitleProps, DialogTriggerProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[component_styles("./style.css")]
pub(crate) struct Styles;

/// The root dialog component — a context provider that manages open state.
/// Place [`DialogTrigger`] and [`DialogContent`] as children.
#[component]
pub fn Dialog(props: DialogRootProps) -> Element {
    rsx! {
        dialog::DialogRoot {
            id: props.id,
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            {props.children}
        }
    }
}

/// A button that opens the dialog. Must be a child of [`Dialog`].
#[component]
pub fn DialogTrigger(props: DialogTriggerProps) -> Element {
    rsx! {
        dialog::DialogTrigger { attributes: props.attributes, {props.children} }
    }
}

/// The styled dialog panel with animated backdrop. Must be a child of [`Dialog`].
#[component]
pub fn DialogContent(props: DialogContentProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_dialog.to_string()
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dialog::DialogContent {
            id: props.id,
            backdrop_class: Styles::dx_dialog_backdrop.to_string(),
            attributes: merged,
            {props.children}
        }
    }
}

/// A button that closes the dialog. Must be inside a [`DialogContent`].
#[component]
pub fn DialogClose(props: DialogCloseProps) -> Element {
    let base = attributes!(button {
        class: Styles::dx_dialog_close.to_string()
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dialog::DialogClose { attributes: merged, {props.children} }
    }
}

/// Props for layout container components ([`DialogHeader`], [`DialogBody`], [`DialogFooter`]).
#[derive(Props, Clone, PartialEq)]
pub struct DialogLayoutProps {
    /// Additional attributes to apply to the container element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
}

/// Groups title and description at the top of a dialog.
#[component]
pub fn DialogHeader(props: DialogLayoutProps) -> Element {
    rsx! {
        div {
            ..merge_attributes(vec![
                attributes!(div { class: Styles::dx_dialog_header.to_string() }),
                props.attributes,
            ]),
            {props.children}
        }
    }
}

/// The scrollable content area between header and footer.
#[component]
pub fn DialogBody(props: DialogLayoutProps) -> Element {
    rsx! {
        div {
            ..merge_attributes(vec![
                attributes!(div { class: Styles::dx_dialog_body.to_string() }),
                props.attributes,
            ]),
            {props.children}
        }
    }
}

/// Groups action buttons at the bottom of a dialog.
#[component]
pub fn DialogFooter(props: DialogLayoutProps) -> Element {
    rsx! {
        div {
            ..merge_attributes(vec![
                attributes!(div { class: Styles::dx_dialog_footer.to_string() }),
                props.attributes,
            ]),
            {props.children}
        }
    }
}

#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    let base = attributes!(h2 {
        class: Styles::dx_dialog_title.to_string()
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dialog::DialogTitle { id: props.id, attributes: merged, {props.children} }
    }
}

#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element {
    let base = attributes!(p {
        class: Styles::dx_dialog_description.to_string()
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dialog::DialogDescription { id: props.id, attributes: merged, {props.children} }
    }
}
