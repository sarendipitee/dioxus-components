use dioxus::prelude::*;

use dioxus_icons::lucide::{ChevronDown, X};
use dioxus_primitives::{
    dioxus_attributes::attributes,
    merge_attributes,
    popover::{PopoverContentProps, PopoverTriggerProps},
    time_picker::{
        self, parse_default_time_picker_value, time_picker_preset_range,
        TimePickerAmPmSegmentProps, TimePickerFormat, TimePickerHourSegmentProps, TimePickerLabels,
        TimePickerMinuteSegmentProps, TimePickerSecondSegmentProps, TimePickerSeparatorProps,
        TimePickerSteps, TimePickerType, TimePickerValue,
    },
    ContentAlign, ContentSide,
};
use std::rc::Rc;
use time::Time;

use super::super::popover::*;

#[css_module("/src/components/time_picker/style.css")]
struct Styles;

/// Visual variant for the preview [`TimePicker`] field.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TimePickerVariant {
    /// Default filled input styling.
    #[default]
    Default,
    /// Outline input styling.
    Outline,
    /// Ghost input styling.
    Ghost,
}

impl TimePickerVariant {
    fn class(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Outline => "outline",
            Self::Ghost => "ghost",
        }
    }
}

/// Size for the preview [`TimePicker`] field.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TimePickerSize {
    /// Small field.
    Sm,
    /// Default field.
    #[default]
    Md,
    /// Large field.
    Lg,
}

impl TimePickerSize {
    fn class(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
        }
    }
}

/// Controls how clear and right-section content are rendered.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TimePickerClearSectionMode {
    /// Render both clear and custom right-section content when both are supplied.
    #[default]
    Both,
    /// Render the clear button only in the right section.
    Clear,
    /// Render only custom right-section content.
    RightSection,
}

/// A selectable TimePicker preset.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimePickerPreset {
    /// Visible preset label.
    pub label: String,
    /// Preset value.
    pub value: Time,
}

impl TimePickerPreset {
    /// Create a preset from a label and time value.
    pub fn new(label: impl Into<String>, value: Time) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }
}

/// A labelled group of TimePicker presets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimePickerPresetGroup {
    /// Visible group label.
    pub label: String,
    /// Presets in this group.
    pub presets: Vec<TimePickerPreset>,
}

#[allow(dead_code)]
impl TimePickerPresetGroup {
    /// Create a grouped preset collection.
    pub fn new(label: impl Into<String>, presets: Vec<TimePickerPreset>) -> Self {
        Self {
            label: label.into(),
            presets,
        }
    }
}

/// Generate labelled presets for an inclusive time range.
#[allow(dead_code)]
pub fn get_time_range(start: Time, end: Time, step_seconds: u32) -> Vec<TimePickerPreset> {
    time_picker_preset_range(start, end, step_seconds)
        .into_iter()
        .map(|value| {
            TimePickerPreset::new(
                format_time(value, true, TimePickerFormat::TwentyFourHour),
                value,
            )
        })
        .collect()
}

/// The props for the preview [`TimePicker`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TimePickerProps {
    /// Callback when value changes.
    #[props(default)]
    pub on_value_change: Callback<Option<Time>>,

    /// Callback when the selected time or duration changes.
    #[props(default)]
    pub on_picker_value_change: Callback<Option<TimePickerValue>>,

    /// The selected time.
    #[props(default)]
    pub selected_time: ReadSignal<Option<Time>>,

    /// The selected time-picker value.
    #[props(default)]
    pub selected_value: ReadSignal<Option<TimePickerValue>>,

    /// Whether the time picker is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether the time picker is read-only.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub read_only: ReadSignal<bool>,

    /// Lower limit of the selectable time range.
    #[props(default = time::macros::time!(00:00))]
    pub min_time: Time,

    /// Upper limit of the selectable time range.
    #[props(default = time::macros::time!(23:59:59))]
    pub max_time: Time,

    /// Include a seconds segment and emit second-precision values.
    #[props(default = false)]
    pub with_seconds: bool,

    /// Whether the picker edits a clock time or a duration.
    #[props(default = TimePickerType::Time)]
    pub picker_type: TimePickerType,

    /// Display format for clock time.
    #[props(default = TimePickerFormat::TwentyFourHour)]
    pub format: TimePickerFormat,

    /// Minimum number of hour digits rendered in duration mode.
    #[props(default = 2)]
    pub min_hours_digits: usize,

    /// Step sizes for keyboard and dropdown editing.
    #[props(default)]
    pub steps: TimePickerSteps,

    /// Labels for the AM and PM period options.
    #[props(default = ("AM".to_string(), "PM".to_string()))]
    pub am_pm_labels: (String, String),

    /// Accessibility labels for segments and clear affordances.
    #[props(default)]
    pub labels: TimePickerLabels,

    /// Callback parser used by paste handling.
    #[props(default = Callback::new(parse_default_time_picker_value))]
    pub on_paste_split: Callback<String, Option<TimePickerValue>>,

    /// Callback when focus enters the aggregate time input.
    #[props(default)]
    pub onfocusin: Callback<Event<FocusData>>,

    /// Callback when focus leaves the aggregate time input.
    #[props(default)]
    pub onfocusout: Callback<Event<FocusData>>,

    /// Callback when display hour placeholder.
    #[props(default = Callback::new(|_| "H".to_string()))]
    pub on_format_hour_placeholder: Callback<(), String>,

    /// Callback when display minute placeholder.
    #[props(default = Callback::new(|_| "M".to_string()))]
    pub on_format_minute_placeholder: Callback<(), String>,

    /// Callback when display second placeholder.
    #[props(default = Callback::new(|_| "S".to_string()))]
    pub on_format_second_placeholder: Callback<(), String>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub roving_loop: ReadSignal<bool>,

    /// Whether a clear button should be rendered.
    #[props(default = false)]
    pub clearable: bool,

    /// Callback that receives the mounted hour segment.
    #[props(default)]
    pub hours_ref: Callback<Rc<MountedData>>,

    /// Callback that receives the mounted minute segment.
    #[props(default)]
    pub minutes_ref: Callback<Rc<MountedData>>,

    /// Callback that receives the mounted second segment.
    #[props(default)]
    pub seconds_ref: Callback<Rc<MountedData>>,

    /// Callback that receives the mounted AM/PM segment.
    #[props(default)]
    pub am_pm_ref: Callback<Rc<MountedData>>,

    /// Controls clear and right-section rendering.
    #[props(default)]
    pub clear_section_mode: TimePickerClearSectionMode,

    /// Optional right-section content.
    #[props(default)]
    pub right_section: Option<Element>,

    /// Render the dropdown picker trigger and content.
    #[props(default = false)]
    pub with_dropdown: bool,

    /// Controlled dropdown open state.
    #[props(default)]
    pub dropdown_opened: ReadSignal<Option<bool>>,

    /// Default dropdown open state when uncontrolled.
    #[props(default)]
    pub default_dropdown_opened: bool,

    /// Callback fired when dropdown open state changes.
    #[props(default)]
    pub on_dropdown_open_change: Callback<bool>,

    /// Side of the input where dropdown content is placed.
    #[props(default = ContentSide::Bottom)]
    pub dropdown_side: ContentSide,

    /// Alignment of dropdown content relative to the input.
    #[props(default = ContentAlign::Center)]
    pub dropdown_align: ContentAlign,

    /// CSS width for dropdown content.
    #[props(default)]
    pub dropdown_width: Option<String>,

    /// CSS max-height for dropdown scroll columns.
    #[props(default = "14rem".to_string())]
    pub max_dropdown_content_height: String,

    /// Flat preset buttons.
    #[props(default)]
    pub presets: Vec<TimePickerPreset>,

    /// Grouped preset buttons.
    #[props(default)]
    pub preset_groups: Vec<TimePickerPresetGroup>,

    /// Optional field label.
    #[props(default)]
    pub label: Option<String>,

    /// Optional field description.
    #[props(default)]
    pub description: Option<String>,

    /// Optional visible error text.
    #[props(default)]
    pub error: Option<String>,

    /// Visual variant.
    #[props(default)]
    pub variant: TimePickerVariant,

    /// Control size.
    #[props(default)]
    pub size: TimePickerSize,

    /// CSS border-radius for the input.
    #[props(default)]
    pub radius: Option<String>,

    /// Additional attributes to extend the time picker element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// # TimePicker
///
/// A styled segmented time picker for the preview demos.
#[component]
pub fn TimePicker(props: TimePickerProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_time_picker.to_string(),
        "data-variant": props.variant.class(),
        "data-size": props.size.class(),
        "data-error": props.error.is_some().to_string(),
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    let has_value = (props.selected_time)().is_some() || (props.selected_value)().is_some();
    let show_clear = props.clearable
        && has_value
        && props.clear_section_mode != TimePickerClearSectionMode::RightSection;
    let show_right_section = props.right_section.is_some()
        && props.clear_section_mode != TimePickerClearSectionMode::Clear;
    let field_style = props
        .radius
        .as_ref()
        .map(|radius| format!("--dx-time-picker-radius: {radius};"))
        .unwrap_or_default();

    rsx! {
        div {
            class: Styles::dx_time_picker_field.to_string(),
            if let Some(label) = props.label.clone() {
                label { class: Styles::dx_time_picker_label.to_string(), {label} }
            }
            if let Some(description) = props.description.clone() {
                div { class: Styles::dx_time_picker_description.to_string(), {description} }
            }
            time_picker::TimePicker {
                on_value_change: props.on_value_change,
                on_picker_value_change: props.on_picker_value_change,
                selected_time: props.selected_time,
                selected_value: props.selected_value,
                disabled: props.disabled,
                read_only: props.read_only,
                min_time: props.min_time,
                max_time: props.max_time,
                with_seconds: props.with_seconds,
                picker_type: props.picker_type,
                format: props.format,
                min_hours_digits: props.min_hours_digits,
                steps: props.steps,
                am_pm_labels: props.am_pm_labels.clone(),
                labels: props.labels.clone(),
                hours_ref: props.hours_ref,
                minutes_ref: props.minutes_ref,
                seconds_ref: props.seconds_ref,
                am_pm_ref: props.am_pm_ref,
                on_paste_split: props.on_paste_split,
                onfocusin: props.onfocusin,
                onfocusout: props.onfocusout,
                roving_loop: props.roving_loop,
                attributes: merged,
                if props.with_dropdown && props.picker_type == TimePickerType::Time {
                    PopoverRoot {
                        is_modal: false,
                        open: props.dropdown_opened,
                        default_open: props.default_dropdown_opened,
                        on_open_change: props.on_dropdown_open_change,
                        TimePickerInput {
                            with_seconds: props.with_seconds,
                            picker_type: props.picker_type,
                            format: props.format,
                            on_format_hour_placeholder: props.on_format_hour_placeholder,
                            on_format_minute_placeholder: props.on_format_minute_placeholder,
                            on_format_second_placeholder: props.on_format_second_placeholder,
                            style: field_style.clone(),
                            if show_right_section {
                                span { class: Styles::dx_time_picker_right_section.to_string(), {props.right_section} }
                            }
                            TimePickerPopoverTrigger {}
                            if show_clear {
                                TimePickerClearButton {
                                    aria_label: props.labels.clear.clone(),
                                    onclick: move |_| {
                                        props.on_value_change.call(None);
                                        props.on_picker_value_change.call(None);
                                    },
                                }
                            }
                            TimePickerDropdown {
                                selected_time: props.selected_time,
                                selected_value: props.selected_value,
                                on_value_change: props.on_value_change,
                                on_picker_value_change: props.on_picker_value_change,
                                disabled: props.disabled,
                                read_only: props.read_only,
                                min_time: props.min_time,
                                max_time: props.max_time,
                                with_seconds: props.with_seconds,
                                format: props.format,
                                steps: props.steps,
                                am_pm_labels: props.am_pm_labels.clone(),
                                side: props.dropdown_side,
                                align: props.dropdown_align,
                                width: props.dropdown_width.clone(),
                                max_height: props.max_dropdown_content_height.clone(),
                                presets: props.presets.clone(),
                                preset_groups: props.preset_groups.clone(),
                            }
                        }
                    }
                } else {
                    TimePickerInput {
                        with_seconds: props.with_seconds,
                        picker_type: props.picker_type,
                        format: props.format,
                        on_format_hour_placeholder: props.on_format_hour_placeholder,
                        on_format_minute_placeholder: props.on_format_minute_placeholder,
                        on_format_second_placeholder: props.on_format_second_placeholder,
                        style: field_style.clone(),
                        if show_clear {
                            TimePickerClearButton {
                                aria_label: props.labels.clear.clone(),
                                onclick: move |_| {
                                    props.on_value_change.call(None);
                                    props.on_picker_value_change.call(None);
                                },
                            }
                        }
                        if show_right_section {
                            span { class: Styles::dx_time_picker_right_section.to_string(), {props.right_section} }
                        }
                    }
                }
            }
            if let Some(error) = props.error.clone() {
                div { class: Styles::dx_time_picker_error.to_string(), role: "alert", {error} }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub(crate) struct TimePickerInputProps {
    /// Include a seconds segment.
    #[props(default = false)]
    pub with_seconds: bool,

    /// Whether the picker edits a clock time or a duration.
    #[props(default = TimePickerType::Time)]
    pub picker_type: TimePickerType,

    /// Display format for clock time.
    #[props(default = TimePickerFormat::TwentyFourHour)]
    pub format: TimePickerFormat,

    /// Callback when display hour placeholder.
    #[props(default = Callback::new(|_| "H".to_string()))]
    pub on_format_hour_placeholder: Callback<(), String>,

    /// Callback when display minute placeholder.
    #[props(default = Callback::new(|_| "M".to_string()))]
    pub on_format_minute_placeholder: Callback<(), String>,

    /// Callback when display second placeholder.
    #[props(default = Callback::new(|_| "S".to_string()))]
    pub on_format_second_placeholder: Callback<(), String>,

    /// Additional attributes for the input element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the time picker element.
    #[props(default)]
    pub children: Option<Element>,
}

#[component]
pub(crate) fn TimePickerInput(props: TimePickerInputProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_time_picker_group.to_string()
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    let extra_children = props.children;

    rsx! {
        time_picker::TimePickerInput {
            on_format_hour_placeholder: props.on_format_hour_placeholder,
            on_format_minute_placeholder: props.on_format_minute_placeholder,
            on_format_second_placeholder: props.on_format_second_placeholder,
            attributes: merged,
            time_picker::TimePickerInputValue {
                on_format_hour_placeholder: props.on_format_hour_placeholder,
                on_format_minute_placeholder: props.on_format_minute_placeholder,
                on_format_second_placeholder: props.on_format_second_placeholder,
                TimePickerHourSegment {}
                TimePickerSeparator {}
                TimePickerMinuteSegment {}
                if props.with_seconds {
                    TimePickerSeparator {}
                    TimePickerSecondSegment {}
                }
                if props.format == TimePickerFormat::TwelveHour && props.picker_type == TimePickerType::Time {
                    TimePickerAmPmSegment {}
                }
            }
            if let Some(extra_children) = extra_children {
                {extra_children}
            }
        }
    }
}

#[component]
pub(crate) fn TimePickerHourSegment(props: TimePickerHourSegmentProps) -> Element {
    rsx! {
        time_picker::TimePickerHourSegment {
            class: Styles::dx_time_segment.to_string(),
            attributes: props.attributes,
        }
    }
}

#[component]
pub(crate) fn TimePickerMinuteSegment(props: TimePickerMinuteSegmentProps) -> Element {
    rsx! {
        time_picker::TimePickerMinuteSegment {
            class: Styles::dx_time_segment.to_string(),
            attributes: props.attributes,
        }
    }
}

#[component]
pub(crate) fn TimePickerSecondSegment(props: TimePickerSecondSegmentProps) -> Element {
    rsx! {
        time_picker::TimePickerSecondSegment {
            class: Styles::dx_time_segment.to_string(),
            attributes: props.attributes,
        }
    }
}

#[component]
pub(crate) fn TimePickerAmPmSegment(props: TimePickerAmPmSegmentProps) -> Element {
    rsx! {
        time_picker::TimePickerAmPmSegment {
            class: Styles::dx_time_segment.to_string(),
            attributes: props.attributes,
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub(crate) struct TimePickerClearButtonProps {
    /// Callback when the clear button is clicked.
    #[props(default)]
    pub onclick: EventHandler<MouseEvent>,

    /// Additional attributes for the clear button.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub(crate) fn TimePickerClearButton(props: TimePickerClearButtonProps) -> Element {
    rsx! {
        button {
            type: "button",
            class: Styles::dx_time_picker_clear.to_string(),
            aria_label: "Clear time",
            onclick: move |event| props.onclick.call(event),
            ..props.attributes,
            X {
                size: 12,
            }
        }
    }
}

#[component]
pub(crate) fn TimePickerSeparator(props: TimePickerSeparatorProps) -> Element {
    rsx! {
        time_picker::TimePickerSeparator {
            class: Styles::dx_time_segment.to_string(),
            symbol: props.symbol,
            attributes: props.attributes,
        }
    }
}

#[component]
pub(crate) fn TimePickerPopoverTrigger(props: PopoverTriggerProps) -> Element {
    rsx! {
        PopoverTrigger {
            class: Styles::dx_time_picker_popover_trigger.to_string(),
            aria_label: "Show time options",
            attributes: props.attributes,
            ChevronDown {
                size: 18,
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub(crate) struct TimePickerDropdownProps {
    pub selected_time: ReadSignal<Option<Time>>,
    pub selected_value: ReadSignal<Option<TimePickerValue>>,
    pub on_value_change: Callback<Option<Time>>,
    pub on_picker_value_change: Callback<Option<TimePickerValue>>,
    pub disabled: ReadSignal<bool>,
    pub read_only: ReadSignal<bool>,
    pub min_time: Time,
    pub max_time: Time,
    pub with_seconds: bool,
    pub format: TimePickerFormat,
    pub steps: TimePickerSteps,
    pub am_pm_labels: (String, String),
    pub side: ContentSide,
    pub align: ContentAlign,
    pub width: Option<String>,
    pub max_height: String,
    pub presets: Vec<TimePickerPreset>,
    pub preset_groups: Vec<TimePickerPresetGroup>,
}

#[component]
pub(crate) fn TimePickerDropdown(props: TimePickerDropdownProps) -> Element {
    let style = props
        .width
        .as_ref()
        .map(|width| format!("width: {width};"))
        .unwrap_or_default();
    let current = move || current_time(props.selected_time, props.selected_value, props.min_time);
    let select = move |time: Time| {
        props
            .on_picker_value_change
            .call(Some(TimePickerValue::Time(time)));
        props.on_value_change.call(Some(time));
    };

    rsx! {
        TimePickerPopoverContent {
            side: props.side,
            align: props.align,
            style,
            div { class: Styles::dx_time_picker_dropdown_grid.to_string(),
                TimePickerDropdownColumn {
                    label: "Hours",
                    max_height: props.max_height.clone(),
                    for hour in dropdown_hours(props.steps.hours, props.format) {
                        TimePickerDropdownOption {
                            label: format_hour(hour, props.format),
                            selected: hour_matches(current(), hour, props.format),
                            disabled: option_disabled(replace_hour(current(), hour, props.format), props.min_time, props.max_time, props.disabled, props.read_only),
                            onclick: move |_| select(replace_hour(current(), hour, props.format)),
                        }
                    }
                }
                TimePickerDropdownColumn {
                    label: "Minutes",
                    max_height: props.max_height.clone(),
                    for minute in dropdown_units(props.steps.minutes, 59) {
                        TimePickerDropdownOption {
                            label: format!("{minute:02}"),
                            selected: current().minute() == minute,
                            disabled: option_disabled(replace_minute(current(), minute), props.min_time, props.max_time, props.disabled, props.read_only),
                            onclick: move |_| select(replace_minute(current(), minute)),
                        }
                    }
                }
                if props.with_seconds {
                    TimePickerDropdownColumn {
                        label: "Seconds",
                        max_height: props.max_height.clone(),
                        for second in dropdown_units(props.steps.seconds, 59) {
                            TimePickerDropdownOption {
                                label: format!("{second:02}"),
                                selected: current().second() == second,
                                disabled: option_disabled(replace_second(current(), second), props.min_time, props.max_time, props.disabled, props.read_only),
                                onclick: move |_| select(replace_second(current(), second)),
                            }
                        }
                    }
                }
                if props.format == TimePickerFormat::TwelveHour {
                    TimePickerDropdownColumn {
                        label: "Period",
                        max_height: props.max_height.clone(),
                        TimePickerDropdownOption {
                            label: props.am_pm_labels.0.clone(),
                            selected: current().hour() < 12,
                            disabled: option_disabled(replace_period(current(), false), props.min_time, props.max_time, props.disabled, props.read_only),
                            onclick: move |_| select(replace_period(current(), false)),
                        }
                        TimePickerDropdownOption {
                            label: props.am_pm_labels.1.clone(),
                            selected: current().hour() >= 12,
                            disabled: option_disabled(replace_period(current(), true), props.min_time, props.max_time, props.disabled, props.read_only),
                            onclick: move |_| select(replace_period(current(), true)),
                        }
                    }
                }
            }
            if !props.presets.is_empty() {
                TimePickerPresetList {
                    selected_time: props.selected_time,
                    selected_value: props.selected_value,
                    on_value_change: props.on_value_change,
                    on_picker_value_change: props.on_picker_value_change,
                    disabled: props.disabled,
                    read_only: props.read_only,
                    min_time: props.min_time,
                    max_time: props.max_time,
                    presets: props.presets.clone(),
                }
            }
            for group in props.preset_groups.clone() {
                div { class: Styles::dx_time_picker_preset_group.to_string(),
                    div { class: Styles::dx_time_picker_preset_group_label.to_string(), {group.label} }
                    TimePickerPresetList {
                        selected_time: props.selected_time,
                        selected_value: props.selected_value,
                        on_value_change: props.on_value_change,
                        on_picker_value_change: props.on_picker_value_change,
                        disabled: props.disabled,
                        read_only: props.read_only,
                        min_time: props.min_time,
                        max_time: props.max_time,
                        presets: group.presets,
                    }
                }
            }
        }
    }
}

#[component]
pub(crate) fn TimePickerPopoverContent(props: PopoverContentProps) -> Element {
    rsx! {
        PopoverContent {
            class: Styles::dx_time_picker_dropdown.to_string(),
            id: props.id,
            side: props.side,
            align: props.align,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub(crate) struct TimePickerDropdownColumnProps {
    pub label: String,
    pub max_height: String,
    pub children: Element,
}

#[component]
pub(crate) fn TimePickerDropdownColumn(props: TimePickerDropdownColumnProps) -> Element {
    rsx! {
        div { class: Styles::dx_time_picker_dropdown_column.to_string(),
            div { class: Styles::dx_time_picker_dropdown_label.to_string(), {props.label} }
            div {
                class: Styles::dx_time_picker_dropdown_options.to_string(),
                style: "max-height: {props.max_height};",
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub(crate) struct TimePickerDropdownOptionProps {
    pub label: String,
    pub selected: bool,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
}

#[component]
pub(crate) fn TimePickerDropdownOption(props: TimePickerDropdownOptionProps) -> Element {
    rsx! {
        button {
            type: "button",
            class: Styles::dx_time_picker_dropdown_option.to_string(),
            "data-selected": props.selected.to_string(),
            disabled: props.disabled,
            onclick: move |event| props.onclick.call(event),
            {props.label}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub(crate) struct TimePickerPresetListProps {
    pub selected_time: ReadSignal<Option<Time>>,
    pub selected_value: ReadSignal<Option<TimePickerValue>>,
    pub on_value_change: Callback<Option<Time>>,
    pub on_picker_value_change: Callback<Option<TimePickerValue>>,
    pub disabled: ReadSignal<bool>,
    pub read_only: ReadSignal<bool>,
    pub min_time: Time,
    pub max_time: Time,
    pub presets: Vec<TimePickerPreset>,
}

#[component]
pub(crate) fn TimePickerPresetList(props: TimePickerPresetListProps) -> Element {
    let selected = move || current_time(props.selected_time, props.selected_value, props.min_time);
    let select = move |time: Time| {
        props
            .on_picker_value_change
            .call(Some(TimePickerValue::Time(time)));
        props.on_value_change.call(Some(time));
    };

    rsx! {
        div { class: Styles::dx_time_picker_preset_row.to_string(),
            for preset in props.presets.clone() {
                button {
                    type: "button",
                    class: Styles::dx_time_picker_preset.to_string(),
                    "data-selected": (selected() == preset.value).to_string(),
                    disabled: option_disabled(preset.value, props.min_time, props.max_time, props.disabled, props.read_only),
                    onclick: move |_| select(preset.value),
                    {preset.label}
                }
            }
        }
    }
}

fn current_time(
    selected_time: ReadSignal<Option<Time>>,
    selected_value: ReadSignal<Option<TimePickerValue>>,
    fallback: Time,
) -> Time {
    match (selected_value)() {
        Some(TimePickerValue::Time(time)) => time,
        Some(TimePickerValue::Duration {
            hours,
            minutes,
            seconds,
        }) => Time::from_hms((hours % 24) as u8, minutes, seconds).expect("valid duration time"),
        _ => (selected_time)().unwrap_or(fallback),
    }
}

fn dropdown_units(step: u32, max: u8) -> Vec<u8> {
    let step = step.max(1);
    let mut values = Vec::new();
    let mut current = 0u32;
    while current <= max as u32 {
        values.push(current as u8);
        current = current.saturating_add(step);
        if current == u32::MAX {
            break;
        }
    }
    values
}

fn dropdown_hours(step: u32, format: TimePickerFormat) -> Vec<u8> {
    match format {
        TimePickerFormat::TwentyFourHour => dropdown_units(step, 23),
        TimePickerFormat::TwelveHour => {
            let mut hours: Vec<u8> = dropdown_units(step, 12)
                .into_iter()
                .filter(|hour| *hour >= 1)
                .collect();
            if !hours.contains(&12) {
                hours.push(12);
                hours.sort_unstable();
            }
            hours
        }
    }
}

#[allow(dead_code)]
fn format_time(time: Time, with_seconds: bool, format: TimePickerFormat) -> String {
    match (format, with_seconds) {
        (TimePickerFormat::TwentyFourHour, false) => {
            format!("{:02}:{:02}", time.hour(), time.minute())
        }
        (TimePickerFormat::TwentyFourHour, true) => {
            format!(
                "{:02}:{:02}:{:02}",
                time.hour(),
                time.minute(),
                time.second()
            )
        }
        (TimePickerFormat::TwelveHour, false) => {
            let suffix = if time.hour() < 12 { "AM" } else { "PM" };
            format!(
                "{:02}:{:02} {suffix}",
                display_hour(time.hour()),
                time.minute()
            )
        }
        (TimePickerFormat::TwelveHour, true) => {
            let suffix = if time.hour() < 12 { "AM" } else { "PM" };
            format!(
                "{:02}:{:02}:{:02} {suffix}",
                display_hour(time.hour()),
                time.minute(),
                time.second()
            )
        }
    }
}

fn format_hour(hour: u8, format: TimePickerFormat) -> String {
    match format {
        TimePickerFormat::TwentyFourHour => format!("{hour:02}"),
        TimePickerFormat::TwelveHour => format!("{hour:02}"),
    }
}

fn display_hour(hour: u8) -> u8 {
    match hour % 12 {
        0 => 12,
        hour => hour,
    }
}

fn hour_matches(time: Time, hour: u8, format: TimePickerFormat) -> bool {
    match format {
        TimePickerFormat::TwentyFourHour => time.hour() == hour,
        TimePickerFormat::TwelveHour => display_hour(time.hour()) == hour,
    }
}

fn replace_hour(time: Time, hour: u8, format: TimePickerFormat) -> Time {
    let hour = match format {
        TimePickerFormat::TwentyFourHour => hour,
        TimePickerFormat::TwelveHour => {
            let is_pm = time.hour() >= 12;
            let base = hour % 12;
            if is_pm {
                base + 12
            } else {
                base
            }
        }
    };
    Time::from_hms(hour, time.minute(), time.second()).expect("valid time")
}

fn replace_minute(time: Time, minute: u8) -> Time {
    Time::from_hms(time.hour(), minute, time.second()).expect("valid time")
}

fn replace_second(time: Time, second: u8) -> Time {
    Time::from_hms(time.hour(), time.minute(), second).expect("valid time")
}

fn replace_period(time: Time, pm: bool) -> Time {
    let display = display_hour(time.hour()) % 12;
    let hour = if pm { display + 12 } else { display };
    Time::from_hms(hour, time.minute(), time.second()).expect("valid time")
}

fn option_disabled(
    value: Time,
    min_time: Time,
    max_time: Time,
    disabled: ReadSignal<bool>,
    read_only: ReadSignal<bool>,
) -> bool {
    disabled() || read_only() || value < min_time || value > max_time
}
