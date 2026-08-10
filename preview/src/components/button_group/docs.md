Use `ButtonGroup` to visually join related actions while keeping each `Button` independent. Groups support horizontal rows and vertical stacks; buttons retain native focus, disabled, and click behavior.

## Component Structure

```rust
ButtonGroup {
    Button { "Archive" }
    Button { "Report" }
}
```

Set `orientation: ButtonGroupOrientation::Vertical` for stacked actions. Use nested groups when composing larger toolbars, or `ButtonGroupSeparator` to draw a divider between adjacent buttons.

## Nesting and Composition

Nested groups keep each cluster's borders distinct while the outer group spaces them apart:

```rust
ButtonGroup {
    ButtonGroup {
        Button { variant: ButtonVariant::Outline, "Back" }
    }
    ButtonGroup {
        Button { variant: ButtonVariant::Outline, "Next" }
        Button { variant: ButtonVariant::Outline, "More" }
    }
}
```

## Separators

Draw a divider between adjacent buttons with `ButtonGroupSeparator`:

```rust
ButtonGroup {
    Button { variant: ButtonVariant::Secondary, "Save" }
    ButtonGroupSeparator {}
    Button { variant: ButtonVariant::Secondary, "Save as…" }
}
```

Separators work in both orientations and are handy when you want a crisp internal division between secondary or destructive buttons that lack an outline border.

## Custom Children

`ButtonGroup` is layout-only: each child keeps its own styling, focus behavior, disabled state, and event handlers. You can place input shells or popup triggers inside a group — a menu or popover trigger collapses to its button so it merges with neighbouring members. Keep `<DropdownMenu>` or `<Popover>` roots as direct children of the group to avoid an extra wrapper box breaking the segmented layout.