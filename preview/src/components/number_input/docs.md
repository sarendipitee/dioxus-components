`NumberInput` is a numeric field with built-in stepper controls, value formatting,
and optional clamping. It is built on the shared [`InputBase`] shell and therefore
inherits the label, description, error, variant, size, radius, and loading
affordances of the other inputs.

## Basic usage

```rust
let mut quantity = use_signal(|| Some(1.0_f64));

rsx! {
    NumberInput {
        label: "Quantity",
        value: quantity(),
        on_change: move |v| quantity.set(v),
        min: 0.0,
        max: 100.0,
    }
}
```

## Steppers and keyboard

The ▲ / ▼ stepper buttons and the ↑ / ↓ arrow keys increment/decrement by
`step` (default `1.0`). Pass `hide_controls: true` to hide the buttons while
keeping arrow-key behaviour.

## Decimal precision

`decimal_scale` pins the number of decimal places shown after the field blurs:

```rust
NumberInput {
    label: "Price",
    prefix: "$",
    decimal_scale: 2,
    step: 0.01,
}
```

## Custom separators

Use `thousands_separator` and `decimal_separator` for locale-aware display:

```rust
NumberInput {
    label: "Amount",
    thousands_separator: ".",
    decimal_separator: ",",
    decimal_scale: 2,
}
```

The separators are display-only — the internal value is always a plain `f64`.

## Prefix and suffix

`prefix` and `suffix` render non-editable text decorations inside the shell:

```rust
NumberInput { prefix: "$", suffix: "USD" }
```

## Clamping

The default `clamp_behavior` is `ClampBehavior::Blur` — out-of-range values are
clamped when the field loses focus. Set `ClampBehavior::Strict` to prevent the
user from typing past the bounds at all, or `ClampBehavior::None` to disable
clamping entirely.
