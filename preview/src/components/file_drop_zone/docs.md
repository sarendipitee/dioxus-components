FileDropZone is the styled intake wrapper for dropping or selecting files in workflows like asset uploads, document import wizards, and media pickers. It handles the underlying file input and drag behavior, then returns `accepted` and `rejected` results so your app can choose the next step (preview, upload, retry, or annotate).

The main goal of this component is controlled intake UX: it gives users a predictable drop surface while letting you keep upload logic and business rules outside of the visual layer.

## Component Structure

```rust
FileDropZone {
    on_accepted: move |accepted: Vec<AcceptedFile>| {
        // store or upload the accepted files
    },
    p { "Drop files here or click to select" }
}
```

### Demo components and interaction states

- `FileDropZone`: The outer drop zone wrapper. It forwards key primitive props and callbacks (`accept`, `multiple`, `min_size`, `max_size`, `max_files`, `disabled`, `loading`, `open_request`, `on_accepted`, `on_rejected`, and `on_drop`) so each demo can exercise a specific behavior.
- `FileDropZoneIdle`: Content used when no drag interaction is active.
- `FileDropZoneAcceptDisplay`: Content used while dragged files pass validation and are currently droppable.
- `FileDropZoneRejectDisplay`: Content used while dragged files violate any active rule and should be rejected.

### Validation behavior

Selection events are validated against `accept`, `min_size`, `max_size`, and `max_files`. Rejected files are reported through `on_rejected` with stable [`RejectionCode`] values and clear message text, which makes this page ideal for testing user-facing failure states like unsupported type, too-large files, and file-count limits.
