//! Defines the [`Textarea`] primitive component.

use crate::dioxus_attributes::attributes;
use crate::merge_attributes;
use dioxus::core::AttributeValue;
use dioxus::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static TEXTAREA_ID: AtomicUsize = AtomicUsize::new(0);

/// The props for the [`Textarea`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TextareaProps {
    /// Callback fired when the textarea input value changes.
    #[props(default)]
    pub oninput: Option<EventHandler<FormEvent>>,
    /// Callback fired when the textarea value is committed.
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,
    /// Callback fired when the textarea fails HTML validation.
    #[props(default)]
    pub oninvalid: Option<EventHandler<FormEvent>>,
    /// Callback fired when text selection changes on the textarea.
    #[props(default)]
    pub onselect: Option<EventHandler<SelectionEvent>>,
    /// Callback fired when the selection range changes.
    #[props(default)]
    pub onselectionchange: Option<EventHandler<SelectionEvent>>,
    /// Callback fired when the textarea receives focus.
    #[props(default)]
    pub onfocus: Option<EventHandler<FocusEvent>>,
    /// Callback fired when the textarea loses focus.
    #[props(default)]
    pub onblur: Option<EventHandler<FocusEvent>>,
    /// Callback fired when focus enters the textarea subtree.
    #[props(default)]
    pub onfocusin: Option<EventHandler<FocusEvent>>,
    /// Callback fired when focus leaves the textarea subtree.
    #[props(default)]
    pub onfocusout: Option<EventHandler<FocusEvent>>,
    /// Callback fired on keydown events.
    #[props(default)]
    pub onkeydown: Option<EventHandler<KeyboardEvent>>,
    /// Callback fired on keypress events.
    #[props(default)]
    pub onkeypress: Option<EventHandler<KeyboardEvent>>,
    /// Callback fired on keyup events.
    #[props(default)]
    pub onkeyup: Option<EventHandler<KeyboardEvent>>,
    /// Callback fired when text composition starts.
    #[props(default)]
    pub oncompositionstart: Option<EventHandler<CompositionEvent>>,
    /// Callback fired when text composition updates.
    #[props(default)]
    pub oncompositionupdate: Option<EventHandler<CompositionEvent>>,
    /// Callback fired when text composition ends.
    #[props(default)]
    pub oncompositionend: Option<EventHandler<CompositionEvent>>,
    /// Callback fired when text is copied.
    #[props(default)]
    pub oncopy: Option<EventHandler<ClipboardEvent>>,
    /// Callback fired when text is cut.
    #[props(default)]
    pub oncut: Option<EventHandler<ClipboardEvent>>,
    /// Callback fired when text is pasted.
    #[props(default)]
    pub onpaste: Option<EventHandler<ClipboardEvent>>,
    /// Callback fired when the textarea is mounted.
    #[props(default)]
    pub onmounted: Option<EventHandler<MountedEvent>>,
    /// Optional content rendered below the textarea inside the same container.
    #[props(default)]
    pub bottom_section: Option<Element>,
    /// Whether the textarea should grow and shrink to fit its content.
    #[props(default = false)]
    pub autosize: bool,
    /// Minimum number of rows used when autosizing.
    #[props(default)]
    pub min_rows: Option<usize>,
    /// Maximum number of rows used when autosizing.
    #[props(default)]
    pub max_rows: Option<usize>,
    /// Line height, in CSS pixels, used by autosizing fallbacks.
    #[props(default = 24.0)]
    pub autosize_line_height_px: f64,
    /// Total vertical padding and border, in CSS pixels, used by autosizing fallbacks.
    #[props(default = 16.0)]
    pub autosize_vertical_chrome_px: f64,
    /// Additional attributes to apply to the textarea element.
    #[props(extends = GlobalAttributes)]
    #[props(extends = textarea)]
    pub attributes: Vec<Attribute>,
    /// Additional attributes to apply to the optional root container.
    #[props(default)]
    pub root_attributes: Vec<Attribute>,
    /// Additional attributes to apply to the optional bottom section container.
    #[props(default)]
    pub bottom_section_attributes: Vec<Attribute>,
    /// Children rendered inside the textarea element.
    pub children: Element,
}

/// # Textarea
///
/// The `Textarea` primitive renders a multi-line text input with optional autosizing.
/// Styling is controlled by the caller via forwarded attributes.
#[component]
pub fn Textarea(props: TextareaProps) -> Element {
    let TextareaProps {
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
    } = props;
    let textarea_id = use_hook(|| TEXTAREA_ID.fetch_add(1, Ordering::Relaxed));
    let value = textarea_value_attribute(&attributes);
    let autosize_handle = use_textarea_autosize(
        textarea_id,
        autosize,
        min_rows,
        max_rows,
        autosize_line_height_px,
        autosize_vertical_chrome_px,
        value,
    );

    let base = attributes!(textarea {
        "data-textarea-id": "{textarea_id}",
        style: autosize_handle.current_style(),
    });
    let merged = merge_attributes(vec![base, attributes]);

    let textarea = rsx! {
        textarea {
            oninput: move |e| {
                autosize_handle.schedule_resize();
                _ = oninput.map(|callback| callback(e));
            },
            onchange: move |e| _ = onchange.map(|callback| callback(e)),
            oninvalid: move |e| _ = oninvalid.map(|callback| callback(e)),
            onselect: move |e| _ = onselect.map(|callback| callback(e)),
            onselectionchange: move |e| _ = onselectionchange.map(|callback| callback(e)),
            onfocus: move |e| _ = onfocus.map(|callback| callback(e)),
            onblur: move |e| _ = onblur.map(|callback| callback(e)),
            onfocusin: move |e| _ = onfocusin.map(|callback| callback(e)),
            onfocusout: move |e| _ = onfocusout.map(|callback| callback(e)),
            onkeydown: move |e| _ = onkeydown.map(|callback| callback(e)),
            onkeypress: move |e| _ = onkeypress.map(|callback| callback(e)),
            onkeyup: move |e| _ = onkeyup.map(|callback| callback(e)),
            oncompositionstart: move |e| _ = oncompositionstart.map(|callback| callback(e)),
            oncompositionupdate: move |e| _ = oncompositionupdate.map(|callback| callback(e)),
            oncompositionend: move |e| _ = oncompositionend.map(|callback| callback(e)),
            oncopy: move |e| _ = oncopy.map(|callback| callback(e)),
            oncut: move |e| _ = oncut.map(|callback| callback(e)),
            onpaste: move |e| _ = onpaste.map(|callback| callback(e)),
            onmounted: move |e| {
                autosize_handle.set_mounted();
                _ = onmounted.map(|callback| callback(e));
            },
            ..merged,
            {children}
        }
    };

    if bottom_section.is_none() {
        return textarea;
    }

    rsx! {
        div {
            ..root_attributes,
            {textarea}
            div {
                ..bottom_section_attributes,
                {bottom_section}
            }
        }
    }
}

fn textarea_value_attribute(attributes: &[Attribute]) -> Option<AttributeValue> {
    attributes
        .iter()
        .find(|attribute| attribute.name == "value")
        .map(|attribute| attribute.value.clone())
}

#[derive(Clone, Copy)]
struct TextareaAutosize {
    style: Signal<String>,
    mounted: Signal<bool>,
    revision: Signal<u64>,
}

impl TextareaAutosize {
    fn current_style(self) -> String {
        self.style.read().clone()
    }

    fn schedule_resize(self) {
        let mut revision = self.revision;
        *revision.write() += 1;
    }

    fn set_mounted(self) {
        let mut mounted = self.mounted;
        mounted.set(true);
        self.schedule_resize();
    }
}

fn use_textarea_autosize(
    textarea_id: usize,
    autosize: bool,
    min_rows: Option<usize>,
    max_rows: Option<usize>,
    line_height_px: f64,
    vertical_chrome_px: f64,
    value: Option<AttributeValue>,
) -> TextareaAutosize {
    let handle = TextareaAutosize {
        style: use_signal(String::new),
        mounted: use_signal(|| false),
        revision: use_signal(|| 0_u64),
    };

    use_effect(use_reactive(
        (
            &autosize,
            &min_rows,
            &max_rows,
            &line_height_px,
            &vertical_chrome_px,
            &value,
        ),
        move |_| handle.schedule_resize(),
    ));

    use_effect(move || {
        let revision = handle.revision;
        let mounted = handle.mounted;
        let mut style_signal = handle.style;
        let _ = revision();

        if !mounted() {
            return;
        }

        if !autosize {
            if !style_signal.read().is_empty() {
                style_signal.set(String::new());
            }
            return;
        }

        let metrics = TextareaMetrics {
            line_height_px,
            vertical_chrome_px,
        };
        let style = textarea_autosize_style_from_pretext(
            textarea_id,
            min_rows,
            max_rows,
            metrics,
            value.as_ref(),
        )
        .unwrap_or_default();
        style_signal.set(style);
    });

    handle
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct TextareaMetrics {
    line_height_px: f64,
    vertical_chrome_px: f64,
}

fn textarea_autosize_style(
    scroll_height: f64,
    metrics: TextareaMetrics,
    min_rows: Option<usize>,
    max_rows: Option<usize>,
) -> String {
    let mut height = scroll_height;
    let mut overflow_y = "hidden";

    if let Some(min_rows) = min_rows.filter(|rows| *rows > 0) {
        let min_height = metrics.line_height_px * min_rows as f64 + metrics.vertical_chrome_px;
        height = height.max(min_height);
    }

    if let Some(max_rows) = max_rows.filter(|rows| *rows > 0) {
        let max_height = metrics.line_height_px * max_rows as f64 + metrics.vertical_chrome_px;
        if scroll_height > max_height {
            overflow_y = "auto";
        }
        height = height.min(max_height);
    }

    format!("height: {height}px; overflow-y: {overflow_y};")
}

#[cfg(any(test, all(feature = "web", target_arch = "wasm32")))]
fn textarea_terminal_blank_line_compensation(value: &str, line_height_px: f64) -> f64 {
    if value.ends_with('\n') {
        line_height_px
    } else {
        0.0
    }
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn textarea_autosize_style_from_pretext(
    textarea_id: usize,
    min_rows: Option<usize>,
    max_rows: Option<usize>,
    fallback_metrics: TextareaMetrics,
    _value: Option<&AttributeValue>,
) -> Option<String> {
    use gpui_pretext::{
        layout, prepare, EngineProfile, PrepareOptions, TextMeasure, WhiteSpaceMode,
    };
    use wasm_bindgen::JsCast;

    struct CanvasTextMeasure {
        context: web_sys::CanvasRenderingContext2d,
    }

    impl TextMeasure for CanvasTextMeasure {
        fn measure_width(&self, text: &str) -> f64 {
            self.context
                .measure_text(text)
                .ok()
                .map(|metrics| metrics.width())
                .unwrap_or_default()
        }
    }

    let measurement = textarea_measurement_from_dom(textarea_id, fallback_metrics)?;
    let document = web_sys::window()?.document()?;
    let canvas = document
        .create_element("canvas")
        .ok()?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .ok()?;
    let context = canvas
        .get_context("2d")
        .ok()??
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .ok()?;
    context.set_font(&measurement.font);

    let prepared = prepare(
        &measurement.value,
        &CanvasTextMeasure { context },
        &EngineProfile::default(),
        &PrepareOptions {
            white_space: WhiteSpaceMode::PreWrap,
        },
    );
    let layout = layout(
        &prepared,
        measurement.content_width_px.max(1.0),
        measurement.metrics.line_height_px,
        &EngineProfile::default(),
    );
    let content_height = (layout.height
        + textarea_terminal_blank_line_compensation(
            &measurement.value,
            measurement.metrics.line_height_px,
        ))
    .max(measurement.metrics.line_height_px);

    Some(textarea_autosize_style(
        content_height + measurement.metrics.vertical_chrome_px,
        measurement.metrics,
        min_rows,
        max_rows,
    ))
}

#[cfg(not(all(feature = "web", target_arch = "wasm32")))]
fn textarea_autosize_style_from_pretext(
    _textarea_id: usize,
    min_rows: Option<usize>,
    max_rows: Option<usize>,
    metrics: TextareaMetrics,
    value: Option<&AttributeValue>,
) -> Option<String> {
    let value = textarea_value_text(value)?;
    let line_count = textarea_line_count(&value).max(1) as f64;
    let scroll_height = line_count * metrics.line_height_px + metrics.vertical_chrome_px;

    Some(textarea_autosize_style(
        scroll_height,
        metrics,
        min_rows,
        max_rows,
    ))
}

fn textarea_value_text(value: Option<&AttributeValue>) -> Option<String> {
    match value? {
        AttributeValue::Text(text) => Some(text.clone()),
        AttributeValue::Float(number) => Some(number.to_string()),
        AttributeValue::Int(number) => Some(number.to_string()),
        AttributeValue::Bool(boolean) => Some(boolean.to_string()),
        AttributeValue::None => Some(String::new()),
        AttributeValue::Listener(_) | AttributeValue::Any(_) => None,
    }
}

fn textarea_line_count(value: &str) -> usize {
    value.lines().count() + usize::from(value.ends_with('\n'))
}

fn textarea_content_width_from_layout_width(
    width_px: f64,
    horizontal_chrome_px: f64,
    scrollbar_gutter_px: f64,
) -> f64 {
    (width_px - horizontal_chrome_px - scrollbar_gutter_px).max(0.0)
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
struct TextareaMeasurement {
    content_width_px: f64,
    font: String,
    metrics: TextareaMetrics,
    value: String,
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn parse_px(css_value: &str) -> Option<f64> {
    let trimmed = css_value.trim();
    let px = trimmed.strip_suffix("px")?;
    px.trim().parse().ok()
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn textarea_content_width_px(
    style: &web_sys::CssStyleDeclaration,
    padding_left: f64,
    padding_right: f64,
    border_left: f64,
    border_right: f64,
    scrollbar_gutter_px: f64,
) -> Option<f64> {
    let width_px = parse_px(&style.get_property_value("width").ok()?)?;
    let box_sizing = style.get_property_value("box-sizing").ok()?;
    let horizontal_chrome_px = if box_sizing.trim() == "border-box" {
        padding_left + padding_right + border_left + border_right
    } else {
        0.0
    };

    Some(textarea_content_width_from_layout_width(
        width_px,
        horizontal_chrome_px,
        scrollbar_gutter_px,
    ))
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn textarea_measurement_from_dom(
    textarea_id: usize,
    fallback_metrics: TextareaMetrics,
) -> Option<TextareaMeasurement> {
    use wasm_bindgen::JsCast;

    let window = web_sys::window()?;
    let document = window.document()?;
    let element = document
        .query_selector(&format!(r#"textarea[data-textarea-id="{textarea_id}"]"#))
        .ok()??;
    let textarea = element.dyn_into::<web_sys::HtmlTextAreaElement>().ok()?;
    let style = window.get_computed_style(&textarea).ok()??;

    let padding_top = parse_px(&style.get_property_value("padding-top").ok()?)?;
    let padding_bottom = parse_px(&style.get_property_value("padding-bottom").ok()?)?;
    let padding_left = parse_px(&style.get_property_value("padding-left").ok()?)?;
    let padding_right = parse_px(&style.get_property_value("padding-right").ok()?)?;
    let border_left = parse_px(&style.get_property_value("border-left-width").ok()?)?;
    let border_right = parse_px(&style.get_property_value("border-right-width").ok()?)?;
    let border_top = parse_px(&style.get_property_value("border-top-width").ok()?)?;
    let border_bottom = parse_px(&style.get_property_value("border-bottom-width").ok()?)?;
    let line_height_px = parse_px(&style.get_property_value("line-height").ok()?)
        .unwrap_or(fallback_metrics.line_height_px);
    let font = textarea_font_string(&style)?;
    let scrollbar_gutter_px = (f64::from(textarea.offset_width())
        - f64::from(textarea.client_width())
        - border_left
        - border_right)
        .max(0.0);
    let content_width_px = textarea_content_width_px(
        &style,
        padding_left,
        padding_right,
        border_left,
        border_right,
        scrollbar_gutter_px,
    )?;

    Some(TextareaMeasurement {
        content_width_px,
        font,
        metrics: TextareaMetrics {
            line_height_px,
            vertical_chrome_px: padding_top + padding_bottom + border_top + border_bottom,
        },
        value: textarea.value(),
    })
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn textarea_font_string(style: &web_sys::CssStyleDeclaration) -> Option<String> {
    let font_style = style.get_property_value("font-style").ok()?;
    let font_variant = style.get_property_value("font-variant").ok()?;
    let font_weight = style.get_property_value("font-weight").ok()?;
    let font_stretch = style.get_property_value("font-stretch").ok()?;
    let font_size = style.get_property_value("font-size").ok()?;
    let font_family = style.get_property_value("font-family").ok()?;

    Some(format!(
        "{} {} {} {} {} {}",
        font_style.trim(),
        font_variant.trim(),
        font_weight.trim(),
        font_stretch.trim(),
        font_size.trim(),
        font_family.trim()
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        textarea_content_width_from_layout_width, textarea_terminal_blank_line_compensation,
    };

    #[test]
    fn content_width_subtracts_border_box_chrome_and_scrollbar() {
        assert_eq!(
            textarea_content_width_from_layout_width(240.5, 18.25, 12.0),
            210.25
        );
    }

    #[test]
    fn content_width_preserves_fractional_content_box_width() {
        assert_eq!(
            textarea_content_width_from_layout_width(240.5, 0.0, 12.0),
            228.5
        );
    }

    #[test]
    fn terminal_blank_line_compensation_is_zero_without_trailing_newline() {
        assert_eq!(
            textarea_terminal_blank_line_compensation("alpha\nbeta", 24.0),
            0.0
        );
    }

    #[test]
    fn terminal_blank_line_compensation_adds_one_line_for_trailing_newline() {
        assert_eq!(
            textarea_terminal_blank_line_compensation("alpha\n", 24.0),
            24.0
        );
    }

    #[test]
    fn terminal_blank_line_compensation_adds_one_line_for_multiple_trailing_blank_lines() {
        assert_eq!(
            textarea_terminal_blank_line_compensation("alpha\n\n", 24.0),
            24.0
        );
    }
}
