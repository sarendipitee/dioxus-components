The button component represents a single, explicit action in the interface—"save", "add", "discard", "retry", and similar commands—while keeping interaction behavior (hover, focus, active, disabled) visually aligned across every variant.

This page’s demos show how the component is styled and how to use it for common interaction patterns: choosing a tone/variant for the action’s priority, switching between sizes, and placing content (text, icon, or both) inside a consistent control surface.

## Component Structure

```rust
button {
    // Global html attributes
    class: "dx-button",
    "data-style": "default",
    "data-size": "default",
    // Children
    {children}
}
```
