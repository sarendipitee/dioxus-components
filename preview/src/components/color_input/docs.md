`ColorInput` is the styled color control for forms where users need both exact text input and visual color picking in a single field.

It combines the shared input baseline with a built-in color preview lane, so people can read the currently selected value, type a color string directly, and launch `ColorPickerSurface` from the same control when focus lands in the field.

Use the demos below to compare the component states: a normal text-driven entry flow, and the popover picker flow for interactive selection.

Use `ColorPicker` directly when you need an inline picker that is not wrapped in form-field chrome.

```rust
ColorInput {
    label: rsx! { "Accent color" },
    color,
    on_color_change: move |value| color.set(value),
}
```
