This page demonstrates `Popover`, a compact anchored overlay that stays attached to a trigger element and is meant for local, in-context interactions.

Use it for quick command surfaces like row actions in a table, tiny form helpers, or ephemeral metadata blocks that should appear exactly next to the control that opened them. Unlike a full modal, the popover keeps the current task in place while presenting focused follow-up content.

The examples below show the structural pieces and how placement is controlled with `side` and `align` so you can tune where the panel appears relative to the trigger.

## Component Structure

```rust
// PopoverRoot owns the visibility state and positioning context for trigger + content.
PopoverRoot {
    // PopoverTrigger is the anchor: users interact with this node to open the popover.
    PopoverTrigger {
        "Show Popover"
    }
    // PopoverContent is the anchored floating panel rendered from the specified side/alignment.
    PopoverContent {
        side: ContentSide::Top,
        align: ContentAlign::Center,
        {children}
    }
}
```
