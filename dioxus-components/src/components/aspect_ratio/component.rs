use dioxus::prelude::*;
use crate::component_styles;
use dioxus_primitives::{
    aspect_ratio::AspectRatioProps, dioxus_attributes::attributes, merge_attributes,
};

#[component_styles("./style.css")]
struct Styles;

#[component]
pub fn AspectRatio(props: AspectRatioProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_aspect_ratio_container.to_string(),
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    dioxus_primitives::aspect_ratio::AspectRatio(AspectRatioProps {
        ratio: props.ratio,
        attributes,
        children: props.children,
    })
}
