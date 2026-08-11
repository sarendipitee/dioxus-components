`Collapsible` reveals one optional region without turning a page into a group of accordions. Use it when visible summary content should remain in place while secondary details expand below it.

Use the uncontrolled form for most disclosures. Use `default_open` when details should start visible, and controlled state only when another control must coordinate the same panel. For several neighboring sections, use `Accordion` instead.

## Inline Actions

Use `CollapsibleTriggerVariant::InlineActions` for a compact section label with a rotating chevron and trailing controls. Pass controls through `actions`; they render beside the trigger instead of inside its button. The chevron and controls appear on row hover or keyboard focus.

```rust
Collapsible {
    CollapsibleTrigger {
        variant: CollapsibleTriggerVariant::InlineActions,
        actions: rsx! {
            Button { aria_label: "More actions", "..." }
            Button { aria_label: "Add item", "+" }
        },
        "Recents"
    }
    CollapsibleContent { "Recent items" }
}
```

## Component Structure

```rust
// Root owns open and disabled state.
Collapsible {
    // Trigger is a button with aria-expanded and aria-controls.
    CollapsibleTrigger {}
    // Content mounts only while open unless keep_mounted is enabled.
    CollapsibleContent {}
}
```
