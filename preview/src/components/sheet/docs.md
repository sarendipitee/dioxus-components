Sheet in this preview is demonstrated as a focused side panel for workflow steps that should stay close to the current page.

Use it when an interaction needs temporary focus—a quick settings form, a compact details view, or an action confirmation—while users keep their place in the underlying screen.

These demos are intentionally scoped to:
- Opening a sheet in a controlled way (`open` prop).
- Anchoring the panel to a specific edge with `data-side`.
- Structuring the content with title, description, close, and footer areas that mirror real modal workflows.

## Component Structure

```rust
Sheet {
    open: open(),
    // Choose the anchor edge that matches your workflow. Available sides: Top, Right (default), Bottom, Left.
    "data-side": SheetSide::Right.as_str(),
    SheetContentClose {}
    SheetHeader {
        SheetTitle { "Edit Profile" }
        SheetDescription { "Make changes to your profile here." }
    }
    SheetFooter {
        SheetClose { "Close" }
    }
}
```

## SheetClose with `as` prop

The `as` prop is demonstrated to keep close behavior while swapping the rendered element type.

This is useful when your design calls for a button, link, or custom control in the same close interaction pattern.

```rust
// Default: renders as <button>
SheetClose { "Close" }

// Custom element: the preset click handler is passed through in `attributes`
SheetClose {
    as: |attributes| rsx! {
        a { href: "#", ..attributes, "Go back" }
    }
}
```
