use dioxus::prelude::*;
use crate::component_styles;
use dioxus_primitives::alert_dialog::{
    self, AlertDialogActionProps, AlertDialogActionsProps, AlertDialogCancelProps,
    AlertDialogContentProps, AlertDialogDescriptionProps, AlertDialogRootProps,
    AlertDialogTitleProps, AlertDialogTriggerProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[component_styles("./style.css")]
struct Styles;

/// The root alert dialog component — a context provider that manages open state.
/// Place [`AlertDialogContent`] as a child.
#[component]
pub fn AlertDialog(props: AlertDialogRootProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogRoot { ..props }
    }
}

/// A button that opens the alert dialog. Must be a child of [`AlertDialog`].
#[component]
pub fn AlertDialogTrigger(props: AlertDialogTriggerProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogTrigger { ..props }
    }
}

/// The styled alert dialog panel with animated backdrop. Must be a child of [`AlertDialog`].
#[component]
pub fn AlertDialogContent(props: AlertDialogContentProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_alert_dialog.to_string()
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        alert_dialog::AlertDialogContent {
            id: props.id,
            backdrop_class: Styles::dx_alert_dialog_backdrop.to_string(),
            attributes: merged,
            {props.children}
        }
    }
}

/// Props for layout container components ([`AlertDialogHeader`], [`AlertDialogBody`]).
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogLayoutProps {
    /// Additional attributes to apply to the container element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
}

/// Groups title and description at the top of an alert dialog.
#[component]
pub fn AlertDialogHeader(props: AlertDialogLayoutProps) -> Element {
    rsx! {
        div {
            ..merge_attributes(vec![
                attributes!(div { class: Styles::dx_alert_dialog_header.to_string() }),
                props.attributes,
            ]),
            {props.children}
        }
    }
}

/// The scrollable content area of an alert dialog.
#[component]
pub fn AlertDialogBody(props: AlertDialogLayoutProps) -> Element {
    rsx! {
        div {
            ..merge_attributes(vec![
                attributes!(div { class: Styles::dx_alert_dialog_body.to_string() }),
                props.attributes,
            ]),
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogTitle(props: AlertDialogTitleProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_alert_dialog_title.to_string()
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    rsx! {
        alert_dialog::AlertDialogTitle { attributes: merged, {props.children} }
    }
}

#[component]
pub fn AlertDialogDescription(props: AlertDialogDescriptionProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_alert_dialog_description.to_string()
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    rsx! {
        alert_dialog::AlertDialogDescription { attributes: merged, {props.children} }
    }
}

#[component]
pub fn AlertDialogActions(props: AlertDialogActionsProps) -> Element {
    let merged = merge_attributes(vec![
        attributes!(div {
            class: Styles::dx_alert_dialog_actions.to_string()
        }),
        props.attributes,
    ]);
    rsx! {
        alert_dialog::AlertDialogActions { attributes: merged, {props.children} }
    }
}

#[component]
pub fn AlertDialogCancel(props: AlertDialogCancelProps) -> Element {
    let merged = merge_attributes(vec![
        attributes!(div {
            class: Styles::dx_alert_dialog_cancel.to_string()
        }),
        props.attributes,
    ]);
    rsx! {
        alert_dialog::AlertDialogCancel {
            on_click: props.on_click,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogAction(props: AlertDialogActionProps) -> Element {
    let merged = merge_attributes(vec![
        attributes!(div {
            class: Styles::dx_alert_dialog_action.to_string()
        }),
        props.attributes,
    ]);
    rsx! {
        alert_dialog::AlertDialogAction {
            on_click: props.on_click,
            attributes: merged,
            {props.children}
        }
    }
}
