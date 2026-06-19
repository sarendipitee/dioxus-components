Use `Item` when you need a single, consistently styled row in a list-like interface: profile cards in a contact list, selectable search hits, or setting entries with one clear headline and optional controls.

The demo below shows the canonical row composition used by this component:
- `ItemGroup` stacks rows and inserts visual separation for readability.
- `Item` wraps the row shell and chooses a visual variant and size.
- `ItemHeader` / `ItemFooter` hold optional section-level context.
- `ItemMedia` reserves a dedicated media slot for icons or images.
- `ItemContent` hosts title and description text.
- `ItemActions` places row-level controls without breaking layout balance.

## Component Structure

```rust
ItemGroup {
    Item {
        // Available variants: Default, Outline, Muted
        variant: ItemVariant::Outline,

        // Available sizes: Default, Sm
        size: ItemSize::Default,

        ItemHeader {
            "Optional header"
        }

        ItemMedia {
            // Media variants: Default, Icon, Image
            variant: ItemMediaVariant::Image,
            img { src: "/path/to/image.png", alt: "Description" }
        }

        ItemContent {
            ItemTitle { "Item title" }
            ItemDescription { "Detailed description that can span multiple lines." }
        }

        ItemActions {
            button { "Primary action" }
        }

        ItemFooter {
            "Optional footer"
        }
    }

    ItemSeparator {}

        Item {
        // ... next item in the group
    }
}
```

In this component's demos, the media slot is the only required visual anchor for the row: it can stay empty for text-first variants, or present an image/icon when identity is important. Use the `Outline` variant for bordered, card-like rows in dense settings panes and `Muted` for secondary, less prominent items.
