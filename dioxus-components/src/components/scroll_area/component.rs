use dioxus::prelude::*;
use dioxus_primitives::{
    dioxus_attributes::attributes,
    merge_attributes,
    scroll_area::{self, ScrollAreaProps},
};

#[css_module("/src/components/scroll_area/style.css")]
struct Styles;

#[component]
pub fn ScrollArea(props: ScrollAreaProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_scroll_area.to_string(),
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    scroll_area::ScrollArea(ScrollAreaProps {
        direction: props.direction,
        always_show_scrollbars: props.always_show_scrollbars,
        scroll_type: props.scroll_type,
        attributes,
        children: props.children,
    })
}
