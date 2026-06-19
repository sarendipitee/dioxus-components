The Tooltip component shows a compact info bubble for dense interfaces where controls need explanation but no permanent label can fit.

Use it on trigger elements like icon buttons, tags, or truncated text to reveal secondary context exactly when the user hovers or focuses. The examples in this page illustrate how side and alignment settings affect placement in constrained layouts, so you can keep overlays legible and predictable.

## Component Structure

```rust
// The Tooltip component wraps both the interactive trigger and the overlay content.
Tooltip {
    // The TooltipTrigger contains the element that should reveal the tooltip on hover or keyboard focus.
    TooltipTrigger {
        // Trigger elements can be icon buttons, status chips, or any compact control.
        {children}
    }
    // TooltipContent defines the contextual copy that appears beside the trigger.
    TooltipContent {
        // The side of the trigger where the bubble appears. Try Top, Right, Bottom, or Left.
        side: ContentSide::Top,
        // Align the bubble relative to the trigger when space allows: Start, Center, or End.
        align: ContentAlign::Center,
        // Keep tooltip content short: a sentence, action hint, or validation hint is usually best.
        {children}
    }
}
```
