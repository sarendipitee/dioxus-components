The Card component is the building block for grouped UI “chunks” that need a clear information hierarchy in a small footprint. Use it when you want a consistent title area, a dedicated body region, and optional footer actions in one reusable pattern (for example: user summary panels, feature highlights, pricing snippets, and action cards).

## Component Structure (as shown in this demo)

```rust
// The Card component must wrap all card elements.
Card {
    // CardHeader contains the title, description, and optional action.
    CardHeader {
        // CardTitle displays the main heading.
        CardTitle { "Card Title" }
        // CardDescription provides supporting text.
        CardDescription { "Card description goes here." }
        // CardAction positions action elements (e.g., buttons) in the header.
        CardAction {
            Button { "Action" }
        }
    }
    // CardContent holds the main body content for readable details, metrics, or lists.
    CardContent {
        p { "Main content of the card." }
    }
    // CardFooter contains footer actions or compact metadata.
    CardFooter {
        Button { "Submit" }
    }
}
```

## Layout Notes for this component page

- Use this pattern when the card header needs both identity (`CardTitle`/`CardDescription`) and controls (`CardAction`) without extra wrapper divs.
- When `CardAction` is added to `CardHeader`, the header becomes a two-column layout so the title/description stay aligned while controls remain right-aligned.
- Keep the heaviest content in `CardContent` and reserve `CardFooter` for secondary actions, status labels, or metadata so the content order stays predictable in every demo.
