Use `ToggleGroup` when you need a compact set of related choices that should behave like a single segmented control, such as view filters, content views, or mode switches. The group manages how each toggle item is related to the others so users can move through choices consistently and see a shared active state.

## Component Structure

```rust
// The ToggleGroup owns the shared container, layout direction, and group semantics.
ToggleGroup {
    // Horizontal groups are common for toolbar-like controls; set `horizontal: false` for compact vertical stacks.
    horizontal: true,
    // ToggleItem creates a focusable option inside the group, with keyboard order driven by its index.
    ToggleItem {
        // Index controls the traversal order for arrow-key focus movement.
        index: 0,
        // The visible label/content for this option.
        {children}
    }
}
```
