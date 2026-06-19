The AspectRatio component reserves a proportional box for its children so that previews, video frames, and card artwork keep their intended shape as their container width changes.

Use it whenever a fixed visual proportion matters more than absolute height: for example, a 16:9 hero strip, an image tile, or a media card in a responsive grid. The examples on this page show how adjusting the `ratio` affects the rendered frame while content stays scaled and aligned to the surrounding layout.

## Component Structure

```rust
AspectRatio {
    // The aspect ratio to maintain (width / height)
    ratio: 16. / 9.,
    // The children of the AspectRatio component will be rendered within it.
    {children}
}
```

Try changing the ratio to match common media dimensions (4:3, 1:1, or 21:9) and watch each demo panel keep its frame stable while the parent width updates.
