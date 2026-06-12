`TagsInput` is the combobox-family adapter for freeform tag entry.
It owns tag parsing, duplicate handling, removable pills, and the inner search text used while composing tags.

Use it when users should type arbitrary values and commit them into a tag list, typically by pressing Enter.

## Component Structure

```rust
let mut values = use_signal(|| Some(vec!["dioxus".to_string()]));

TagsInput {
    values,
    on_values_change: move |next| values.set(Some(next)),
    placeholder: "Add tag and press Enter...",
}
```
