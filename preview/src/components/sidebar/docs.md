Sidebar provides the navigation layout commonly used in dashboards and other multi-page applications. It can sit on either side of the page and hold navigation links, actions, filters, and account controls.

These examples cover:

- left- and right-aligned sidebars,
- standard, floating, and inset variants,
- off-canvas, icon-only, and fixed layouts, and
- nested menus with actions, badges, and tooltips.

## Component Structure

```rust
// Provides sidebar state and the ⌘/Ctrl+B keyboard shortcut
SidebarProvider {
    Sidebar {
        side: SidebarSide::Left,                     // left/right placement
        variant: SidebarVariant::Sidebar,            // Sidebar | Floating | Inset
        collapsible: SidebarCollapsible::Offcanvas,  // behavior: Offcanvas | Icon | None
        min_width: 192.0,                          // desktop resize lower bound in px
        max_width: 480.0,                          // desktop resize upper bound in px

        // Header
        SidebarHeader {
            SidebarTrigger {}                        // toggle button (as)
        }

        // Scrollable content
        SidebarContent {
            SidebarGroup {
                SidebarGroupLabel { "..." }          // optional label (as)
                SidebarGroupAction { "..." }         // optional action (as)
                SidebarGroupContent {                // wraps menus
                    SidebarMenu {
                        SidebarMenuItem {
                            SidebarMenuButton {      // primary item (as)
                                is_active: true,     // highlight state
                                tooltip: rsx!("..."),// Option<Element>; wraps tooltip only when Some
                                Icon {}              // icon node
                                span { "..." }       // text node
                            }
                            SidebarMenuAction { show_on_hover: true, Icon {} } // trailing action (as)
                            SidebarMenuBadge { "+..." }                        // optional badge
                        }
                        SidebarMenuItem {            // nested submenu
                            SidebarMenuSub {
                                SidebarMenuSubItem {
                                    SidebarMenuSubButton { "..." } // submenu button/link (as)
                                }
                            }
                        }
                    }
                }
            }
        }

        // Footer
        SidebarFooter {
            SidebarMenu { /* ... */ }
        }
    }
    // Optional resize handle for desktop layouts
    SidebarRail {}

    // Main content
    SidebarInset { /* ... */ }
}
```

## Behavior

- Set `side` to `Left` or `Right` to choose where the sidebar appears.
- Set `variant` to `Sidebar`, `Floating`, or `Inset` to change how it sits alongside the page.
- Set `collapsible` to `Offcanvas`, `Icon`, or `None` to hide the sidebar, reduce it to icons, or keep it open.
- Press ⌘/Ctrl+B to toggle the sidebar. Focus indicators are included for keyboard users.
- Add a `tooltip` to `SidebarMenuButton` when its label needs to remain available in the icon-only layout.

## Custom rendering with `as`

`SidebarTrigger`, `SidebarGroupLabel`, `SidebarGroupAction`, `SidebarMenuButton`, `SidebarMenuAction`, and `SidebarMenuSubButton` support the `as` prop. Return your custom element from the callback and spread `..attrs` onto it so the component keeps its attributes, state, and event handlers.
