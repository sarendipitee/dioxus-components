The toolbar component is the action rail for command-driven interfaces: it groups related controls into one predictable region so users can move between actions without scanning the page.

This page demonstrates common toolbar patterns, from a minimal title row to dense action clusters with visual separators and independently managed buttons. Use it when you need a single locus for primary commands (formatting, navigation, state toggles, destructive actions, and grouped utilities) instead of scattering controls across layout sections.

## Component Structure

```rust
// The Toolbar component wraps all toolbar items.
Toolbar {
    // The aria_label of the toolbar, used for accessibility purposes.
    aria_label: "Toolbar Title",
    // The ToolbarButton component represents each individual button in the toolbar.
    ToolbarButton {
        // The index of the toolbar button, used to determine the order in which buttons are focused.
        index: 0,
        on_click: |_: ()| {
            // This callback is triggered when the button is clicked.
        },
        // The contents of the toolbar button
        {children}
    }
    // The ToolbarSeparator component represents a separator line in the toolbar.
    ToolbarSeparator {
        // The orientation of the separator, true for horizontal and false for vertical.
        horizontal: true,
        // The decorative property controls if the separator is decorative and should not be visible to screen readers.
        decorative: false,
    }
}
```
