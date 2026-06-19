The `Collapsible` component is designed for progressive disclosure: it keeps the page visually compact while giving users control over when secondary details appear.

Use it for interfaces where users must make a choice, read supporting context, or expand configuration fields without leaving the current screen. Typical demos in this page show reveal patterns like FAQ-style content, compact option groups, and nested details areas that stay out of the way until opened.

## Component Structure

```rust
// The collapsible component wraps one disclosure unit.
Collapsible {
    // The trigger is the explicit control users click or focus to open/close the panel.
    CollapsibleTrigger {}
    // The content stays hidden until opened, then appears in-place below the trigger.
    CollapsibleContent {}
}
```
