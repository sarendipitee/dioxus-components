use dioxus::prelude::*;
use dioxus_icons::lucide::{Check, Minus};
use dioxus_primitives::checkbox::{self, CheckboxState};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

use crate::components::input::{
    attribute_text, build_input_field_text_state, use_input_id, InputContent, InputFieldText,
    InputFieldTextClasses, InputLabel,
};

#[css_module("/src/components/checkbox/style.css")]
struct Styles;

/// Styled checkbox wrapper with optional field text content.
#[derive(Props, Clone, PartialEq)]
pub struct StyledCheckboxProps {
    /// Label content rendered by the shared field wrapper.
    #[props(default, into)]
    pub label: InputLabel,
    /// Description rendered below the label and included in `aria-describedby`.
    #[props(default, into)]
    pub description: InputContent,
    /// Error rendered below the checkbox and included in `aria-describedby`.
    #[props(default, into)]
    pub error: InputContent,
    /// The controlled state of the checkbox.
    pub checked: ReadSignal<Option<CheckboxState>>,
    /// The default state of the checkbox when it is not controlled.
    #[props(default = CheckboxState::Unchecked)]
    pub default_checked: CheckboxState,
    /// Whether the checkbox is required in a form.
    #[props(default)]
    pub required: ReadSignal<bool>,
    /// Whether the checkbox is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,
    /// The name of the checkbox, used in forms.
    #[props(default)]
    pub name: ReadSignal<String>,
    /// The value of the checkbox, which can be used in forms.
    #[props(default = ReadSignal::new(Signal::new(String::from("on"))))]
    pub value: ReadSignal<String>,
    /// Callback that is called when the checked state changes.
    #[props(default)]
    pub on_checked_change: Callback<CheckboxState>,
    /// Additional attributes to apply to the checkbox button.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// Optional custom indicator content rendered inside the checkbox button.
    #[props(default)]
    pub children: Option<Element>,
}

/// Styled checkbox wrapper with optional field text content.
#[component]
pub fn Checkbox(props: StyledCheckboxProps) -> Element {
    let generated_id = use_input_id("dx-checkbox");
    let field = build_input_field_text_state(
        attribute_text(&props.attributes, "id").unwrap_or(generated_id),
        &props.description,
        &props.error,
        attribute_text(&props.attributes, "aria-describedby")
            .or_else(|| attribute_text(&props.attributes, "aria_describedby"))
            .as_deref(),
    );
    let required = (props.required)();
    let disabled = (props.disabled)();
    let checkbox_attributes = merge_attributes(vec![
        attributes!(button {
            id: field.id.clone(),
            class: Styles::dx_checkbox.to_string(),
            "aria-invalid": field.invalid,
            "aria-describedby": field.described_by.clone(),
        }),
        props.attributes,
    ]);

    rsx! {
        div {
            class: Styles::dx_checkbox_field.to_string(),
            "data-disabled": disabled,
            "data-error": field.invalid,
            "data-required": required,
            checkbox::Checkbox {
                checked: props.checked,
                default_checked: props.default_checked,
                required: props.required,
                disabled: props.disabled,
                name: props.name,
                value: props.value,
                on_checked_change: props.on_checked_change,
                attributes: checkbox_attributes,
                match props.children {
                    Some(children) => rsx! {
                        checkbox::CheckboxIndicator { class: Styles::dx_checkbox_indicator.to_string(),
                            {children}
                        }
                    },
                    None => rsx! {
                        checkbox::CheckboxIndicator {
                            state: CheckboxState::Checked,
                            class: Styles::dx_checkbox_indicator.to_string(),
                            Check { size: "1rem" }
                        }
                        checkbox::CheckboxIndicator {
                            state: CheckboxState::Indeterminate,
                            class: Styles::dx_checkbox_indicator.to_string(),
                            Minus { size: "1rem" }
                        }
                    },
                }
            }
            div { class: Styles::dx_checkbox_label_container.to_string(),
                InputFieldText {
                    input_id: field.id,
                    label: props.label,
                    description: props.description,
                    error: props.error,
                    required,
                    classes: InputFieldTextClasses {
                        label: Styles::dx_checkbox_label.to_string(),
                        required: Styles::dx_checkbox_required.to_string(),
                        description: Styles::dx_checkbox_description.to_string(),
                        error: Styles::dx_checkbox_error.to_string(),
                    },
                }
            }
        }
    }
}
