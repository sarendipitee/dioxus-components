Sheet in this preview is demonstrated as a focused side panel for workflow steps that should stay close to the current page.

Use it when an interaction needs temporary focus—a quick settings form, a compact details view, or an action confirmation—while users keep their place in the underlying screen.

These demos are intentionally scoped to:
- Opening a sheet in a controlled way (`open` prop).
- Anchoring the panel to a specific edge with the `side` prop.
- Structuring the content with title, description, body, and footer areas that mirror real modal workflows.

## Component Structure

```rust
use dioxus_components::sheet::{Sheet, SheetSide};

Sheet {
    open: open(),
    on_open_change: move |value| open.set(value),
    side: SheetSide::Right,
    title: "Edit Profile",
    description: "Make changes to your profile here.",
    footer: rsx! {
        Button { "Save changes" }
    },
    div { "Profile form content" }
}
```

## Edge Placement

Use `SheetSide::Top`, `SheetSide::Right`, `SheetSide::Bottom`, or `SheetSide::Left` to choose the entry edge. `SheetSide::Right` is the default.

```rust
Sheet {
    open: open(),
    on_open_change: move |value| open.set(value),
    side: SheetSide::Left,
    title: "Navigation",
    description: "Choose a destination.",
    nav { "Links" }
}
```
