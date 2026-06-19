Use `Slider` when you need a single adjustable numeric value (such as volume, opacity, playback speed, or timeout delay).  
The control converts a bounded numeric domain into a draggable thumb and emits each value change so parent state can stay in sync.

## Component Structure

```rust
Slider {
    // Choose a numeric baseline and interaction bounds for one value.
    value: 0.0,
    horizontal: true,
    step: 1.0,
    on_value_change: |value: f64| {
        // value is the current thumb position within the configured range.
    },
}
```

Use `RangeSlider` when users need to pick a start/end window (for example, date spans or budget floors/ceilings).  
Each thumb moves independently, and changes return a `Range<f64>` you can use to represent the selected interval.

```rust
RangeSlider {
    default_value: 20.0..80.0, // Start and end of the selected interval
    step: 1.0,
    min_distance: 0.0,
    on_value_change: |value: std::ops::Range<f64>| {
        // value.start and value.end are the live interval boundaries.
    },
}
```
