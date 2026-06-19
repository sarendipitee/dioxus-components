`Autocomplete` is the component for free-form text entry with guided selection.
It keeps the input value in sync with a combobox-backed option registry, updates suggestions as you type, and lets users commit one matching option with keyboard or pointer interaction.

Use it when search should feel like typing, not selecting from a static list: suggestion filtering happens in real time, and the demo below shows how to wire value state, update behavior, and a no-results empty state.

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

The sample demonstrates the core autocomplete flow this page exists to explain: bound input state, suggestion rendering, and explicit option selection.
