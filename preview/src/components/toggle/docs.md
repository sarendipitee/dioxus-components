A toggle switch is a compact binary control for states that are either enabled or disabled. In this demo page, it is shown as the switch a user would click to turn a single setting on and off without opening a dialog.

## Component Structure

```rust
// The Toggle component renders a persistent on/off control inside setting surfaces.
Toggle {
    // The child label or icon is read as the visible control text.
    "Bold",
}
```

Pair this component with short labels and contextual helper text when users are editing preferences, permission flags, or feature enablement. The demos below show the same component in different copy lengths and container contexts so you can evaluate spacing, density, and accessibility while preserving its binary intent.
