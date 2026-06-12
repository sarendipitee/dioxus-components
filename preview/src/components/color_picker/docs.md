`ColorPicker` provides the inline color picking surface. It renders the color area, hue control, and swatch without shared input field chrome or a popover trigger.

Use `ColorInput` from the `color_input` registry entry when you need a labeled field, shared input sizing, validation text, clear affordance, or dropdown composition.

## Component Structure

```rust
ColorPicker {
    color,
    on_color_change: move |value: Hsv<encoding::Srgb, f64>| color.set(value),
}
```

Pass children to append custom content after the default picker controls.
