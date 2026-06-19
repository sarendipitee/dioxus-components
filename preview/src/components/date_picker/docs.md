`DatePicker` and `DateRangePicker` are inline, form-neutral calendar panels. They expose the full interaction layer for choosing one date or a contiguous date range: month navigation, disabled-day behavior, locale formatting, keyboard focus management, and roving focus all remain in the component itself.

These demos intentionally show the bare calendar experience, so you can reason about selection rules and focus flow without mixing in field labels, input validation copy, or popover shell styling. Use `DateInput` or `DateRangePickerInput` from the `date_input` registry entry when a full input control (label + helper text + trigger pattern) is part of the component contract.

## Component Structure

```rust
DatePicker {
    selected_date,
    on_value_change: move |value: Option<Date>| selected_date.set(value),
    month_count: 1,
}
```

For field-entry APIs, use `DateInput` and `DateRangePickerInput`.
