use crate::component_styles;
use dioxus::prelude::*;
use dioxus_primitives::time_picker as prim;
use time::{macros::time, Time};

pub use prim::{
    parse_default_time_picker_value, time_picker_preset_range, TimePickerAmPmSegment,
    TimePickerClearButton, TimePickerFormat, TimePickerHourSegment, TimePickerInput,
    TimePickerInputValue, TimePickerLabels, TimePickerMinuteSegment, TimePickerSecondSegment,
    TimePickerSeparator, TimePickerSteps, TimePickerType, TimePickerValue,
};

#[component_styles("./style.css")]
struct Styles;

/// Generate evenly spaced clock-time presets between `start` and `end` at the given `interval`.
pub fn time_range(start: Time, end: Time, interval: time::Duration) -> Vec<Time> {
    let step = interval.whole_seconds().max(0) as u32;
    time_picker_preset_range(start, end, step)
}

fn display_12h_hour(h: u8) -> u32 {
    if h.is_multiple_of(12) {
        12
    } else {
        (h % 12) as u32
    }
}

fn to_24h_hour(display: u32, is_pm: bool) -> u8 {
    let h = (display % 12) as u8;
    if is_pm {
        h + 12
    } else {
        h
    }
}

fn preset_label(
    t: Time,
    format: TimePickerFormat,
    with_seconds: bool,
    am_pm: &(String, String),
) -> String {
    let h = t.hour();
    let m = t.minute();
    let s = t.second();
    match format {
        TimePickerFormat::TwentyFourHour => {
            if with_seconds {
                format!("{h:02}:{m:02}:{s:02}")
            } else {
                format!("{h:02}:{m:02}")
            }
        }
        TimePickerFormat::TwelveHour => {
            let dh = display_12h_hour(h);
            let period = if h < 12 { &am_pm.0 } else { &am_pm.1 };
            if with_seconds {
                format!("{dh}:{m:02}:{s:02} {period}")
            } else {
                format!("{dh}:{m:02} {period}")
            }
        }
    }
}

/// Styled scrolling-column time picker surface.
///
/// In `Time` mode this renders one scrollable column per time segment (hour,
/// minute, optional second, and AM/PM when using the twelve-hour format) plus
/// optional preset quick-select buttons. In `Duration` mode it falls back to the
/// primitive segmented [`TimePickerInput`].
#[component]
pub fn TimePicker(
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
    /// Whether the picker is disabled.
    #[props(default)]
    disabled: ReadSignal<bool>,
    /// Whether the picker is read-only.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    read_only: ReadSignal<bool>,
    /// Lower limit of the selectable time range.
    #[props(default = time!(00:00))]
    min_time: Time,
    /// Upper limit of the selectable time range.
    #[props(default = time!(23:59))]
    max_time: Time,
    /// Include a seconds column.
    #[props(default = false)]
    with_seconds: bool,
    /// Whether the picker edits a clock time or a duration.
    #[props(default = TimePickerType::Time)]
    picker_type: TimePickerType,
    /// Display format for clock time.
    #[props(default = TimePickerFormat::TwentyFourHour)]
    format: TimePickerFormat,
    /// Labels for the AM and PM period options.
    #[props(default = ("AM".to_string(), "PM".to_string()))]
    am_pm_labels: (String, String),
    /// Preset times rendered as quick-select buttons below the columns.
    #[props(default)]
    presets: Vec<Time>,
    /// Optional hook invoked when a preset is selected, in place of the default
    /// `on_value_change` emission (used by `TimeInput` to close its popover).
    #[props(default)]
    on_preset_select: Option<Callback<Time>>,
    /// Step sizes used to generate selectable hour, minute, and second options.
    #[props(default)]
    steps: TimePickerSteps,
    /// Accessibility labels for the picker, its columns, and clear affordance.
    #[props(default)]
    labels: ReadSignal<TimePickerLabels>,
    /// Whether to render a clear affordance after the picker columns.
    #[props(default = false)]
    clearable: bool,
    /// Minimum number of digits displayed for duration hours.
    #[props(default = 2)]
    min_hours_digits: usize,
    /// Callback that receives the mounted hour segment in duration mode.
    #[props(default)]
    hours_ref: Callback<std::rc::Rc<MountedData>>,
    /// Callback that receives the mounted minute segment in duration mode.
    #[props(default)]
    minutes_ref: Callback<std::rc::Rc<MountedData>>,
    /// Callback that receives the mounted second segment in duration mode.
    #[props(default)]
    seconds_ref: Callback<std::rc::Rc<MountedData>>,
    /// Callback that receives the mounted AM/PM segment in duration mode.
    #[props(default)]
    am_pm_ref: Callback<std::rc::Rc<MountedData>>,
    /// Callback parser used by duration-mode paste handling.
    #[props(default = Callback::new(parse_default_time_picker_value))]
    on_paste_split: Callback<String, Option<TimePickerValue>>,
    /// Callback when focus enters the duration-mode aggregate input.
    #[props(default)]
    onfocusin: Callback<Event<FocusData>>,
    /// Callback when focus leaves the duration-mode aggregate input.
    #[props(default)]
    onfocusout: Callback<Event<FocusData>>,
    /// Whether duration-mode segment focus loops at the endpoints.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    roving_loop: ReadSignal<bool>,
    /// Additional attributes to extend the picker root element.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child input composition used by duration mode.
    #[props(default)]
    children: Option<Element>,
) -> Element {
    // Preserve the primitive child-composition contract. The reviewed styled
    // column surface remains the default for clock-time pickers without children.
    if picker_type == TimePickerType::Duration || children.is_some() {
        let children = children.unwrap_or_else(|| rsx! { prim::TimePickerInput {} });
        return rsx! {
            prim::TimePicker {
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
                am_pm_labels,
                steps,
                labels: labels(),
                clearable,
                min_hours_digits,
                hours_ref,
                minutes_ref,
                seconds_ref,
                am_pm_ref,
                on_paste_split,
                onfocusin,
                onfocusout,
                roving_loop,
                attributes,
                {children}
            }
        };
    }

    let effective_min = if with_seconds {
        min_time
    } else {
        Time::from_hms(min_time.hour(), min_time.minute(), 0).expect("valid minimum time")
    };
    let effective_max = if with_seconds {
        max_time
    } else {
        Time::from_hms(max_time.hour(), max_time.minute(), 0).expect("valid maximum time")
    };
    let current = selected_value().or_else(|| selected_time().map(TimePickerValue::Time));
    let (cur_hour, cur_minute, cur_second, cur_period) = match current {
        Some(TimePickerValue::Time(t)) => {
            let h24 = t.hour();
            let period = (h24 >= 12) as u32;
            let display_h = if format == TimePickerFormat::TwelveHour {
                display_12h_hour(h24)
            } else {
                h24 as u32
            };
            (
                Some(display_h),
                Some(t.minute() as u32),
                Some(if with_seconds { t.second() as u32 } else { 0 }),
                Some(period),
            )
        }
        _ => (None, None, None, None),
    };
    let is_twelve = format == TimePickerFormat::TwelveHour;
    let default_hour = if is_twelve { 12 } else { 0 };
    let emit = move |h_display: u32, m: u32, s: u32, period: u32| {
        let h24 = if is_twelve {
            to_24h_hour(h_display, period == 1)
        } else {
            h_display as u8
        };
        let second = if with_seconds { s as u8 } else { 0 };
        if let Ok(t) = Time::from_hms(h24, m as u8, second) {
            if t >= effective_min && t <= effective_max {
                on_value_change.call(Some(t));
                on_picker_value_change.call(Some(TimePickerValue::Time(t)));
            }
        }
    };

    let hour_step = steps.hours.max(1) as usize;
    let minute_step = steps.minutes.max(1) as usize;
    let second_step = steps.seconds.max(1) as usize;
    let mut hour_items: Vec<(u32, String)> = if is_twelve {
        (1..=12u32)
            .step_by(hour_step)
            .map(|h| (h, format!("{h:02}")))
            .collect()
    } else {
        (0..=23u32)
            .step_by(hour_step)
            .map(|h| (h, format!("{h:02}")))
            .collect()
    };
    if let Some(hour) = cur_hour {
        hour_items.push((hour, format!("{hour:02}")));
        hour_items.sort_by_key(|item| item.0);
        hour_items.dedup_by_key(|item| item.0);
    }
    let mut minute_items: Vec<(u32, String)> = (0..=59u32)
        .step_by(minute_step)
        .map(|m| (m, format!("{m:02}")))
        .collect();
    if let Some(minute) = cur_minute {
        minute_items.push((minute, format!("{minute:02}")));
        minute_items.sort_by_key(|item| item.0);
        minute_items.dedup_by_key(|item| item.0);
    }
    let mut second_items: Vec<(u32, String)> = (0..=59u32)
        .step_by(second_step)
        .map(|s| (s, format!("{s:02}")))
        .collect();
    if let Some(second) = cur_second {
        second_items.push((second, format!("{second:02}")));
        second_items.sort_by_key(|item| item.0);
        second_items.dedup_by_key(|item| item.0);
    }
    let period_items: Vec<(u32, String)> = vec![
        (0u32, am_pm_labels.0.clone()),
        (1u32, am_pm_labels.1.clone()),
    ];
    let labels_value = labels();
    let option_disabled = move |h_display: u32, m: u32, s: u32, period: u32| {
        let h24 = if is_twelve {
            to_24h_hour(h_display, period == 1)
        } else {
            h_display as u8
        };
        Time::from_hms(h24, m as u8, if with_seconds { s as u8 } else { 0 })
            .map(|time| time < effective_min || time > effective_max)
            .unwrap_or(true)
    };
    let hour_disabled: Vec<u32> = hour_items
        .iter()
        .map(|item| item.0)
        .filter(|value| {
            option_disabled(
                *value,
                cur_minute.unwrap_or(0),
                cur_second.unwrap_or(0),
                cur_period.unwrap_or(0),
            )
        })
        .collect();
    let minute_disabled: Vec<u32> = minute_items
        .iter()
        .map(|item| item.0)
        .filter(|value| {
            option_disabled(
                cur_hour.unwrap_or(default_hour),
                *value,
                cur_second.unwrap_or(0),
                cur_period.unwrap_or(0),
            )
        })
        .collect();
    let second_disabled: Vec<u32> = second_items
        .iter()
        .map(|item| item.0)
        .filter(|value| {
            option_disabled(
                cur_hour.unwrap_or(default_hour),
                cur_minute.unwrap_or(0),
                *value,
                cur_period.unwrap_or(0),
            )
        })
        .collect();
    let period_disabled: Vec<u32> = period_items
        .iter()
        .map(|item| item.0)
        .filter(|value| {
            option_disabled(
                cur_hour.unwrap_or(default_hour),
                cur_minute.unwrap_or(0),
                cur_second.unwrap_or(0),
                *value,
            )
        })
        .collect();

    rsx! {
        div {
            class: Styles::dx_time_picker,
            "data-disabled": disabled(),
            role: "group",
            aria_label: labels_value.group.clone(),
            aria_disabled: disabled(),
            aria_readonly: read_only(),
            "data-readonly": read_only(),
            ..attributes,
            div { class: Styles::dx_time_picker_columns,
                TimePickerColumnWidget {
                    label: labels_value.hour.clone(),
                    items: hour_items,
                    selected: cur_hour,
                    disabled: disabled(),
                    disabled_values: hour_disabled,
                    read_only: read_only(),
                    on_select: move |value| emit(
                        value,
                        cur_minute.unwrap_or(0),
                        cur_second.unwrap_or(0),
                        cur_period.unwrap_or(0),
                    ),
                }
                TimePickerColumnWidget {
                    label: labels_value.minute.clone(),
                    items: minute_items,
                    selected: cur_minute,
                    disabled: disabled(),
                    disabled_values: minute_disabled,
                    read_only: read_only(),
                    on_select: move |value| emit(
                        cur_hour.unwrap_or(default_hour),
                        value,
                        cur_second.unwrap_or(0),
                        cur_period.unwrap_or(0),
                    ),
                }
                if with_seconds {
                    TimePickerColumnWidget {
                        label: labels_value.second.clone(),
                        items: second_items,
                        selected: cur_second,
                        disabled: disabled(),
                        disabled_values: second_disabled,
                        read_only: read_only(),
                        on_select: move |value| emit(
                            cur_hour.unwrap_or(default_hour),
                            cur_minute.unwrap_or(0),
                            value,
                            cur_period.unwrap_or(0),
                        ),
                    }
                }
                if is_twelve {
                    TimePickerColumnWidget {
                        label: labels_value.am_pm.clone(),
                        items: period_items,
                        selected: cur_period,
                        disabled: disabled(),
                        disabled_values: period_disabled,
                        read_only: read_only(),
                        on_select: move |value| emit(
                            cur_hour.unwrap_or(default_hour),
                            cur_minute.unwrap_or(0),
                            cur_second.unwrap_or(0),
                            value,
                        ),
                    }
                }
            }
            if !presets.is_empty() {
                div { class: Styles::dx_time_picker_presets,
                    for preset in presets {
                        button {
                            key: "{preset}",
                            class: Styles::dx_time_picker_preset_btn,
                            r#type: "button",
                            disabled: disabled() || read_only() || preset < effective_min || preset > effective_max,
                            onclick: move |_| {
                                if let Some(cb) = &on_preset_select {
                                    cb.call(preset);
                                } else {
                                    on_value_change.call(Some(preset));
                                    on_picker_value_change.call(Some(TimePickerValue::Time(preset)));
                                }
                            },
                            { preset_label(preset, format, with_seconds, &am_pm_labels) }
                        }
                    }
                }
            }
            if clearable {
                button {
                    class: Styles::dx_time_picker_preset_btn,
                    r#type: "button",
                    aria_label: labels_value.clear,
                    disabled: disabled() || read_only() || current.is_none(),
                    onclick: move |_| {
                        on_value_change.call(None);
                        on_picker_value_change.call(None);
                    },
                    "Clear"
                }
            }
        }
    }
}

#[component]
fn TimePickerColumnWidget(
    /// Accessible label shown atop the column.
    label: String,
    /// Selectable `(value, display)` pairs.
    items: Vec<(u32, String)>,
    /// Currently selected value, if any.
    selected: Option<u32>,
    /// Callback invoked with the selected value.
    on_select: Callback<u32>,
    /// Whether the column is disabled.
    disabled: bool,
    /// Whether the column is read-only rather than disabled.
    read_only: bool,
    /// Values outside the currently selectable range.
    disabled_values: Vec<u32>,
) -> Element {
    rsx! {
        div {
            class: Styles::dx_time_picker_col,
            role: "listbox",
            aria_label: label.clone(),
            aria_disabled: disabled,
            aria_readonly: read_only,
            span { class: Styles::dx_time_picker_col_label, {label.clone()} }
            for (value, display) in items {
                {
                    let is_selected = selected == Some(value);
                    let is_disabled = disabled || read_only || disabled_values.contains(&value);
                    rsx! {
                        button {
                            key: "{value}",
                            class: Styles::dx_time_picker_col_item,
                            role: "option",
                            aria_selected: is_selected,
                            "data-selected": is_selected,
                            disabled: is_disabled,
                            onclick: move |_| {
                                if !is_disabled {
                                    on_select.call(value);
                                }
                            },
                            onmounted: move |event| {
                                if is_selected {
                                    let data = event.data();
                                    spawn(async move {
                                        data.scroll_to(ScrollBehavior::Instant).await.ok();
                                    });
                                }
                            },
                            {display}
                        }
                    }
                }
            }
        }
    }
}
