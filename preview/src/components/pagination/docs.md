The pagination component in this demo targets list-heavy screens—search results, admin tables, and report pages—where users must jump to a specific position without losing context. It exposes a complete item model (`previous`, numbered links, and ellipsis placeholders) and two rendering modes: explicit composition of each piece, or automatic link generation from `total` and active state.

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

This demo shows how `Pagination` can derive links from state instead of explicit
children. Set `total` for the dataset page count and keep `value` as your active
page signal; the component calculates which numbers and truncation points to
render, including optional edges and prev/next controls. The handler receives
the newly selected page in `on_change`, which is the signal you should write back
to drive your list query.

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

The same range calculation powers both modes and is also available as
`pagination_range(total, active, siblings, boundaries)`, returning
`Vec<PaginationRangeItem>` for custom rendering pipelines. Use this helper when
you need to mirror the built-in clipping behavior in a non-`Pagination` layout.

## Notes

- `PaginationLink` uses `is_active` to indicate the current page.
- `PaginationPrevious` and `PaginationNext` show labels on larger (non-mobile) screens; labels are hidden on smaller screens to keep the control compact.
- In data-backed mode, `Pagination` clamps the active page into `1..=total` and disables the previous/next/edge controls at the boundaries.
