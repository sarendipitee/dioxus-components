use std::sync::atomic::{AtomicUsize, Ordering};

use crate::component_styles;
use dioxus::prelude::*;
use dioxus_icons::lucide::{Check, ChevronsUpDown};
use dioxus_primitives::combobox::{
    self, default_combobox_filter, AutocompleteProps, ComboboxEmptyProps, ComboboxOptionProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

use crate::input::{element_label, Input, InputBase, InputRadius, InputSize, InputVariant};

#[component_styles("./style.css")]
struct Styles;

fn use_combobox_input_id() -> String {
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

    use_hook(|| {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        format!("dx-combobox-input-{id}")
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
        ids.push(id);
    }
    if description {
        ids.push(format!("{id}-description"));
    }
    if error {
        ids.push(format!("{id}-error"));
    }

    (!ids.is_empty()).then(|| ids.join(" "))
}

#[derive(Props, Clone, PartialEq)]
pub struct ComboboxProps<T: Clone + PartialEq + 'static = String> {
    /// Id shared by the label and search input.
    #[props(default)]
    pub id: Option<String>,

    #[props(default)]
    pub value: Option<ReadSignal<Option<T>>>,

    #[props(default)]
    pub default_value: Option<T>,

    #[props(default)]
    pub on_value_change: Callback<Option<T>>,

    #[props(default)]
    pub disabled: ReadSignal<bool>,

    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    #[props(default)]
    pub default_open: ReadSignal<bool>,

    #[props(default)]
    pub on_open_change: Callback<bool>,

    #[props(default)]
    pub query: ReadSignal<Option<String>>,

    #[props(default)]
    pub default_query: ReadSignal<String>,

    #[props(default)]
    pub on_query_change: Callback<String>,

    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    #[props(default = Callback::new(|(q, t): (String, String)| default_combobox_filter(&q, &t)))]
    pub filter: Callback<(String, String), bool>,

    #[props(default)]
    pub placeholder: ReadSignal<String>,

    #[props(default)]
    pub aria_label: Option<String>,

    #[props(default)]
    pub list_aria_label: Option<String>,

    /// Label rendered by the shared input wrapper.
    #[props(default)]
    pub label: Option<Element>,

    /// Description rendered by the shared input wrapper.
    #[props(default)]
    pub description: Option<Element>,

    /// Error rendered by the shared input wrapper and reflected on the shell.
    #[props(default)]
    pub error: Option<Element>,

    /// Marks the field as required.
    #[props(default = false)]
    pub required: bool,

    /// Shows the required asterisk without changing native validation.
    #[props(default = false)]
    pub with_asterisk: bool,

    /// Visual variant for the shared input shell.
    #[props(default)]
    pub variant: InputVariant,

    /// Size preset for the shared input shell.
    #[props(default)]
    pub size: InputSize,

    /// Radius preset for the shared input shell.
    #[props(default)]
    pub radius: InputRadius,

    /// Optional content rendered before the search input.
    #[props(default)]
    pub left_section: Option<Element>,

    /// Optional content rendered after the search input.
    #[props(default)]
    pub right_section: Option<Element>,

    /// Wraps children in the styled listbox container.
    #[props(default = true)]
    pub with_list: bool,

    /// Existing ids to prepend to generated description and error ids.
    #[props(default)]
    pub described_by: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn Combobox<T: Clone + PartialEq + 'static>(props: ComboboxProps<T>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_combobox
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    let input_id = props.id.unwrap_or_else(use_combobox_input_id);
    let aria_describedby = described_by(
        &input_id,
        props.description.is_some(),
        props.error.is_some(),
        props.described_by.as_deref(),
    );

    rsx! {
        combobox::Combobox {
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            query: props.query,
            default_query: props.default_query,
            on_query_change: props.on_query_change,
            roving_loop: props.roving_loop,
            filter: props.filter,
            attributes: merged,
            InputBase {
                id: input_id.clone(),
                label: element_label(props.label),
                description: props.description,
                error: props.error.clone(),
                required: props.required,
                with_asterisk: props.with_asterisk,
                disabled: (props.disabled)(),
                variant: props.variant,
                size: props.size,
                radius: props.radius,
                left_section: props.left_section,
                right_section: props.right_section.unwrap_or_else(|| rsx! {
                    ChevronsUpDown {
                        class: Styles::dx_combobox_expand_icon,
                        size: "16px",
                    }
                }),
                described_by: props.described_by,
                wrapper_attributes: attributes!(div {
                    class: Styles::dx_combobox_field,
                }),
                input_attributes: attributes!(div {
                    class: Styles::dx_combobox_input_wrapper,
                }),
                combobox::ComboboxInput {
                    id: Some(input_id),
                    class: Styles::dx_combobox_input,
                    placeholder: props.placeholder,
                    aria_label: props.aria_label.clone(),
                    "aria-describedby": aria_describedby,
                    "aria-invalid": props.error.is_some(),
                }
            }
            if props.with_list {
                combobox::ComboboxList {
                    class: Styles::dx_combobox_list,
                    aria_label: props.list_aria_label.clone(),
                    {props.children}
                }
            } else {
                {props.children}
            }
        }
    }
}

#[component]
pub fn Autocomplete(props: AutocompleteProps) -> Element {
    rsx! {
        Combobox::<String> {
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            query: props.query,
            default_query: props.default_query,
            on_query_change: props.on_query_change,
            roving_loop: props.roving_loop,
            filter: props.filter,
            placeholder: props.placeholder,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// Props for the styled [`MultiSelect`] adapter.
#[derive(Props, Clone, PartialEq)]
pub struct MultiSelectProps<T: Clone + PartialEq + 'static = String> {
    /// The controlled list of selected values.
    #[props(default)]
    pub values: ReadSignal<Option<Vec<T>>>,

    /// The default list of selected values.
    #[props(default)]
    pub default_values: Vec<T>,

    /// Callback when the list of selected values changes.
    #[props(default)]
    pub on_values_change: Callback<Vec<T>>,

    /// Whether the multi-select is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// The controlled open state of the popup.
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    /// The initial open state when uncontrolled.
    #[props(default)]
    pub default_open: ReadSignal<bool>,

    /// Callback fired when the popup open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// The controlled search query.
    #[props(default)]
    pub query: ReadSignal<Option<String>>,

    /// The initial search query when uncontrolled.
    #[props(default)]
    pub default_query: ReadSignal<String>,

    /// Callback fired when the search query changes.
    #[props(default)]
    pub on_query_change: Callback<String>,

    /// Whether arrow-key navigation should wrap.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Custom filter callback. Receives `(query, option_text_value)`.
    #[props(default = Callback::new(|(q, t): (String, String)| default_combobox_filter(&q, &t)))]
    pub filter: Callback<(String, String), bool>,

    /// Search placeholder.
    #[props(default)]
    pub placeholder: ReadSignal<String>,

    /// Maximum number of selected values.
    #[props(default)]
    pub max_values: Option<usize>,

    /// Renders a selected value as a pill inside the target.
    #[props(default)]
    pub render_value: Option<Callback<T, Element>>,

    /// Visual variant for the shared input shell.
    #[props(default)]
    pub variant: InputVariant,

    /// Size preset for the shared input shell.
    #[props(default)]
    pub size: InputSize,

    /// Radius preset for the shared input shell.
    #[props(default)]
    pub radius: InputRadius,

    /// Additional attributes for the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Option children.
    pub children: Element,
}

#[component]
pub fn MultiSelect<T: Clone + PartialEq + 'static>(props: MultiSelectProps<T>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_combobox_pills_root,
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        Input {
            variant: props.variant,
            size: props.size,
            radius: props.radius,
            disabled: (props.disabled)(),
            right_section: Some(rsx! {
                ChevronsUpDown {
                    class: Styles::dx_combobox_expand_icon,
                    size: "16px",
                }
            }),
            attributes: attributes!(div {
                class: Styles::dx_combobox_pill_field,
            }),
            combobox::MultiSelect::<T> {
                values: props.values,
                default_values: props.default_values,
                on_values_change: props.on_values_change,
                disabled: props.disabled,
                open: props.open,
                default_open: props.default_open,
                on_open_change: props.on_open_change,
                query: props.query,
                default_query: props.default_query,
                on_query_change: props.on_query_change,
                roving_loop: props.roving_loop,
                filter: props.filter,
                placeholder: props.placeholder,
                max_values: props.max_values,
                render_value: props.render_value,
                attributes,
                {props.children}
            }
        }
    }
}

/// Props for the styled [`PillsInput`] adapter.
#[derive(Props, Clone, PartialEq)]
pub struct PillsInputProps {
    /// Whether the input is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Visual variant for the shared input shell.
    #[props(default)]
    pub variant: InputVariant,

    /// Size preset for the shared input shell.
    #[props(default)]
    pub size: InputSize,

    /// Radius preset for the shared input shell.
    #[props(default)]
    pub radius: InputRadius,

    /// Additional attributes for the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Pill and input children.
    pub children: Element,
}

#[component]
pub fn PillsInput(props: PillsInputProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_combobox_pills_input,
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        Input {
            variant: props.variant,
            size: props.size,
            radius: props.radius,
            disabled: (props.disabled)(),
            attributes: attributes!(div {
                class: Styles::dx_combobox_pill_field,
            }),
            combobox::PillsInput {
                disabled: props.disabled,
                attributes,
                {props.children}
            }
        }
    }
}

/// Props for the styled [`TagsInput`] adapter.
#[derive(Props, Clone, PartialEq)]
pub struct TagsInputProps {
    /// The controlled tag list.
    #[props(default)]
    pub values: ReadSignal<Option<Vec<String>>>,

    /// The default tag list.
    #[props(default)]
    pub default_values: Vec<String>,

    /// Callback fired when tags change.
    #[props(default)]
    pub on_values_change: Callback<Vec<String>>,

    /// Placeholder for the text input.
    #[props(default)]
    pub placeholder: ReadSignal<String>,

    /// Whether duplicate tags are allowed.
    #[props(default)]
    pub allow_duplicates: ReadSignal<bool>,

    /// Whether the tags input is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Visual variant for the shared input shell.
    #[props(default)]
    pub variant: InputVariant,

    /// Size preset for the shared input shell.
    #[props(default)]
    pub size: InputSize,

    /// Radius preset for the shared input shell.
    #[props(default)]
    pub radius: InputRadius,

    /// Additional attributes for the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn TagsInput(props: TagsInputProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_combobox_tags_input,
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        Input {
            variant: props.variant,
            size: props.size,
            radius: props.radius,
            disabled: (props.disabled)(),
            attributes: attributes!(div {
                class: Styles::dx_combobox_pill_field,
            }),
            combobox::TagsInput {
                values: props.values,
                default_values: props.default_values,
                on_values_change: props.on_values_change,
                placeholder: props.placeholder,
                allow_duplicates: props.allow_duplicates,
                disabled: props.disabled,
                attributes,
            }
        }
    }
}

#[component]
pub fn ComboboxEmpty(props: ComboboxEmptyProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_combobox_empty
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        combobox::ComboboxEmpty {
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn ComboboxOption<T: Clone + PartialEq + 'static>(props: ComboboxOptionProps<T>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_combobox_option
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        combobox::ComboboxOption::<T> {
            value: props.value,
            text_value: props.text_value,
            disabled: props.disabled,
            id: props.id,
            index: props.index,
            aria_label: props.aria_label,
            aria_roledescription: props.aria_roledescription,
            attributes: merged,
            {props.children}
            combobox::ComboboxItemIndicator {
                Check {
                    class: Styles::dx_combobox_check_icon,
                    size: "16px",
                }
            }
        }
    }
}
