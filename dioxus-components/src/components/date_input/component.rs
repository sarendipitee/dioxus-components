use crate::component_styles;
use dioxus::prelude::*;
use dioxus_icons::lucide::ChevronDown;
use dioxus_primitives::popover::PopoverRootProps;
use dioxus_primitives::{
    calendar::DateRange, date_picker, dioxus_attributes::attributes, merge_attributes, ContentAlign,
};
use time::{Date, Month};

use crate::components::date_picker::{
    DatePickerDaySegment, DatePickerMonthSegment, DatePickerSeparator, DatePickerSurface,
    DatePickerYearSegment, DateRangePickerEndValue, DateRangePickerStartValue,
    DateRangePickerSurface,
};
use crate::components::input::{
    use_input_control_context, InputBase, InputContent, InputLabel, InputRadius, InputSize,
    InputVariant,
};
use crate::components::popover::{PopoverContent, PopoverOpenTrigger, PopoverRoot};

#[component_styles("./style.css")]
struct Styles;

fn fixed_date(year: i32, month: Month, day: u8) -> Date {
    Date::from_calendar_date(year, month, day).expect("valid fixed date")
}

/// Styled single-date input composition built on the shared input foundation.
#[component]
pub fn DateInput(
    /// Callback when value changes.
    #[props(default)]
    on_value_change: Callback<Option<Date>>,
    /// The selected date.
    #[props(default)]
    selected_date: ReadSignal<Option<Date>>,
    /// Whether the date input is disabled.
    #[props(default)]
    disabled: ReadSignal<bool>,
    /// Whether the date input allows user editing.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    read_only: ReadSignal<bool>,
    /// Lower limit of available dates.
    #[props(default = fixed_date(1925, Month::January, 1))]
    min_date: Date,
    /// Upper limit of available dates.
    #[props(default = fixed_date(2050, Month::December, 31))]
    max_date: Date,
    /// Callback when displaying the day placeholder.
    #[props(default = Callback::new(|_| "D".to_string()))]
    on_format_day_placeholder: Callback<(), String>,
    /// Callback when displaying the month placeholder.
    #[props(default = Callback::new(|_| "M".to_string()))]
    on_format_month_placeholder: Callback<(), String>,
    /// Callback when displaying the year placeholder.
    #[props(default = Callback::new(|_| "Y".to_string()))]
    on_format_year_placeholder: Callback<(), String>,
    /// Number of visible calendar months.
    #[props(default = 1)]
    month_count: u8,
    /// Unavailable date ranges.
    #[props(default)]
    disabled_ranges: ReadSignal<Vec<DateRange>>,
    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    roving_loop: ReadSignal<bool>,
    /// Label rendered above the input.
    #[props(default, into)]
    label: InputLabel,
    /// Description rendered below the label.
    #[props(default, into)]
    description: InputContent,
    /// Error rendered below the input.
    #[props(default, into)]
    error: InputContent,
    /// Marks the input as required.
    #[props(default = false)]
    required: bool,
    /// Shows the required asterisk without native validation.
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
    /// Additional attributes to extend the date input.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let is_disabled = disabled();
    let month_count = month_count.max(1);

    rsx! {
        date_picker::DatePicker {
            on_value_change,
            selected_date,
            disabled,
            read_only,
            min_date,
            max_date,
            disabled_ranges,
            roving_loop,
            attributes,
            date_picker::DatePickerPopover {
                popover_root: DateInputPopoverRoot,
                open: None,
                close_on_input_focus: false,
                InputBase {
                    label,
                    description,
                    error: error.clone(),
                    required,
                    with_asterisk,
                    disabled: is_disabled,
                    loading,
                    variant,
                    size,
                    radius,
                    right_section: rsx! {
                        DateInputPopoverTrigger {
                            disabled: is_disabled,
                        }
                    },
                    DateInputControl {
                        disabled: is_disabled,
                        on_format_day_placeholder,
                        on_format_month_placeholder,
                        on_format_year_placeholder,
                    }
                }
                DateInputPopoverContent { align: ContentAlign::Center,
                    DatePickerSurface { month_count }
                }
            }
        }
    }
}

/// Styled date range input composition built on the shared input foundation.
#[component]
pub fn DateRangePickerInput(
    /// Callback when value changes.
    #[props(default)]
    on_range_change: Callback<Option<DateRange>>,
    /// The selected date range.
    #[props(default)]
    selected_range: ReadSignal<Option<DateRange>>,
    /// Whether the date range input is disabled.
    #[props(default)]
    disabled: ReadSignal<bool>,
    /// Whether the date range input allows user editing.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    read_only: ReadSignal<bool>,
    /// Lower limit of available dates.
    #[props(default = fixed_date(1925, Month::January, 1))]
    min_date: Date,
    /// Upper limit of available dates.
    #[props(default = fixed_date(2050, Month::December, 31))]
    max_date: Date,
    /// Number of visible calendar months.
    #[props(default = 1)]
    month_count: u8,
    /// Unavailable date ranges.
    #[props(default)]
    disabled_ranges: ReadSignal<Vec<DateRange>>,
    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    roving_loop: ReadSignal<bool>,
    /// Label rendered above the input.
    #[props(default, into)]
    label: InputLabel,
    /// Description rendered below the label.
    #[props(default, into)]
    description: InputContent,
    /// Error rendered below the input.
    #[props(default, into)]
    error: InputContent,
    /// Marks the input as required.
    #[props(default = false)]
    required: bool,
    /// Shows the required asterisk without native validation.
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
    /// Additional attributes to extend the date range input.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let is_disabled = disabled();
    let month_count = month_count.max(1);

    rsx! {
        date_picker::DateRangePicker {
            on_range_change,
            selected_range,
            disabled,
            read_only,
            min_date,
            max_date,
            disabled_ranges,
            roving_loop,
            attributes,
            date_picker::DatePickerPopover {
                popover_root: DateInputPopoverRoot,
                open: None,
                close_on_input_focus: false,
                InputBase {
                    label,
                    description,
                    error: error.clone(),
                    required,
                    with_asterisk,
                    disabled: is_disabled,
                    variant,
                    size,
                    radius,
                    loading,
                    right_section: rsx! {
                        DateInputPopoverTrigger {
                            disabled: is_disabled,
                        }
                    },
                    DateRangeInputControl {
                        disabled: is_disabled,
                    }
                }
                DateInputPopoverContent { align: ContentAlign::Center,
                    DateRangePickerSurface { month_count }
                }
            }
        }
    }
}

#[component]
fn DateInputPopoverRoot(props: PopoverRootProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_date_input_popover_root.to_string()
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        PopoverRoot {
            id: props.id,
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            attributes,
            {props.children}
        }
    }
}

#[derive(Clone, Copy)]
struct ApplyInputControlToDateSegment(bool);

#[component]
fn DateInputControl(
    disabled: bool,
    on_format_day_placeholder: Callback<(), String>,
    on_format_month_placeholder: Callback<(), String>,
    on_format_year_placeholder: Callback<(), String>,
) -> Element {
    use_context_provider(|| ApplyInputControlToDateSegment(true));

    rsx! {
        date_picker::DatePickerInput {
            attributes: attributes!(div {
                class: Styles::dx_date_input_group.to_string(),
            }),
            open_on_focus: !disabled,
            on_format_day_placeholder,
            on_format_month_placeholder,
            on_format_year_placeholder,
            date_picker::DatePickerInputValue {
                on_format_day_placeholder,
                on_format_month_placeholder,
                on_format_year_placeholder,
                DateInputYearSegment {}
                DatePickerSeparator {}
                DatePickerMonthSegment {}
                DatePickerSeparator {}
                DatePickerDaySegment {}
            }
        }
    }
}

#[component]
fn DateRangeInputControl(disabled: bool) -> Element {
    rsx! {
        date_picker::DateRangePickerInput {
            attributes: attributes!(div {
                class: Styles::dx_date_input_group.to_string(),
            }),
            open_on_focus: !disabled,
            date_picker::DateRangePickerInputValue {
                DateInputRangeStartValue {
                    DateInputYearSegment {}
                    DatePickerSeparator {}
                    DatePickerMonthSegment {}
                    DatePickerSeparator {}
                    DatePickerDaySegment {}
                }
                DatePickerSeparator { symbol: '—' }
                DateRangePickerEndValue {
                    DateInputYearSegment {}
                    DatePickerSeparator {}
                    DatePickerMonthSegment {}
                    DatePickerSeparator {}
                    DatePickerDaySegment {}
                }
            }
        }
    }
}

#[component]
fn DateInputRangeStartValue(children: Element) -> Element {
    use_context_provider(|| ApplyInputControlToDateSegment(true));

    rsx! {
        DateRangePickerStartValue { {children} }
    }
}

#[component]
fn DateInputYearSegment() -> Element {
    let should_apply_control = try_use_context::<ApplyInputControlToDateSegment>()
        .map(|ctx| ctx.0)
        .unwrap_or(false);
    let control = should_apply_control
        .then(use_input_control_context)
        .flatten()
        .map(|ctx| {
            attributes!(span {
                id: ctx.id,
                "aria-describedby": ctx.described_by,
                "aria-invalid": ctx.invalid,
            })
        });

    rsx! {
        DatePickerYearSegment { attributes: control.unwrap_or_default() }
    }
}

#[component]
fn DateInputPopoverTrigger(
    disabled: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let attributes = merge_attributes(vec![
        attributes!(button {
            aria_label: "Show Calendar",
            disabled,
        }),
        attributes,
    ]);

    rsx! {
        PopoverOpenTrigger {
            class: Styles::dx_date_input_popover_trigger.to_string(),
            attributes,
            ChevronDown {
                class: Styles::dx_date_input_trigger.to_string(),
                size: "14px",
                stroke: "oklch(var(--input-fg-muted))",
            }
        }
    }
}

#[component]
fn DateInputPopoverContent(
    #[props(default)] id: Option<String>,
    #[props(default = dioxus_primitives::ContentSide::Bottom)] side: dioxus_primitives::ContentSide,
    #[props(default = ContentAlign::Center)] align: ContentAlign,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        PopoverContent {
            class: Styles::dx_date_input_popover_content.to_string(),
            id,
            side,
            align,
            attributes,
            {children}
        }
    }
}
