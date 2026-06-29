`Popover` is a compact anchored overlay that stays attached to its trigger and is scoped to local, in-context interactions.

Use it for quick action menus, inline forms, metadata cards, or any ephemeral surface that belongs next to the control that opened it. Unlike `Dialog`, a popover keeps the current view in place — use `Dialog` for tasks that warrant full focus interruption.

## Component Structure

```rust
use dioxus_components::popover::*;
use dioxus_primitives::{ContentAlign, ContentSide};

// Popover owns the open state and positions content relative to the trigger.
// Omit `open`/`on_open_change` to use it uncontrolled (manages its own state).
Popover {
    open: open(),
    on_open_change: move |v| open.set(v),

    // PopoverTrigger toggles open/closed on click.
    // Use PopoverOpenTrigger for an open-only trigger (e.g. an input adornment).
    PopoverTrigger { "Open" }

    PopoverContent {
        side: ContentSide::Bottom,   // Top | Right | Bottom | Left
        align: ContentAlign::Center, // Start | Center | End
        PopoverContentTitle { "Details" }
        PopoverContentDescription { "Short supporting copy." }
        {children}
    }
}
```

Content automatically flips and shifts when near a viewport edge to stay in view.

## Props

### Popover

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `open` | `Option<bool>` | — | Controlled open state |
| `on_open_change` | `Callback<bool>` | — | Called when open state should change |
| `default_open` | `bool` | `false` | Initial open state when uncontrolled |
| `is_modal` | `bool` | `true` | Traps focus inside the popover when open |

### PopoverContent

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `side` | `ContentSide` | `Bottom` | Preferred side relative to trigger |
| `align` | `ContentAlign` | `Center` | Alignment along the side axis |

### PopoverContentTitle / PopoverContentDescription

Use these optional content slots for popovers that need a short heading and supporting copy.

## Triggers

`PopoverTrigger` toggles the popover open and closed. `PopoverOpenTrigger` only opens it — useful for input adornments where the field itself can also trigger the open and clicking the icon should be idempotent.

## Dismiss Behavior

Popovers close on:
- Click outside the root element
- `Escape` key
- Any `on_open_change(false)` call (e.g. a Cancel button inside the content)

## Focus

When `is_modal: true` (default), focus is trapped inside the content while open. Set `is_modal: false` for surfaces like info cards or suggestion lists where the user should remain free to interact with the rest of the page.

## Data Attributes

`PopoverContent` exposes these for CSS-driven conditional styling:

| Attribute | Values |
|-----------|--------|
| `data-state` | `open` \| `closed` |
| `data-side` | `top` \| `right` \| `bottom` \| `left` (resolved, post-flip) |
| `data-align` | `start` \| `center` \| `end` (resolved, post-shift) |
