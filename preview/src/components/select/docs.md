The Select component renders a controlled dropdown field where one selected value is tracked and propagated through `on_value_change`. This page demonstrates how to use Select for menu-style choices (including grouped options) while keeping keyboard behavior, focus handling, and value updates explicit and predictable.

Use the demos on this page to compare the base composition pattern with grouped sections and option entries. The examples are useful when you need a single-selection UI that feels like a form control: a compact trigger, a structured option list, and a clear callback when the active selection changes.

## Component Structure

```rust
Select::<String> {
    value: "option1",
    on_value_change: |value: String| {
        // Handle the change in selected value.
    },
    SelectGroup {
        SelectGroupLabel { "Group A" }
        SelectOption::<String> {
            index: 0,
            value: "option1",
            "Option 1"
        }
    }
}
```
