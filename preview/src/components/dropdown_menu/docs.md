The DropdownMenu component is used to create a dropdown menu that can be triggered by a button click. It allows users to select an option from a list of items.

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
    // The dropdown menu content contains all the items that will be displayed in the dropdown menu.
    DropdownMenuContent {
        // Each dropdown menu item represents an individual option in the dropdown menu. Items are displayed in order based on the order of the index property.
        DropdownMenuItem {
            // The index of the item, used to determine the order in which items are displayed.
            index: 0,
            // The value of the item which will be passed to the on_select callback when the item is selected.
            value: "",
            on_select: |value: String| {
                // This callback is triggered when the item is selected.
                // The value parameter contains the value of the selected item.
            },
        }
    }
}
```
