use dioxus::prelude::*;
use dioxus_primitives::time_picker as prim;
use time::{macros::time, Time};

pub use prim::{
    parse_default_time_picker_value, time_picker_preset_range, TimePickerAmPmSegment,
    TimePickerClearButton, TimePickerFormat, TimePickerHourSegment, TimePickerInput,
    TimePickerInputValue, TimePickerLabels, TimePickerMinuteSegment, TimePickerSecondSegment,
    TimePickerSeparator, TimePickerSteps, TimePickerType, TimePickerValue,
};

#[css_module("/src/components/time_picker/style.css")]
struct Styles;

/// Generate evenly spaced clock-time presets between `start` and `end` at the given `interval`.
pub fn time_range(start: Time, end: Time, interval: time::Duration) -> Vec<Time> {
    let step = interval.whole_seconds().max(0) as u32;
    time_picker_preset_range(start, end, step)
}

fn display_12h_hour(h: u8) -> u32 {
    if h % 12 == 0 {
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
    /// Additional attributes to extend the picker root element.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    if picker_type == TimePickerType::Duration {
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
                attributes,
                prim::TimePickerInput {}
            }
        };
    }

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
                Some(t.second() as u32),
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
        if let Ok(t) = Time::from_hms(h24, m as u8, s as u8) {
            on_value_change.call(Some(t));
        }
    };

    let hour_items: Vec<(u32, String)> = if is_twelve {
        (1..=12u32).map(|h| (h, format!("{h:02}"))).collect()
    } else {
        (0..=23u32).map(|h| (h, format!("{h:02}"))).collect()
    };
    let minute_items: Vec<(u32, String)> = (0..=59u32).map(|m| (m, format!("{m:02}"))).collect();
    let second_items: Vec<(u32, String)> = (0..=59u32).map(|s| (s, format!("{s:02}"))).collect();
    let period_items: Vec<(u32, String)> = vec![
        (0u32, am_pm_labels.0.clone()),
        (1u32, am_pm_labels.1.clone()),
    ];

    rsx! {
        div {
            class: Styles::dx_time_picker,
            "data-disabled": disabled(),
            ..attributes,
            div { class: Styles::dx_time_picker_columns,
                TimePickerColumnWidget {
                    label: "Hr".to_string(),
                    items: hour_items,
                    selected: cur_hour,
                    disabled: disabled() || read_only(),
                    on_select: move |v| emit(
                        v,
                        cur_minute.unwrap_or(0),
                        cur_second.unwrap_or(0),
                        cur_period.unwrap_or(0),
                    ),
                }
                TimePickerColumnWidget {
                    label: "Min".to_string(),
                    items: minute_items,
                    selected: cur_minute,
                    disabled: disabled() || read_only(),
                    on_select: move |v| emit(
                        cur_hour.unwrap_or(default_hour),
                        v,
                        cur_second.unwrap_or(0),
                        cur_period.unwrap_or(0),
                    ),
                }
                if with_seconds {
                    TimePickerColumnWidget {
                        label: "Sec".to_string(),
                        items: second_items,
                        selected: cur_second,
                        disabled: disabled() || read_only(),
                        on_select: move |v| emit(
                            cur_hour.unwrap_or(default_hour),
                            cur_minute.unwrap_or(0),
                            v,
                            cur_period.unwrap_or(0),
                        ),
                    }
                }
                if is_twelve {
                    TimePickerColumnWidget {
                        label: "AM/PM".to_string(),
                        items: period_items,
                        selected: cur_period,
                        disabled: disabled() || read_only(),
                        on_select: move |v| emit(
                            cur_hour.unwrap_or(default_hour),
                            cur_minute.unwrap_or(0),
                            cur_second.unwrap_or(0),
                            v,
                        ),
                    }
                }
            }
            if !presets.is_empty() {
                div { class: Styles::dx_time_picker_presets,
                    for preset in presets.iter().copied() {
                        button {
                            class: Styles::dx_time_picker_preset_btn,
                            disabled: disabled() || read_only(),
                            onclick: move |_| {
                                if let Some(cb) = &on_preset_select {
                                    cb.call(preset);
                                } else {
                                    on_value_change.call(Some(preset));
                                }
                            },
                            { preset_label(preset, format, with_seconds, &am_pm_labels) }
                        }
                    }
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
    /// Whether the column is interactive.
    disabled: bool,
) -> Element {
    let aria_label = label.clone();
    rsx! {
        div {
            class: Styles::dx_time_picker_col,
            role: "listbox",
            "aria-label": aria_label,
            span { class: Styles::dx_time_picker_col_label, {label} }
            for (value, display) in items {
                {
                    let is_selected = selected == Some(value);
                    rsx! {
                        button {
                            key: "{value}",
                            class: Styles::dx_time_picker_col_item,
                            role: "option",
                            "aria-selected": is_selected,
                            "data-selected": is_selected,
                            disabled,
                            onclick: move |_| {
                                if !disabled {
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
