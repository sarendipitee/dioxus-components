# DropdownMenu Demo

Use this page to preview the dropdown menu as an action surface anchored to a trigger element. The component is intended for command-style interactions where users select grouped actions, jump to nested options, or flip inline state in one compact overlay.

The demos below focus on practical menu behavior instead of generic layout: labeled sections for scan-friendly organization, check/radio items for local state, and nested entries that keep deeper tasks discoverable without leaving the current context.

```rust
// The dropdown menu component wraps a trigger and a menu surface in a single semantic flow.
DropdownMenu {
    // The trigger can host any element; the examples keep it as a button for a clear entry point.
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

This example demonstrates the core pattern you should reuse for real feature menus: a trigger, a labeled set of grouped actions, then mixed item types (plain, checkbox, and separators) inside the same menu shell.
