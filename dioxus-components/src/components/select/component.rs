use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use crate::component_styles;
use dioxus::prelude::*;
use dioxus_icons::lucide::{Check, ChevronDown};
use dioxus_primitives::select::{self, SelectGroupLabelProps, SelectOptionProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

use crate::input::{Input, InputContent, InputLabel, InputRadius, InputSize, InputVariant, InputWrapper};

pub use dioxus_primitives::select::SelectGroup;

#[component_styles("./style.css")]
struct Styles;

fn use_select_input_id() -> String {
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

    use_hook(|| {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        format!("dx-select-input-{id}")
    })
}

fn described_by(
    id: &str,
    description: bool,
    error: bool,
    described_by: Option<&str>,
) -> Option<String> {
    let mut ids = Vec::new();

    if let Some(id) = described_by.filter(|id| !id.is_empty()) {
        ids.push(id.to_string());
    }
    if description {
        ids.push(format!("{id}-description"));
    }
    if error {
        ids.push(format!("{id}-error"));
    }

    (!ids.is_empty()).then(|| ids.join(" "))
}

/// Props for the styled [`Select`] component.
#[derive(Props, Clone, PartialEq)]
pub struct SelectProps<T: Clone + PartialEq + 'static = String> {
    /// Id shared by the label and trigger.
    #[props(default)]
    pub id: Option<String>,
    /// The controlled selected value.
    #[props(default)]
    pub value: Option<ReadSignal<Option<T>>>,
    /// The initial selected value when uncontrolled.
    #[props(default)]
    pub default_value: Option<T>,
    /// Callback fired when the selected value changes.
    #[props(default)]
    pub on_value_change: Callback<Option<T>>,
    /// Whether the select is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,
    /// The controlled open state.
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,
    /// The initial open state when uncontrolled.
    #[props(default)]
    pub default_open: ReadSignal<bool>,
    /// Callback fired when open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,
    /// Name used for form submission.
    #[props(default)]
    pub name: ReadSignal<String>,
    /// Whether focus wraps when reaching the end of the list.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,
    /// Timeout before clearing the typeahead buffer.
    #[props(default = ReadSignal::new(Signal::new(Duration::from_millis(1000))))]
    pub typeahead_timeout: ReadSignal<Duration>,
    /// Label rendered by the shared input wrapper.
    #[props(default, into)]
    pub label: InputLabel,
    /// Description rendered by the shared input wrapper.
    #[props(default, into)]
    pub description: InputContent,
    /// Error rendered by the shared input wrapper and reflected on the shell.
    #[props(default, into)]
    pub error: InputContent,
    /// Marks the field as required.
    #[props(default = false)]
    pub required: bool,
    /// Shows the required asterisk without changing form validation.
    #[props(default = false)]
    pub with_asterisk: bool,
    /// Shows a loading spinner in the trailing section and marks the field busy.
    #[props(default = false)]
    pub loading: bool,
    /// Visual variant for the shared input shell.
    #[props(default)]
    pub variant: InputVariant,
    /// Size preset for the shared input shell.
    #[props(default)]
    pub size: InputSize,
    /// Radius preset for the shared input shell.
    #[props(default)]
    pub radius: InputRadius,
    /// Optional content rendered before the selected value.
    #[props(default)]
    pub left_section: Option<Element>,
    /// Optional content rendered after the selected value.
    #[props(default)]
    pub right_section: Option<Element>,
    /// Existing ids to prepend to generated description and error ids.
    #[props(default)]
    pub described_by: Option<String>,
    /// Placeholder rendered when the selected option text has not been resolved.
    #[props(default = ReadSignal::new(Signal::new(String::from("Select an option"))))]
    pub placeholder: ReadSignal<String>,
    /// Additional attributes for the select root.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// Option children.
    pub children: Element,
}

/// Props for the styled [`SelectMulti`] component.
#[derive(Props, Clone, PartialEq)]
pub struct SelectMultiProps<T: Clone + PartialEq + 'static = String> {
    /// Id shared by the label and trigger.
    #[props(default)]
    pub id: Option<String>,
    /// The controlled selected values.
    #[props(default)]
    pub values: ReadSignal<Option<Vec<T>>>,
    /// The initial selected values when uncontrolled.
    #[props(default)]
    pub default_values: Vec<T>,
    /// Callback fired when selected values change.
    #[props(default)]
    pub on_values_change: Callback<Vec<T>>,
    /// Whether the select is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,
    /// The controlled open state.
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,
    /// The initial open state when uncontrolled.
    #[props(default)]
    pub default_open: ReadSignal<bool>,
    /// Callback fired when open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,
    /// Name used for form submission.
    #[props(default)]
    pub name: ReadSignal<String>,
    /// Whether focus wraps when reaching the end of the list.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,
    /// Timeout before clearing the typeahead buffer.
    #[props(default = ReadSignal::new(Signal::new(Duration::from_millis(1000))))]
    pub typeahead_timeout: ReadSignal<Duration>,
    /// Label rendered by the shared input wrapper.
    #[props(default, into)]
    pub label: InputLabel,
    /// Description rendered by the shared input wrapper.
    #[props(default, into)]
    pub description: InputContent,
    /// Error rendered by the shared input wrapper and reflected on the shell.
    #[props(default, into)]
    pub error: InputContent,
    /// Marks the field as required.
    #[props(default = false)]
    pub required: bool,
    /// Shows the required asterisk without changing form validation.
    #[props(default = false)]
    pub with_asterisk: bool,
    /// Shows a loading spinner in the trailing section and marks the field busy.
    #[props(default = false)]
    pub loading: bool,
    /// Visual variant for the shared input shell.
    #[props(default)]
    pub variant: InputVariant,
    /// Size preset for the shared input shell.
    #[props(default)]
    pub size: InputSize,
    /// Radius preset for the shared input shell.
    #[props(default)]
    pub radius: InputRadius,
    /// Optional content rendered before the selected value.
    #[props(default)]
    pub left_section: Option<Element>,
    /// Optional content rendered after the selected value.
    #[props(default)]
    pub right_section: Option<Element>,
    /// Existing ids to prepend to generated description and error ids.
    #[props(default)]
    pub described_by: Option<String>,
    /// Additional attributes for the select root.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// Option children.
    pub children: Element,
}

#[component]
pub fn Select<T: Clone + PartialEq + 'static>(props: SelectProps<T>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_select.to_string()
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    let input_id = props.id.unwrap_or_else(use_select_input_id);
    let aria_describedby = described_by(
        &input_id,
        props.description.is_some(),
        props.error.is_some(),
        props.described_by.as_deref(),
    );

    rsx! {
        select::Select {
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            name: props.name,
            roving_loop: props.roving_loop,
            typeahead_timeout: props.typeahead_timeout,
            attributes: merged,
            InputWrapper {
                id: input_id.clone(),
                label: props.label,
                description: props.description,
                error: props.error.clone(),
                required: props.required,
                with_asterisk: props.with_asterisk,
                disabled: (props.disabled)(),
                described_by: props.described_by,
                select::SelectTrigger {
                    id: input_id,
                    class: Styles::dx_select_trigger.to_string(),
                    "aria-describedby": aria_describedby,
                    "aria-invalid": props.error.is_some(),
                    Input {
                        variant: props.variant,
                        size: props.size,
                        radius: props.radius,
                        disabled: (props.disabled)(),
                        error: props.error.is_some(),
                        loading: props.loading,
                        left_section: props.left_section,
                        right_section: props.right_section.unwrap_or_else(|| rsx! {
                            ChevronDown { class: "dx-select-expand-icon", size: "14px", stroke: "currentColor" }
                        }),
                        attributes: attributes!(div { class : Styles::dx_select_input.to_string(), }),
                        select::SelectValue { placeholder: props.placeholder }
                    }
                }
            }
            select::SelectList { class: Styles::dx_select_list.to_string(), {props.children} }
        }
    }
}

#[component]
pub fn SelectMulti<T: Clone + PartialEq + 'static>(props: SelectMultiProps<T>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_select.to_string()
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    let input_id = props.id.unwrap_or_else(use_select_input_id);
    let aria_describedby = described_by(
        &input_id,
        props.description.is_some(),
        props.error.is_some(),
        props.described_by.as_deref(),
    );

    rsx! {
        select::SelectMulti {
            values: props.values,
            default_values: props.default_values,
            on_values_change: props.on_values_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            name: props.name,
            roving_loop: props.roving_loop,
            typeahead_timeout: props.typeahead_timeout,
            attributes: merged,
            InputWrapper {
                id: input_id.clone(),
                label: props.label,
                description: props.description,
                error: props.error.clone(),
                required: props.required,
                with_asterisk: props.with_asterisk,
                disabled: (props.disabled)(),
                described_by: props.described_by,
                select::SelectTrigger {
                    id: input_id,
                    class: Styles::dx_select_trigger.to_string(),
                    "aria-describedby": aria_describedby,
                    "aria-invalid": props.error.is_some(),
                    Input {
                        variant: props.variant,
                        size: props.size,
                        radius: props.radius,
                        disabled: (props.disabled)(),
                        error: props.error.is_some(),
                        loading: props.loading,
                        left_section: props.left_section,
                        right_section: props.right_section.unwrap_or_else(|| rsx! {
                            ChevronDown { class: "dx-select-expand-icon", size: "14px", stroke: "currentColor" }
                        }),
                        attributes: attributes!(div { class : Styles::dx_select_input.to_string(), }),
                        select::SelectValue {}
                    }
                }
            }
            select::SelectList { class: Styles::dx_select_list.to_string(), {props.children} }
        }
    }
}

#[component]
pub fn SelectGroupLabel(props: SelectGroupLabelProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_select_group_label.to_string()
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        select::SelectGroupLabel { id: props.id, attributes: merged, {props.children} }
    }
}

#[component]
pub fn SelectOption<T: Clone + PartialEq + 'static>(props: SelectOptionProps<T>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_select_option.to_string()
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        select::SelectOption::<T> {
            value: props.value,
            text_value: props.text_value,
            disabled: props.disabled,
            id: props.id,
            index: props.index,
            aria_label: props.aria_label,
            aria_roledescription: props.aria_roledescription,
            attributes: merged,
            {props.children}
            select::SelectItemIndicator {
                Check { size: "1rem", stroke: "var(--input-fg-muted)" }
            }
        }
    }
}
