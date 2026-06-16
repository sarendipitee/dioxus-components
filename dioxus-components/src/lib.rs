//! Styled Dioxus components generated from the registry component source.

use dioxus::prelude::*;

pub mod components;

pub use components::*;
pub use dioxus_attributes::component_styles;

const DIOXUS_COMPONENTS_STYLESHEET: &str =
    include_str!(concat!(env!("OUT_DIR"), "/dioxus-components.css"));

/// Inject the combined `dioxus-components` stylesheet into the document.
#[component]
pub fn DioxusComponentsStyles() -> Element {
    rsx! {
        document::Style {
            {DIOXUS_COMPONENTS_STYLESHEET}
        }
    }
}
