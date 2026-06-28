use crate::component_styles;
use dioxus::prelude::*;
use dioxus_primitives::switch::{self};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

use crate::components::input::{
    attribute_text, build_input_field_text_state, use_input_id, InputContent, InputFieldText,
    InputFieldTextClasses, InputLabel,
};

#[component_styles("./style.css")]
struct Styles;

#[component]
pub fn Switch(
    #[props(default)] checked: ReadSignal<Option<bool>>,
    #[props(default = false)] default_checked: bool,
    #[props(default)] disabled: ReadSignal<bool>,
    #[props(default)] required: ReadSignal<bool>,
    #[props(default)] name: ReadSignal<String>,
    #[props(default = ReadSignal::new(Signal::new(String::from("on"))))] value: ReadSignal<String>,
    #[props(default)] on_checked_change: Callback<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    /// Label rendered inline beside the switch.
    #[props(default, into)]
    label: InputLabel,
    /// Description rendered below the label.
    #[props(default, into)]
    description: InputContent,
    /// Error rendered below the switch.
    #[props(default, into)]
    error: InputContent,
    /// Shows the required asterisk without changing native validation.
    #[props(default = false)]
    with_asterisk: bool,
) -> Element {
    let generated_id = use_input_id("dx-switch");
    let field = build_input_field_text_state(
        attribute_text(&attributes, "id").unwrap_or(generated_id),
        &description,
        &error,
        attribute_text(&attributes, "aria-describedby")
            .or_else(|| attribute_text(&attributes, "aria_describedby"))
            .as_deref(),
    );
    let is_required = (required)();
    let is_disabled = (disabled)();

    let switch_attributes = merge_attributes(vec![
        attributes!(button {
            id: field.id.clone(),
            class: Styles::dx_switch.to_string(),
            "aria-invalid": field.invalid,
            "aria-describedby": field.described_by.clone(),
        }),
        attributes,
    ]);

    rsx! {
        div {
            class: Styles::dx_switch_field.to_string(),
            "data-disabled": is_disabled,
            "data-error": field.invalid,
            "data-required": is_required,
            switch::Switch {
                checked,
                default_checked,
                disabled,
                required,
                name,
                value,
                on_checked_change,
                attributes: switch_attributes,
                switch::SwitchThumb { class: Styles::dx_switch_thumb.to_string() }
            }
            div { class: Styles::dx_switch_label_container.to_string(),
                InputFieldText {
                    input_id: field.id,
                    label,
                    description,
                    error,
                    required: is_required,
                    with_asterisk,
                    classes: InputFieldTextClasses {
                        label: Styles::dx_switch_label.to_string(),
                        required: Styles::dx_switch_required.to_string(),
                        description: Styles::dx_switch_description.to_string(),
                        error: Styles::dx_switch_error.to_string(),
                    },
                }
            }
        }
    }
}
