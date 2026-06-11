use dioxus::prelude::*;
use dioxus_primitives::{
    dioxus_attributes::attributes,
    merge_attributes,
    textarea::{self, TextareaProps as PrimitiveTextareaProps},
};

#[css_module("/src/components/textarea/style.css")]
struct Styles;

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum TextareaVariant {
    #[default]
    Default,
    Fade,
    Outline,
    Ghost,
}

impl TextareaVariant {
    pub fn class(&self) -> &'static str {
        match self {
            TextareaVariant::Default => "default",
            TextareaVariant::Fade => "fade",
            TextareaVariant::Outline => "outline",
            TextareaVariant::Ghost => "ghost",
        }
    }
}

#[component]
pub fn Textarea(
    oninput: Option<EventHandler<FormEvent>>,
    onchange: Option<EventHandler<FormEvent>>,
    oninvalid: Option<EventHandler<FormEvent>>,
    onselect: Option<EventHandler<SelectionEvent>>,
    onselectionchange: Option<EventHandler<SelectionEvent>>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
    onfocusin: Option<EventHandler<FocusEvent>>,
    onfocusout: Option<EventHandler<FocusEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    onkeypress: Option<EventHandler<KeyboardEvent>>,
    onkeyup: Option<EventHandler<KeyboardEvent>>,
    oncompositionstart: Option<EventHandler<CompositionEvent>>,
    oncompositionupdate: Option<EventHandler<CompositionEvent>>,
    oncompositionend: Option<EventHandler<CompositionEvent>>,
    oncopy: Option<EventHandler<ClipboardEvent>>,
    oncut: Option<EventHandler<ClipboardEvent>>,
    onpaste: Option<EventHandler<ClipboardEvent>>,
    onmounted: Option<EventHandler<MountedEvent>>,
    #[props(default)] bottom_section: Option<Element>,
    #[props(default = false)] autosize: bool,
    #[props(default)] min_rows: Option<usize>,
    #[props(default)] max_rows: Option<usize>,
    #[props(default = 24.0)] autosize_line_height_px: f64,
    #[props(default = 16.0)] autosize_vertical_chrome_px: f64,
    #[props(default)] variant: TextareaVariant,
    #[props(extends = GlobalAttributes)]
    #[props(extends = textarea)]
    attributes: Vec<Attribute>,
    #[props(default)] root_attributes: Vec<Attribute>,
    #[props(default)] bottom_section_attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let textarea_base = attributes!(textarea {
        class: Styles::dx_textarea.to_string(),
        "data-slot": "textarea",
        "data-style": variant.class(),
    });
    let root_base = attributes!(div {
        class: "dx-textarea-root",
        style: "display: flex; flex-direction: column; gap: 0.5rem;",
    });
    let bottom_section_base = attributes!(div {
        class: "dx-textarea-bottom-section",
        style: "display: flex; align-items: center; justify-content: space-between; gap: 0.75rem; color: var(--secondary-color-5); font-size: 0.875rem;",
    });

    let attributes = merge_attributes(vec![textarea_base, attributes]);
    let root_attributes = merge_attributes(vec![root_base, root_attributes]);
    let bottom_section_attributes =
        merge_attributes(vec![bottom_section_base, bottom_section_attributes]);

    textarea::Textarea(PrimitiveTextareaProps {
        oninput,
        onchange,
        oninvalid,
        onselect,
        onselectionchange,
        onfocus,
        onblur,
        onfocusin,
        onfocusout,
        onkeydown,
        onkeypress,
        onkeyup,
        oncompositionstart,
        oncompositionupdate,
        oncompositionend,
        oncopy,
        oncut,
        onpaste,
        onmounted,
        bottom_section,
        autosize,
        min_rows,
        max_rows,
        autosize_line_height_px,
        autosize_vertical_chrome_px,
        attributes,
        root_attributes,
        bottom_section_attributes,
        children,
    })
}
