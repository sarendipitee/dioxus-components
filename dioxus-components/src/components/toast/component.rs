use crate::component_styles;
use dioxus::prelude::*;
use dioxus_primitives::toast::{
    self, Toast, ToastActionButtonProps, ToastActionsProps, ToastCloseButtonProps,
    ToastContentProps, ToastDescriptionProps, ToastPosition, ToastProps, ToastTitleProps,
};
use std::time::Duration;

#[component_styles("./style.css")]
struct Styles;

#[component]
fn StyledToast(props: ToastProps) -> Element {
    let has_action = props.action.is_some();
    let has_cancel = props.cancel.is_some();
    let action = props.action.clone();
    let cancel = props.cancel.clone();

    rsx! {
        Toast {
            id: props.id,
            index: props.index,
            title: props.title,
            description: props.description,
            toast_type: props.toast_type,
            on_close: props.on_close,
            permanent: props.permanent,
            duration: props.duration,
            action: props.action,
            cancel: props.cancel,
            class: Styles::dx_toast.to_string(),
            attributes: props.attributes,
            ToastContent {
                ToastTitle {}
                ToastDescription {}
                if has_action || has_cancel {
                    ToastActions {
                        if let Some(a) = action {
                            ToastActionButton { on_click: a.on_click, {a.label} }
                        }
                        if let Some(c) = cancel {
                            ToastActionButton {
                                on_click: c.on_click,
                                "data-cancel": "true",
                                {c.label}
                            }
                        }
                    }
                }
            }
            ToastCloseButton {}
        }
    }
}

#[component]
fn ToastContent(props: ToastContentProps) -> Element {
    rsx! {
        toast::ToastContent {
            class: Styles::dx_toast_content.to_string(),
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
fn ToastTitle(props: ToastTitleProps) -> Element {
    rsx! {
        toast::ToastTitle {
            class: Styles::dx_toast_title.to_string(),
            attributes: props.attributes,
            children: props.children,
        }
    }
}

#[component]
fn ToastDescription(props: ToastDescriptionProps) -> Element {
    rsx! {
        toast::ToastDescription {
            class: Styles::dx_toast_description.to_string(),
            attributes: props.attributes,
            children: props.children,
        }
    }
}

#[component]
fn ToastCloseButton(props: ToastCloseButtonProps) -> Element {
    rsx! {
        toast::ToastCloseButton {
            class: Styles::dx_toast_close.to_string(),
            attributes: props.attributes,
            children: props.children,
        }
    }
}

#[component]
fn ToastActions(props: ToastActionsProps) -> Element {
    rsx! {
        toast::ToastActions {
            class: Styles::dx_toast_actions.to_string(),
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
fn ToastActionButton(props: ToastActionButtonProps) -> Element {
    rsx! {
        toast::ToastActionButton {
            on_click: props.on_click,
            class: Styles::dx_toast_action.to_string(),
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ToastProvider(
    #[props(default = ReadSignal::new(Signal::new(Some(Duration::from_secs(5)))))]
    default_duration: ReadSignal<Option<Duration>>,
    #[props(default = ReadSignal::new(Signal::new(10)))] max_toasts: ReadSignal<usize>,
    #[props(default)] position: ToastPosition,
    #[props(default)] render_toast: Option<Callback<toast::ToastPropsWithOwner, Element>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let render_toast = render_toast.unwrap_or_else(|| {
        Callback::new(|p: toast::ToastPropsWithOwner| rsx! { StyledToast { ..p } })
    });

    rsx! {
        toast::ToastProvider {
            class: Styles::dx_toast_container.to_string(),
            default_duration,
            max_toasts,
            position,
            render_toast,
            attributes,
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus_primitives::toast::{use_toast, ToastOptions};

    #[component]
    fn ToastSuccess() -> Element {
        let toast_api = use_toast();
        use_hook(move || {
            toast_api.success(
                "Saved".to_string(),
                ToastOptions::new()
                    .description("Everything synced")
                    .permanent(true),
            );
        });
        rsx! {}
    }

    #[component]
    fn ToastWithActions() -> Element {
        let toast_api = use_toast();
        use_hook(move || {
            toast_api.info(
                "File deleted".to_string(),
                ToastOptions::new()
                    .permanent(true)
                    .action("Undo", move |_| {})
                    .cancel("Dismiss", move |_| {}),
            );
        });
        rsx! {}
    }

    #[component]
    fn ToastLoading() -> Element {
        let toast_api = use_toast();
        use_hook(move || {
            toast_api.loading("Uploading\u{2026}".to_string(), ToastOptions::new());
        });
        rsx! {}
    }

    #[test]
    fn styled_toast_preserves_primitive_fallback_children() {
        let mut dom = VirtualDom::new(|| rsx! { ToastProvider { ToastSuccess {} } });
        dom.rebuild_in_place();
        dom.mark_all_dirty();
        dom.render_immediate_to_vec();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("Saved"));
        assert!(html.contains("Everything synced"));
        assert!(html.contains('\u{00d7}') || html.contains("&#215;") || html.contains("&times;"));
    }

    #[test]
    fn action_button_renders_in_toast() {
        let mut dom = VirtualDom::new(|| rsx! { ToastProvider { ToastWithActions {} } });
        dom.rebuild_in_place();
        dom.mark_all_dirty();
        dom.render_immediate_to_vec();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("File deleted"));
        assert!(html.contains("Undo"));
        assert!(html.contains("Dismiss"));
    }

    #[test]
    fn loading_toast_renders_with_loading_type() {
        let mut dom = VirtualDom::new(|| rsx! { ToastProvider { ToastLoading {} } });
        dom.rebuild_in_place();
        dom.mark_all_dirty();
        dom.render_immediate_to_vec();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("Uploading"));
        assert!(html.contains("data-type=\"loading\""));
    }
}
