`TextInput` is the canonical single-line text entry component in the component library. It is intended for places where users type plain characters—email addresses, usernames, tags, IDs, or search-like fields—and it gives you consistent behavior for labels, helper copy, status text, sizing, and disabled/error visual states.

This page’s demos show `TextInput` as the form-level field with built-in field metadata and layout, while keeping the actual input semantics as a native `<input>` underneath. When you need only the shell styles (no labels or field copy), use `Input`; when you need a custom wrapper for non-text content, start from `InputBase`.

```rust
TextInput {
    label: rsx! { "Email" },
    description: rsx! { "We'll only use this for account updates." },
    placeholder: "name@example.com",
}
```
