Skeleton renders placeholder geometry that matches the layout of real content while data is loading (text lines, cards, avatars, or form fields).

In the demos on this page, each rectangle is intentionally sized and spaced to represent a specific UI region. This keeps the page structure stable during async fetches, reduces jumpiness when content appears, and gives users an immediate visual cue that the interface is actively loading.

## Component Structure

```rust
Skeleton {
    // Accepts all GlobalAttributes, commonly used with style for sizing
    style: "width: 15rem; height: 1rem;",
}
```

You can use multiple Skeleton instances to build a complete loading layout—for example, a title row followed by metadata lines and a button placeholder—by composing width and height values to mirror your final arrangement.
