The FileDropZone component is a styled file selection and drag-and-drop zone. It owns a hidden file input and handles click, keyboard (Enter/Space), and drag/drop selection, then runs validation and reports structured accepted and rejected files. It is not an uploader: you receive the selected files and decide what to do with them.

## Component Structure

```rust
FileDropZone {
    on_accepted: move |accepted: Vec<AcceptedFile>| {
        // store or upload the accepted files
    },
    p { "Drop files here or click to select" }
}
```

### Components

- `FileDropZone`: The outer drop zone. Forwards every primitive prop (`accept`, `multiple`, `max_size`, `max_files`, `disabled`, `loading`, `open_request`, and the `on_accepted` / `on_rejected` / `on_drop` callbacks).
- `FileDropZoneIdle`: Children shown while the zone is idle (not accepting or rejecting a drag).
- `FileDropZoneAcceptDisplay`: Children shown only while a drag appears acceptable.
- `FileDropZoneRejectDisplay`: Children shown only while a drag appears unacceptable.

### Validation

Selected files are validated against `accept`, `min_size`, `max_size`, and `max_files`. Rejected files are reported through `on_rejected` with stable [`RejectionCode`] codes and human-readable messages.
