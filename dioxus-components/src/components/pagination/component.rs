use crate::component_styles;
use dioxus::prelude::*;
use dioxus_icons::lucide::{ChevronLeft, ChevronRight, ChevronsLeft, ChevronsRight, Ellipsis};
use dioxus_primitives::use_controlled;
#[component_styles("./style.css")]
struct Styles;

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum PaginationLinkSize {
    #[default]
    Icon,
    Default,
}

impl PaginationLinkSize {
    pub fn class(&self) -> &'static str {
        match self {
            PaginationLinkSize::Icon => "icon",
            PaginationLinkSize::Default => "default",
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum PaginationLinkKind {
    First,
    Previous,
    Next,
    Last,
}

impl PaginationLinkKind {
    pub fn attr(&self) -> &'static str {
        match self {
            PaginationLinkKind::First => "first",
            PaginationLinkKind::Previous => "previous",
            PaginationLinkKind::Next => "next",
            PaginationLinkKind::Last => "last",
        }
    }
}

/// The pagination navigation landmark, usable two ways:
///
/// - **Composable** — pass [`PaginationContent`]/[`PaginationItem`]/[`PaginationLink`]
///   (and the control helpers) as children to lay the control out by hand.
/// - **Data-backed** — set `total` (and optionally a controlled `value`/`on_change`
///   pair) and the page links are derived for you, including truncation gaps. Any
///   `children` are ignored in this mode.
///
/// In data-backed mode, state follows the controlled/uncontrolled convention: pass
/// `value` to control the active page, or omit it and provide `default_value` for
/// the initial page. The active page is clamped into `1..=total`, and the
/// previous/next/edge controls are disabled at the boundaries.
#[component]
pub fn Pagination(
    /// Total number of pages. Set this to enable data-backed rendering.
    #[props(default)]
    total: Option<usize>,
    /// Controlled active page (one-based). Leave unset for uncontrolled usage.
    #[props(default)]
    value: ReadSignal<Option<usize>>,
    /// Initial active page when uncontrolled.
    #[props(default = 1)]
    default_value: usize,
    /// Fired with the next page whenever the active page changes.
    #[props(default)]
    on_change: Callback<usize>,
    /// Number of page links shown on each side of the active page.
    #[props(default = 1)]
    siblings: usize,
    /// Number of page links pinned at the start and end.
    #[props(default = 1)]
    boundaries: usize,
    /// Whether to render the previous/next controls (data-backed mode).
    #[props(default = true)]
    with_controls: bool,
    /// Whether to render the first/last edge controls (data-backed mode).
    #[props(default = false)]
    with_edges: bool,
    /// Whether the whole paginator is disabled (data-backed mode).
    #[props(default)]
    disabled: ReadSignal<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default)] children: Element,
) -> Element {
    let (active, set_active) = use_controlled(value, default_value, on_change);

    let set_page = use_callback(move |page: usize| {
        if let Some(total) = total {
            if total > 0 {
                set_active.call(page.clamp(1, total));
            }
        }
    });

    let current = active();
    let is_disabled = disabled();
    let total_pages = total.unwrap_or(0);
    let at_first = is_disabled || total_pages == 0 || current <= 1;
    let at_last = is_disabled || total_pages == 0 || current >= total_pages;
    let range = total
        .map(|total| pagination_range(total, current, siblings, boundaries))
        .unwrap_or_default();

    rsx! {
        nav {
            class: Styles::dx_pagination,
            "data-slot": "pagination",
            role: "navigation",
            aria_label: "pagination",
            ..attributes,
            if total.is_some() {
                PaginationContent {
                    if with_edges {
                        PaginationItem {
                            PaginationFirst {
                                disabled: at_first,
                                onclick: move |_| set_page.call(1),
                            }
                        }
                    }
                    if with_controls {
                        PaginationItem {
                            PaginationPrevious {
                                disabled: at_first,
                                onclick: move |_| set_page.call(current.saturating_sub(1)),
                            }
                        }
                    }
                    for (index, item) in range.iter().copied().enumerate() {
                        PaginationItem {
                            key: "{index}",
                            {
                                match item {
                                    PaginationRangeItem::Page(page) => rsx! {
                                        PaginationLink {
                                            is_active: page == current,
                                            disabled: is_disabled,
                                            aria_label: "Go to page {page}",
                                            onclick: move |_| set_page.call(page),
                                            "{page}"
                                        }
                                    },
                                    PaginationRangeItem::Dots => rsx! {
                                        PaginationEllipsis {}
                                    },
                                }
                            }
                        }
                    }
                    if with_controls {
                        PaginationItem {
                            PaginationNext {
                                disabled: at_last,
                                onclick: move |_| set_page.call(current + 1),
                            }
                        }
                    }
                    if with_edges {
                        PaginationItem {
                            PaginationLast {
                                disabled: at_last,
                                onclick: move |_| set_page.call(total_pages),
                            }
                        }
                    }
                }
            } else {
                {children}
            }
        }
    }
}

#[component]
pub fn PaginationContent(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        ul {
            class: Styles::dx_pagination_content,
            "data-slot": "pagination-content",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn PaginationItem(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        li {
            "data-slot": "pagination-item",
            ..attributes,
            {children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PaginationLinkProps {
    #[props(default)]
    pub is_active: bool,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub size: PaginationLinkSize,
    #[props(default)]
    pub data_kind: Option<PaginationLinkKind>,
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = a)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
}

#[component]
pub fn PaginationLink(props: PaginationLinkProps) -> Element {
    let aria_current = if props.is_active { Some("page") } else { None };
    let data_kind = props.data_kind.map(|kind| kind.attr());
    let tab_index = if props.disabled { "-1" } else { "0" };
    rsx! {
        a {
            class: Styles::dx_pagination_link,
            "data-slot": "pagination-link",
            "data-active": props.is_active,
            "data-disabled": props.disabled,
            "data-size": props.size.class(),
            "data-kind": data_kind,
            aria_current: aria_current,
            aria_disabled: props.disabled,
            tabindex: tab_index,
            onclick: move |event| {
                if !props.disabled {
                    if let Some(f) = &props.onclick {
                        f.call(event);
                    }
                }
            },
            onmousedown: move |event| {
                if let Some(f) = &props.onmousedown {
                    f.call(event);
                }
            },
            onmouseup: move |event| {
                if let Some(f) = &props.onmouseup {
                    f.call(event);
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}

/// Renders a first-page pagination link.
#[component]
pub fn PaginationFirst(
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes)]
    #[props(extends = a)]
    attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        PaginationLink {
            size: PaginationLinkSize::Default,
            disabled,
            aria_label: "Go to first page",
            data_kind: Some(PaginationLinkKind::First),
            onclick,
            onmousedown,
            onmouseup,
            attributes,
            ChevronsLeft { size: "1rem" }
            span { class: Styles::dx_pagination_label, "First" }
        }
    }
}

/// Renders a previous-page pagination link.
#[component]
pub fn PaginationPrevious(
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes)]
    #[props(extends = a)]
    attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        PaginationLink {
            size: PaginationLinkSize::Default,
            disabled,
            aria_label: "Go to previous page",
            data_kind: Some(PaginationLinkKind::Previous),
            onclick,
            onmousedown,
            onmouseup,
            attributes,
            ChevronLeft { size: "1rem" }
            span { class: Styles::dx_pagination_label, "Previous" }
        }
    }
}

/// Renders a next-page pagination link.
#[component]
pub fn PaginationNext(
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes)]
    #[props(extends = a)]
    attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        PaginationLink {
            size: PaginationLinkSize::Default,
            disabled,
            aria_label: "Go to next page",
            data_kind: Some(PaginationLinkKind::Next),
            onclick,
            onmousedown,
            onmouseup,
            attributes,
            span { class: Styles::dx_pagination_label, "Next" }
            ChevronRight { size: "1rem" }
        }
    }
}

/// Renders a last-page pagination link.
#[component]
pub fn PaginationLast(
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes)]
    #[props(extends = a)]
    attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        PaginationLink {
            size: PaginationLinkSize::Default,
            disabled,
            aria_label: "Go to last page",
            data_kind: Some(PaginationLinkKind::Last),
            onclick,
            onmousedown,
            onmouseup,
            attributes,
            span { class: Styles::dx_pagination_label, "Last" }
            ChevronsRight { size: "1rem" }
        }
    }
}

#[component]
pub fn PaginationEllipsis(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        span {
            class: Styles::dx_pagination_ellipsis,
            "data-slot": "pagination-ellipsis",
            aria_hidden: "true",
            ..attributes,
            Ellipsis { size: "1rem" }
            span { class: Styles::dx_sr_only, "More pages" }
        }
    }
}

/// A single slot in a computed pagination range.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PaginationRangeItem {
    /// A one-based page number rendered as a link.
    Page(usize),
    /// A truncation gap rendered as an ellipsis.
    Dots,
}

/// Computes the page slots to display for a paginator, using one-based page
/// numbers and inserting truncation gaps once the range exceeds what fits.
///
/// - `total` is the number of pages. Returns an empty vector when it is zero.
/// - `active` is the current page; it is clamped into `1..=total`.
/// - `siblings` is the number of page links shown on each side of `active`.
/// - `boundaries` is the number of page links pinned at the start and end.
///
/// Truncated gaps are represented by [`PaginationRangeItem::Dots`].
pub fn pagination_range(
    total: usize,
    active: usize,
    siblings: usize,
    boundaries: usize,
) -> Vec<PaginationRangeItem> {
    // Inclusive page range; yields nothing when `start > end` (e.g. `boundaries == 0`).
    // `start` is floored at 1 since page 0 never exists.
    fn pages(start: usize, end: usize) -> Vec<PaginationRangeItem> {
        (start.max(1)..=end)
            .map(PaginationRangeItem::Page)
            .collect()
    }

    if total == 0 {
        return Vec::new();
    }

    let active = active.clamp(1, total);

    // When everything fits, render every page with no truncation.
    let total_page_numbers = siblings * 2 + 3 + boundaries * 2;
    if total_page_numbers >= total {
        return pages(1, total);
    }

    // One-based start is always 1, so the left edge `start + boundaries - 1` reduces to `boundaries`.
    let left_sibling = active.saturating_sub(siblings).max(boundaries);
    let right_bound = total.saturating_sub(boundaries);
    let right_sibling = (active + siblings).min(right_bound);

    let should_show_left_dots = left_sibling > boundaries + 2;
    let should_show_right_dots = right_sibling < right_bound;

    // Trailing boundary block: `range(total - boundaries + 1, total)`, underflow-safe.
    let tail_start = (total + 1).saturating_sub(boundaries);

    match (should_show_left_dots, should_show_right_dots) {
        (false, true) => {
            let left_item_count = siblings * 2 + boundaries + 2;
            let mut range = pages(1, left_item_count);
            range.push(PaginationRangeItem::Dots);
            range.extend(pages(tail_start, total));
            range
        }
        (true, false) => {
            let right_item_count = boundaries + 1 + 2 * siblings;
            let mut range = pages(1, boundaries);
            range.push(PaginationRangeItem::Dots);
            range.extend(pages(total.saturating_sub(right_item_count), total));
            range
        }
        _ => {
            let mut range = pages(1, boundaries);
            range.push(PaginationRangeItem::Dots);
            range.extend(pages(left_sibling, right_sibling));
            range.push(PaginationRangeItem::Dots);
            range.extend(pages(tail_start, total));
            range
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{pagination_range, PaginationRangeItem};

    fn pages(items: &[PaginationRangeItem]) -> Vec<Option<usize>> {
        items
            .iter()
            .map(|item| match item {
                PaginationRangeItem::Page(page) => Some(*page),
                PaginationRangeItem::Dots => None,
            })
            .collect()
    }

    #[test]
    fn empty_when_no_pages() {
        assert!(pagination_range(0, 1, 1, 1).is_empty());
    }

    #[test]
    fn single_page() {
        assert_eq!(pages(&pagination_range(1, 1, 1, 1)), vec![Some(1)]);
    }

    #[test]
    fn no_truncation_when_everything_fits() {
        // siblings*2 + 3 + boundaries*2 = 7 >= 5, so all pages render.
        assert_eq!(
            pages(&pagination_range(5, 3, 1, 1)),
            vec![Some(1), Some(2), Some(3), Some(4), Some(5)]
        );
    }

    #[test]
    fn right_dots_near_start() {
        // total=10, active=1 -> 1 2 3 4 5 … 10
        assert_eq!(
            pages(&pagination_range(10, 1, 1, 1)),
            vec![Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(10)]
        );
    }

    #[test]
    fn left_dots_near_end() {
        // total=10, active=10 -> 1 … 6 7 8 9 10
        assert_eq!(
            pages(&pagination_range(10, 10, 1, 1)),
            vec![Some(1), None, Some(6), Some(7), Some(8), Some(9), Some(10)]
        );
    }

    #[test]
    fn both_dots_in_middle() {
        // total=10, active=6 -> 1 … 5 6 7 … 10
        assert_eq!(
            pages(&pagination_range(10, 6, 1, 1)),
            vec![Some(1), None, Some(5), Some(6), Some(7), None, Some(10)]
        );
    }

    #[test]
    fn clamps_active_into_range() {
        // active above total behaves like the last page.
        assert_eq!(
            pages(&pagination_range(10, 99, 1, 1)),
            pages(&pagination_range(10, 10, 1, 1))
        );
        // active at zero behaves like the first page.
        assert_eq!(
            pages(&pagination_range(10, 0, 1, 1)),
            pages(&pagination_range(10, 1, 1, 1))
        );
    }

    #[test]
    fn siblings_larger_than_total_does_not_panic() {
        assert_eq!(
            pages(&pagination_range(3, 2, 5, 1)),
            vec![Some(1), Some(2), Some(3)]
        );
    }

    #[test]
    fn zero_boundaries_does_not_panic() {
        // boundaries=0 drops the pinned edge pages, leaving dotted gaps.
        let range = pagination_range(10, 5, 1, 0);
        assert_eq!(pages(&range), vec![None, Some(4), Some(5), Some(6), None]);
    }
}
