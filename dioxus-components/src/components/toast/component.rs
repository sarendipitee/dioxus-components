use crate::component_styles;
use crate::components::typography::{
    TextAlign, TextWrap, TypographySize, TypographyTone, TypographyWeight,
};
use dioxus::prelude::*;
use dioxus_icons::lucide::{CircleAlert, CircleCheck, Info, TriangleAlert};
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use dioxus_primitives::toast::{
    self, Toast, ToastActionButtonProps, ToastActionsProps, ToastCloseButtonProps,
    ToastContentProps, ToastDescriptionProps, ToastPosition, ToastProps, ToastTitleProps,
    ToastType,
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

    // Loading toasts convey state through the spinner, so they get no leading icon.
    let icon = match props.toast_type {
        ToastType::Success => Some(rsx! { CircleCheck { size: "1.25rem" } }),
        ToastType::Error => Some(rsx! { CircleAlert { size: "1.25rem" } }),
        ToastType::Warning => Some(rsx! { TriangleAlert { size: "1.25rem" } }),
        ToastType::Info => Some(rsx! { Info { size: "1.25rem" } }),
        ToastType::Loading => None,
    };

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
            removing: props.removing,
            class: Styles::dx_toast,
            attributes: props.attributes,
            if let Some(icon) = icon {
                ToastIcon { {icon} }
            }
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

/// Renders the leading variant icon for a toast.
#[component]
fn ToastIcon(children: Element) -> Element {
    rsx! {
        div {
            class: Styles::dx_toast_icon.to_string(),
            "data-slot": "toast-icon",
            "aria-hidden": "true",
            {children}
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
    let base = typography_slot_attributes(
        format!("{} dx_heading", Styles::dx_toast_title),
        "toast-title",
        TypographySize::Md,
        TypographyTone::Default,
        TypographyWeight::Semibold,
    );
    let attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        toast::ToastTitle {
            attributes,
            children: props.children,
        }
    }
}

#[component]
fn ToastDescription(props: ToastDescriptionProps) -> Element {
    let base = typography_slot_attributes(
        format!("{} dx_text", Styles::dx_toast_description),
        "toast-description",
        TypographySize::Sm,
        TypographyTone::Default,
        TypographyWeight::Inherit,
    );
    let attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        toast::ToastDescription {
            attributes,
            children: props.children,
        }
    }
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

#[component]
fn ToastCloseButton(props: ToastCloseButtonProps) -> Element {
    rsx! {
        toast::ToastCloseButton {
            class: Styles::dx_toast_close,
            attributes: props.attributes,
            children: props.children,
        }
    }
}

#[component]
fn ToastActions(props: ToastActionsProps) -> Element {
    rsx! {
        toast::ToastActions {
            class: Styles::dx_toast_actions,
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
            class: Styles::dx_toast_action,
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
            class: Styles::dx_toast_container,
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
                "Saved",
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
                "File deleted",
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
            toast_api.loading("Uploading\u{2026}", ToastOptions::new());
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
