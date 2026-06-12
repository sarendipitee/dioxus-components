`TextInput` is the public text-field component for ordinary string entry. It renders a native `<input>` inside the shared `InputBase` wrapper, so label, description, error, size, radius, sections, and disabled styling match the rest of the input family.

Use `InputBase` when you are building a custom field component around non-text content. Use `Input` directly only when you need the visual shell without wrapper metadata.

```rust
TextInput {
    label: rsx! { "Email" },
    description: rsx! { "We'll only use this for account updates." },
    placeholder: "name@example.com",
}
```
