The Menubar component renders a styled application menubar on top of the shared menu primitive APIs. Top-level menus can contain labels, separators, checkable items, radio groups, right-side item sections, and nested submenus.

## Component Structure

```rust
// The Menubar component wraps the entire menu bar and contains the individual menus in the order of their index.
Menubar {
    // The MenubarMenu contains the individual menus that can be opened.
    MenubarMenu {
        // The index of the menu, used to determine the order in which menus are displayed.
        index: 0,
        // The menubar trigger is the element that will display the menu when activated.
        MenubarTrigger {
            // The content of the trigger button
            {children}
        }
        Menu {
            MenuItem {
                index: 0usize,
                value: "new".to_string(),
                on_select: |_| {},
                "New"
                MenuItemSection { "⌘N" }
            }
            MenuSeparator {}
            MenuCheckboxItem {
                value: "status_bar".to_string(),
                index: 1usize,
                checked: status_bar,
                on_checked_change: move |checked| status_bar.set(checked),
                MenuItemIndicator { visible: status_bar(), "✓" }
                "Status bar"
            }
        }
    }
}
```
