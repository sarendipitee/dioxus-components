`DatePicker` and `DateRangePicker` are inline date selection surfaces. They own calendar state, disabled-date logic, locale formatting, and roving focus, but they do not render shared input field chrome or a popover trigger.

Use `DateInput` or `DateRangePickerInput` from the `date_input` registry entry when you need a labeled field, shared input sizing, validation text, or dropdown composition.

## Component Structure

```rust
DatePicker {
    selected_date,
    on_value_change: move |value: Option<Date>| selected_date.set(value),
    month_count: 1,
}
```

For field-entry APIs, use `DateInput` and `DateRangePickerInput`.
