use crate::component_styles;
use dioxus::prelude::*;
use dioxus_primitives::{
    dioxus_attributes::attributes,
    merge_attributes,
    progress::{self, ProgressProps},
};

#[component_styles("./style.css")]
struct Styles;

#[component]
pub fn Progress(props: ProgressProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_progress
    });
    let attributes = merge_attributes(vec![base, props.attributes]);
    rsx! {
        progress::Progress {
            attributes: attributes,
            value: props.value,
            max: props.max,
            progress::ProgressIndicator { class: Styles::dx_progress_indicator }
            {props.children}
        }
    }
}
