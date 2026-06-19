The TimePicker page demonstrates the core picker primitive contract: `TimePicker` owns the selection state and behavioral props, while `TimePickerInput {}` renders the styled field shell inside the same surface.

Pair this component with `TimeInput` from the `time_input` registry whenever you want consistent label, description, error, and sizing treatment with the rest of your form fields.

Use `selected_time: Option<Time>` plus `on_value_change` for ordinary clock-time entry (for example, choosing a meeting start). Switch to `selected_value: Option<TimePickerValue>` and `picker_type: TimePickerType::Duration` with `on_picker_value_change` when input can represent spans longer than one day.

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

This page intentionally documents only current public fields: `labels`, `clearable`, `with_seconds`, `format`, `selected_time`, `selected_value`, `picker_type`, `am_pm_labels`, and `steps`. Each demo is intentionally scoped to one usage mode so you can copy the block that matches your input contract without adapting legacy API examples.
