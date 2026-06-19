VirtualList is used when a page needs to scroll through many rows without paying the rendering cost of every item.  
It works by mapping scroll position to a small render window, mounting only the rows currently in view (plus a configurable overscan buffer) so input and animation stay responsive even with thousands of records.

In this component page, the demos are aimed at two practical goals:

1. show that only a narrow slice of rows is instantiated at once, which keeps startup and memory usage predictable, and  
2. show how to render each row by index when your backing data is dynamic and potentially large.

## Component Structure

```rust
VirtualList {
    // Total number of rows in the dataset.
    count: 2000usize,
    // Number of extra rows kept mounted above and below the viewport.
    buffer: 8usize,
    // Render callback for the absolute row index.
    render_item: move |idx: usize| rsx! {
        article { key: "{idx}", "{rows[idx].title}" }
    },
}
```

Use the `buffer` value to trade memory for jumpiness: a higher overscan reduces pop-in during fast scrolls, while a lower value keeps the DOM smaller.
