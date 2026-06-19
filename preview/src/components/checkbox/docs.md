Use the Checkbox component when users need to make a single on/off decision for one item at a time: selecting a task, accepting terms, or toggling an option in a settings row.

This demo page shows how the control behaves across the key interactions you will actually ship in UI: checked vs. unchecked state, disabled mode, and the way the indicator slot renders only when the control is selected.

Underneath, a checkbox is composed from a trigger element plus an indicator element, so you can swap indicator content (for example a check icon, checkmark text, or custom glyph) while keeping keyboard focus and accessibility behavior in one place.

## Component Structure

```rust
// A minimal checkbox pattern: a focusable checkbox trigger with an optional indicator.
Checkbox {
    // The indicator only renders visible content when the checkbox state is checked.
    CheckboxIndicator {
        // Typical indicator content for the checked state.
        {children}
    }
}
```
