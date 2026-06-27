use crate::component_styles;
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

use crate::components::input::{
    attribute_text, merge_described_by, use_input_id, InputBase, InputContent, InputLabel,
    InputRadius, InputSize, InputVariant,
};

#[component_styles("./style.css")]
struct Styles;

/// When `min`/`max` are enforced.
#[derive(Copy, Clone, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum ClampBehavior {
    /// Clamp the value when the field loses focus.
    #[default]
    Blur,
    /// Prevent the user from entering values outside `min`/`max`.
    Strict,
    /// No automatic clamping; the value may exceed bounds.
    None,
}

fn do_format(v: f64, decimal_scale: Option<usize>, decimal_sep: &str, thousands_sep: &str) -> String {
    let raw = match decimal_scale {
        Some(scale) => format!("{:.prec$}", v, prec = scale),
        None => v.to_string(),
    };

    let (int_part, dec_part) = match raw.find('.') {
        Some(pos) => (raw[..pos].to_string(), Some(raw[pos + 1..].to_string())),
        None => (raw, None),
    };

    let int_fmt = if !thousands_sep.is_empty() {
        let neg = int_part.starts_with('-');
        let digits = if neg { &int_part[1..] } else { &int_part };
        let mut rev = String::new();
        for (i, c) in digits.chars().rev().enumerate() {
            if i > 0 && i % 3 == 0 {
                rev.extend(thousands_sep.chars().rev());
            }
            rev.push(c);
        }
        if neg {
            rev.push('-');
        }
        rev.chars().rev().collect::<String>()
    } else {
        int_part
    };

    match dec_part {
        Some(dec) => format!("{}{}{}", int_fmt, decimal_sep, dec),
        None => int_fmt,
    }
}

fn do_parse(s: &str, decimal_sep: &str, thousands_sep: &str) -> Option<f64> {
    if s.is_empty() || s == "-" {
        return None;
    }
    let s = if !thousands_sep.is_empty() {
        s.replace(thousands_sep, "")
    } else {
        s.to_string()
    };
    let s = if decimal_sep != "." { s.replace(decimal_sep, ".") } else { s };
    s.parse::<f64>().ok()
}

fn do_clamp(v: f64, min: Option<f64>, max: Option<f64>) -> f64 {
    let v = min.map_or(v, |lo| v.max(lo));
    max.map_or(v, |hi| v.min(hi))
}

fn sanitize(s: &str, allow_decimal: bool, allow_negative: bool, decimal_char: char) -> String {
    let mut out = String::new();
    let mut seen_decimal = false;
    let mut seen_minus = false;

    for c in s.chars() {
        if c == '-' && !seen_minus && out.is_empty() && allow_negative {
            seen_minus = true;
            out.push(c);
        } else if c == decimal_char && !seen_decimal && allow_decimal {
            seen_decimal = true;
            out.push(c);
        } else if c.is_ascii_digit() {
            out.push(c);
        }
    }
    out
}

/// Numeric input with stepper controls, value clamping, and display formatting.
/// Mirrors the feature set of Mantine's `NumberInput`.
#[component]
pub fn NumberInput(
    /// Controlled value. Pair with `on_change` to close the loop. When `Some`,
    /// the display syncs whenever this value changes between renders.
    #[props(default)]
    value: Option<f64>,
    /// Initial value for uncontrolled mode.
    #[props(default)]
    default_value: Option<f64>,
    /// Called whenever the parsed value changes. Receives `None` when the field
    /// is empty or holds an incomplete expression (e.g. `"-"` or `"1."`).
    #[props(default)]
    on_change: Option<Callback<Option<f64>>>,
    /// Lower bound. `None` removes the limit.
    #[props(default)]
    min: Option<f64>,
    /// Upper bound. `None` removes the limit.
    #[props(default)]
    max: Option<f64>,
    /// Amount added or subtracted by the stepper buttons and arrow keys.
    #[props(default = 1.0)]
    step: f64,
    /// Fixed number of decimal places applied when the field blurs.
    /// `None` preserves the user's input precision.
    #[props(default)]
    decimal_scale: Option<usize>,
    /// Whether the user may enter a decimal value. Defaults to `true`.
    #[props(default = true)]
    allow_decimal: bool,
    /// Whether the user may enter a negative value. Defaults to `true`.
    #[props(default = true)]
    allow_negative: bool,
    /// When `min`/`max` are enforced.
    #[props(default)]
    clamp_behavior: ClampBehavior,
    /// Text rendered before the value inside the input shell (e.g. `"$"`).
    #[props(default, into)]
    prefix: String,
    /// Text rendered after the value inside the input shell (e.g. `"%"`).
    #[props(default, into)]
    suffix: String,
    /// Thousands grouping separator rendered in the display value (e.g. `","`).
    #[props(default, into)]
    thousands_separator: String,
    /// Decimal point character accepted and rendered. Defaults to `"."`.
    #[props(default = ".".to_string(), into)]
    decimal_separator: String,
    /// Hides the increment/decrement stepper buttons.
    #[props(default = false)]
    hide_controls: bool,
    // ---- InputBase props ----
    /// Label rendered above the input.
    #[props(default, into)]
    label: InputLabel,
    /// Description rendered below the label.
    #[props(default, into)]
    description: InputContent,
    /// Error rendered below the input and reflected with `aria-invalid`.
    #[props(default, into)]
    error: InputContent,
    /// Marks the field required.
    #[props(default = false)]
    required: bool,
    /// Shows the required asterisk without affecting native validation.
    #[props(default = false)]
    with_asterisk: bool,
    /// Marks the field disabled.
    #[props(default = false)]
    disabled: bool,
    /// Shows a loading spinner in the trailing section and marks the field busy.
    #[props(default = false)]
    loading: bool,
    /// Existing ids to prepend to generated described-by ids.
    #[props(default)]
    described_by: Option<String>,
    /// Visual variant for the input shell.
    #[props(default)]
    variant: InputVariant,
    /// Size preset for the shell.
    #[props(default)]
    size: InputSize,
    /// Radius preset for the shell.
    #[props(default)]
    radius: InputRadius,
    /// Optional content rendered before the prefix.
    #[props(default)]
    left_section: Option<Element>,
    /// Root wrapper attributes.
    #[props(default)]
    wrapper_attributes: Vec<Attribute>,
    /// Shell attributes.
    #[props(default)]
    input_attributes: Vec<Attribute>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    onmounted: Option<EventHandler<MountedEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = input)]
    attributes: Vec<Attribute>,
) -> Element {
    let generated_id = use_input_id("dx-number-input");
    let input_id = attribute_text(&attributes, "id").unwrap_or(generated_id);
    let invalid = error.is_some();

    let user_described_by = attribute_text(&attributes, "aria-describedby")
        .or_else(|| attribute_text(&attributes, "aria_describedby"))
        .or(described_by);
    let description_id = description.is_some().then(|| format!("{input_id}-description"));
    let error_id = error.is_some().then(|| format!("{input_id}-error"));
    let aria_describedby = merge_described_by(
        description_id.as_deref(),
        error_id.as_deref(),
        user_described_by.as_deref(),
    );

    let decimal_char = decimal_separator.chars().next().unwrap_or('.');

    // Pre-clone separators for each closure that needs them
    let dec_sep = decimal_separator.clone();
    let thou_sep = thousands_separator.clone();
    let dec_sep_step = decimal_separator.clone();
    let thou_sep_step = thousands_separator.clone();
    let dec_sep_input = decimal_separator.clone();
    let thou_sep_input = thousands_separator.clone();
    let dec_sep_blur = decimal_separator.clone();
    let thou_sep_blur = thousands_separator.clone();
    let dec_sep_kd = decimal_separator.clone();
    let thou_sep_kd = thousands_separator.clone();

    // --- Display string (persists across renders) ---
    let mut display_str = use_signal(move || {
        value
            .or(default_value)
            .map(|v| do_format(v, decimal_scale, &dec_sep, &thou_sep))
            .unwrap_or_default()
    });

    // Sync controlled value when it changes between renders
    let mut prev_controlled = use_signal(|| value);
    if value != *prev_controlled.peek() {
        prev_controlled.set(value);
        let next_display = value
            .map(|v| do_format(v, decimal_scale, &decimal_separator, &thousands_separator))
            .unwrap_or_default();
        display_str.set(next_display);
    }

    // --- Step helpers ---
    let current_str = display_str.read().clone();
    let current_val = do_parse(&current_str, &decimal_separator, &thousands_separator);
    let increment_disabled = disabled || max.is_some_and(|hi| current_val.is_some_and(|v| v >= hi));
    let decrement_disabled = disabled || min.is_some_and(|lo| current_val.is_some_and(|v| v <= lo));

    let step_fn = {
        let dec_sep = dec_sep_step;
        let thou_sep = thou_sep_step;
        move |delta: f64| {
            let cur = display_str.read().clone();
            let base = do_parse(&cur, &dec_sep, &thou_sep).unwrap_or(0.0);
            let next = do_clamp(base + delta, min, max);
            display_str.set(do_format(next, decimal_scale, &dec_sep, &thou_sep));
            if let Some(cb) = on_change {
                cb.call(Some(next));
            }
        }
    };

    let mut step_fn_up = step_fn.clone();
    let mut step_fn_down = step_fn;

    // --- Left section ---
    let left_section = if prefix.is_empty() && left_section.is_none() {
        None
    } else {
        Some(rsx! {
            if let Some(ls) = left_section {
                {ls}
            }
            if !prefix.is_empty() {
                span {
                    class: Styles::dx_number_input_affix.to_string(),
                    "data-slot": "number-input-prefix",
                    "data-position": "prefix",
                    aria_hidden: "true",
                    "{prefix}"
                }
            }
        })
    };

    // --- Right section (suffix + steppers) ---
    let right_section = if suffix.is_empty() && hide_controls {
        None
    } else {
        Some(rsx! {
            if !suffix.is_empty() {
                span {
                    class: Styles::dx_number_input_affix.to_string(),
                    "data-slot": "number-input-suffix",
                    "data-position": "suffix",
                    aria_hidden: "true",
                    "{suffix}"
                }
            }
            if !hide_controls {
                div {
                    class: Styles::dx_number_input_controls.to_string(),
                    "data-slot": "number-input-controls",
                    button {
                        class: Styles::dx_number_input_step_btn.to_string(),
                        "data-slot": "number-input-increment",
                        r#type: "button",
                        tabindex: "-1",
                        "aria-label": "Increment",
                        disabled: increment_disabled,
                        onclick: move |_| step_fn_up(step),
                        span {
                            class: Styles::dx_number_input_chevron.to_string(),
                            "data-direction": "up",
                            aria_hidden: "true",
                        }
                    }
                    button {
                        class: Styles::dx_number_input_step_btn.to_string(),
                        "data-slot": "number-input-decrement",
                        r#type: "button",
                        tabindex: "-1",
                        "aria-label": "Decrement",
                        disabled: decrement_disabled,
                        onclick: move |_| step_fn_down(-step),
                        span {
                            class: Styles::dx_number_input_chevron.to_string(),
                            "data-direction": "down",
                            aria_hidden: "true",
                        }
                    }
                }
            }
        })
    };

    // --- Native input attributes ---
    let native_base = attributes!(input {
        id: input_id.clone(),
        class: "dx_text_input_control",
        "data-slot": "number-input-control",
        inputmode: if allow_decimal { "decimal" } else { "numeric" },
        disabled: disabled,
        required: required,
        "aria-invalid": invalid,
        "aria-describedby": aria_describedby,
        value: display_str.read().clone(),
    });
    let native_attributes = merge_attributes(vec![native_base, attributes]);

    rsx! {
        InputBase {
            id: input_id,
            label,
            description,
            error: error.clone(),
            required,
            with_asterisk,
            disabled,
            loading,
            described_by: user_described_by,
            variant,
            size,
            radius,
            left_section,
            right_section,
            wrapper_attributes,
            input_attributes,
            input {
                oninput: move |e: FormEvent| {
                    let raw = e.value();
                    let filtered = sanitize(&raw, allow_decimal, allow_negative, decimal_char);
                    let filtered = if clamp_behavior == ClampBehavior::Strict {
                        if let Some(v) = do_parse(&filtered, &dec_sep_input, &thou_sep_input) {
                            let clamped = do_clamp(v, min, max);
                            if (clamped - v).abs() > f64::EPSILON {
                                do_format(clamped, decimal_scale, &dec_sep_input, &thou_sep_input)
                            } else {
                                filtered
                            }
                        } else {
                            filtered
                        }
                    } else {
                        filtered
                    };
                    display_str.set(filtered.clone());
                    let parsed = do_parse(&filtered, &dec_sep_input, &thou_sep_input);
                    if let Some(cb) = on_change {
                        cb.call(parsed);
                    }
                },
                onblur: move |e: FocusEvent| {
                    let cur = display_str.read().clone();
                    if let Some(v) = do_parse(&cur, &dec_sep_blur, &thou_sep_blur) {
                        let v = if clamp_behavior == ClampBehavior::Blur {
                            do_clamp(v, min, max)
                        } else {
                            v
                        };
                        display_str.set(do_format(v, decimal_scale, &dec_sep_blur, &thou_sep_blur));
                        if let Some(cb) = on_change {
                            cb.call(Some(v));
                        }
                    } else if cur.trim() == "-" || cur.trim().is_empty() {
                        display_str.set(String::new());
                        if let Some(cb) = on_change {
                            cb.call(None);
                        }
                    }
                    if let Some(cb) = onblur {
                        cb.call(e);
                    }
                },
                onkeydown: move |e: KeyboardEvent| {
                    match e.key() {
                        Key::ArrowUp => {
                            e.prevent_default();
                            let cur = display_str.read().clone();
                            let base = do_parse(&cur, &dec_sep_kd, &thou_sep_kd).unwrap_or(0.0);
                            let next = do_clamp(base + step, min, max);
                            display_str.set(do_format(next, decimal_scale, &dec_sep_kd, &thou_sep_kd));
                            if let Some(cb) = on_change {
                                cb.call(Some(next));
                            }
                        }
                        Key::ArrowDown => {
                            e.prevent_default();
                            let cur = display_str.read().clone();
                            let base = do_parse(&cur, &dec_sep_kd, &thou_sep_kd).unwrap_or(0.0);
                            let next = do_clamp(base - step, min, max);
                            display_str.set(do_format(next, decimal_scale, &dec_sep_kd, &thou_sep_kd));
                            if let Some(cb) = on_change {
                                cb.call(Some(next));
                            }
                        }
                        _ => {}
                    }
                    if let Some(cb) = onkeydown {
                        cb.call(e);
                    }
                },
                onfocus: move |e| {
                    if let Some(cb) = onfocus { cb.call(e); }
                },
                onmounted: move |e| {
                    if let Some(cb) = onmounted { cb.call(e); }
                },
                ..native_attributes,
            }
        }
    }
}
