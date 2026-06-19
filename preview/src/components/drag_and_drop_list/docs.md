The DragAndDropList demo page shows a vertical task-style list where each row can be reordered in place using a drag gesture, touch input, or keyboard controls, keeping the item identity and position updates obvious while interacting.

## Component Structure

```rust
DragAndDropList {
    // Items to be rendered
    items
    // Whether the list items should be removable
    is_removable
}
```

Use this component when you need a simple ordered list with user-driven reordering in a dashboard, settings, or form-builder workflow. The controls below the code example illustrate how removal toggles and reorder interactions affect the list state while staying keyboard-accessible.
