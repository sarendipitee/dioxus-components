The TimePicker component renders a segmented time input with keyboard-friendly spinbutton segments, optional seconds, 12-hour mode, duration entry, clearing, presets, and an optional dropdown picker.

The default controlled value is `selected_time: Option<Time>`. For duration values with hours beyond 23, use `selected_value: Option<TimePickerValue>` with `picker_type: TimePickerType::Duration`.

## Component Structure

```rust
TimePicker {
    selected_time,
    on_value_change: move |v: Option<Time>| selected_time.set(v),
    with_seconds: true,
    format: TimePickerFormat::TwelveHour,
    am_pm_labels: ("am".to_string(), "pm".to_string()),
    min_time: time!(09:00),
    max_time: time!(17:30),
    steps: TimePickerSteps {
        hours: 1,
        minutes: 15,
        seconds: 10,
    },
    clearable: true,
    with_dropdown: true,
}
```

Duration mode keeps the dropdown disabled and renders an unbounded hour segment:

```rust
TimePicker {
    picker_type: TimePickerType::Duration,
    selected_value,
    on_picker_value_change: move |v| selected_value.set(v),
    with_seconds: true,
    min_hours_digits: 3,
}
```

The preview wrapper also supports `label`, `description`, `error`, `variant`, `size`, `radius`, `right_section`, `clear_section_mode`, `presets`, `preset_groups`, dropdown placement/width/max-height props, custom accessibility labels, and aggregate focus handlers.
