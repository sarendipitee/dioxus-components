use crate::component_styles;
use dioxus::prelude::*;
use dioxus_primitives::separator::{self, SeparatorProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[component_styles("./style.css")]
struct Styles;

#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_separator.to_string(),
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        separator::Separator {
            horizontal: props.horizontal,
            decorative: props.decorative,
            attributes: merged,
            {props.children}
        }
    }
}
