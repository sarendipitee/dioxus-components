AlertDialog is a stricter variant of Dialog for high-stakes decisions where the user **must** make an explicit choice. Three things distinguish it from Dialog:

1. **Non-dismissible** — pressing Escape or clicking the backdrop does nothing. The user must click a button.
2. **`role="alertdialog"`** — signals to screen readers that a response is required before continuing, not just a supplemental overlay.
3. **Preset button styling** — the `cancel` action renders as a secondary button and the `confirm` action renders as a destructive button, enforcing the visual hierarchy of the decision without extra configuration.

Use AlertDialog when the consequence of proceeding is severe enough that an accidental dismiss would be worse than forcing a deliberate choice.

## Component Structure

```rust
let mut open = use_signal(|| false);
rsx! {
    button { onclick: move |_| open.set(true), type: "button", "Delete account" }
    AlertDialog {
        open: open(),
        on_open_change: move |v| open.set(v),
        title: "Are you absolutely sure?",
        description: "This action cannot be undone.",
        cancel: "Cancel",
        confirm: "Yes, delete account",
        on_confirm: move |_| { /* destructive action */ },
    }
}
```

### Component Parts
- **AlertDialog**: Styled wrapper that manages open state, enforces non-dismissible behavior, and sets `role="alertdialog"`.
- **title**: Shorthand for the required accessible title. Text renders on the primitive title node.
- **description**: Shorthand for consequence text. Text renders on the primitive description node and inherits the alert dialog foreground.
- **cancel**: Safe exit button. It uses secondary button styling and closes the dialog.
- **confirm**: Affirmative button. It uses destructive button styling, fires `on_confirm`, then closes the dialog.
- **children**: Optional body content displayed between the description and action row.

The shorthand API keeps the primitive `AlertDialogTitle` and `AlertDialogDescription` wrappers internally, so `aria-labelledby` and `aria-describedby` stay wired. For lower-level composition, use the primitives from `dioxus_primitives::alert_dialog`.
