use crate::component_styles;
use dioxus::prelude::*;
use dioxus_primitives::progress::{self, ProgressProps};

#[component_styles("./style.css")]
struct Styles;

#[component]
pub fn Progress(props: ProgressProps) -> Element {
    rsx! {
        progress::Progress {
            class: Styles::dx_progress.to_string(),
            value: props.value,
            max: props.max,
            attributes: props.attributes,
            progress::ProgressIndicator { class: Styles::dx_progress_indicator.to_string() }
        }
    }
}
