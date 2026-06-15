use dioxus::prelude::*;
use crate::component_styles;
use dioxus_primitives::label::{self, LabelProps};
#[component_styles("./style.css")]
struct Styles;

#[component]
pub fn Label(props: LabelProps) -> Element {
    rsx! {
        label::Label {
            class: Styles::dx_label.to_string(),
            html_for: props.html_for,
            attributes: props.attributes,
            {props.children}
        }
    }
}
