`TagsInput` is the tag-entry component for collecting arbitrary values a user types and immediately promoting each committed value into a removable chip.
This demo page is intentionally scoped to the practical editing flow: users enter tokens, confirm with Enter, and the control turns entries into a managed list they can inspect, remove, and reorder in a consistent pattern.

The examples highlight why you would use this component over a plain input: it combines value normalization, duplicate prevention, and interactive chip affordances so tag-like data stays predictable while the user is typing.

Use this component in interfaces where users build collections of values, such as keywords, labels, recipients, or filter terms.

## Component Structure

```rust
let mut values = use_signal(|| Some(vec!["dioxus".to_string()]));

TagsInput {
    values,
    on_values_change: move |next| values.set(Some(next)),
    placeholder: "Add tag and press Enter...",
}
```
