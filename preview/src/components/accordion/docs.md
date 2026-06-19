The accordion component is a compact navigation pattern for content that is naturally split into numbered or themed sections—such as step-by-step guides, grouped settings, and FAQ items—so users can focus on one chunk at a time without losing context.

In this component page, each demo shows how a parent container manages multiple disclosure items while keeping the trigger and body structure predictable for keyboard and pointer interaction.

## Component Structure

```rust
// The accordion wrapper coordinates all items so only one or many panels can be revealed according to your variant.
Accordion {
    // Every item pairs a trigger with its collapsible content.
    AccordionItem {
        // Indexes remain zero-based so keyboard order and state tracking stay deterministic.
        index: 0,
        // The trigger toggles this specific section open/closed.
        AccordionTrigger {}
        // The content is mounted only when the item is expanded in this demo layer.
        AccordionContent {}
    }
}
```
