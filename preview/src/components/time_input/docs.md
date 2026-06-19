`TimeInput` is the production-ready form field for selecting a clock value in a full input context. It composes the `TimePicker` primitive into the shared `InputBase` container so it inherits the same label, description, error, sizing, radius, and clear affordances as other inputs in this library.

Use this component when you need a time chooser that participates in form layout conventions (validation messaging, helper text, consistent spacing), rather than rendering raw time segments by themselves.

The demos below show how to use the field in common scenarios:

- controlled time state updates via `selected_time` and `on_value_change`
- toggling clearability to let users reset a chosen time
- integrating with regular input decoration like labels, required text patterns, and helper messaging

Use `TimePicker` directly only when you only need the picker panel itself, without the field shell.

```rust
TimeInput {
    label: rsx! { "Start time" },
    selected_time,
    on_value_change: move |value| selected_time.set(value),
    clearable: true,
}
```
