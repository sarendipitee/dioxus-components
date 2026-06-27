`MaskInput` formats a value against a mask pattern as the user types. It is
built on the shared [`InputBase`] shell, so it inherits the label, description,
error, sections, variant, size, radius, and loading affordances of the other
inputs. The masking behaviour itself lives in the headless `use_mask` hook, which
can be attached to any native `<input>`.

## Mask pattern

The default tokens mirror Mantine's:

| Token | Matches            |
| ----- | ------------------ |
| `9`   | digit `[0-9]`      |
| `a`   | letter `[A-Za-z]`  |
| `A`   | uppercase `[A-Z]`  |
| `*`   | alphanumeric       |
| `#`   | sign or digit `[-+0-9]` |

Any other character is a literal. Prefix a token with `\` to make it a literal,
and use `?` to mark the remaining slots optional.

```rust
MaskInput {
    label: "Phone number",
    mask: "(999) 999-9999",
    on_change_raw: move |(raw, masked): (String, String)| {
        // raw = "1234567890", masked = "(123) 456-7890"
    },
}
```

## Custom tokens and transforms

Extend the token map with `tokens` and reshape input with `transform`:

```rust
fn upper(c: char) -> bool { c.is_ascii_uppercase() }

MaskInput {
    label: "License plate",
    mask: "cc-999",
    tokens: vec![('c', upper as CharPredicate)],
    transform: move |c: char| c.to_ascii_uppercase(),
}
```

## Behaviour

- `always_show_mask` shows the full mask placeholder even when empty.
- `slot_char` sets the placeholder character (e.g. `"_"`); empty disables it.
- `show_mask_on_focus` (default `true`) reveals the mask on focus.
- `auto_clear` clears the field on blur if the mask is incomplete.
- `on_complete` fires when every required slot is filled.

The hook also supports custom undo/redo (`Ctrl/Cmd+Z`, `Ctrl/Cmd+Shift+Z` /
`Ctrl/Cmd+Y`) and skips over literal characters with the arrow keys.
