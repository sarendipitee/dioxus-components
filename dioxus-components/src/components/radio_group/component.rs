use crate::component_styles;
use dioxus::prelude::*;
use dioxus_primitives::radio_group::{self, RadioGroupProps, RadioItemProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[component_styles("./style.css")]
struct Styles;

#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element {
    rsx! {
        radio_group::RadioGroup {
            class: Styles::dx_radio_group.to_string(),
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            required: props.required,
            name: props.name,
            horizontal: props.horizontal,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
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
            attributes,
            {props.children}
        }
    }
}
