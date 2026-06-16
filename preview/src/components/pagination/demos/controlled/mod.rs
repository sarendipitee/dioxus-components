use dioxus_components::pagination::Pagination;
use dioxus::prelude::*;

/// A data-backed pagination whose active page is owned by the parent.
///
/// Pass `total` plus a controlled `value`/`on_change` pair and the page links
/// (with truncation) are derived for you instead of being composed by hand.
#[component]
pub fn Demo() -> Element {
    let mut page = use_signal(|| Some(3usize));
    let total = 10usize;

    rsx! {
        div {
            "data-testid": "pagination-controlled-demo",
            style: "display: grid; gap: 0.75rem; justify-items: center;",
            div {
                "data-testid": "pagination-controlled-value",
                "Page {page().unwrap_or(1)} of {total}"
            }
            Pagination {
                total: Some(total),
                value: page,
                on_change: move |next| page.set(Some(next)),
                siblings: 1,
                boundaries: 1,
                with_edges: true,
            }
        }
    }
}
