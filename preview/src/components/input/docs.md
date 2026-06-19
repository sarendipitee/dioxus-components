The input modules in this folder are the shared form-control primitives used by every styled field in the library. Most apps should use `TextInput` directly for native text entry, while this page is for composing custom form controls that still need the same field structure and accessibility.

This page demonstrates the three construction layers behind any field component:

```rust
InputBase {
    label: rsx! { "Project slug" },
    left_section: rsx! { span { "#" } },
    input { placeholder: "release-notes" }
}
```

- Use `InputBase` when creating a custom field that needs both field metadata (`label`, `description`, `error`, `required`) and custom child content.
- Use `Input` when you need only the visual shell (variant, size, radius, disabled/error state, and optional side sections) and can provide your own wrapper logic.
- Use `InputWrapper` when you want only the semantic wrapper behavior without the shared shell container.
- Use `TextInput` for production-ready native text entry. Its full API and examples live on the dedicated `text_input` page.

## Foundation Parts

- `InputWrapper` handles semantic field scaffolding: id-linked label/description/error text, required marker, and wrapper status classes for error/disabled states.
- `Input` provides the reusable shell used by all fields: visual variants, sizing, radius presets, and left/right section layout.
- `InputBase` combines both, wires stable field ids, and exposes the metadata hooks needed when your child component renders its own interactive control.
- `InputClearButton` owns the shared clear affordance so custom inputs inherit the same behavior on right sections.
- `TextInput` is the canonical native-text implementation built on top of `InputBase`, not the lower-level building block itself.

This layering is intentional: `Input` is the narrow styling surface, `InputBase` is the composition boundary, and anything you build for non-text controls should start from there.
