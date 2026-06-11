use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use dioxus_primitives::textarea::Textarea as PrimitiveTextarea;

#[css_module("/src/components/textarea/style.css")]
struct Styles;

/// Visual variant for the preview [`Textarea`] field.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum TextareaVariant {
    /// Default filled textarea styling.
    #[default]
    Default,
    /// Filled textarea styling that fades on interaction.
    Fade,
    /// Outlined textarea styling.
    Outline,
    /// Transparent textarea styling.
    Ghost,
}

impl TextareaVariant {
    /// Return the CSS class token for this textarea variant.
    pub fn class(self) -> &'static str {
        match self {
            TextareaVariant::Default => "default",
            TextareaVariant::Fade => "fade",
            TextareaVariant::Outline => "outline",
            TextareaVariant::Ghost => "ghost",
        }
    }
}

/// Controls the native resize affordance for [`Textarea`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum TextareaResize {
    /// Disable manual resizing.
    None,
    /// Allow manual vertical resizing.
    #[default]
    Vertical,
    /// Allow manual horizontal and vertical resizing.
    Both,
}

impl TextareaResize {
    /// Return the CSS resize token for this resize mode.
    pub fn class(self) -> &'static str {
        match self {
            TextareaResize::None => "none",
            TextareaResize::Vertical => "vertical",
            TextareaResize::Both => "both",
        }
    }
}

#[component]
/// A styled multi-line text input.
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
    /// Optional content rendered below the textarea inside the textarea container.
    #[props(default)]
    bottom_section: Option<Element>,
    /// Automatically grow and shrink the textarea height to fit its content.
    #[props(default = false)]
    autosize: bool,
    /// Minimum number of rows used when autosizing.
    #[props(default)]
    min_rows: Option<usize>,
    /// Maximum number of rows used when autosizing.
    #[props(default)]
    max_rows: Option<usize>,
    /// Native resize affordance for the textarea.
    #[props(default)]
    resize: TextareaResize,
    #[props(default)] variant: TextareaVariant,
    #[props(extends=GlobalAttributes)]
    #[props(extends=textarea)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let textarea_base = attributes!(textarea {
        class: Styles::dx_textarea,
        "data-slot": "textarea",
        "data-style": variant.class(),
        "data-resize": resize.class(),
        "data-autosize": autosize.then_some("true"),
    });
    let merged = merge_attributes(vec![textarea_base, attributes]);
    let root_attributes = attributes!(div {
        class: Styles::dx_textarea_root,
        "data-slot": "textarea-root",
    });
    let bottom_section_attributes = attributes!(div {
        class: Styles::dx_textarea_bottom_section,
        "data-slot": "textarea-bottom-section",
    });
    let vertical_chrome_px = 16.0
        + match variant {
            TextareaVariant::Outline => 2.0,
            _ => 0.0,
        };

    rsx! {
        PrimitiveTextarea {
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
            autosize_line_height_px: 24.0,
            autosize_vertical_chrome_px: vertical_chrome_px,
            attributes: merged,
            root_attributes,
            bottom_section_attributes,
            {children}
        }
    }
}
