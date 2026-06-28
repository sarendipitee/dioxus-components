use dioxus::prelude::*;
use dioxus_primitives::mask::{use_mask, MaskPattern, UseMaskOptions};
pub use dioxus_primitives::mask::{CharPredicate, MaskItem, MaskModify, UseMask};

use crate::components::input::{
    attribute_text, merge_described_by, use_input_id, InputBase, InputContent, InputLabel,
    InputRadius, InputSize, InputVariant,
};

/// Masked text input built on the shared [`InputBase`] shell and the headless
/// [`use_mask`] hook. Formats the value as the user types, supports custom
/// tokens, undo/redo, and reports both the masked and raw values.
///
/// Mirrors the feature set of Mantine's `MaskInput`.
#[component]
pub fn MaskInput(
    /// Mask pattern string (e.g. `"(999) 999-9999"`) or array of literals and
    /// token predicates.
    #[props(into)]
    mask: MaskPattern,
    /// Extra tokens layered over (and overriding) the default token map.
    #[props(default)]
    tokens: Vec<(char, CharPredicate)>,
    /// Called before masking on each keystroke; may override mask options for
    /// that keystroke based on the current raw value.
    #[props(default)]
    modify: Option<Callback<String, Option<MaskModify>>>,
    /// Decouples raw and display values (parity with Mantine; currently unused).
    #[props(default = false)]
    separate: bool,
    /// Character shown in unfilled slots. Empty disables placeholders. `"_"` by
    /// default.
    #[props(default = "_".to_string(), into)]
    slot_char: String,
    /// Show the mask pattern even when empty and unfocused.
    #[props(default = false)]
    always_show_mask: bool,
    /// Show the mask placeholder on focus. `true` by default.
    #[props(default = true)]
    show_mask_on_focus: bool,
    /// Transform each character before validation and insertion.
    #[props(default)]
    transform: Option<Callback<char, char>>,
    /// Clear the value on blur when the mask is incomplete.
    #[props(default = false)]
    auto_clear: bool,
    /// Initial value seeded into the input on mount.
    #[props(default, into)]
    default_value: String,
    /// Called on every change with `(raw_value, masked_value)`.
    #[props(default)]
    on_change_raw: Option<Callback<(String, String)>>,
    /// Called when all required mask slots are filled with `(masked, raw)`.
    #[props(default)]
    on_complete: Option<Callback<(String, String)>>,
    /// Receives the [`UseMask`] handle once created, e.g. to call `reset`.
    #[props(default)]
    mask_ref: Option<Callback<UseMask>>,

    /// Label rendered above the input.
    #[props(default, into)]
    label: InputLabel,
    /// Description rendered below the label.
    #[props(default, into)]
    description: InputContent,
    /// Error rendered below the input and reflected with `aria-invalid`.
    #[props(default, into)]
    error: InputContent,
    /// Marks the field as required.
    #[props(default = false)]
    required: bool,
    /// Shows the required asterisk without changing native validation.
    #[props(default = false)]
    with_asterisk: bool,
    /// Marks the field disabled.
    #[props(default = false)]
    disabled: bool,
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
    /// Optional content rendered before the native input.
    #[props(default)]
    left_section: Option<Element>,
    /// Optional content rendered after the native input.
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

    onfocus: Option<EventHandler<FocusEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    onmounted: Option<EventHandler<MountedEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = input)]
    attributes: Vec<Attribute>,
) -> Element {
    let generated_id = use_input_id("dx-mask-input");
    let input_id = attribute_text(&attributes, "id").unwrap_or(generated_id);
    let invalid = error.is_some();

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

    let mask = use_mask(UseMaskOptions {
        mask,
        tokens,
        modify,
        separate,
        slot_char,
        always_show_mask,
        show_mask_on_focus,
        transform,
        auto_clear,
        invalid,
        on_change_raw,
        on_complete,
        initial_value: default_value,
    });

    use_hook(move || {
        if let Some(cb) = mask_ref {
            cb.call(mask);
        }
    });

    // On the web target the input is uncontrolled — the hook writes its value
    // imperatively to keep the caret stable. On other renderers, bind the
    // computed masked value so it still renders.
    #[cfg(all(feature = "web", target_arch = "wasm32"))]
    let value_attr: Option<String> = None;
    #[cfg(not(all(feature = "web", target_arch = "wasm32")))]
    let value_attr: Option<String> = Some(mask.value.cloned());

    rsx! {
        InputBase {
            id: input_id.clone(),
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
                id: input_id,
                class: "dx_text_input_control",
                "data-slot": "mask-input-control",
                "data-dx-mask-id": "{mask.mask_id}",
                disabled,
                required,
                "aria-invalid": invalid,
                "aria-describedby": aria_describedby,
                value: value_attr,
                onmounted: move |e: MountedEvent| {
                    mask.onmounted.call(e.clone());
                    if let Some(cb) = onmounted {
                        cb.call(e);
                    }
                },
                oninput: move |e| mask.oninput.call(e),
                onkeydown: move |e: KeyboardEvent| {
                    mask.onkeydown.call(e.clone());
                    if let Some(cb) = onkeydown {
                        cb.call(e);
                    }
                },
                onfocus: move |e: FocusEvent| {
                    mask.onfocus.call(e.clone());
                    if let Some(cb) = onfocus {
                        cb.call(e);
                    }
                },
                onblur: move |e: FocusEvent| {
                    mask.onblur.call(e.clone());
                    if let Some(cb) = onblur {
                        cb.call(e);
                    }
                },
                onmousedown: move |e| mask.onmousedown.call(e),
                onmouseup: move |e| mask.onmouseup.call(e),
                ..attributes,
            }
        }
    }
}
