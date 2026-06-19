`PillsInput` is the row-level layout primitive for multi-entry fields where each user token becomes a compact pill.
Use it to build patterns like topic tags, skill lists, email recipients, or comma-separated filter chips while keeping the input field and pill rendering co-located in a single visual line.

This component intentionally stays focused on structure and style: you manage the token source, parsing, and delete behavior in your own app logic while `PillsInput` handles consistent spacing, focus treatment, and composition with a nested `input` field.

The demos below show common setups for:

- Entering freeform tokens and converting completed entries into pill children.
- Rendering predefined pill content beside the input for editable lists.
- Keeping a short placeholder while users continue typing and remove pills via interactions in the host app.

## Component Demo Pattern

The base pattern is:

## Component Structure

```rust
PillsInput {
    Pill {
        "Rust"
    }
    input {
        r#type: "text",
        "data-pills-input-field": true,
        placeholder: "Add technology..."
    }
}
```

Use this structure as the shell for all demos: pills are explicit children, and the inner `input` is marked with `data-pills-input-field` so behavior and styling hooks can target the editable area.
