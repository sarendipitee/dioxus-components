The Navbar component is the styled preview of `dioxus-components`' `Navbar` primitive for building a persistent top-level navigation shell. Use it when you need a single header surface that keeps brand elements, route groups, and utility actions discoverable while the page content changes below.

This demo covers how to compose the navbar out of `Navbar`, `NavbarNav`, `NavbarTrigger`, `NavbarContent`, and `NavbarItem`, while preserving explicit menu ordering through the `index` prop and per-item handling through `on_select`.

## Component Structure

```rust
// The Navbar component wraps the entire menu bar and contains the individual menus in the order of their index.
Navbar {
    // NavbarNav contains each navigable menu group.
    NavbarNav {
        // The index of the menu, used to determine the order in which menus are displayed.
        index: 0,
        // The trigger opens a grouped menu and can host custom label content.
        NavbarTrigger {
            // The content of the trigger button.
            {children}
        }
        // The menu content holds the navigable routes or actions for that section.
        NavbarContent {
            // Each item represents a single destination or action in the section.
            NavbarItem {
                // The value of the item which will be passed to the on_select callback when the item is selected.
                value: "",
                on_select: |value: String| {
                    // Called whenever a user chooses an item; receives the selected `value`.
                },
            }
        }
    }
}
```
