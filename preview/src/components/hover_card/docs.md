The HoverCard component creates a non-intrusive way to reveal rich context for any trigger element, ideal for situations where users need extra detail but should never lose their place. Typical uses in this page include profile previews, compact metadata callouts, and action cues that appear next to the element being pointed at.

## Component Structure

```rust
// The HoverCard component wraps the trigger element and the content that will be displayed on hover.
HoverCard {
    HoverCardTrigger {
        // Anything inside the trigger (icon, text, rich markup) acts as the hover target.
        {children}
    }
    HoverCardContent {
        side: ContentSide::Bottom,
        align: ContentAlign::Start,
        {children}
    }
}
```

Use the code above as the foundation for each demo on this page: place the element that should activate the card in `HoverCardTrigger`, then define the exact panel content and placement in `HoverCardContent`. Adjust `side` and `align` per demo when you want the popover to mirror the trigger's position in dense layouts.
