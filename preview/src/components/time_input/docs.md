`TimeInput` is the styled field-entry component for time values. It renders the primitive `TimePicker` segmented control inside the shared `InputBase` shell, so label, description, error, size, radius, sections, and clear behavior match the rest of the input family.

Use `TimePicker` directly when you need the picker surface and primitive time-selection behavior without field chrome.

```rust
TimeInput {
    label: rsx! { "Start time" },
    selected_time,
    on_value_change: move |value| selected_time.set(value),
    clearable: true,
}
```
