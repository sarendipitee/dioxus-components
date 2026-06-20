use crate::components::button::{Button, ButtonVariant};
use crate::components::dialog::DialogStyles;
use dioxus::prelude::*;
use dioxus_primitives::alert_dialog::{
    self, AlertDialogActionProps, AlertDialogActionsProps, AlertDialogCancelProps,
    AlertDialogContentProps, AlertDialogDescriptionProps, AlertDialogRootProps,
    AlertDialogTitleProps, AlertDialogTriggerProps,
};
use dioxus_primitives::dialog::DialogCtx;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

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
        class: DialogStyles::dx_dialog.to_string()
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        alert_dialog::AlertDialogContent {
            id: props.id,
            backdrop_class: DialogStyles::dx_dialog_backdrop.to_string(),
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
                attributes!(div { class: DialogStyles::dx_dialog_header }),
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
                attributes!(div { class: DialogStyles::dx_dialog_body }),
                props.attributes,
            ]),
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogTitle(props: AlertDialogTitleProps) -> Element {
    let base = attributes!(h2 {
        class: DialogStyles::dx_dialog_title.to_string()
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    rsx! {
        alert_dialog::AlertDialogTitle {
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogDescription(props: AlertDialogDescriptionProps) -> Element {
    let base = attributes!(p {
        class: DialogStyles::dx_dialog_description.to_string()
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    rsx! {
        alert_dialog::AlertDialogDescription {
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogActions(props: AlertDialogActionsProps) -> Element {
    let merged = merge_attributes(vec![
        attributes!(div {
            class: DialogStyles::dx_dialog_footer.to_string()
        }),
        props.attributes,
    ]);
    rsx! {
        alert_dialog::AlertDialogActions { attributes: merged, {props.children} }
    }
}

#[component]
pub fn AlertDialogCancel(props: AlertDialogCancelProps) -> Element {
    let ctx: DialogCtx = use_context();
    let open = ctx.open_memo();
    let user_on_click = props.on_click;
    let on_click = use_callback(move |evt: MouseEvent| {
        ctx.set_open(false);
        if let Some(cb) = &user_on_click {
            cb.call(evt);
        }
    });
    let tabindex = if open() { "0" } else { "-1" };
    let attrs = merge_attributes(vec![
        attributes!(button { tabindex: tabindex }),
        props.attributes,
    ]);
    rsx! {
        Button {
            variant: ButtonVariant::Secondary,
            onclick: move |evt| on_click(evt),
            attributes: attrs,
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogAction(props: AlertDialogActionProps) -> Element {
    let ctx: DialogCtx = use_context();
    let open = ctx.open_memo();
    let user_on_click = props.on_click;
    let on_click = use_callback(move |evt: MouseEvent| {
        ctx.set_open(false);
        if let Some(cb) = &user_on_click {
            cb.call(evt);
        }
    });
    let tabindex = if open() { "0" } else { "-1" };
    let attrs = merge_attributes(vec![
        attributes!(button { tabindex: tabindex }),
        props.attributes,
    ]);
    rsx! {
        Button {
            variant: ButtonVariant::Destructive,
            onclick: move |evt| on_click(evt),
            attributes: attrs,
            {props.children}
        }
    }
}
