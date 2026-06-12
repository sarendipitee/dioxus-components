The input package is mostly a shared foundation for styled field components. Most app code should start with `TextInput` for native text entry or a higher-level field component such as `Select`, `DateInput`, `TimeInput`, or `ColorInput`.

This page focuses on the lower-level building blocks:

```rust
InputBase {
    label: rsx! { "Project slug" },
    left_section: rsx! { span { "#" } },
    input { placeholder: "release-notes" }
}
```

- Use `InputBase` when building a new input-like component that needs label, description, error, required, disabled, sections, and the shared field shell around custom content.
- Use `Input` only when you already have surrounding field wiring and need the visual shell itself.
- Use `InputWrapper` only when a component needs wrapper chrome without the shared shell.
- Use `TextInput` when you want a complete text field. It is documented on the dedicated `text_input` page.

## Foundation Parts

- `InputWrapper` renders label, description, error, required marker, and wrapper state.
- `Input` renders the shared visual shell with variant, size, radius, disabled/error state, and left/right sections.
- `InputBase` composes `InputWrapper` and `Input`, generates shared field ids, and provides control metadata for custom children.
- `InputClearButton` provides the shared clear affordance for right sections.
- `TextInput` is the public native text-entry adapter built on top of `InputBase`.

The naming follows the same layering model used by Mantine: `Input` is the low-level visual shell, while `InputBase` is the convenience composition of wrapper plus shell. If you expect `Input` to mean "a normal text input", use `TextInput` instead.
