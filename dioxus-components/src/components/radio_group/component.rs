use crate::component_styles;
use dioxus::prelude::*;
use dioxus_primitives::radio_group::{self, RadioItemProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

use crate::components::input::{
    attribute_text, use_input_control_context, InputContent, InputLabel, InputWrapper,
};

#[component_styles("./style.css")]
struct Styles;

#[component]
pub fn RadioGroup(
    #[props(default)] value: ReadSignal<Option<String>>,
    #[props(default)] default_value: String,
    #[props(default)] on_value_change: Callback<String>,
    #[props(default)] disabled: ReadSignal<bool>,
    #[props(default)] required: ReadSignal<bool>,
    #[props(default)] name: ReadSignal<String>,
    #[props(default)] horizontal: ReadSignal<bool>,
    #[props(default = ReadSignal::new(Signal::new(true)))] roving_loop: ReadSignal<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    /// Label rendered above the radio group.
    #[props(default, into)]
    label: InputLabel,
    /// Description rendered below the label.
    #[props(default, into)]
    description: InputContent,
    /// Error rendered below the group.
    #[props(default, into)]
    error: InputContent,
    /// Shows the required asterisk without changing native validation.
    #[props(default = false)]
    with_asterisk: bool,
) -> Element {
    let is_disabled = (disabled)();
    let is_required = (required)();
    let id = attribute_text(&attributes, "id");
    let described_by = attribute_text(&attributes, "aria-describedby")
        .or_else(|| attribute_text(&attributes, "aria_describedby"));
    rsx! {
        InputWrapper {
            id,
            described_by,
            label,
            description,
            error,
            required: is_required,
            with_asterisk,
            disabled: is_disabled,
            RadioGroupControl {
                value,
                default_value,
                on_value_change,
                disabled,
                required,
                name,
                horizontal,
                roving_loop,
                attributes,
                {children}
            }
        }
    }
}

#[component]
fn RadioGroupControl(
    value: ReadSignal<Option<String>>,
    default_value: String,
    on_value_change: Callback<String>,
    disabled: ReadSignal<bool>,
    required: ReadSignal<bool>,
    name: ReadSignal<String>,
    horizontal: ReadSignal<bool>,
    roving_loop: ReadSignal<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let control_attrs = use_input_control_context().map(|ctx| {
        attributes!(div {
            id: ctx.id,
            "aria-describedby": ctx.described_by,
            "aria-invalid": ctx.invalid,
        })
    });
    let base = attributes!(div {
        class: Styles::dx_radio_group,
    });
    let attributes = match control_attrs {
        Some(control_attrs) => merge_attributes(vec![base, attributes, control_attrs]),
        None => merge_attributes(vec![base, attributes]),
    };

    rsx! {
        radio_group::RadioGroup {
            value,
            default_value,
            on_value_change,
            disabled,
            required,
            name,
            horizontal,
            roving_loop,
            attributes,
            {children}
        }
    }
}

#[component]
pub fn RadioItem(props: RadioItemProps) -> Element {
    let base = attributes!(button {
        class: Styles::dx_radio_item,
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        radio_group::RadioItem {
            value: props.value,
            index: props.index,
            disabled: props.disabled,
            id: props.id,
            attributes,
            {props.children}
        }
    }
}
