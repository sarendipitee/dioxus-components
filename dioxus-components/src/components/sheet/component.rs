use crate::component_styles;
use crate::components::dialog::DialogStyles;
use dioxus::prelude::*;
use dioxus_icons::lucide::X;
use dioxus_primitives::dialog;
use dioxus_primitives::overlay::OverlayKind;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes, TextOrElement};

#[component_styles("./style.css")]
struct Styles;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SheetSide {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

impl SheetSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            SheetSide::Top => "top",
            SheetSide::Right => "right",
            SheetSide::Bottom => "bottom",
            SheetSide::Left => "left",
        }
    }
}

/// Props for the [`Sheet`] component.
#[derive(Props, Clone, PartialEq)]
pub struct SheetProps {
    /// The ID of the sheet root element.
    pub id: ReadSignal<Option<String>>,

    /// Whether the sheet is modal. Defaults to `true`.
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

    /// Whether clicking the backdrop closes the sheet. Defaults to `true`.
    #[props(default = true)]
    pub close_on_backdrop_click: bool,

    /// Whether pressing Escape closes the sheet. Defaults to `true`.
    #[props(default = true)]
    pub close_on_escape: bool,

    /// Which side the sheet slides in from. Defaults to `Right`.
    #[props(default)]
    pub side: SheetSide,

    /// The title of the sheet. Rendered as an `<h2>` with `aria-labelledby` binding.
    /// Omit or pass `""` to hide.
    #[props(default, into)]
    pub title: TextOrElement<()>,

    /// The description of the sheet. Rendered as a `<p>` with `aria-describedby` binding.
    /// Omit or pass `""` to hide.
    #[props(default, into)]
    pub description: TextOrElement<()>,

    /// Whether to render a close button. Defaults to `true`.
    #[props(default = true)]
    pub with_close_button: bool,

    /// Optional footer content.
    /// Omit or pass `""` to hide.
    #[props(default, into)]
    pub footer: TextOrElement<()>,

    /// Additional attributes applied to the inner content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The body content of the sheet.
    pub children: Element,
}

/// A slide-in panel from the edge of the screen.
///
/// Built on the dialog primitive with `role="dialog"`, focus trap, scroll lock,
/// and optional title, description, close button, and footer.
#[component]
pub fn Sheet(props: SheetProps) -> Element {
    let title_has = !props.title.is_empty();
    let title_el = title_has.then(|| props.title.into_element());
    let desc_has = !props.description.is_empty();
    let desc_el = desc_has.then(|| props.description.into_element());
    let footer_has = !props.footer.is_empty();
    let footer_el = footer_has.then(|| props.footer.into_element());

    let root_attributes = attributes!(div {
        "data-slot": "sheet-root",
    });
    let base = attributes!(div {
        class: Styles::dx_sheet,
        "data-slot": "sheet-content",
        "data-side": props.side.as_str(),
    });
    let content_attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dialog::DialogRoot {
            id: props.id,
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            attributes: root_attributes,

            dialog::DialogContent {
                close_on_backdrop_click: props.close_on_backdrop_click,
                close_on_escape: props.close_on_escape,
                backdrop_class: DialogStyles::dx_dialog_backdrop,
                overlay_kind: OverlayKind::Sheet,
                overlay_stack_key: Some(props.side.as_str().to_string()),
                attributes: content_attributes,

                if props.with_close_button {
                    dialog::DialogClose {
                        class: DialogStyles::dx_dialog_close,
                        X { size: "20px" }
                    }
                }

                if title_el.is_some() || desc_el.is_some() {
                    div { class: DialogStyles::dx_dialog_header.to_string(),
                        if let Some(t) = title_el {
                            dialog::DialogTitle { {t} }
                        }
                        if let Some(d) = desc_el {
                            dialog::DialogDescription { {d} }
                        }
                    }
                }

                {props.children}

                if let Some(f) = footer_el {
                    div { class: DialogStyles::dx_dialog_footer.to_string(), {f} }
                }
            }
        }
    }
}
