//! Defines the [`TimePicker`] component and its subcomponents.

use std::rc::Rc;

use crate::{
    focus::{use_focus_controlled_item_disabled, use_focus_provider, FocusState},
    use_unique_id,
};

use dioxus::prelude::*;
use time::{macros::time, Time};

/// A value emitted by [`TimePicker`] when it is configured beyond plain clock time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimePickerValue {
    /// A normal 24-hour clock time.
    Time(Time),
    /// A duration value. Duration hours are intentionally not bounded by
    /// [`Time`]'s 0-23 hour range.
    Duration {
        /// Duration hours.
        hours: u32,
        /// Duration minutes.
        minutes: u8,
        /// Duration seconds.
        seconds: u8,
    },
}

impl TimePickerValue {
    fn from_time(time: Time, precision: TimePrecision) -> Self {
        Self::Time(normalize_time_precision(time, precision))
    }

    fn as_time(self) -> Option<Time> {
        match self {
            Self::Time(time) => Some(time),
            Self::Duration { .. } => None,
        }
    }
}

/// The editable value kind for [`TimePicker`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimePickerType {
    /// A normal 24-hour clock time.
    Time,
    /// A duration with unbounded hours.
    Duration,
}

/// Display format for normal clock times.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimePickerFormat {
    /// 24-hour time.
    TwentyFourHour,
    /// 12-hour time with an AM/PM segment.
    TwelveHour,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TimePrecision {
    Minute,
    Second,
}

fn normalize_time_precision(time: Time, precision: TimePrecision) -> Time {
    match precision {
        TimePrecision::Minute => floor_to_minute(time),
        TimePrecision::Second => {
            Time::from_hms(time.hour(), time.minute(), time.second()).expect("valid time")
        }
    }
}

/// Accessibility labels used by the primitive time picker segments and actions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimePickerLabels {
    /// Label for the root time input group.
    pub group: String,
    /// Label for the hour segment.
    pub hour: String,
    /// Label for the minute segment.
    pub minute: String,
    /// Label for the second segment.
    pub second: String,
    /// Label for the AM/PM segment.
    pub am_pm: String,
    /// Label for the clear affordance.
    pub clear: String,
}

impl Default for TimePickerLabels {
    fn default() -> Self {
        Self {
            group: "Time".to_string(),
            hour: "hour".to_string(),
            minute: "minute".to_string(),
            second: "second".to_string(),
            am_pm: "AM/PM".to_string(),
            clear: "Clear time".to_string(),
        }
    }
}

/// Step sizes used by keyboard increment/decrement behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimePickerSteps {
    /// Hour step.
    pub hours: u32,
    /// Minute step.
    pub minutes: u32,
    /// Second step.
    pub seconds: u32,
}

impl Default for TimePickerSteps {
    fn default() -> Self {
        Self {
            hours: 1,
            minutes: 1,
            seconds: 1,
        }
    }
}

fn has_subsecond_precision(time: Time) -> bool {
    time.nanosecond() != 0
}

fn floor_to_minute(time: Time) -> Time {
    Time::from_hms(time.hour(), time.minute(), 0).expect("valid time")
}

fn ceil_to_minute(time: Time) -> Option<Time> {
    if time.second() == 0 && time.nanosecond() == 0 {
        return Some(time);
    }

    if time.hour() == 23 && time.minute() == 59 {
        return None;
    }

    let total_minutes = (time.hour() as u16) * 60 + (time.minute() as u16) + 1;
    let hour = (total_minutes / 60) as u8;
    let minute = (total_minutes % 60) as u8;
    Some(Time::from_hms(hour, minute, 0).expect("valid time"))
}

fn ceil_to_second(time: Time) -> Option<Time> {
    if !has_subsecond_precision(time) {
        return Some(time);
    }

    if time.hour() == 23 && time.minute() == 59 && time.second() == 59 {
        return None;
    }

    let total_seconds =
        (time.hour() as u32) * 3600 + (time.minute() as u32) * 60 + time.second() as u32 + 1;
    let hour = (total_seconds / 3600) as u8;
    let minute = ((total_seconds % 3600) / 60) as u8;
    let second = (total_seconds % 60) as u8;
    Some(Time::from_hms(hour, minute, second).expect("valid time"))
}

fn minute_precision_eq(left: Option<Time>, right: Option<Time>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.hour() == right.hour() && left.minute() == right.minute(),
        (None, None) => true,
        _ => false,
    }
}

fn second_precision_eq(left: Option<Time>, right: Option<Time>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => {
            left.hour() == right.hour()
                && left.minute() == right.minute()
                && left.second() == right.second()
        }
        (None, None) => true,
        _ => false,
    }
}

fn value_precision_eq(
    left: Option<TimePickerValue>,
    right: Option<TimePickerValue>,
    precision: TimePrecision,
) -> bool {
    match (left, right) {
        (Some(TimePickerValue::Time(left)), Some(TimePickerValue::Time(right))) => {
            match precision {
                TimePrecision::Minute => minute_precision_eq(Some(left), Some(right)),
                TimePrecision::Second => second_precision_eq(Some(left), Some(right)),
            }
        }
        (Some(TimePickerValue::Duration { .. }), Some(TimePickerValue::Duration { .. })) => {
            left == right
        }
        (None, None) => true,
        _ => false,
    }
}

fn step_segment(value: u32, min: u32, max: u32, step: u32, direction: i8) -> u32 {
    let step = step.max(1);
    if min >= max {
        return min;
    }

    match direction {
        1 => {
            let next = value.saturating_add(step);
            if next > max {
                min
            } else {
                next
            }
        }
        -1 => {
            if value <= min || value - min < step {
                max
            } else {
                value - step
            }
        }
        _ => value,
    }
}

fn twelve_hour_display(hour: u8) -> u32 {
    match hour % 12 {
        0 => 12,
        hour => hour as u32,
    }
}

fn to_24_hour(hour: u32, period: AmPm) -> u8 {
    let hour = (hour % 12) as u8;
    match period {
        AmPm::Am => hour,
        AmPm::Pm => hour + 12,
    }
}

/// Default parser used by the primitive paste hook.
pub fn parse_default_time_picker_value(text: String) -> Option<TimePickerValue> {
    let parts: Vec<_> = text
        .trim()
        .split(|character: char| character == ':' || character.is_ascii_whitespace())
        .filter(|part| !part.is_empty())
        .collect();
    if parts.len() < 2 || parts.len() > 4 {
        return None;
    }
    if parts.len() == 4 && parts[2].parse::<u8>().is_err() {
        return None;
    }

    let mut hour = parts[0].parse::<u32>().ok()?;
    let minute = parts[1].parse::<u8>().ok()?;
    let (second, period) = match parts.get(2) {
        Some(part) if part.parse::<u8>().is_ok() => (part.parse::<u8>().ok()?, parts.get(3)),
        Some(period) => (0, Some(period)),
        None => (0, None),
    };
    if minute > 59 || second > 59 {
        return None;
    }

    if let Some(period) = period {
        match period.to_ascii_lowercase().as_str() {
            "am" => {
                if hour == 12 {
                    hour = 0;
                }
            }
            "pm" => {
                if hour < 12 {
                    hour = hour.checked_add(12)?;
                }
            }
            _ => return None,
        }
    }

    if hour > 23 {
        return Some(TimePickerValue::Duration {
            hours: hour,
            minutes: minute,
            seconds: second,
        });
    }

    Time::from_hms(hour as u8, minute, second)
        .ok()
        .map(TimePickerValue::Time)
}

/// Generate evenly spaced clock-time presets between `start` and `end`, inclusive.
///
/// Values are rounded to whole seconds because [`TimePicker`] emits either minute
/// or second precision values. A zero `step_seconds` returns an empty list.
pub fn time_picker_preset_range(start: Time, end: Time, step_seconds: u32) -> Vec<Time> {
    if step_seconds == 0 || start > end {
        return Vec::new();
    }

    let start_seconds =
        (start.hour() as u32) * 3600 + (start.minute() as u32) * 60 + start.second() as u32;
    let end_seconds = (end.hour() as u32) * 3600 + (end.minute() as u32) * 60 + end.second() as u32;

    let mut values = Vec::new();
    let mut current = start_seconds;
    while current <= end_seconds {
        let hour = (current / 3600) as u8;
        let minute = ((current % 3600) / 60) as u8;
        let second = (current % 60) as u8;
        values.push(Time::from_hms(hour, minute, second).expect("valid time"));
        current = current.saturating_add(step_seconds);
        if current == u32::MAX {
            break;
        }
    }

    values
}

#[derive(Copy, Clone)]
struct TimePickerContext {
    selected_time: ReadSignal<Option<Time>>,
    selected_value: ReadSignal<Option<TimePickerValue>>,
    on_value_change: Callback<Option<Time>>,
    on_picker_value_change: Callback<Option<TimePickerValue>>,
    disabled: ReadSignal<bool>,
    read_only: ReadSignal<bool>,
    focus: FocusState,
    picker_type: TimePickerType,
    format: TimePickerFormat,
    precision: TimePrecision,
    steps: TimePickerSteps,
    min_hours_digits: usize,
    labels: Memo<TimePickerLabels>,
    hours_ref: Callback<Rc<MountedData>>,
    minutes_ref: Callback<Rc<MountedData>>,
    seconds_ref: Callback<Rc<MountedData>>,
    am_pm_ref: Callback<Rc<MountedData>>,
    #[allow(dead_code)]
    on_paste_split: Callback<String, Option<TimePickerValue>>,
}

impl TimePickerContext {
    fn current_value(&self) -> Option<TimePickerValue> {
        self.selected_value.peek().cloned().or_else(|| {
            self.selected_time
                .peek()
                .cloned()
                .map(|time| TimePickerValue::from_time(time, self.precision))
        })
    }

    /// Reactively reads the controlled value so effects re-run when the
    /// `selected_value`/`selected_time` props change externally.
    ///
    /// Unlike [`current_value`], this subscribes to the underlying signals; use
    /// it inside effects that must resync internal segment state with external
    /// updates (e.g. selecting a value from a column picker).
    fn reactive_current_value(&self) -> Option<TimePickerValue> {
        self.selected_value.read().clone().or_else(|| {
            self.selected_time
                .read()
                .clone()
                .map(|time| TimePickerValue::from_time(time, self.precision))
        })
    }

    fn set_value(&mut self, value: Option<TimePickerValue>) {
        if !value_precision_eq(self.current_value(), value, self.precision) {
            self.on_picker_value_change.call(value);
            self.on_value_change
                .call(value.and_then(TimePickerValue::as_time));
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AmPm {
    Am,
    Pm,
}

impl AmPm {
    fn from_hour(hour: u8) -> Self {
        if hour < 12 {
            Self::Am
        } else {
            Self::Pm
        }
    }

    fn index(self) -> u32 {
        match self {
            Self::Am => 0,
            Self::Pm => 1,
        }
    }

    fn from_index(index: u32) -> Self {
        if index == 0 {
            Self::Am
        } else {
            Self::Pm
        }
    }
}

/// The props for the [`TimePicker`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TimePickerProps {
    /// Callback when the selected time changes.
    #[props(default)]
    pub on_value_change: Callback<Option<Time>>,

    /// Callback when the selected value changes.
    ///
    /// This receives [`TimePickerValue::Duration`] in duration mode and mirrors
    /// normal clock times as [`TimePickerValue::Time`].
    #[props(default)]
    pub on_picker_value_change: Callback<Option<TimePickerValue>>,

    /// The selected time.
    ///
    /// The time picker operates at minute precision. Displayed and emitted values
    /// always use `second = 0` and `nanosecond = 0`. When this prop includes
    /// sub-minute precision, the UI ignores it and preserves the existing value
    /// until the user changes the hour or minute.
    #[props(default)]
    pub selected_time: ReadSignal<Option<Time>>,

    /// The selected value.
    ///
    /// Prefer `selected_time` for normal clock-time usage. This value model is
    /// available for duration mode, where [`Time`] cannot represent hours
    /// beyond 23.
    #[props(default)]
    pub selected_value: ReadSignal<Option<TimePickerValue>>,

    /// Whether the time picker is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether the time picker is read-only.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub read_only: ReadSignal<bool>,

    /// Lower limit of the selectable time range.
    ///
    /// Bounds are enforced at minute precision. If this includes sub-minute
    /// precision, the effective lower bound is rounded up to the next minute.
    #[props(default = time!(00:00))]
    pub min_time: Time,

    /// Upper limit of the selectable time range.
    ///
    /// Bounds are enforced at minute precision. If this includes sub-minute
    /// precision, the effective upper bound is rounded down to the current minute.
    #[props(default = time!(23:59))]
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

    /// Step sizes for ArrowUp and ArrowDown keyboard editing.
    #[props(default)]
    pub steps: TimePickerSteps,

    /// Labels for the AM and PM period options.
    #[props(default = ("AM".to_string(), "PM".to_string()))]
    pub am_pm_labels: (String, String),

    /// Accessibility labels for segments and clear affordances.
    #[props(default)]
    pub labels: TimePickerLabels,

    /// Whether the primitive renders a clear affordance after the input value.
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

    /// Callback parser used by paste handling.
    #[props(default = Callback::new(parse_default_time_picker_value))]
    pub on_paste_split: Callback<String, Option<TimePickerValue>>,

    /// Callback when focus enters the aggregate time input.
    #[props(default)]
    pub onfocusin: Callback<Event<FocusData>>,

    /// Callback when focus leaves the aggregate time input.
    #[props(default)]
    pub onfocusout: Callback<Event<FocusData>>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub roving_loop: ReadSignal<bool>,

    /// Additional attributes to extend the time picker element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the time picker element.
    pub children: Element,
}

/// # TimePicker
///
/// The [`TimePicker`] component provides an accessible segmented time input.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::time_picker::*;
/// use time::Time;
///
/// #[component]
/// fn Demo() -> Element {
///     let mut selected_time = use_signal(|| None::<Time>);
///     rsx! {
///         TimePicker {
///             selected_time: selected_time(),
///             on_value_change: move |time| selected_time.set(time),
///             TimePickerInput {}
///         }
///     }
/// }
/// ```
///
/// # Styling
///
/// The [`TimePicker`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the TimePicker is disabled. Possible values are `true` or `false`.
#[component]
pub fn TimePicker(props: TimePickerProps) -> Element {
    let focus = use_focus_provider(props.roving_loop);
    let precision = if props.with_seconds {
        TimePrecision::Second
    } else {
        TimePrecision::Minute
    };
    let min_time = match precision {
        TimePrecision::Minute => ceil_to_minute(props.min_time).unwrap_or(time!(23:59)),
        TimePrecision::Second => ceil_to_second(props.min_time).unwrap_or(time!(23:59:59)),
    };
    let max_time = normalize_time_precision(props.max_time, precision);
    let min_time = min_time.min(max_time);
    let labels = props.labels.clone();
    let labels = use_memo(move || labels.clone());

    use_context_provider(|| TimePickerContext {
        selected_time: props.selected_time,
        selected_value: props.selected_value,
        on_value_change: props.on_value_change,
        on_picker_value_change: props.on_picker_value_change,
        disabled: props.disabled,
        read_only: props.read_only,
        focus,
        picker_type: props.picker_type,
        format: props.format,
        precision,
        steps: props.steps,
        min_hours_digits: props.min_hours_digits,
        labels,
        hours_ref: props.hours_ref,
        minutes_ref: props.minutes_ref,
        seconds_ref: props.seconds_ref,
        am_pm_ref: props.am_pm_ref,
        on_paste_split: props.on_paste_split,
    });

    use_context_provider(|| TimeElementBounds { min_time, max_time });
    use_context_provider(|| props.am_pm_labels.clone());

    let clear_button = props.clearable.then(|| rsx! { TimePickerClearButton {} });

    rsx! {
        div {
            role: "group",
            aria_label: labels().group,
            "data-disabled": (props.disabled)(),
            onfocusin: move |event| props.onfocusin.call(event),
            onfocusout: move |event| props.onfocusout.call(event),
            ..props.attributes,
            {props.children}
            {clear_button}
        }
    }
}

#[derive(Clone, Copy)]
struct TimeElementBounds {
    min_time: Time,
    max_time: Time,
}

#[derive(Clone, Copy)]
struct TimeElementContext {
    hour_value: Signal<Option<u32>>,
    minute_value: Signal<Option<u32>>,
    second_value: Signal<Option<u32>>,
    period_value: Signal<Option<u32>>,
    on_format_hour_placeholder: Callback<(), String>,
    on_format_minute_placeholder: Callback<(), String>,
    on_format_second_placeholder: Callback<(), String>,
}

#[derive(Props, Clone, PartialEq)]
struct TimeSegmentProps {
    /// The index of the segment.
    pub index: ReadSignal<usize>,

    /// The controlled segment value.
    pub value: ReadSignal<Option<u32>>,

    /// Default value used by arrow-key editing when the segment is empty.
    pub default: u32,

    /// Callback when the segment value changes.
    #[props(default)]
    pub on_value_change: Callback<Option<u32>>,

    /// The minimum segment value.
    pub min: ReadSignal<u32>,

    /// The maximum segment value.
    pub max: ReadSignal<u32>,

    /// Keyboard increment/decrement step.
    #[props(default = 1)]
    pub step: u32,

    /// Callback that receives the mounted segment.
    #[props(default)]
    pub segment_ref: Callback<Rc<MountedData>>,

    /// Minimum number of digits to render.
    #[props(default = 2)]
    pub min_digits: usize,

    /// Callback when display placeholder.
    pub on_format_placeholder: Callback<(), String>,

    /// Additional attributes for the value element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
fn TimeSegment(props: TimeSegmentProps) -> Element {
    let mut text_value = use_signal(|| "".to_string());
    use_effect(move || {
        let text = match (props.value)() {
            Some(value) => value.to_string(),
            None => String::default(),
        };
        text_value.set(text);
    });

    let mut reset_value = use_signal(|| false);
    let display_value = use_memo(move || match (props.value)() {
        Some(value) => format!("{value:0width$}", width = props.min_digits),
        None => props
            .on_format_placeholder
            .call(())
            .repeat(props.min_digits),
    });
    let now_value = use_memo(move || (props.value)().unwrap_or(props.default));
    let mut ctx = use_context::<TimePickerContext>();
    let is_blocked = move || (ctx.disabled)() || (ctx.read_only)();

    let mut set_value = move |text: String| {
        if text.is_empty() {
            props.on_value_change.call(None);
            ctx.focus.focus_prev();
            return;
        }

        let min = props.min.cloned();
        let max = props.max.cloned();
        let value = text.parse::<u32>().map(|v| v.min(max)).ok();

        if let Some(value) = value {
            let in_range = value >= min && value <= max;
            let new_value = (text + "0").parse::<u32>().unwrap_or(value);
            if in_range && new_value > max {
                ctx.focus.focus_next();
            }
        }

        props.on_value_change.call(value);
    };

    use_effect(move || {
        if !ctx.focus.is_focused(props.index.cloned()) {
            if let Some(value) = (props.value)() {
                let clamped_value = value.clamp(props.min.cloned(), props.max.cloned());
                if clamped_value != value {
                    props.on_value_change.call(Some(clamped_value));
                }
            }
        }
    });

    let step_up = move |value: u32| {
        step_segment(value, props.min.cloned(), props.max.cloned(), props.step, 1)
    };

    let step_down = move |value: u32| {
        step_segment(
            value,
            props.min.cloned(),
            props.max.cloned(),
            props.step,
            -1,
        )
    };

    let handle_keydown = move |event: Event<KeyboardData>| {
        if is_blocked() {
            event.prevent_default();
            event.stop_propagation();
            return;
        }
        let key = event.key();
        match key {
            Key::Character(actual_char) => {
                if event.modifiers().ctrl() || event.modifiers().meta() || event.modifiers().alt() {
                    return;
                }
                if actual_char.parse::<u8>().is_ok() {
                    let mut text = text_value();
                    if text.len() == props.min_digits || reset_value() {
                        text = String::default();
                        reset_value.set(false);
                    }
                    text.push_str(&actual_char);
                    set_value(text);
                }
                event.prevent_default();
                event.stop_propagation();
            }
            Key::Backspace => {
                let mut text = text_value();
                if event.modifiers().ctrl() || event.modifiers().meta() {
                    text.clear();
                } else {
                    text.pop();
                }
                set_value(text);
            }
            Key::Delete => {
                let mut text = text_value();
                if !text.is_empty() {
                    text.remove(0);
                }
                set_value(text);
            }
            Key::ArrowLeft => ctx.focus.focus_prev(),
            Key::ArrowRight | Key::Enter => {
                ctx.focus.focus_next();
                event.prevent_default();
                event.stop_propagation();
            }
            Key::ArrowUp => {
                let value = match (props.value)() {
                    Some(value) => step_up(value),
                    None => props.default,
                };
                props.on_value_change.call(Some(value));
            }
            Key::ArrowDown => {
                let value = match (props.value)() {
                    Some(value) => step_down(value),
                    None => props.default,
                };
                props.on_value_change.call(Some(value));
            }
            Key::Home => {
                props.on_value_change.call(Some(props.min.cloned()));
                event.prevent_default();
                event.stop_propagation();
            }
            Key::End => {
                props.on_value_change.call(Some(props.max.cloned()));
                event.prevent_default();
                event.stop_propagation();
            }
            _ => (),
        }
    };

    let disabled = move || (ctx.disabled)();
    let mut focus_onmounted = use_focus_controlled_item_disabled(props.index, disabled);
    let segment_ref = props.segment_ref;
    let tab_index = if disabled() { "-1" } else { "0" };

    let span_id = use_unique_id();
    let id = use_memo(move || format!("span-{span_id}"));
    let label_id = format!("{id}-label");

    rsx! {
        span {
            id,
            role: "spinbutton",
            aria_valuemin: props.min.to_string(),
            aria_valuemax: props.max.to_string(),
            aria_valuenow: now_value.to_string(),
            aria_labelledby: "{label_id}",
            aria_disabled: disabled(),
            aria_readonly: (ctx.read_only)(),
            inputmode: "numeric",
            contenteditable: !(ctx.read_only)(),
            spellcheck: false,
            tabindex: tab_index,
            enterkeyhint: "next",
            onkeydown: handle_keydown,
            onmounted: move |event| {
                let data = event.data();
                segment_ref.call(data.clone());
                focus_onmounted(event);
            },
            onfocus: move |_| {
                reset_value.set(true);
                ctx.focus.set_focus(Some(props.index.cloned()));
            },
            "no-time": (props.value)().is_none(),
            "data-disabled": (ctx.disabled)(),
            ..props.attributes,
            {display_value}
        }
    }
}

/// The props for the [`TimePickerHourSegment`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TimePickerHourSegmentProps {
    /// Additional attributes for the hour segment element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// The props for the [`TimePickerMinuteSegment`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TimePickerMinuteSegmentProps {
    /// Additional attributes for the minute segment element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// The props for the [`TimePickerSecondSegment`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TimePickerSecondSegmentProps {
    /// Additional attributes for the second segment element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// The props for the [`TimePickerAmPmSegment`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TimePickerAmPmSegmentProps {
    /// Additional attributes for the AM/PM segment element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// The props for the [`TimePickerClearButton`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TimePickerClearButtonProps {
    /// Additional attributes for the clear button element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// The props for the [`TimePickerSeparator`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TimePickerSeparatorProps {
    /// The separator symbol.
    #[props(default = ':')]
    pub symbol: char,

    /// Additional attributes for the separator element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// An hour segment in a time input.
#[component]
pub fn TimePickerHourSegment(props: TimePickerHourSegmentProps) -> Element {
    let mut ctx = use_context::<TimeElementContext>();
    let picker_ctx = use_context::<TimePickerContext>();
    let bounds = use_context::<TimeElementBounds>();
    let (min_hour, max_hour, default, min_digits) =
        if picker_ctx.picker_type == TimePickerType::Duration {
            (0, u32::MAX, 0, picker_ctx.min_hours_digits.max(1))
        } else if picker_ctx.format == TimePickerFormat::TwelveHour {
            (1, 12, 12, 2)
        } else {
            (
                bounds.min_time.hour() as u32,
                bounds.max_time.hour() as u32,
                0,
                2,
            )
        };
    let label = (picker_ctx.labels)().hour;

    rsx! {
        TimeSegment {
            aria_label: label,
            index: 0usize,
            value: ctx.hour_value,
            default,
            on_value_change: move |value: Option<u32>| ctx.hour_value.set(value),
            min: min_hour,
            max: max_hour,
            step: picker_ctx.steps.hours,
            segment_ref: picker_ctx.hours_ref,
            min_digits,
            on_format_placeholder: ctx.on_format_hour_placeholder,
            attributes: props.attributes,
        }
    }
}

/// A minute segment in a time input.
#[component]
pub fn TimePickerMinuteSegment(props: TimePickerMinuteSegmentProps) -> Element {
    let mut ctx = use_context::<TimeElementContext>();
    let picker_ctx = use_context::<TimePickerContext>();
    let bounds = use_context::<TimeElementBounds>();
    let min_hour = bounds.min_time.hour();
    let max_hour = bounds.max_time.hour();
    let min_minute = match (ctx.hour_value)() {
        Some(hour)
            if picker_ctx.format == TimePickerFormat::TwentyFourHour && hour == min_hour as u32 =>
        {
            bounds.min_time.minute() as u32
        }
        _ => 0,
    };
    let max_minute = match (ctx.hour_value)() {
        Some(hour)
            if picker_ctx.format == TimePickerFormat::TwentyFourHour && hour == max_hour as u32 =>
        {
            bounds.max_time.minute() as u32
        }
        _ => 59,
    };
    let label = (picker_ctx.labels)().minute;

    rsx! {
        TimeSegment {
            aria_label: label,
            index: 1usize,
            value: ctx.minute_value,
            default: 0u32,
            on_value_change: move |value: Option<u32>| ctx.minute_value.set(value),
            min: min_minute,
            max: max_minute,
            step: picker_ctx.steps.minutes,
            segment_ref: picker_ctx.minutes_ref,
            on_format_placeholder: ctx.on_format_minute_placeholder,
            attributes: props.attributes,
        }
    }
}

/// A second segment in a time input.
#[component]
pub fn TimePickerSecondSegment(props: TimePickerSecondSegmentProps) -> Element {
    let mut ctx = use_context::<TimeElementContext>();
    let picker_ctx = use_context::<TimePickerContext>();
    let bounds = use_context::<TimeElementBounds>();
    let min_second = match ((ctx.hour_value)(), (ctx.minute_value)()) {
        (Some(hour), Some(minute))
            if picker_ctx.format == TimePickerFormat::TwentyFourHour
                && hour == bounds.min_time.hour() as u32
                && minute == bounds.min_time.minute() as u32 =>
        {
            bounds.min_time.second() as u32
        }
        _ => 0,
    };
    let max_second = match ((ctx.hour_value)(), (ctx.minute_value)()) {
        (Some(hour), Some(minute))
            if picker_ctx.format == TimePickerFormat::TwentyFourHour
                && hour == bounds.max_time.hour() as u32
                && minute == bounds.max_time.minute() as u32 =>
        {
            bounds.max_time.second() as u32
        }
        _ => 59,
    };
    let label = (picker_ctx.labels)().second;

    rsx! {
        TimeSegment {
            aria_label: label,
            index: 2usize,
            value: ctx.second_value,
            default: 0u32,
            on_value_change: move |value: Option<u32>| ctx.second_value.set(value),
            min: min_second,
            max: max_second,
            step: picker_ctx.steps.seconds,
            segment_ref: picker_ctx.seconds_ref,
            on_format_placeholder: ctx.on_format_second_placeholder,
            attributes: props.attributes,
        }
    }
}

/// An AM/PM segment in a 12-hour time input.
#[component]
pub fn TimePickerAmPmSegment(props: TimePickerAmPmSegmentProps) -> Element {
    let mut element_ctx = use_context::<TimeElementContext>();
    let mut picker_ctx = use_context::<TimePickerContext>();
    let labels = use_context::<(String, String)>();
    let value = use_memo(move || (element_ctx.period_value)().unwrap_or(0));
    let label = (picker_ctx.labels)().am_pm;
    let display = move || {
        if value() == 0 {
            labels.0.clone()
        } else {
            labels.1.clone()
        }
    };
    let disabled = move || (picker_ctx.disabled)();
    let tab_index = if disabled() { "-1" } else { "0" };
    let index = use_signal(|| 3usize);
    let mut focus_onmounted = use_focus_controlled_item_disabled(index, disabled);
    let am_pm_ref = picker_ctx.am_pm_ref;

    rsx! {
        span {
            role: "spinbutton",
            aria_label: label,
            aria_valuemin: "0",
            aria_valuemax: "1",
            aria_valuenow: value().to_string(),
            aria_disabled: disabled(),
            aria_readonly: (picker_ctx.read_only)(),
            contenteditable: false,
            tabindex: tab_index,
            enterkeyhint: "next",
            onkeydown: move |event: Event<KeyboardData>| {
                if (picker_ctx.disabled)() || (picker_ctx.read_only)() {
                    event.prevent_default();
                    event.stop_propagation();
                    return;
                }
                match event.key() {
                    Key::ArrowUp | Key::ArrowDown | Key::Home | Key::End | Key::Character(_) => {
                        let next = if value() == 0 { 1 } else { 0 };
                        element_ctx.period_value.set(Some(next));
                        event.prevent_default();
                        event.stop_propagation();
                    }
                    Key::ArrowLeft => picker_ctx.focus.focus_prev(),
                    Key::ArrowRight | Key::Enter => {
                        picker_ctx.focus.focus_next();
                        event.prevent_default();
                        event.stop_propagation();
                    }
                    _ => {}
                }
            },
            onmounted: move |event| {
                let data = event.data();
                am_pm_ref.call(data.clone());
                focus_onmounted(event);
            },
            onfocus: move |_| {
                picker_ctx.focus.set_focus(Some(3));
            },
            "data-disabled": (picker_ctx.disabled)(),
            ..props.attributes,
            {display()}
        }
    }
}

/// A clear affordance for the time picker.
#[component]
pub fn TimePickerClearButton(props: TimePickerClearButtonProps) -> Element {
    let mut ctx = use_context::<TimePickerContext>();
    let labels = (ctx.labels)();

    rsx! {
        button {
            type: "button",
            aria_label: labels.clear,
            disabled: (ctx.disabled)() || (ctx.read_only)(),
            onclick: move |_| ctx.set_value(None),
            ..props.attributes,
        }
    }
}

/// A separator in a time input.
#[component]
pub fn TimePickerSeparator(props: TimePickerSeparatorProps) -> Element {
    rsx! {
        span {
            aria_hidden: "true",
            tabindex: "-1",
            "is-separator": true,
            "no-time": true,
            ..props.attributes,
            "{props.symbol}"
        }
    }
}

/// The props for the [`TimePickerInputValue`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TimePickerInputValueProps {
    /// Callback when display hour placeholder.
    #[props(default = Callback::new(|_| "H".to_string()))]
    pub on_format_hour_placeholder: Callback<(), String>,

    /// Callback when display minute placeholder.
    #[props(default = Callback::new(|_| "M".to_string()))]
    pub on_format_minute_placeholder: Callback<(), String>,

    /// Callback when display second placeholder.
    #[props(default = Callback::new(|_| "S".to_string()))]
    pub on_format_second_placeholder: Callback<(), String>,

    /// The children of the time value.
    #[props(default)]
    pub children: Option<Element>,
}

/// The editable time value for a time picker input.
#[component]
pub fn TimePickerInputValue(props: TimePickerInputValueProps) -> Element {
    let mut ctx = use_context::<TimePickerContext>();
    let bounds = use_context::<TimeElementBounds>();
    let selected_value = ctx.current_value();

    let initial_parts = move || match selected_value {
        Some(TimePickerValue::Time(time)) => {
            let hour = if ctx.format == TimePickerFormat::TwelveHour {
                twelve_hour_display(time.hour())
            } else {
                time.hour() as u32
            };
            (
                Some(hour),
                Some(time.minute() as u32),
                Some(time.second() as u32),
                Some(AmPm::from_hour(time.hour()).index()),
            )
        }
        Some(TimePickerValue::Duration {
            hours,
            minutes,
            seconds,
        }) => (
            Some(hours),
            Some(minutes as u32),
            Some(seconds as u32),
            None,
        ),
        None => (None, None, None, None),
    };
    let (hour, minute, second, period) = initial_parts();
    let mut hour_value = use_signal(move || hour);
    let mut minute_value = use_signal(move || minute);
    let mut second_value = use_signal(move || second);
    let mut period_value = use_signal(move || period);

    // Tracks the last value the segments were synced to so the resync effect
    // can distinguish a genuine external change (e.g. selecting a value from a
    // column picker) from an echo of this component's own emit or a mid-edit
    // partial state. Without this, a reactive resync would clobber segments the
    // user is editing or feed its own emits back in a loop.
    let mut last_synced = use_signal(move || ctx.current_value());

    // Resync the internal segment signals when the controlled value changes
    // externally. This reads the props reactively so updates that bypass the
    // segments (column pickers, programmatic changes) are reflected in the
    // display. Echoes of our own emits and partial-edit states are suppressed
    // by comparing against `last_synced`.
    use_effect(move || {
        let incoming = ctx.reactive_current_value();
        if value_precision_eq(incoming, last_synced.peek().clone(), ctx.precision) {
            return;
        }
        last_synced.set(incoming);
        match incoming {
            Some(TimePickerValue::Time(time)) => {
                let hour = if ctx.format == TimePickerFormat::TwelveHour {
                    twelve_hour_display(time.hour())
                } else {
                    time.hour() as u32
                };
                hour_value.set(Some(hour));
                minute_value.set(Some(time.minute() as u32));
                second_value.set(Some(time.second() as u32));
                period_value.set(Some(AmPm::from_hour(time.hour()).index()));
            }
            Some(TimePickerValue::Duration {
                hours,
                minutes,
                seconds,
            }) => {
                hour_value.set(Some(hours));
                minute_value.set(Some(minutes as u32));
                second_value.set(Some(seconds as u32));
                period_value.set(None);
            }
            None => {
                hour_value.set(None);
                minute_value.set(None);
                second_value.set(None);
                period_value.set(None);
            }
        }
    });

    use_effect(move || {
        // Emit the segment-derived value and record it as the last synced value
        // so the resync effect treats the resulting prop change as an echo
        // rather than an external update that would overwrite in-progress edits.
        let mut emit = move |value: Option<TimePickerValue>| {
            last_synced.set(value);
            ctx.set_value(value);
        };
        if let (Some(hour), Some(minute)) = (hour_value(), minute_value()) {
            let second = if ctx.precision == TimePrecision::Second {
                second_value()
            } else {
                Some(0)
            };
            if let Some(second) = second {
                if ctx.picker_type == TimePickerType::Duration {
                    if minute <= 59 && second <= 59 {
                        emit(Some(TimePickerValue::Duration {
                            hours: hour,
                            minutes: minute as u8,
                            seconds: second as u8,
                        }));
                    }
                } else {
                    let hour = if ctx.format == TimePickerFormat::TwelveHour {
                        to_24_hour(hour, AmPm::from_index(period_value().unwrap_or(0)))
                    } else {
                        hour as u8
                    };
                    if let Ok(time) = Time::from_hms(hour, minute as u8, second as u8) {
                        if time >= bounds.min_time && time <= bounds.max_time {
                            emit(Some(TimePickerValue::from_time(time, ctx.precision)));
                        }
                    }
                }
            }
        } else {
            emit(None);
        }
    });

    use_context_provider(|| TimeElementContext {
        hour_value,
        minute_value,
        second_value,
        period_value,
        on_format_hour_placeholder: props.on_format_hour_placeholder,
        on_format_minute_placeholder: props.on_format_minute_placeholder,
        on_format_second_placeholder: props.on_format_second_placeholder,
    });

    let children = props.children.unwrap_or_else(|| {
        rsx! {
            TimePickerHourSegment {}
            TimePickerSeparator {}
            TimePickerMinuteSegment {}
            if ctx.precision == TimePrecision::Second {
                TimePickerSeparator {}
                TimePickerSecondSegment {}
            }
            if ctx.format == TimePickerFormat::TwelveHour && ctx.picker_type == TimePickerType::Time {
                TimePickerAmPmSegment {}
            }
        }
    });

    rsx! {
        {children}
    }
}

/// The props for the [`TimePickerInput`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TimePickerInputProps {
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

/// # TimePickerInput
///
/// The input element for the [`TimePicker`] component which allows users to enter a time value.
#[component]
pub fn TimePickerInput(props: TimePickerInputProps) -> Element {
    let children = props.children.unwrap_or_else(|| {
        rsx! {
            TimePickerInputValue {
                on_format_hour_placeholder: props.on_format_hour_placeholder,
                on_format_minute_placeholder: props.on_format_minute_placeholder,
                on_format_second_placeholder: props.on_format_second_placeholder,
            }
        }
    });

    rsx! {
        div { ..props.attributes,
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::time;

    #[component]
    fn ControlledTimePicker() -> Element {
        rsx! {
            TimePicker {
                selected_time: Some(time!(09:30)),
                TimePickerInput {}
            }
        }
    }

    #[component]
    fn PlaceholderTimePicker() -> Element {
        rsx! {
            TimePicker {
                TimePickerInput {}
            }
        }
    }

    #[test]
    fn time_picker_input_renders_controlled_time_on_first_render() {
        let mut dom = VirtualDom::new(ControlledTimePicker);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("09"));
        assert!(html.contains("30"));
        assert!(!html.contains("HH"));
        assert!(!html.contains("MM"));
    }

    #[test]
    fn time_picker_input_renders_placeholders_when_empty() {
        let mut dom = VirtualDom::new(PlaceholderTimePicker);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("HH"));
        assert!(html.contains("MM"));
        assert!(html.contains("role=\"spinbutton\""));
    }

    #[test]
    fn minute_precision_helpers_ignore_subminute_selected_time() {
        assert!(minute_precision_eq(
            Some(time!(09:30:45)),
            Some(time!(09:30:00))
        ));
        assert!(!minute_precision_eq(
            Some(time!(09:31:00)),
            Some(time!(09:30:00))
        ));
    }

    #[test]
    fn minute_precision_bounds_round_as_documented() {
        assert_eq!(ceil_to_minute(time!(09:30:45)), Some(time!(09:31)));
        assert_eq!(ceil_to_minute(time!(09:30)), Some(time!(09:30)));
        assert_eq!(ceil_to_minute(time!(23:59:59)), None);
        assert_eq!(floor_to_minute(time!(09:30:45)), time!(09:30));
    }

    #[test]
    fn second_precision_bounds_round_as_documented() {
        assert_eq!(ceil_to_second(time!(09:30:45.001)), Some(time!(09:30:46)));
        assert_eq!(ceil_to_second(time!(09:30:45)), Some(time!(09:30:45)));
        assert_eq!(ceil_to_second(time!(23:59:59.001)), None);
    }

    #[component]
    fn SecondsTimePicker() -> Element {
        rsx! {
            TimePicker {
                selected_time: Some(time!(09:30:45)),
                with_seconds: true,
                TimePickerInput {}
            }
        }
    }

    #[test]
    fn with_seconds_renders_second_segment_and_selected_seconds() {
        let mut dom = VirtualDom::new(SecondsTimePicker);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("09"));
        assert!(html.contains("30"));
        assert!(html.contains("45"));
        assert!(html.contains("aria-label=\"second\""));
        assert_eq!(html.matches("role=\"spinbutton\"").count(), 3);
    }

    #[component]
    fn TwelveHourTimePicker() -> Element {
        rsx! {
            TimePicker {
                selected_time: Some(time!(13:05)),
                format: TimePickerFormat::TwelveHour,
                am_pm_labels: ("a.m.".to_string(), "p.m.".to_string()),
                TimePickerInput {}
            }
        }
    }

    #[test]
    fn twelve_hour_mode_renders_converted_hour_and_period_label() {
        let mut dom = VirtualDom::new(TwelveHourTimePicker);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("01"));
        assert!(html.contains("05"));
        assert!(html.contains("p.m."));
        assert!(html.contains("aria-label=\"AM/PM\""));
    }

    #[component]
    fn DurationTimePicker() -> Element {
        rsx! {
            TimePicker {
                picker_type: TimePickerType::Duration,
                selected_value: Some(TimePickerValue::Duration {
                    hours: 125,
                    minutes: 4,
                    seconds: 5,
                }),
                with_seconds: true,
                min_hours_digits: 4,
                TimePickerInput {}
            }
        }
    }

    #[test]
    fn duration_mode_renders_unbounded_hours_and_minimum_hour_digits() {
        let mut dom = VirtualDom::new(DurationTimePicker);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("0125"));
        assert!(html.contains("04"));
        assert!(html.contains("05"));
    }

    #[test]
    fn step_segment_wraps_by_configured_step() {
        assert_eq!(step_segment(10, 0, 23, 5, 1), 15);
        assert_eq!(step_segment(22, 0, 23, 5, 1), 0);
        assert_eq!(step_segment(10, 0, 23, 5, -1), 5);
        assert_eq!(step_segment(2, 0, 23, 5, -1), 23);
    }

    #[test]
    fn twelve_hour_helpers_convert_to_twenty_four_hour_time() {
        assert_eq!(to_24_hour(12, AmPm::Am), 0);
        assert_eq!(to_24_hour(1, AmPm::Pm), 13);
        assert_eq!(twelve_hour_display(0), 12);
        assert_eq!(twelve_hour_display(13), 1);
    }

    #[test]
    fn default_paste_parser_accepts_seconds_and_periods() {
        assert_eq!(
            parse_default_time_picker_value("1:02:03 pm".to_string()),
            Some(TimePickerValue::Time(time!(13:02:03)))
        );
        assert_eq!(
            parse_default_time_picker_value("12:00 am".to_string()),
            Some(TimePickerValue::Time(time!(00:00)))
        );
        assert_eq!(
            parse_default_time_picker_value("36:15".to_string()),
            Some(TimePickerValue::Duration {
                hours: 36,
                minutes: 15,
                seconds: 0
            })
        );
        assert_eq!(
            parse_default_time_picker_value("125:04:05".to_string()),
            Some(TimePickerValue::Duration {
                hours: 125,
                minutes: 4,
                seconds: 5
            })
        );
    }

    #[test]
    fn time_picker_preset_range_generates_inclusive_steps() {
        assert_eq!(
            time_picker_preset_range(time!(09:00), time!(10:00), 30 * 60),
            vec![time!(09:00), time!(09:30), time!(10:00)]
        );
        assert!(time_picker_preset_range(time!(10:00), time!(09:00), 30 * 60).is_empty());
        assert!(time_picker_preset_range(time!(09:00), time!(10:00), 0).is_empty());
    }

    #[component]
    fn LabelledTimePicker() -> Element {
        rsx! {
            TimePicker {
                with_seconds: true,
                clearable: true,
                labels: TimePickerLabels {
                    group: "Custom time".to_string(),
                    hour: "Custom hour".to_string(),
                    minute: "Custom minute".to_string(),
                    second: "Custom second".to_string(),
                    am_pm: "Custom period".to_string(),
                    clear: "Custom clear".to_string(),
                },
                TimePickerInput {}
            }
        }
    }

    #[test]
    fn custom_accessibility_labels_are_rendered() {
        let mut dom = VirtualDom::new(LabelledTimePicker);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("aria-label=\"Custom time\""));
        assert!(html.contains("aria-label=\"Custom hour\""));
        assert!(html.contains("aria-label=\"Custom minute\""));
        assert!(html.contains("aria-label=\"Custom second\""));
        assert!(html.contains("aria-label=\"Custom clear\""));
    }

    #[component]
    fn DisabledTimePicker() -> Element {
        rsx! {
            TimePicker {
                selected_time: Some(time!(09:30)),
                disabled: true,
                TimePickerInput {}
            }
        }
    }

    #[component]
    fn ReadOnlyTimePicker() -> Element {
        rsx! {
            TimePicker {
                selected_time: Some(time!(09:30:45)),
                read_only: true,
                TimePickerInput {}
            }
        }
    }

    #[test]
    fn disabled_segments_are_removed_from_tab_order() {
        let mut dom = VirtualDom::new(DisabledTimePicker);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("data-disabled=true"));
        assert_eq!(html.matches("role=\"spinbutton\"").count(), 2);
        assert_eq!(
            html.matches("tabindex=\"-1\" enterkeyhint=\"next\"")
                .count(),
            2
        );
        assert_eq!(html.matches("aria-disabled=true").count(), 2);
    }

    #[test]
    fn read_only_segments_expose_read_only_semantics_without_normalizing_display() {
        let mut dom = VirtualDom::new(ReadOnlyTimePicker);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("09"));
        assert!(html.contains("30"));
        assert_eq!(html.matches("aria-readonly=true").count(), 2);
        assert_eq!(html.matches("contenteditable=false").count(), 2);
    }
}
