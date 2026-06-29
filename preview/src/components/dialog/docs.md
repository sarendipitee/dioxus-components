The Dialog component is for momentary task switches: it overlays the current view so users can confirm an action, review item details, or complete a small form without losing their place.

In the demos below, this is the pattern to look for: the dialog is only shown when a boolean `open` flag is true, and each variant shows how title/description text guides users to the decision point before they confirm, save, or dismiss.

## Component Structure

```rust
// The dialog component must wrap all dialog elements.
Dialog {
    // Toggle visibility by binding this to your local open state.
    open: open(),
    // Use a clear action-oriented title so users understand the temporary context.
    title: "Confirm item update",
    // Provide a short description that explains consequence and next step.
    description: "Changes are staged in this dialog until you save or cancel.",
}
```
