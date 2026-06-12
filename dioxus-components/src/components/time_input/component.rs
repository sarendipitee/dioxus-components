use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::time_picker::{
    TimePicker, TimePickerAmPmSegment, TimePickerFormat, TimePickerHourSegment, TimePickerInput,
    TimePickerInputValue, TimePickerLabels, TimePickerMinuteSegment, TimePickerSecondSegment,
    TimePickerSeparator, TimePickerSteps, TimePickerType, TimePickerValue,
};
use time::{macros::time, Time};

use crate::components::input::{
    element_label, use_input_control_context, InputBase, InputClearButton, InputRadius, InputSize,
    InputVariant,
};
use crate::components::popover::{PopoverContent, PopoverRoot};

#[css_module("/src/components/time_input/style.css")]
struct Styles;

/// Controls how `TimeInput` combines its clear button and custom right section.
#[derive(Copy, Clone, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum TimeInputClearSectionMode {
    /// Render both the clear button and the custom right section.
    #[default]
    Both,
    /// Render only the clear button when clearable.
    Clear,
    /// Render only the custom right section.
    RightSection,
}

fn time_input_right_section(
    clearable: bool,
    disabled: bool,
    labels: TimePickerLabels,
    mode: TimeInputClearSectionMode,
    right_section: Option<Element>,
    on_clear: EventHandler<MouseEvent>,
) -> Option<Element> {
    let clear = (clearable && mode != TimeInputClearSectionMode::RightSection).then(|| {
        rsx! {
            InputClearButton {
                aria_label: labels.clear.clone(),
                disabled,
                onclick: move |event| on_clear.call(event),
            }
        }
    });
    let right_section = (mode != TimeInputClearSectionMode::Clear)
        .then_some(right_section)
        .flatten();

    match (clear, right_section) {
        (Some(clear), Some(right_section)) => Some(rsx! {
            div {
                style: "display: inline-flex; align-items: center; gap: 0.25rem;",
                {clear}
                {right_section}
            }
        }),
        (Some(clear), None) => Some(clear),
        (None, Some(right_section)) => Some(right_section),
        (None, None) => None,
    }
}

fn use_time_input_id() -> String {
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

    use_hook(move || {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        format!("dx-time-input-{id}")
    })
}

/// Styled time input composition built on the shared input foundation.
#[component]
pub fn TimeInput(
    /// Callback when the selected clock time changes.
    #[props(default)]
    on_value_change: Callback<Option<Time>>,
    /// Callback when the selected picker value changes.
    #[props(default)]
    on_picker_value_change: Callback<Option<TimePickerValue>>,
    /// The selected clock time.
    #[props(default)]
    selected_time: ReadSignal<Option<Time>>,
    /// The selected picker value.
    #[props(default)]
    selected_value: ReadSignal<Option<TimePickerValue>>,
    /// Whether the time input is disabled.
    #[props(default)]
    disabled: ReadSignal<bool>,
    /// Whether the time input is read-only.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    read_only: ReadSignal<bool>,
    /// Lower limit of the selectable time range.
    #[props(default = time!(00:00))]
    min_time: Time,
    /// Upper limit of the selectable time range.
    #[props(default = time!(23:59))]
    max_time: Time,
    /// Include a seconds segment.
    #[props(default = false)]
    with_seconds: bool,
    /// Whether the input edits a clock time or duration.
    #[props(default = TimePickerType::Time)]
    picker_type: TimePickerType,
    /// Display format for clock time.
    #[props(default = TimePickerFormat::TwentyFourHour)]
    format: TimePickerFormat,
    /// Minimum number of hour digits rendered in duration mode.
    #[props(default = 2)]
    min_hours_digits: usize,
    /// Step sizes for keyboard editing.
    #[props(default)]
    steps: TimePickerSteps,
    /// Labels for AM and PM period options.
    #[props(default = ("AM".to_string(), "PM".to_string()))]
    am_pm_labels: (String, String),
    /// List of preset times shown as quick-select buttons in the picker.
    #[props(default)]
    presets: Vec<Time>,
    /// Accessibility labels for segments and clear affordances.
    #[props(default)]
    labels: TimePickerLabels,
    /// Whether to render a shared clear affordance in the right section.
    #[props(default = false)]
    clearable: bool,
    /// Chooses how clear and custom right-section content combine.
    #[props(default)]
    clear_section_mode: TimeInputClearSectionMode,
    /// Callback that receives the mounted hour segment.
    #[props(default)]
    hours_ref: Callback<std::rc::Rc<MountedData>>,
    /// Callback that receives the mounted minute segment.
    #[props(default)]
    minutes_ref: Callback<std::rc::Rc<MountedData>>,
    /// Callback that receives the mounted second segment.
    #[props(default)]
    seconds_ref: Callback<std::rc::Rc<MountedData>>,
    /// Callback that receives the mounted AM/PM segment.
    #[props(default)]
    am_pm_ref: Callback<std::rc::Rc<MountedData>>,
    /// Callback parser used by paste handling.
    #[props(default = Callback::new(dioxus_primitives::time_picker::parse_default_time_picker_value))]
    on_paste_split: Callback<String, Option<TimePickerValue>>,
    /// Callback when focus enters the aggregate time input.
    #[props(default)]
    onfocusin: Callback<Event<FocusData>>,
    /// Callback when focus leaves the aggregate time input.
    #[props(default)]
    onfocusout: Callback<Event<FocusData>>,
    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    roving_loop: ReadSignal<bool>,
    /// Label rendered above the input.
    #[props(default)]
    label: Option<Element>,
    /// Description rendered below the label.
    #[props(default)]
    description: Option<Element>,
    /// Error rendered below the input.
    #[props(default)]
    error: Option<Element>,
    /// Marks the input as required.
    #[props(default = false)]
    required: bool,
    /// Shows the required asterisk without native validation.
    #[props(default = false)]
    with_asterisk: bool,
    /// Visual variant for the shell.
    #[props(default)]
    variant: InputVariant,
    /// Size preset for the shell.
    #[props(default)]
    size: InputSize,
    /// Radius preset for the shell.
    #[props(default)]
    radius: InputRadius,
    /// Optional content rendered before the segmented control.
    #[props(default)]
    left_section: Option<Element>,
    /// Optional content rendered after the segmented control.
    #[props(default)]
    right_section: Option<Element>,
    /// Wrapper attributes.
    #[props(default)]
    wrapper_attributes: Vec<Attribute>,
    /// Shell attributes.
    #[props(default)]
    input_attributes: Vec<Attribute>,
    /// Additional attributes to extend the primitive time picker root.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let is_disabled = disabled();
    let mut open = use_signal(|| false);
    let popover_id = format!("{}-popover", use_time_input_id());
    let clear_labels = labels.clone();
    let on_clear = EventHandler::new(move |_| {
        on_value_change.call(None);
        on_picker_value_change.call(None);
    });
    let right_section = time_input_right_section(
        clearable,
        is_disabled,
        clear_labels,
        clear_section_mode,
        right_section,
        on_clear,
    );
    rsx! {
        InputBase {
            label: element_label(label),
            description,
            error: error.clone(),
            required,
            with_asterisk,
            disabled: is_disabled,
            variant,
            size,
            radius,
            left_section,
            right_section,
            wrapper_attributes,
            input_attributes,
            PopoverRoot {
                is_modal: false,
                open: Some(open()),
                on_open_change: move |value| open.set(value),
                attributes: attributes!(div {
                    class: Styles::dx_time_input_popover_root.to_string()
                }),
                TimePicker {
                    on_value_change,
                    on_picker_value_change,
                    selected_time,
                    selected_value,
                    disabled,
                    read_only,
                    min_time,
                    max_time,
                    with_seconds,
                    picker_type,
                    format,
                    min_hours_digits,
                    steps,
                    am_pm_labels: am_pm_labels.clone(),
                    labels,
                    clearable: false,
                    hours_ref,
                    minutes_ref,
                    seconds_ref,
                    am_pm_ref,
                    on_paste_split,
                    onfocusin: move |event| {
                        if !is_disabled {
                            open.set(true);
                        }
                        onfocusin.call(event);
                    },
                    onfocusout,
                    roving_loop,
                    attributes,
                    StyledTimePickerInput {
                        with_seconds,
                        picker_type,
                        format,
                        popover_id: popover_id.clone(),
                        popover_open: open(),
                    }
                    TimeInputPopoverContent { id: popover_id,
                        crate::components::time_picker::TimePicker {
                            selected_time,
                            selected_value,
                            on_value_change: move |t: Option<Time>| {
                                on_value_change.call(t);
                                on_picker_value_change.call(t.map(TimePickerValue::Time));
                            },
                            on_picker_value_change,
                            on_preset_select: move |t: Time| {
                                on_value_change.call(Some(t));
                                on_picker_value_change.call(Some(TimePickerValue::Time(t)));
                                open.set(false);
                            },
                            disabled,
                            read_only,
                            with_seconds,
                            format,
                            picker_type,
                            min_time,
                            max_time,
                            am_pm_labels: am_pm_labels.clone(),
                            presets: presets.clone(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StyledTimePickerInput(
    with_seconds: bool,
    picker_type: TimePickerType,
    format: TimePickerFormat,
    popover_id: String,
    popover_open: bool,
) -> Element {
    let attributes = use_input_control_context()
        .map(|ctx| {
            attributes!(span {
                id: ctx.id,
                "aria-describedby": ctx.described_by,
                "aria-invalid": ctx.invalid,
                "aria-controls": popover_id,
                "aria-expanded": popover_open,
                "aria-haspopup": "dialog",
            })
        })
        .unwrap_or_default();

    rsx! {
        TimePickerInput {
            attributes: attributes!(div {
                class: Styles::dx_time_input_field.to_string()
            }),
            StyledTimePickerSegments {
                with_seconds,
                picker_type,
                format,
                apply_control_attributes: true,
                control_attributes: attributes,
            }
        }
    }
}

#[component]
fn StyledTimePickerSegments(
    with_seconds: bool,
    picker_type: TimePickerType,
    format: TimePickerFormat,
    apply_control_attributes: bool,
    #[props(default)] control_attributes: Vec<Attribute>,
) -> Element {
    let hour_attributes = apply_control_attributes
        .then_some(control_attributes)
        .unwrap_or_default();

    rsx! {
        TimePickerInputValue {
            TimePickerHourSegment {
                attributes: hour_attributes,
            }
            TimePickerSeparator {}
            TimePickerMinuteSegment {}
            if with_seconds {
                TimePickerSeparator {}
                TimePickerSecondSegment {}
            }
            if format == TimePickerFormat::TwelveHour && picker_type == TimePickerType::Time {
                TimePickerAmPmSegment {}
            }
        }
    }
}

#[component]
fn TimeInputPopoverContent(#[props(default)] id: Option<String>, children: Element) -> Element {
    rsx! {
        PopoverContent {
            id,
            class: Styles::dx_time_input_popover_content.to_string(),
            {children}
        }
    }
}
