The Sidebar component in this page is the canonical navigation shell for multi-section apps: it anchors core entry points, tool actions, and context filters to an edge while keeping the main work area readable.

The demo pages below show how to combine sidebar primitives into concrete layouts, including:

- fixed side placement on the left or right,
- floating vs. inset chrome variants,
- three collapse modes (off-canvas, icon-only rail, and non-collapsible), and
- nested menu patterns with optional actions, badges, and tooltips.

## Component Structure

```rust
// Provider: supplies open/side/collapsible signals and ⌘/Ctrl+B toggle
SidebarProvider {
    Sidebar {
        side: SidebarSide::Left,                     // left/right placement
        variant: SidebarVariant::Sidebar,            // chrome: Sidebar | Floating | Inset
        collapsible: SidebarCollapsible::Offcanvas,  // behavior: Offcanvas | Icon | None

        // Layout - Header
        SidebarHeader {
            SidebarTrigger {}                        // toggle button (as)
        }

        // Layout - Scrollable content area
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

        // Layout -  Footer
        SidebarFooter {
            SidebarMenu { /* ... */ }
        }
    }

    // Optional desktop rail controller placed between rail and content
    SidebarRail {}                               // draggable resize handle

    // Layout - Main content area beside the rail
    SidebarInset { /* ... */ }
}
```

## Behaviors
- Layout control starts at the provider and `Sidebar`: use `side` to attach to left or right edge, then switch `variant` for a floating or inset container treatment without changing menu composition.
- Collapse behavior is controlled by `collapsible` (`Offcanvas`, `Icon`, `None`) so you can test wide desktop rails, compact icon strips, or fully static sidebars in place.
- Keyboard support and accessibility are built in: ⌘/Ctrl+B toggles from the provider, and focus visibility is handled through the focus-visible styles declared in `sidebar/style.css`.
- Menu tooltip behavior is per-item via `SidebarMenuButton { tooltip }`; passing `None` keeps the item label-only and avoids tooltip wrappers in dense icon-only layouts.

## Custom Rendering with `as`
Supported components: `SidebarTrigger`, `SidebarGroupLabel`, `SidebarGroupAction`, `SidebarMenuButton`, `SidebarMenuAction`, `SidebarMenuSubButton`. Use `as: |attrs| rsx! { ... }` and spread `..attrs` to preserve merged attributes, state markers, and event handlers while swapping in your own rendering.

## Why these demos exist
The sidebar page is intentionally split into small scenarios so you can validate a real navigation pattern quickly: placement, chrome variants, collapse ergonomics, and nested menu behavior under keyboard interaction.
