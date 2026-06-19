The Toast component is for short, non-blocking status messages that confirm or warn about user actions without stealing focus from the main interface. Use it for transient feedback like saves, API responses, copy confirmations, or recoverable errors that should disappear automatically or stay visible while the user addresses them.

## Component Structure

The page demonstrates the standard toast context flow: a `ToastProvider` wraps interactive content, and any descendant component can request a toast through `consume_toast()`. Each call creates a message in the provider-managed queue so multiple events can be surfaced in a predictable order while keeping the UI free of heavy modal interruptions.

```rust
// The Toast provider provides the toast context to its children and handler rendering any toasts that are sent.
ToastProvider {
    // Any child component can consume the toast context and send a toast to be rendered.
    button {
        onclick: |event: MouseEvent| {
            // Consume the toast context to send a toast.
            let toast_api = consume_toast();
            toast_api
                .error(
                    "Critical Error".to_string(),
                    ToastOptions::new()
                        .description("Some info you need")
                        .duration(Duration::from_secs(60))
                        .permanent(false),
                );
        },
        "Show Toast"
    }
}
```

The demo is intentionally lightweight so you can quickly verify two behaviors:

1. The same context can trigger a toast from nested child controls.
2. Keyboard users can jump focus to the active toast area with `f6` for quick dismissal or inspection.
