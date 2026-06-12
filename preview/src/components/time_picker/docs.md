The TimePicker preview reflects the picker-surface API: a primitive-style `TimePicker` container with `TimePickerInput {}` as its child. Use `TimeInput` from the `time_input` registry entry for first-class styled field chrome with label, description, error, sections, and shared input sizing.

For normal clock-time usage, control the picker with `selected_time: Option<Time>` and `on_value_change`. For duration values that can exceed 23 hours, switch to `selected_value: Option<TimePickerValue>` with `picker_type: TimePickerType::Duration` and `on_picker_value_change`.

## Basic clock time

```rust
TimePicker {
    selected_time,
    on_value_change: move |value| selected_time.set(value),
    TimePickerInput {}
}
```

## Clearable

```rust
TimePicker {
    selected_time,
    on_value_change: move |value| selected_time.set(value),
    clearable: true,
    labels: TimePickerLabels {
        group: "Reminder time".to_string(),
        clear: "Clear reminder time".to_string(),
        ..Default::default()
    },
    TimePickerInput {}
}
```

## Seconds and 12-hour

```rust
TimePicker {
    selected_time,
    on_value_change: move |value| selected_time.set(value),
    with_seconds: true,
    format: TimePickerFormat::TwelveHour,
    am_pm_labels: ("am".to_string(), "pm".to_string()),
    steps: TimePickerSteps {
        hours: 1,
        minutes: 15,
        seconds: 15,
    },
    TimePickerInput {}
}
```

## Duration mode

```rust
TimePicker {
    selected_value,
    on_picker_value_change: move |value| selected_value.set(value),
    picker_type: TimePickerType::Duration,
    with_seconds: true,
    min_hours_digits: 3,
    TimePickerInput {}
}
```

The stale preview-only wrapper props are no longer documented here. Use the current public fields instead: `labels`, `clearable`, `with_seconds`, `format`, `selected_time`, `selected_value`, `picker_type`, `am_pm_labels`, and `steps`.
