use crate::component_styles;
use dioxus::prelude::*;
use dioxus_icons::lucide::{Eye, EyeOff};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

use crate::components::input::{
    attribute_text, merge_described_by, use_input_id, InputBase, InputContent, InputLabel,
    InputRadius, InputSize, InputVariant,
};

#[component_styles("./style.css")]
struct Styles;

/// Password entry input with a visibility toggle, built on top of [`InputBase`].
///
/// The native control renders as `type="password"` and switches to `type="text"`
/// while the value is revealed. Visibility can be left uncontrolled (the bundled
/// eye toggle owns it) or controlled via `visible` + `on_visibility_change`.
#[component]
pub fn PasswordInput(
    /// Label content rendered above the password input.
    #[props(default, into)]
    label: InputLabel,
    /// Description rendered below the label.
    #[props(default, into)]
    description: InputContent,
    /// Error rendered below the input and reflected with `aria-invalid`.
    #[props(default, into)]
    error: InputContent,
    /// Marks the native input as required.
    #[props(default = false)]
    required: bool,
    /// Shows the required asterisk without changing native validation.
    #[props(default = false)]
    with_asterisk: bool,
    /// Shows a loading spinner in the trailing section and marks the field busy.
    /// While loading the spinner replaces the visibility toggle.
    #[props(default = false)]
    loading: bool,
    /// Controlled visibility state. When `Some`, the bundled toggle no longer owns
    /// the state — pair it with `on_visibility_change` to close the loop.
    #[props(default)]
    visible: Option<bool>,
    /// Initial visibility for uncontrolled mode. Defaults to hidden.
    #[props(default = false)]
    default_visible: bool,
    /// Called with the next visibility whenever the toggle is activated.
    #[props(default)]
    on_visibility_change: Option<Callback<bool>>,
    /// Renders the eye toggle button in the trailing section. Defaults to `true`.
    #[props(default = true)]
    visibility_toggle: bool,
    /// Accessible label for the toggle while the value is hidden.
    #[props(default = "Show password".to_string(), into)]
    show_label: String,
    /// Accessible label for the toggle while the value is shown.
    #[props(default = "Hide password".to_string(), into)]
    hide_label: String,
    /// Visual variant for the shell.
    #[props(default)]
    variant: InputVariant,
    /// Size preset for the shell.
    #[props(default)]
    size: InputSize,
    /// Radius preset for the shell.
    #[props(default)]
    radius: InputRadius,
    /// Optional content rendered before the native input.
    #[props(default)]
    left_section: Option<Element>,
    /// Optional content rendered after the native input, before the toggle.
    #[props(default)]
    right_section: Option<Element>,
    /// Existing ids to prepend to generated described-by ids.
    #[props(default)]
    described_by: Option<String>,
    /// Root wrapper attributes.
    #[props(default)]
    wrapper_attributes: Vec<Attribute>,
    /// Shell attributes.
    #[props(default)]
    input_attributes: Vec<Attribute>,
    oninput: Option<EventHandler<FormEvent>>,
    onchange: Option<EventHandler<FormEvent>>,
    oninvalid: Option<EventHandler<FormEvent>>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
    onfocusin: Option<EventHandler<FocusEvent>>,
    onfocusout: Option<EventHandler<FocusEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    onkeypress: Option<EventHandler<KeyboardEvent>>,
    onkeyup: Option<EventHandler<KeyboardEvent>>,
    oncopy: Option<EventHandler<ClipboardEvent>>,
    oncut: Option<EventHandler<ClipboardEvent>>,
    onpaste: Option<EventHandler<ClipboardEvent>>,
    onmounted: Option<EventHandler<MountedEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = input)]
    attributes: Vec<Attribute>,
) -> Element {
    let generated_id = use_input_id("dx-password-input");
    let input_id = attribute_text(&attributes, "id").unwrap_or(generated_id);
    let disabled = attribute_text(&attributes, "disabled").as_deref() == Some("true");
    let user_described_by = attribute_text(&attributes, "aria-describedby")
        .or_else(|| attribute_text(&attributes, "aria_describedby"))
        .or(described_by);
    let description_id = description
        .is_some()
        .then(|| format!("{input_id}-description"));
    let error_id = error.is_some().then(|| format!("{input_id}-error"));
    let aria_describedby = merge_described_by(
        description_id.as_deref(),
        error_id.as_deref(),
        user_described_by.as_deref(),
    );

    // --- Visibility state (controlled or uncontrolled) ---
    let mut visible_state = use_signal(move || visible.unwrap_or(default_visible));
    let mut prev_controlled = use_signal(|| visible);
    if visible != *prev_controlled.peek() {
        prev_controlled.set(visible);
        if let Some(v) = visible {
            visible_state.set(v);
        }
    }
    let is_visible = visible_state();

    let toggle = move |_| {
        let next = !*visible_state.peek();
        if visible.is_none() {
            visible_state.set(next);
        }
        if let Some(cb) = on_visibility_change {
            cb.call(next);
        }
    };

    let toggle_label = if is_visible { hide_label } else { show_label };

    let right_section = if !visibility_toggle && right_section.is_none() {
        None
    } else {
        Some(rsx! {
            if let Some(right_section) = right_section {
                {right_section}
            }
            if visibility_toggle {
                button {
                    class: Styles::dx_password_input_toggle.to_string(),
                    "data-slot": "password-input-toggle",
                    r#type: "button",
                    tabindex: "-1",
                    "aria-label": toggle_label,
                    "aria-pressed": is_visible,
                    "aria-controls": input_id.clone(),
                    disabled,
                    onclick: toggle,
                    if is_visible {
                        EyeOff { "aria-hidden": "true" }
                    } else {
                        Eye { "aria-hidden": "true" }
                    }
                }
            }
        })
    };

    let native_base = attributes!(input {
        id: input_id.clone(),
        class: "dx_text_input_control",
        "data-slot": "password-input-control",
        r#type: if is_visible { "text" } else { "password" },
        required: required,
        "aria-invalid": error.is_some(),
        "aria-describedby": aria_describedby,
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
                oninput: move |e| _ = oninput.map(|callback| callback(e)),
                onchange: move |e| _ = onchange.map(|callback| callback(e)),
                oninvalid: move |e| _ = oninvalid.map(|callback| callback(e)),
                onfocus: move |e| _ = onfocus.map(|callback| callback(e)),
                onblur: move |e| _ = onblur.map(|callback| callback(e)),
                onfocusin: move |e| _ = onfocusin.map(|callback| callback(e)),
                onfocusout: move |e| _ = onfocusout.map(|callback| callback(e)),
                onkeydown: move |e| _ = onkeydown.map(|callback| callback(e)),
                onkeypress: move |e| _ = onkeypress.map(|callback| callback(e)),
                onkeyup: move |e| _ = onkeyup.map(|callback| callback(e)),
                oncopy: move |e| _ = oncopy.map(|callback| callback(e)),
                oncut: move |e| _ = oncut.map(|callback| callback(e)),
                onpaste: move |e| _ = onpaste.map(|callback| callback(e)),
                onmounted: move |e| _ = onmounted.map(|callback| callback(e)),
                ..native_attributes,
            }
        }
    }
}
