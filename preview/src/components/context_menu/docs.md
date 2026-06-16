The ContextMenu component renders a styled contextual action menu on top of the shared menu primitive APIs. It supports labels, separators, checkbox and radio items, right-side item sections, and nested submenus.

## Component Structure

```rust
// The context menu component must wrap all context menu items.
ContextMenu {
    // The context menu trigger is the element that will display the context menu when right-clicked.
    ContextMenuTrigger {
        // The content of the trigger
        {children}
    }
    Menu {
        MenuLabel { "Canvas" }
        MenuItem {
            index: 0usize,
            value: "edit".to_string(),
            on_select: |_| {},
            "Edit"
            MenuItemSection { "⌘E" }
        }
        MenuSeparator {}
        MenuCheckboxItem {
            value: "line_numbers".to_string(),
            index: 1usize,
            checked: show_line_numbers,
            on_checked_change: move |checked| show_line_numbers.set(checked),
            MenuItemIndicator { visible: show_line_numbers(), "✓" }
            "Show line numbers"
        }
    }
}
```
