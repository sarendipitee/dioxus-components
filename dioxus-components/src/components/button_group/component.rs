use crate::component_styles;
use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[component_styles("./style.css")]
struct Styles;

/// Layout direction for [`ButtonGroup`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum ButtonGroupOrientation {
    /// Place buttons in one horizontal row.
    #[default]
    Horizontal,
    /// Stack buttons in one vertical column.
    Vertical,
}

impl ButtonGroupOrientation {
    fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// Groups related buttons into a shared segmented control layout.
///
/// `ButtonGroup` only controls layout and visual merging. Each child button
/// retains its normal native semantics, focus behavior, disabled state, and
/// event handlers. Nest groups to keep clusters visually isolated, or add
/// [`ButtonGroupSeparator`] to draw a divider between adjacent buttons.
///
/// # Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_components::button::{Button, ButtonVariant};
/// use dioxus_components::button_group::ButtonGroup;
///
/// #[component]
/// fn Actions() -> Element {
///     rsx! {
///         ButtonGroup {
///             Button { variant: ButtonVariant::Outline, "Archive" }
///             Button { variant: ButtonVariant::Outline, "Report" }
///         }
///     }
/// }
/// ```
#[component]
pub fn ButtonGroup(
    #[props(default)] orientation: ButtonGroupOrientation,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        role: "group",
        class: Styles::dx_button_group,
        "data-orientation": orientation.as_str(),
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div {
            ..merged,
            {children}
        }
    }
}

/// Vertical divider drawn between adjacent buttons in a [`ButtonGroup`].
///
/// Primarily intended for buttons without a visible outline border (for
/// example `secondary` or `destructive` variants) where the group needs a
/// crisp internal division. Outline buttons already provide their own border,
/// so a separator is optional for them.
///
/// # Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_components::button::{Button, ButtonVariant};
/// use dioxus_components::button_group::{ButtonGroup, ButtonGroupSeparator};
///
/// #[component]
/// fn SplitAction() -> Element {
///     rsx! {
///         ButtonGroup {
///             Button { variant: ButtonVariant::Secondary, "Save" }
///             ButtonGroupSeparator {}
///             Button { variant: ButtonVariant::Secondary, "Save as…" }
///         }
///     }
/// }
/// ```
#[component]
pub fn ButtonGroupSeparator(
    #[props(default)] orientation: ButtonGroupOrientation,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let base = attributes!(div {
        role: "separator",
        class: Styles::dx_button_group_separator,
        "aria-orientation": orientation.as_str(),
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div {
            ..merged
        }
    }
}
