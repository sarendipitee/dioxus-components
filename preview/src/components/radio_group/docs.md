The `RadioGroup` component is for scenarios where users must pick exactly one option from a defined set, such as plan tiers, delivery methods, or formatting modes.
It keeps selection state in one place via the group `value` and emits every change through `on_value_change`, so your application can react in one place when the choice switches.

This component page includes demos that show:
- initializing a preselected option for a known default,
- handling user-driven selection updates across multiple options,
- and wiring child items while preserving explicit ordering and per-option semantics.

## Component Structure

```rust
// The RadioGroup component wraps all radio items in the group.
RadioGroup {
    // The value property represents the currently selected radio button in the group.
    value: "option1",
    on_value_change: |value: String| {
        // This callback is triggered when the selected radio button changes.
        // The value parameter contains the value of the newly selected radio button.
    },
    // The RadioItem component represents each individual radio button in the group.
    RadioItem {
        // The index of the radio item, used to determine the order in which items are displayed.
        index: 0,
        // The value of the radio button, which is used to identify the selected option and will be passed to the on_value_change callback when selected.
        value: "option1",
        // The contents of the radio item button
        {children}
    }
}
```
