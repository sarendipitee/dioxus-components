use dioxus::prelude::*;
use crate::component_styles;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[component_styles("./style.css")]
struct Styles;

#[component]
pub fn Skeleton(#[props(extends=GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_skeleton.to_string(),
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged }
    }
}
