The pagination component provides navigational controls for paged content. It exposes a consistent structure for previous/next actions, individual page links, and an optional ellipsis for truncated ranges. The same `Pagination` component works two ways: compose the parts by hand, or set `total` to have the page links derived from your data.

## Component Structure

```rust
// The Pagination component wraps the entire control.
Pagination {
    // PaginationContent groups all items in a horizontal list.
    PaginationContent {
        // PaginationItem is the container for a single pagination element.
        // Use one item at a time and swap the inner component as needed.
        PaginationItem {
            // PaginationPrevious renders a previous-page link.
            // - Set href to your previous page url.
            PaginationPrevious { href: "#" }

            // PaginationLink renders a numbered page link.
            // - is_active marks the current page.
            // - href sets the target page.
            PaginationLink { href: "#", is_active: true, "2" }

            // PaginationEllipsis indicates truncated pages.
            PaginationEllipsis {}

            // PaginationNext renders a next-page link.
            // - Set href to your next page url.
            PaginationNext { href: "#" }
        }
    }
}
```

## Data-backed control

Instead of composing the parts by hand, set `total` on `Pagination` and the page
links are derived from your data. Give it the total number of pages and a
controlled active page, and it computes the visible range (including truncation
ellipses) and emits the next page through `on_change`. Any children are ignored
in this mode.

```rust
let mut page = use_signal(|| Some(3usize));

Pagination {
    // Total number of pages; setting it enables data-backed rendering.
    total: Some(10),
    // Controlled active page; omit for uncontrolled usage with default_value.
    value: page,
    on_change: move |next| page.set(Some(next)),
    // Page links shown on each side of the active page (default 1).
    siblings: 1,
    // Page links pinned at the start and end (default 1).
    boundaries: 1,
    // Show first/last edge controls (default false).
    with_edges: true,
    // Show previous/next controls (default true).
    with_controls: true,
}
```

The same range algorithm is exposed as the pure function
`pagination_range(total, active, siblings, boundaries)`, which returns a
`Vec<PaginationRangeItem>` of page numbers and truncation gaps if you need to
build a custom layout.

## Notes

- `PaginationLink` uses `is_active` to indicate the current page.
- `PaginationPrevious` and `PaginationNext` show labels on larger (non-mobile) screens; labels are hidden on smaller screens to keep the control compact.
- In data-backed mode, `Pagination` clamps the active page into `1..=total` and disables the previous/next/edge controls at the boundaries.
