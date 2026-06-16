The DropdownMenu component renders a styled menu surface on top of the shared primitive menu APIs. It supports labels, separators, grouped items, right-side item sections, checkbox and radio items, and nested submenus.

## Component Structure

```rust
// The dropdown menu component must wrap the trigger and dropdown items.
DropdownMenu {
    // The dropdown menu trigger is an unstyled wrapper that can contain any trigger element.
    DropdownMenuTrigger {
        Button {
            "Open Menu"
        }
    }
    Menu {
        MenuLabel { "Actions" }
        MenuGroup {
            MenuItem::<String> {
                index: 0usize,
                value: "edit".to_string(),
                on_select: |_| {},
                "Edit"
                MenuItemSection { "⌘E" }
            }
        }
        MenuSeparator {}
        MenuCheckboxItem::<String> {
            value: "toolbar".to_string(),
            index: 1usize,
            checked: show_toolbar,
            on_checked_change: move |checked| show_toolbar.set(checked),
            MenuItemIndicator { visible: show_toolbar(), "✓" }
            "Show Toolbar"
        }
    }
}
```
