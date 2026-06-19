`DateInput` is the single-date form component in this demo suite: it combines the shared input shell (label, description, error message, sizing) with a popover calendar so users can type a date or pick one visually.

Use this page to compare three common entry patterns:
1. a clean default date field for standard form usage,
2. a constrained entry flow for a due-date workflow, and
3. a range variant shell (`DateRangePickerInput`) when you need start/end capture while reusing the same field-level styling.

The `DatePicker` and `DateRangePicker` components remain available as standalone inline surfaces when you want just the calendar UI without input chrome.

```rust
DateInput {
    label: rsx! { "Due date" },
    selected_date,
    on_value_change: move |value| selected_date.set(value),
}
```
