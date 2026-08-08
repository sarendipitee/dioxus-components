//! Defines the [`Progress`] component and its sub-components.

use dioxus::prelude::*;

/// The props for the [`Progress`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    /// The current progress value, between 0 and max.
    pub value: ReadSignal<Option<f64>>,

    /// The maximum value. Defaults to 100.
    #[props(default = ReadSignal::new(Signal::new(100.0)))]
    pub max: ReadSignal<f64>,

    /// Additional attributes to apply to the progress element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the progress component.
    pub children: Element,
}

/// # Progress
///
/// The `Progress` component shows the progress of an operation.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::progress::{Progress, ProgressIndicator};
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Progress {
///             aria_label: "Progressbar Demo",
///             value: 50.0,
///             ProgressIndicator {}
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Progress`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the state of the progress. Values are `loading` or `indeterminate`.
/// - `data-value`: The current progress value between 0 and max.
/// - `data-max`: The maximum progress value.
///
/// The [`Progress`] component defines the following css variables you can use to control styling:
/// - `--progress-value`: A value between 0 and 100 representing the current progress percentage.
#[component]
pub fn Progress(props: ProgressProps) -> Element {
    let normalized = use_memo(move || {
        let raw_max = (props.max)();
        let max = if raw_max.is_finite() {
            raw_max.max(0.0)
        } else {
            0.0
        };

        let value = props.value.cloned().map(|raw_value| {
            let value = if raw_value.is_finite() {
                raw_value.max(0.0).min(max)
            } else {
                0.0
            };
            let percentage = if max == 0.0 {
                0.0
            } else {
                (value / max) * 100.0
            };

            (value, percentage)
        });

        (max, value)
    });

    let state = use_memo(move || match normalized().1 {
        Some(_) => "loading",
        None => "indeterminate",
    });

    rsx! {
        div {
            role: "progressbar",
            "aria-valuemin": 0,
            "aria-valuemax": normalized().0,
            "aria-valuenow": normalized().1.map(|(value, _)| value),
            "data-state": state,
            "data-value": normalized().1.map(|(value, _)| value.to_string()),
            "data-max": normalized().0,
            style: normalized().1.map(|(_, percentage)| format!("--progress-value: {percentage}%")),
            ..props.attributes,

            {props.children}
        }
    }
}

/// The props for the [`ProgressIndicator`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ProgressIndicatorProps {
    /// Additional attributes to apply to the indicator element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the indicator component.
    pub children: Element,
}

/// # ProgressIndicator
///
/// The `ProgressIndicator` component represents the visual indicator that shows the progress completion.
///
/// This must be used inside a [`Progress`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::progress::{Progress, ProgressIndicator};
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Progress {
///             aria_label: "Progressbar Demo",
///             value: 50.0,
///             ProgressIndicator {}
///         }
///     }
/// }
/// ```
#[component]
pub fn ProgressIndicator(props: ProgressIndicatorProps) -> Element {
    rsx! {
        div { ..props.attributes, {props.children} }
    }
}
