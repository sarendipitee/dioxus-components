use std::sync::atomic::{AtomicUsize, Ordering};

use crate::component_styles;
use dioxus::core::AttributeValue;
use dioxus::prelude::*;
use dioxus_icons::lucide::X;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes, TextOrElement};

use crate::components::label::Label;

#[component_styles("./style.css")]
struct Styles;

/// Visual variants supported by the shared input shell.
#[derive(Copy, Clone, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum InputVariant {
    /// Bordered field surface.
    #[default]
    Default,
    /// Subtle filled field surface.
    Filled,
    /// Removes field chrome while preserving slots and state attributes.
    Unstyled,
}

impl InputVariant {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Filled => "filled",
            Self::Unstyled => "unstyled",
        }
    }
}

/// Preset sizes for the shared input shell.
#[derive(Copy, Clone, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum InputSize {
    /// Compact field height.
    Sm,
    /// Default field height.
    #[default]
    Md,
    /// Large field height.
    Lg,
}

impl InputSize {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
        }
    }
}

/// Preset radius values for the shared input shell.
#[derive(Copy, Clone, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum InputRadius {
    /// Small radius.
    Sm,
    /// Default radius.
    #[default]
    Md,
    /// Fully rounded field.
    Pill,
}

impl InputRadius {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Pill => "pill",
        }
    }
}

pub(crate) fn use_input_id(prefix: &'static str) -> String {
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

    use_hook(move || {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        format!("{prefix}-{id}")
    })
}

pub(crate) fn merge_described_by(
    description_id: Option<&str>,
    error_id: Option<&str>,
    described_by: Option<&str>,
) -> Option<String> {
    let mut ids = Vec::new();

    if let Some(id) = described_by.filter(|id| !id.is_empty()) {
        ids.push(id);
    }
    if let Some(id) = description_id {
        ids.push(id);
    }
    if let Some(id) = error_id {
        ids.push(id);
    }

    (!ids.is_empty()).then(|| ids.join(" "))
}

pub(crate) fn attribute_text(attributes: &[Attribute], name: &str) -> Option<String> {
    attributes
        .iter()
        .find(|attr| attr.name == name)
        .and_then(|attr| match &attr.value {
            AttributeValue::Text(value) => Some(value.clone()),
            AttributeValue::Float(value) => Some(value.to_string()),
            AttributeValue::Int(value) => Some(value.to_string()),
            AttributeValue::Bool(value) => Some(value.to_string()),
            _ => None,
        })
}

fn attribute_bool(attributes: &[Attribute], name: &str) -> bool {
    attributes
        .iter()
        .find(|attr| attr.name == name)
        .is_some_and(|attr| match &attr.value {
            AttributeValue::Bool(value) => *value,
            AttributeValue::Text(value) => value != "false",
            _ => false,
        })
}

/// Generated field metadata for custom controls composed inside [`InputBase`].
#[derive(Clone, PartialEq)]
pub struct InputControlContext {
    /// Id that should be applied to the actual interactive control.
    pub id: String,
    /// Merged ids for descriptions and errors associated with the control.
    pub described_by: Option<String>,
    /// Whether the field currently has an error.
    pub invalid: bool,
}

/// Returns the nearest [`InputBase`] control metadata for custom input compositions.
pub fn use_input_control_context() -> Option<InputControlContext> {
    try_use_context::<InputControlContext>()
}

/// Optional field content accepted by input wrapper APIs.
#[derive(Clone, Default, PartialEq)]
pub struct InputContent {
    content: Option<TextOrElement<()>>,
}

impl From<String> for InputContent {
    fn from(value: String) -> Self {
        Self {
            content: Some(TextOrElement::Text(value)),
        }
    }
}

impl From<&str> for InputContent {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl From<Element> for InputContent {
    fn from(value: Element) -> Self {
        Self {
            content: Some(TextOrElement::Element(value)),
        }
    }
}

impl From<Option<Element>> for InputContent {
    fn from(value: Option<Element>) -> Self {
        Self {
            content: value.map(TextOrElement::Element),
        }
    }
}

impl From<Option<String>> for InputContent {
    fn from(value: Option<String>) -> Self {
        Self {
            content: value.map(TextOrElement::Text),
        }
    }
}

impl From<TextOrElement<()>> for InputContent {
    fn from(value: TextOrElement<()>) -> Self {
        Self {
            content: Some(value),
        }
    }
}

impl From<Callback<(), Element>> for InputContent {
    fn from(value: Callback<(), Element>) -> Self {
        Self {
            content: Some(TextOrElement::Render(value)),
        }
    }
}

impl InputContent {
    pub(crate) fn into_element(self) -> Option<Element> {
        self.content.map(TextOrElement::into_element)
    }

    pub(crate) fn is_some(&self) -> bool {
        self.content.is_some()
    }
}

/// Optional label content accepted by [`InputWrapper`] and [`InputBase`].
pub type InputLabel = InputContent;

pub(crate) fn element_label(label: Option<Element>) -> InputLabel {
    InputLabel::from(label)
}

#[derive(Clone, PartialEq)]
pub(crate) struct InputFieldTextState {
    pub id: String,
    pub description_id: Option<String>,
    pub error_id: Option<String>,
    pub described_by: Option<String>,
    pub invalid: bool,
}

pub(crate) fn build_input_field_text_state(
    id: String,
    description: &InputContent,
    error: &InputContent,
    described_by: Option<&str>,
) -> InputFieldTextState {
    let description_id = description.is_some().then(|| format!("{id}-description"));
    let error_id = error.is_some().then(|| format!("{id}-error"));
    let described_by =
        merge_described_by(description_id.as_deref(), error_id.as_deref(), described_by);

    InputFieldTextState {
        id,
        description_id,
        error_id,
        described_by,
        invalid: error.is_some(),
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct InputFieldTextClasses {
    pub label: String,
    pub required: String,
    pub description: String,
    pub error: String,
}

#[derive(Clone, PartialEq)]
pub(crate) struct InputFieldTextSlots {
    pub label: Option<&'static str>,
    pub description: Option<&'static str>,
    pub error: Option<&'static str>,
}

#[component]
pub(crate) fn InputFieldText(
    input_id: String,
    #[props(default, into)] label: InputLabel,
    #[props(default, into)] description: InputContent,
    #[props(default, into)] error: InputContent,
    #[props(default = false)] required: bool,
    #[props(default = false)] with_asterisk: bool,
    classes: InputFieldTextClasses,
    #[props(default = InputFieldTextSlots {
        label: None,
        description: None,
        error: None,
    })]
    slots: InputFieldTextSlots,
) -> Element {
    let label_content = label.into_element();
    let description_id = description
        .is_some()
        .then(|| format!("{input_id}-description"));
    let error_id = error.is_some().then(|| format!("{input_id}-error"));
    let description = description.into_element();
    let error = error.into_element();

    rsx! {
        if let Some(label_content) = label_content {
            Label {
                html_for: input_id.clone(),
                attributes: {
                    let mut attributes = vec![Attribute::new(
                        "class",
                        AttributeValue::Text(classes.label),
                        None,
                        false,
                    )];
                    if let Some(slot) = slots.label {
                        attributes.push(Attribute::new(
                            "data-slot",
                            AttributeValue::Text(slot.to_string()),
                            None,
                            false,
                        ));
                    }
                    attributes
                },
                {label_content}
                if required || with_asterisk {
                    span {
                        class: classes.required,
                        "aria-hidden": "true",
                        " *"
                    }
                }
            }
        }
        if let Some((description, description_id)) = description.zip(description_id) {
            div {
                id: "{description_id}",
                class: classes.description,
                "data-slot": slots.description,
                {description}
            }
        }
        if let Some((error, error_id)) = error.zip(error_id) {
            div {
                id: "{error_id}",
                class: classes.error,
                "data-slot": slots.error,
                {error}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct InputWrapperProps {
    /// Optional id for the wrapped control.
    #[props(into)]
    id: Option<String>,
    /// Label content rendered above the input slot and associated with the wrapped control.
    #[props(default, into)]
    label: InputLabel,
    /// Description rendered below the label and included in `aria-describedby`.
    #[props(default, into)]
    description: InputContent,
    /// Error rendered below the input and included in `aria-describedby`.
    #[props(default, into)]
    error: InputContent,
    /// Marks the field as required.
    #[props(default = false)]
    required: bool,
    /// Shows the required asterisk without changing native validation.
    #[props(default = false)]
    with_asterisk: bool,
    /// Marks the wrapper and label as disabled.
    #[props(default = false)]
    disabled: bool,
    /// Existing ids to prepend when a composed control builds `aria-describedby`.
    #[props(default)]
    described_by: Option<String>,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

/// Field-level input wrapper with label, description, error, and required state.
#[component]
pub fn InputWrapper(props: InputWrapperProps) -> Element {
    let InputWrapperProps {
        id,
        label,
        description,
        error,
        required,
        with_asterisk,
        disabled,
        described_by,
        attributes,
        children,
    } = props;
    let generated_id = use_input_id("dx-input");
    let field = build_input_field_text_state(
        id.unwrap_or(generated_id),
        &description,
        &error,
        described_by.as_deref(),
    );
    let control_context = InputControlContext {
        id: field.id.clone(),
        described_by: field.described_by.clone(),
        invalid: field.invalid,
    };
    use_context_provider(|| control_context);

    let base = attributes!(div {
        class: Styles::dx_input_wrapper.to_string(),
        "data-slot": "input-wrapper",
        "data-disabled": disabled,
        "data-error": field.invalid,
        "data-required": required || with_asterisk,
        "data-input-id": field.id.clone(),
        "data-input-describedby": field.described_by.clone().unwrap_or_default(),
    });
    let attributes = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..attributes,
            InputFieldText {
                input_id: field.id,
                label,
                description,
                error,
                required,
                with_asterisk,
                classes: InputFieldTextClasses {
                    label: Styles::dx_input_label.to_string(),
                    required: Styles::dx_input_required.to_string(),
                    description: Styles::dx_input_description.to_string(),
                    error: Styles::dx_input_error.to_string(),
                },
                slots: InputFieldTextSlots {
                    label: Some("input-label"),
                    description: Some("input-description"),
                    error: Some("input-error"),
                },
            }
            {children}
        }
    }
}

/// Shared visual input shell for text fields, triggers, segmented fields, and picker inputs.
#[component]
pub fn Input(
    /// Visual variant for the shell.
    #[props(default)]
    variant: InputVariant,
    /// Size preset for shell height and spacing.
    #[props(default)]
    size: InputSize,
    /// Radius preset for the shell.
    #[props(default)]
    radius: InputRadius,
    /// Marks the shell disabled.
    #[props(default = false)]
    disabled: bool,
    /// Marks the shell invalid.
    #[props(default = false)]
    error: bool,
    /// Shows a loading spinner in the trailing section and marks the shell busy.
    ///
    /// While loading, the spinner replaces any `right_section` content. The control
    /// stays interactive unless `disabled` is also set, so async validation can run
    /// without trapping focus.
    #[props(default = false)]
    loading: bool,
    /// Optional content rendered before the control slot.
    #[props(default)]
    left_section: Option<Element>,
    /// Optional content rendered after the control slot.
    #[props(default)]
    right_section: Option<Element>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_input.to_string(),
        "data-slot": "input",
        "data-variant": variant.as_str(),
        "data-size": size.as_str(),
        "data-radius": radius.as_str(),
        "data-disabled": disabled,
        "data-error": error,
        "data-loading": loading,
        "aria-busy": loading,
    });
    let attributes = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..attributes,
            if let Some(left_section) = left_section {
                div {
                    class: Styles::dx_input_section.to_string(),
                    "data-slot": "input-left-section",
                    "data-position": "left",
                    {left_section}
                }
            }
            div {
                class: Styles::dx_input_control.to_string(),
                "data-slot": "input-control",
                {children}
            }
            if loading {
                div {
                    class: Styles::dx_input_section.to_string(),
                    "data-slot": "input-loading-section",
                    "data-position": "right",
                    span {
                        class: Styles::dx_input_spinner.to_string(),
                        "data-slot": "input-spinner",
                        "aria-hidden": "true",
                    }
                }
            } else if let Some(right_section) = right_section {
                div {
                    class: Styles::dx_input_section.to_string(),
                    "data-slot": "input-right-section",
                    "data-position": "right",
                    {right_section}
                }
            }
        }
    }
}

/// Reusable clear button for input right sections.
#[component]
pub fn InputClearButton(
    /// Accessible label for the clear action.
    #[props(default = ReadSignal::new(Signal::new(String::from("Clear value"))))]
    aria_label: ReadSignal<String>,
    /// Disables the clear button.
    #[props(default = false)]
    disabled: bool,
    /// Clear button click handler.
    onclick: Option<EventHandler<MouseEvent>>,
    /// Optional custom icon content.
    #[props(default)]
    icon: Option<Element>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    attributes: Vec<Attribute>,
) -> Element {
    let base = attributes!(button {
        class: Styles::dx_input_clear_button.to_string(),
        "data-slot": "input-clear-button",
        r#type: "button",
        "aria-label": aria_label,
        disabled: disabled,
    });
    let attributes = merge_attributes(vec![base, attributes]);

    rsx! {
        button {
            onclick: move |event| _ = onclick.map(|callback| callback(event)),
            ..attributes,
            if let Some(icon) = icon {
                {icon}
            } else {
                X { "aria-hidden": "true" }
            }
        }
    }
}

/// Composition helper that renders [`InputWrapper`] around [`Input`].
#[component]
pub fn InputBase(
    /// Optional id shared by the field wrapper and the composed control.
    #[props(default)]
    id: Option<String>,
    /// Label content rendered by the wrapper.
    #[props(default, into)]
    label: InputLabel,
    /// Description rendered by the wrapper.
    #[props(default, into)]
    description: InputContent,
    /// Error rendered by the wrapper and reflected on the input shell.
    #[props(default, into)]
    error: InputContent,
    /// Marks the field as required.
    #[props(default = false)]
    required: bool,
    /// Shows the required asterisk without changing native validation.
    #[props(default = false)]
    with_asterisk: bool,
    /// Marks wrapper and shell disabled.
    #[props(default = false)]
    disabled: bool,
    /// Shows a loading spinner in the shell's trailing section and marks it busy.
    #[props(default = false)]
    loading: bool,
    /// Existing ids to prepend to generated described-by ids.
    #[props(default)]
    described_by: Option<String>,
    /// Visual variant for the shell.
    #[props(default)]
    variant: InputVariant,
    /// Size preset for the shell.
    #[props(default)]
    size: InputSize,
    /// Radius preset for the shell.
    #[props(default)]
    radius: InputRadius,
    /// Optional content rendered before the control slot.
    #[props(default)]
    left_section: Option<Element>,
    /// Optional content rendered after the control slot.
    #[props(default)]
    right_section: Option<Element>,
    #[props(default)] wrapper_attributes: Vec<Attribute>,
    #[props(default)] input_attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    // Id resolution, field text state, and `InputControlContext` are owned by
    // `InputWrapper`. Since `InputBase` renders nothing but `InputWrapper`, every
    // descendant resolves the context from that nearer provider, so duplicating the
    // computation here would be dead work shadowed by the wrapper.
    rsx! {
        InputWrapper {
            id,
            label,
            description,
            error: error.clone(),
            required,
            with_asterisk,
            disabled,
            described_by,
            attributes: wrapper_attributes,
            Input {
                variant,
                size,
                radius,
                disabled,
                error: error.is_some(),
                loading,
                left_section,
                right_section,
                attributes: input_attributes,
                {children}
            }
        }
    }
}

/// Native text-entry input adapter built on top of [`InputBase`].
#[component]
pub fn TextInput(
    /// Label content rendered above the text input.
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
    onwheel: Option<EventHandler<WheelEvent>>,
    oncompositionstart: Option<EventHandler<CompositionEvent>>,
    oncompositionupdate: Option<EventHandler<CompositionEvent>>,
    oncompositionend: Option<EventHandler<CompositionEvent>>,
    oncopy: Option<EventHandler<ClipboardEvent>>,
    oncut: Option<EventHandler<ClipboardEvent>>,
    onpaste: Option<EventHandler<ClipboardEvent>>,
    onmounted: Option<EventHandler<MountedEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = input)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let generated_id = use_input_id("dx-text-input");
    let input_id = attribute_text(&attributes, "id").unwrap_or(generated_id);
    let disabled = attribute_bool(&attributes, "disabled");
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

    let native_base = attributes!(input {
        id: input_id.clone(),
        class: Styles::dx_text_input_control.to_string(),
        "data-slot": "text-input-control",
        disabled: disabled,
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
                onselect: move |e| _ = onselect.map(|callback| callback(e)),
                onselectionchange: move |e| _ = onselectionchange.map(|callback| callback(e)),
                onfocus: move |e| _ = onfocus.map(|callback| callback(e)),
                onblur: move |e| _ = onblur.map(|callback| callback(e)),
                onfocusin: move |e| _ = onfocusin.map(|callback| callback(e)),
                onfocusout: move |e| _ = onfocusout.map(|callback| callback(e)),
                onkeydown: move |e| _ = onkeydown.map(|callback| callback(e)),
                onkeypress: move |e| _ = onkeypress.map(|callback| callback(e)),
                onkeyup: move |e| _ = onkeyup.map(|callback| callback(e)),
                onwheel: move |e| _ = onwheel.map(|callback| callback(e)),
                oncompositionstart: move |e| _ = oncompositionstart.map(|callback| callback(e)),
                oncompositionupdate: move |e| _ = oncompositionupdate.map(|callback| callback(e)),
                oncompositionend: move |e| _ = oncompositionend.map(|callback| callback(e)),
                oncopy: move |e| _ = oncopy.map(|callback| callback(e)),
                oncut: move |e| _ = oncut.map(|callback| callback(e)),
                onpaste: move |e| _ = onpaste.map(|callback| callback(e)),
                onmounted: move |e| _ = onmounted.map(|callback| callback(e)),
                ..native_attributes,
                {children}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[component]
    fn InputWrapperLiteralLabelHarness() -> Element {
        rsx! {
            InputWrapper {
                id: "name",
                label: "Name",
                input { id: "name" }
            }
        }
    }

    #[test]
    fn input_wrapper_accepts_string_literal_label() {
        let mut dom = VirtualDom::new(InputWrapperLiteralLabelHarness);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains(">Name<"));
        assert!(html.contains("for=\"name\""));
    }

    #[component]
    fn InputWrapperOwnedLabelHarness() -> Element {
        let label = String::from("Email");

        rsx! {
            InputWrapper {
                label,
                input {}
            }
        }
    }

    #[test]
    fn input_wrapper_generates_id_for_owned_label() {
        let mut dom = VirtualDom::new(InputWrapperOwnedLabelHarness);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains(">Email<"));
        assert!(html.contains("for=\"dx-input-"));
        assert!(html.contains("data-input-id=\"dx-input-"));
    }

    #[component]
    fn CustomControl() -> Element {
        let control = use_input_control_context().expect("InputBase provides control context");

        rsx! {
            input {
                id: control.id,
                "aria-describedby": control.described_by,
                "aria-invalid": control.invalid,
            }
        }
    }

    #[component]
    fn InputBaseControlContextHarness() -> Element {
        rsx! {
            InputBase {
                id: "env",
                label: "Environment",
                description: "Description copy.",
                error: "Error copy.",
                CustomControl {}
            }
        }
    }

    #[test]
    fn input_base_control_reads_wrapper_context() {
        let mut dom = VirtualDom::new(InputBaseControlContextHarness);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        // The control rendered as an `InputBase` child resolves the context provided
        // by `InputWrapper`, and its ids line up with the field text `InputWrapper` renders.
        assert!(html.contains("data-input-id=\"env\""));
        assert!(html.contains("<input id=\"env\""));
        assert!(html.contains("aria-describedby=\"env-description env-error\""));
        assert!(html.contains("id=\"env-description\""));
        assert!(html.contains("id=\"env-error\""));
        assert!(html.contains("aria-invalid=true"));
    }
}
