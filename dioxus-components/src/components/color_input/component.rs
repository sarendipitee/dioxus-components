use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use crate::component_styles;
use dioxus_primitives::color_picker::Color;
use dioxus_primitives::use_controlled;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
use palette::{encoding, FromColor, Hsv, IntoColor, Srgb};

use crate::components::color_picker::{ColorPickerRoot, ColorPickerSurface, ColorSwatch};
use crate::components::input::{
    element_label, use_input_control_context, Input, InputClearButton, InputControlContext,
    InputRadius, InputSize, InputVariant, InputWrapper,
};
use crate::components::popover::{PopoverContent, PopoverRoot};

#[component_styles("./style.css")]
struct Styles;

#[derive(Clone, Copy)]
struct ColorInputPopoverContext {
    open: Memo<bool>,
    set_open: Callback<bool>,
    disabled: ReadSignal<bool>,
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
    /// Optional fallback color used when clearing.
    #[props(default)]
    clear_color: Option<Hsv<encoding::Srgb, f64>>,
    /// Whether to render a shared clear affordance.
    #[props(default = false)]
    clearable: bool,
    /// Label rendered above the input.
    #[props(default)]
    label: Option<Element>,
    /// Description rendered below the label.
    #[props(default)]
    description: Option<Element>,
    /// Error rendered below the input.
    #[props(default)]
    error: Option<Element>,
    /// Marks the input as required.
    #[props(default = false)]
    required: bool,
    /// Shows the required asterisk without native validation.
    #[props(default = false)]
    with_asterisk: bool,
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
        clear_color,
        clearable,
        label,
        description,
        error,
        required,
        with_asterisk,
        variant,
        size,
        radius,
        right_section,
        open,
        default_open,
        on_open_change,
        attributes,
        children,
    } = props;
    let (popover_open, set_popover_open) = use_controlled(open, default_open, on_open_change);
    let is_disabled = disabled();
    let value = format_color_hex(color());
    let input_id = use_color_input_id();
    let popover_id = format!("{input_id}-popover");
    let description_id = description
        .as_ref()
        .map(|_| format!("{input_id}-description"));
    let error_id = error.as_ref().map(|_| format!("{input_id}-error"));
    let described_by = [description_id.as_deref(), error_id.as_deref()]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" ");
    let control_context = InputControlContext {
        id: input_id.clone(),
        described_by: (!described_by.is_empty()).then_some(described_by),
        invalid: error.is_some(),
    };
    use_context_provider(|| control_context);
    let mut draft_value = use_signal(|| value.clone());

    use_effect(use_reactive!(|value| {
        draft_value.set(value);
    }));

    let clear = (clearable && clear_color.is_some()).then(|| {
        rsx! {
            InputClearButton {
                aria_label: "Clear color",
                disabled: is_disabled,
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
            label: element_label(label),
            description,
            error: error.clone(),
            required,
            with_asterisk,
            disabled: is_disabled,
            PopoverRoot {
                is_modal: false,
                open: Some(popover_open()),
                on_open_change: move |v| set_popover_open.call(v),
                ColorPickerRoot {
                    color,
                    on_color_change,
                    disabled,
                    attributes,
                    ColorInputPopoverContextProvider {
                        open: popover_open,
                        set_open: set_popover_open,
                        disabled,
                        Input {
                            variant,
                            size,
                            radius,
                            disabled: is_disabled,
                            error: error.is_some(),
                            left_section: rsx! {
                                ColorSwatch { class: Styles::dx_color_input_color_swatch, color }
                            },
                            right_section,
                            ColorInputField {
                                value: draft_value,
                                canonical_value: value,
                                popover_id: popover_id.clone(),
                                on_color_change,
                            }
                        }
                        PopoverContent { id: popover_id,
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
    children: Element,
) -> Element {
    use_context_provider(|| ColorInputPopoverContext {
        open,
        set_open,
        disabled,
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
        class: Styles::dx_color_input.to_string(),
        r#type: "text",
        value: current_value.clone(),
        "aria-label": "Color value",
        "aria-controls": popover_id,
        "aria-haspopup": "dialog",
        "aria-expanded": (popover_context.open)(),
        autocapitalize: "off",
        autocomplete: "off",
        spellcheck: "false",
        disabled: if (popover_context.disabled)() { true },
    });
    let attributes = match control_attrs {
        Some(control_attrs) => merge_attributes(vec![base, control_attrs]),
        None => base,
    };

    rsx! {
        input {
            onfocus: move |_| {
                if !(popover_context.disabled)() {
                    popover_context.set_open.call(true);
                }
            },
            oninput: move |event| {
                let next = event.value();
                value.set(next.clone());
                if let Some(color) = parse_color_hex(&next) {
                    on_color_change.call(color);
                }
            },
            onchange: move |event| {
                let next = event.value();
                if let Some(color) = parse_color_hex(&next) {
                    on_color_change.call(color);
                    value.set(format_color_hex(color));
                }
            },
            onblur: move |_| {
                if let Some(color) = parse_color_hex(&value()) {
                    value.set(format_color_hex(color));
                } else {
                    value.set(canonical_value.clone());
                }
            },
            ..attributes,
        }
    }
}
