`MultiSelect` is the multi-value entry control for choosing several items from one option source.
It combines a combobox search field, toggleable options, and an in-field chip list so users can add, remove, and review selected items without leaving the input context.

Use this component when a user workflow needs both discovery and accumulation: for example building tag lists, selecting categories, or composing filters where each choice should be visible, removable, and constrained by a maximum count.

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
