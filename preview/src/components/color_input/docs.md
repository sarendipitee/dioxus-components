`ColorInput` is the styled field-entry component for colors. It renders a shared input shell with a color-preview left section, accepts direct color text entry, and opens `ColorPickerSurface` in a popover when the input receives focus.

Use `ColorPicker` directly when you need an inline color picking surface without generic field chrome.

```rust
ColorInput {
    label: rsx! { "Accent color" },
    color,
    on_color_change: move |value| color.set(value),
}
```
