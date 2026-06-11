The textarea element is used to allow users to enter multi-line text input in a user interface.

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

## API Notes

- `bottom_section` renders optional helper or meta content below the textarea inside the same component container.
- `autosize` grows and shrinks the textarea height to fit its rendered value. When enabled, manual resizing is disabled so native resize handles do not conflict with autosizing.
- `min_rows` and `max_rows` constrain autosize height in row units.
- Native resize behavior is controlled through standard textarea attributes or inline CSS such as `style: "resize: both;"`.
