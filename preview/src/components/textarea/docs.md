The textarea element is used to allow users to enter multi-line text input in a user interface.

## Component Structure

```rust
if bottom_section.is_none() {
    textarea {
        class: "dx-textarea",
        "data-style": "default",
        "data-resize": "vertical",
        "data-autosize": "false",
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

## API Notes

- `bottom_section` renders optional helper or meta content below the textarea inside the same component container.
- `autosize` grows and shrinks the textarea height to fit its rendered value. When enabled, manual resizing is disabled so native resize handles do not conflict with autosizing.
- `min_rows` and `max_rows` constrain autosize height in row units.
- `resize` controls the native resize affordance for non-autosized textareas with `none`, `vertical`, and `both`.
