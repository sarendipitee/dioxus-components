`ColorPicker` is an inline, always-visible color-selection control for cases where color should be adjusted directly in the interface instead of opening a dropdown. It keeps the saturation/value area, hue strip, and current swatch together so users can see every part of the choice in one place.

Prefer this component in places like palette editors, theme overrides, and live design controls where the user is repeatedly tweaking tones. If you need a labeled text field, validation, a clear button, form-friendly chrome, or a trigger-based picker workflow, use `ColorInput` from the `color_input` registry entry instead.

## Component Structure

```rust
ColorPicker {
    color,
    on_color_change: move |value: Hsv<encoding::Srgb, f64>| color.set(value),
}
```

Pass children to append custom content after the default picker controls. The included demos show how to keep that extra content minimal so the main picker interaction stays dominant.
