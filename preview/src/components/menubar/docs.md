The Menubar component turns shared menu primitives into a desktop-style command strip you can mount at the top of an app shell. Use it for File/Edit/View-style action groups where users expect a persistent row of top-level entries and rich nested behavior.

The menubar demos below show a single, realistic menu layout: top-level menus declared in order, primary action items with labels and shortcuts, separators for grouping, checkable entries for toggles, and nested submenus for hierarchical commands.

## Component Structure

```rust
// The Menubar component wraps the entire menu bar and contains the individual menus in the order of their index.
Menubar {
    // MenubarMenu owns one top-level menu section (for example: File, Edit, Help).
    MenubarMenu {
        // The index of the menu, used to determine the order in which menus are displayed.
        index: 0,
        // The trigger label for this top-level section is rendered as the activator.
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
