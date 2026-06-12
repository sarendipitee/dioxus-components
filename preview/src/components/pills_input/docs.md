`PillsInput` is the combobox-family layout wrapper for pill-based entry experiences.
It provides the shared styled field surface while pill children and the inner search/input composition stay flexible.

Use it when you need removable pills and a freeform inner input, but the higher-level value ownership belongs to your own component or app state.

## Component Structure

```rust
PillsInput {
    Pill {
        "Rust"
    }
    input {
        r#type: "text",
        "data-pills-input-field": true,
        placeholder: "Add technology..."
    }
}
```
