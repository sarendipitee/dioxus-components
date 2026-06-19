The Switch component is for binary, immediate decisions—when a setting should be either active or inactive and a state change should happen the moment a user toggles it.

This page demonstrates how to wire `checked` and `on_checked_change` for explicit on/off behavior, and how to represent each state clearly so users can trust what they are about to enable.

## Component Structure

```rust
// The Switch component includes the switch thumb.
Switch {
    // Set the current state of the switch: `true` means on, `false` means off.
    checked: true,
    // Called with the new bool whenever the user flips the control.
    on_checked_change: |checked: bool| {
        // Handle the state transition for this specific preference.
    }
}
```

## Demo guidance

- Use the on/off demo to show a default, always-interactive toggle bound to a preference.
- Add a disabled state demo when you need to present settings that are temporarily locked, such as during loading or permission checks.
- Use labels or nearby helper text in your demo context so users can infer what the control changes before they click.
