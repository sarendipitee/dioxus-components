Use this component when you need multi-line, long-form input that behaves like a form field instead of a plain text block.  
The textarea demo covers two key behaviors: compact form sizing with a controlled root layout, and auto-expanding height that tracks the typed content without disrupting adjacent UI.

## Component Structure

```rust
if bottom_section.is_none() {
    textarea {
        class: "dx-textarea",
        "data-style": "default",
        {children}
    }
} else {
    div {
        class: "dx-textarea-root",
        textarea { ... }
        div {
            class: "dx-textarea-bottom-section",
            {bottom_section}
        }
    }
}
```

The structure shows how the component switches between a single `textarea` element and a wrapper with a dedicated bottom section.  
When a `bottom_section` is provided, it is rendered under the field inside the same container so helper copy, counters, validation hints, or action links stay visually tied to the input.

## API Notes

- `bottom_section` is the component’s hook for contextual metadata below the input (character counters, hints, or form status), kept as part of the same layout block.
- `autosize` dynamically adjusts the textarea height to fit current content. In autosize mode, native resize handles are intentionally disabled so JS-driven growth is authoritative.
- `min_rows` and `max_rows` let you bound that growth in row units, preventing the control from collapsing too small or expanding past layout constraints.
- Native resize behavior can be restored or customized through standard textarea attributes and inline CSS (for example, `style: "resize: both;"`) when automatic growth is not used.

## Demo Focus

This page is intentionally split into demos that show:

- A baseline multiline field for normal text entry.
- A footer slot example that keeps helper text anchored beneath the field.
- An autosizing variant with row caps that grows while preserving form rhythm.
