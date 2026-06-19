The ScrollArea component creates a dedicated viewport for content that outgrows its container. It is used when a component needs a fixed-size region that can still show all content by allowing controlled vertical, horizontal, or dual-axis scrolling.

On this page, the demos show how to wrap overflow-heavy content (like long text blocks or wide sections) in a stable, styled container so it scrolls predictably across mouse wheel, touchpad, and touch gestures while preserving the surrounding layout.

Use this component when you want a reusable scroll surface inside dialogs, side panels, cards, and editors instead of relying on ad-hoc overflow styles in each view.

## Component Structure

```rust
// The ScrollArea component wraps all scrollable content.
ScrollArea {
    // The direction in which the scroll area can scroll. Can be one of Horizontal, Vertical, or Both.
    scroll_direction: ScrollDirection::Vertical,
    // The content of the scrollable area
    {children}
}
```
