use std::sync::atomic::{AtomicUsize, Ordering};

use crate::component_styles;
use dioxus::prelude::*;
use dioxus_primitives::color_picker::Color;
use dioxus_primitives::use_controlled;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
use palette::{encoding, FromColor, Hsv, IntoColor, Srgb};

use crate::components::color_picker::{ColorPickerRoot, ColorPickerSurface, ColorSwatch};
use crate::components::input::{
    use_input_control_context, Input, InputClearButton, InputContent, InputLabel, InputRadius,
    InputSize, InputVariant, InputWrapper,
};
use crate::components::popover::{Popover, PopoverContent};

#[component_styles("./style.css")]
struct Styles;

#[derive(Clone, Copy)]
struct ColorInputPopoverContext {
    open: Memo<bool>,
    set_open: Callback<bool>,
    disabled: ReadSignal<bool>,
    read_only: ReadSignal<bool>,
}

fn normalize_hex(value: &str) -> Option<String> {
    let value = value.trim().strip_prefix('#').unwrap_or(value.trim());

    match value.len() {
        3 if value.bytes().all(|byte| byte.is_ascii_hexdigit()) => Some(
            value
                .chars()
                .flat_map(|ch| [ch.to_ascii_lowercase(), ch.to_ascii_lowercase()])
                .collect(),
        ),
        6 if value.bytes().all(|byte| byte.is_ascii_hexdigit()) => Some(value.to_ascii_lowercase()),
        _ => None,
    }
}

fn parse_color_hex(value: &str) -> Option<Hsv<encoding::Srgb, f64>> {
    let hex = normalize_hex(value)?;
    let red = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let green = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let blue = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some(
        Srgb::new(red, green, blue)
            .into_format::<f64>()
            .into_color(),
    )
}

fn format_color_hex(color: Hsv<encoding::Srgb, f64>) -> String {
    let rgb: Color = Srgb::<f64>::from_color(color).into_format();
    format!("#{rgb:X}")
}

fn use_color_input_id() -> String {
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

    use_hook(move || {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        format!("dx-color-input-{id}")
    })
}

#[derive(Props, Clone, PartialEq)]
pub struct ColorInputProps {
    /// The selected color.
    #[props(default)]
    color: ReadSignal<Hsv<encoding::Srgb, f64>>,
    /// Callback when color changes.
    #[props(default)]
    on_color_change: Callback<Hsv<encoding::Srgb, f64>>,
    /// Whether the color input is disabled.
    #[props(default)]
    disabled: ReadSignal<bool>,
    /// Whether the color input is read-only. Read-only inputs remain focusable and participate in forms.
    #[props(default)]
    read_only: ReadSignal<bool>,
    /// Optional fallback color used when clearing.
    #[props(default)]
    clear_color: Option<Hsv<encoding::Srgb, f64>>,
    /// Whether to render a shared clear affordance.
    #[props(default = false)]
    clearable: bool,
    /// Label rendered above the input.
    #[props(default, into)]
    label: InputLabel,
    /// Description rendered below the label.
    #[props(default, into)]
    description: InputContent,
    /// Error rendered below the input.
    #[props(default, into)]
    error: InputContent,
    /// Marks the input as required.
    #[props(default = false)]
    required: bool,
    /// Shows the required asterisk without native validation.
    #[props(default = false)]
    with_asterisk: bool,
    /// Shows a loading spinner in the trailing section and marks the field busy.
    #[props(default = false)]
    loading: bool,
    /// Visual variant for the shell.
    #[props(default)]
    variant: InputVariant,
    /// Size preset for the shell.
    #[props(default)]
    size: InputSize,
    /// Radius preset for the shell.
    #[props(default)]
    radius: InputRadius,
    /// Optional content rendered after the field value.
    #[props(default)]
    right_section: Option<Element>,
    /// Native form field name applied to the text control.
    #[props(default, into)]
    name: Option<String>,
    /// Id of the form associated with the text control.
    #[props(default, into)]
    form: Option<String>,
    /// Additional attributes applied directly to the native text control.
    #[props(default)]
    input_attributes: Vec<Attribute>,
    /// The controlled open state of the popover.
    open: ReadSignal<Option<bool>>,
    /// The default open state when uncontrolled.
    #[props(default)]
    default_open: bool,
    /// Callback fired when the open state changes.
    #[props(default)]
    on_open_change: Callback<bool>,
    /// Additional attributes to extend the color picker root.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Additional content to append to the picker popover.
    children: Element,
}

/// Styled color input composition built on the shared input foundation.
#[component]
pub fn ColorInput(props: ColorInputProps) -> Element {
    let ColorInputProps {
        color,
        on_color_change,
        disabled,
        read_only,
        clear_color,
        clearable,
        label,
        description,
        error,
        required,
        with_asterisk,
        loading,
        variant,
        size,
        radius,
        right_section,
        name,
        form,
        input_attributes,
        open,
        default_open,
        on_open_change,
        attributes,
        children,
    } = props;
    let (popover_open, set_popover_open) = use_controlled(open, default_open, on_open_change);
    let is_disabled = disabled();
    let is_read_only = read_only();
    let value = format_color_hex(color());
    let input_id = use_color_input_id();
    let popover_id = format!("{input_id}-popover");
    let mut draft_value = use_signal(|| value.clone());

    use_effect(use_reactive!(|value| {
        draft_value.set(value);
    }));

    let clear = (clearable && clear_color.is_some()).then(|| {
        rsx! {
            InputClearButton {
                aria_label: "Clear color",
                disabled: is_disabled || is_read_only,
                onclick: move |_| {
                    if let Some(color) = clear_color {
                        on_color_change.call(color);
                    }
                },
            }
        }
    });
    let right_section = match (clear, right_section) {
        (Some(clear), Some(right_section)) => Some(rsx! {
            div { style: "display: inline-flex; align-items: center; gap: 0.25rem;",
                {clear}
                {right_section}
            }
        }),
        (Some(clear), None) => Some(clear),
        (None, Some(right_section)) => Some(right_section),
        (None, None) => None,
    };

    rsx! {
        InputWrapper {
            id: input_id,
            label,
            description,
            error: error.clone(),
            required,
            with_asterisk,
            disabled: is_disabled,
            Popover {
                is_modal: false,
                open: Some(popover_open()),
                on_open_change: move |v| set_popover_open.call(v),
                ColorInputPopoverContextProvider {
                    open: popover_open,
                    set_open: set_popover_open,
                    disabled,
                    read_only,
                    dioxus_primitives::popover::PopoverTrigger {
                        style: "display: contents;",
                        "data-slot": "input",
                    Input {
                        variant,
                        size,
                        radius,
                        disabled: is_disabled,
                        error: error.is_some(),
                        loading,
                        left_section: rsx! {
                            ColorSwatch { class: Styles::dx_color_input_color_swatch, color }
                        },
                        right_section,
                        ColorInputField {
                            value: draft_value,
                            canonical_value: value,
                            popover_id: popover_id.clone(),
                            on_color_change,
                            read_only: is_read_only,
                            name,
                            form,
                            attributes: input_attributes,
                        }
                    }
                    }
                    PopoverContent {
                        id: popover_id,
                        width: dioxus_primitives::popover::PopoverWidth::Target,
                        target_selector: "[data-slot='input']",
                        ColorPickerRoot {
                            color,
                            on_color_change,
                            disabled: use_memo(move || disabled() || read_only()),
                            attributes,
                            ColorPickerSurface { {children} }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ColorInputPopoverContextProvider(
    open: Memo<bool>,
    set_open: Callback<bool>,
    disabled: ReadSignal<bool>,
    read_only: ReadSignal<bool>,
    children: Element,
) -> Element {
    use_context_provider(|| ColorInputPopoverContext {
        open,
        set_open,
        disabled,
        read_only,
    });

    rsx! {
        {children}
    }
}

#[component]
fn ColorInputField(
    value: Signal<String>,
    canonical_value: String,
    popover_id: String,
    on_color_change: Callback<Hsv<encoding::Srgb, f64>>,
    read_only: bool,
    name: Option<String>,
    form: Option<String>,
    attributes: Vec<Attribute>,
) -> Element {
    let popover_context = use_context::<ColorInputPopoverContext>();
    let control_attrs = use_input_control_context().map(|ctx| {
        attributes!(input {
            id: ctx.id,
            "aria-describedby": ctx.described_by,
            "aria-invalid": ctx.invalid,
        })
    });
    let current_value = value();
    let base = attributes!(input {
        class: Styles::dx_color_input,
        r#type: "text",
        value: current_value.clone(),
        "aria-controls": popover_id,
        "aria-haspopup": "dialog",
        "aria-expanded": (popover_context.open)(),
        autocapitalize: "off",
        autocomplete: "off",
        spellcheck: "false",
        disabled: if (popover_context.disabled)() { true },
        readonly: if read_only { true },
        name,
        form,
    });
    let attributes = match control_attrs {
        Some(control_attrs) => merge_attributes(vec![attributes, base, control_attrs]),
        None => merge_attributes(vec![attributes, base]),
    };
    let handlers = attributes!(input {
        onfocus: move |_| {
            if !(popover_context.disabled)() && !(popover_context.read_only)() {
                popover_context.set_open.call(true);
            }
        },
        oninput: move |event| {
            if !(popover_context.disabled)() && !read_only {
                let next = event.value();
                value.set(next.clone());
                if let Some(color) = parse_color_hex(&next) {
                    on_color_change.call(color);
                }
            }
        },
        onchange: move |event| {
            if !(popover_context.disabled)() && !read_only {
                let next = event.value();
                if let Some(color) = parse_color_hex(&next) {
                    on_color_change.call(color);
                    value.set(format_color_hex(color));
                }
            }
        },
        onblur: move |_| {
            if !(popover_context.disabled)() && !read_only {
                if let Some(color) = parse_color_hex(&value()) {
                    value.set(format_color_hex(color));
                } else {
                    value.set(canonical_value.clone());
                }
            }
        },
    });
    let attributes = merge_attributes(vec![attributes, handlers]);

    rsx! {
        input { ..attributes }
    }
}
