The ContextMenu component is for commands that should be tied to a specific target area. You attach it to a trigger element so users can open a small, anchored command surface from right-click, long-press, or menu-key interaction.

In this page’s demos, the menu is used to represent workspace actions (for example “Edit”) with supporting metadata like keyboard hints, grouped stateful items, and nested choices, so you can verify both structure and behavior in a realistic context menu flow rather than as a generic list.

Key behavior shown here:
- A dedicated trigger wrapper that owns the open/close interaction for that target.
- Mixed item types in one menu: plain actions, labeled sections, toggle-style checkbox items, and nested submenu branches.
- Visual structure helpers like labels and separators that separate command groups without losing keyboard/navigation flow.

## Component Structure

```rust
// Wrap every context menu demo root with a trigger+menu pair.
ContextMenu {
    // The trigger owns the interaction surface users invoke.
    ContextMenuTrigger {
        // Any renderable content can serve as the target.
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
