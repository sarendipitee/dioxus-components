`Autocomplete` is the combobox-family text-entry adapter.
It owns the visible text value while still using combobox option registration, filtering, and submit behavior underneath.

Use it when users should type freely and optionally commit one of the presented options into the field value.

## Component Structure

```rust
let mut value = use_signal(|| None::<String>);

Autocomplete {
    value: Some(value.into()),
    on_value_change: move |next| value.set(next),
    placeholder: "Type a framework...",
    ComboboxEmpty { "No framework found." }
    ComboboxOption::<String> {
        index: 0usize,
        value: "next".to_string(),
        text_value: "Next.js",
        "Next.js"
    }
}
```
