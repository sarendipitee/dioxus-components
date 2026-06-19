Use `Label` whenever a form control needs an explicit, discoverable name for keyboard, pointer, and assistive-technology users.  

In these demos, each label points to a concrete control via `html_for`, so activating the label moves focus to the matching input in the same form row instead of relying on placeholder text alone.

## Component Structure

```rust
Label {
    html_for: "id", // The ID of the form control this label is associated with
}
button {
    id: "id", // The ID of the labeled element
}
```

### What to look for in the demos

1. The label text always appears before the control and describes exactly what the control collects.
2. The `html_for` value and control `id` are the same, which keeps click/focus behavior correct in complex layouts.
3. Variants can style the same semantic relationship consistently across light, dark, and custom compositions without changing the underlying accessibility contract.
