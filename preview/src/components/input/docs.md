> ⚠️ **Low-Level Component**: `Input`, `InputBase`, and `InputWrapper` are low-level form-control primitives and layout shells intended for component authors building custom form controls. **Applications generally should NOT use `Input` directly.** Instead, use high-level, ready-to-use form controls such as [`TextInput`](/components/text_input), [`NumberInput`](/components/number_input), [`PasswordInput`](/components/password_input), or [`Select`](/components/select).

The input primitives in this module provide the shared visual shell, accessibility ID wiring, and layout structure used across all styled form components in the library.

This page demonstrates the construction layers behind field component composition:

```rust
InputBase {
    label: rsx! { "Project slug" },
    left_section: rsx! { span { "#" } },
    input { placeholder: "release-notes" }
}
```

- **Use `TextInput`** (or other specialized input components) for almost all standard form fields in applications.
- **Use `InputBase`** only when creating a custom component (like a date picker or color input) that requires field metadata (`label`, `description`, `error`, `required`) wrapping custom child controls.
- **Use `Input`** only when you need the visual container shell alone (variants, sizes, radius, left/right section slots) and provide custom wrapper/label logic.
- **Use `InputWrapper`** when you need semantic field label/description/error scaffolding without the visual shell container.

## Foundation Parts

- `InputWrapper` handles semantic field scaffolding: id-linked label/description/error text, required markers, and wrapper status classes.
- `Input` provides the reusable visual shell: styling variants, sizing, radius presets, and left/right section layout.
- `InputBase` combines both `InputWrapper` and `Input`, wiring stable field IDs and exposing context hooks for custom child controls.
- `InputClearButton` provides the shared clear button affordance for right-section slots.
- `TextInput` is the canonical, production-ready text entry component built on top of `InputBase`.
