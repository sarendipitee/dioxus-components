The Separator component creates a dedicated visual boundary between related blocks of content while preserving interaction flow. Use it when you need a clear pause between controls, list sections, or card regions—like splitting a settings form from its action buttons, breaking a nav menu into categories, or dividing metadata rows in a compact panel.

## When to use this component

Choose `Separator` when the boundary itself should be decorative or structural, not interactive:

- Separate grouped settings or form fields without introducing extra spacing or new containers.
- Segment horizontal content strips (toolbars, cards, and card footers) so users can scan sections faster.
- Add vertical dividers for multi-column layouts where columns should remain visually distinct but semantically connected.

## Component Structure

```rust
// Use Separator to draw a horizontal or vertical divider line.
Separator {
    // Render a horizontal divider by default; set this to false for a vertical divider.
    horizontal: true,
    // Keep decorative true when the divider is purely presentational and should be skipped by assistive technology.
    decorative: false,
}
```

## Demo intent for this page

This page demonstrates how the same component can either:

- create repeated horizontal spacing rhythm in stacked content, and
- support vertical separation in dense split layouts.

Use the examples below to evaluate contrast, density, and how separator placement affects grouping in your interface.
