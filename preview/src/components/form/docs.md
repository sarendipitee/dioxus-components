
The form component in this demo is the shell for grouped inputs that submit together.
Use it when you need one `on_submit` entry point for multiple controls, consistent field spacing, and predictable event bubbling in a single container.

## Component Structure

```rust
Form {
    on_submit: move |event| {
        // Handle submit events from this form container
    },
    children...
}
```

## Demo notes

The examples on this page focus on how the component behaves as a real wrapper:

- A full form setup showing how controls, labels, and submit controls are composed.
- A minimal usage pattern that keeps the template close to standard HTML-like submission flow while still using Dioxus component composition.
- Variants that demonstrate how submit state is read from the callback rather than from scattered control handlers.
