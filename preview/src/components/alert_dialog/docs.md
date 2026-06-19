The AlertDialog component is for interrupting the user with a focused, high-stakes confirmation flow: one action must be explicitly approved before it can continue. Its primary demos model destructive and irreversible workflows (delete/archive, permission changes, and irreversible submissions) so users can back out or confirm with clear intent.

## Component Structure

```rust
let mut open = use_signal(|| false);
rsx! {
    button { onclick: move |_| open.set(true), type: "button", "Show Alert Dialog" }
    AlertDialog { open: open(), on_open_change: move |v| open.set(v),
        AlertDialogTitle { "Title" }
        AlertDialogDescription { "Description" }
        AlertDialogActions {
            AlertDialogCancel { "Cancel" }
            AlertDialogAction { on_click: move |_| { /* destructive action */ }, "Confirm" }
        }
    }
}
```

### Component Parts
- **AlertDialog**: Provides the dialog context, handles open state, and owns the modal shell with overlay and focus trap behavior.
- **AlertDialogTitle**: The primary heading that communicates exactly what decision the user is about to make.
- **AlertDialogDescription**: Context text that explains the consequence of confirming.
- **AlertDialogActions**: The action bar that keeps confirmation and cancellation controls together.
- **AlertDialogAction**: The affirmative button for the critical action (for example, delete, archive, or submit), closing the dialog and running optional `on_click`.
- **AlertDialogCancel**: The safe exit path for reversing the interaction, also closing the dialog and running optional `on_click`.
