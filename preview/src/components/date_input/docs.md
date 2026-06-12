`DateInput` is the styled single-date field-entry component. It uses the shared input foundation for field chrome, owns the dropdown popover composition, and delegates date selection behavior to the `DatePicker` surface.

`DateRangePickerInput` provides the same shared shell for range selection. Use `DatePicker` or `DateRangePicker` directly when you need an inline picker surface without label, description, error, or input sizing props.

```rust
DateInput {
    label: rsx! { "Due date" },
    selected_date,
    on_value_change: move |value| selected_date.set(value),
}
```
