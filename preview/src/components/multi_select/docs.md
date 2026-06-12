`MultiSelect` is the combobox-family adapter for selecting many values from a filterable option list.
It owns the selected values array, search query, and selected-pill rendering.

Use it when users should be able to search a list and toggle several values within one shared field surface.

## Component Structure

```rust
MultiSelect::<String> {
    default_values: vec!["mushroom".to_string()],
    max_values: 3usize,
    render_value: |value: String| rsx! { "{value}" },
    placeholder: "Pick toppings...",
    ComboboxEmpty { "No toppings found." }
    ComboboxOption::<String> {
        index: 0usize,
        value: "mushroom".to_string(),
        text_value: "Mushroom",
        "Mushroom"
    }
}
```
