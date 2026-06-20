AlertDialog is a stricter variant of Dialog for high-stakes decisions where the user **must** make an explicit choice. Three things distinguish it from Dialog:

1. **Non-dismissible** — pressing Escape or clicking the backdrop does nothing. The user must click a button.
2. **`role="alertdialog"`** — signals to screen readers that a response is required before continuing, not just a supplemental overlay.
3. **Preset button styling** — `AlertDialogCancel` renders as a secondary button and `AlertDialogAction` renders as a destructive button, enforcing the visual hierarchy of the decision without extra configuration.

Use AlertDialog when the consequence of proceeding is severe enough that an accidental dismiss would be worse than forcing a deliberate choice.

## Component Structure

```rust
let mut open = use_signal(|| false);
rsx! {
    button { onclick: move |_| open.set(true), type: "button", "Delete account" }
    AlertDialog { open: open(), on_open_change: move |v| open.set(v),
        AlertDialogContent {
            AlertDialogTitle { "Are you absolutely sure?" }
            AlertDialogDescription { "This action cannot be undone." }
            AlertDialogActions {
                AlertDialogCancel { "Cancel" }
                AlertDialogAction { on_click: move |_| { /* destructive action */ }, "Yes, delete account" }
            }
        }
    }
}
```

### Component Parts
- **AlertDialog**: Root context provider — manages open state, enforces non-dismissible behavior, and sets `role="alertdialog"`.
- **AlertDialogContent**: The modal panel with backdrop. Unlike `DialogContent`, backdrop clicks and Escape are inert.
- **AlertDialogHeader**: Optional layout wrapper grouping title and description.
- **AlertDialogTitle**: The primary heading describing the decision.
- **AlertDialogDescription**: Context explaining the consequence of confirming.
- **AlertDialogActions**: Row that holds the Cancel and Action buttons.
- **AlertDialogCancel**: Safe exit path — secondary button styling, auto-closes the dialog.
- **AlertDialogAction**: Affirmative path — destructive button styling, fires `on_click` then auto-closes the dialog.
