use crate::component_styles;
use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use dioxus_primitives::scroll_spy::{ScrollSpyData, ScrollSpyOptions};
use dioxus_primitives::table_of_contents::TableOfContents as TableOfContentsPrimitive;

#[component_styles("./style.css")]
struct Styles;

#[derive(Props, Clone, PartialEq)]
pub struct TableOfContentsProps {
    /// Options passed to [`use_scroll_spy`].
    #[props(default)]
    pub scroll_spy_options: ScrollSpyOptions,

    /// Data rendered before browser-side heading discovery completes.
    #[props(default)]
    pub initial_data: Vec<ScrollSpyData>,

    /// Minimum heading depth before indentation starts.
    #[props(default = 1)]
    pub min_depth_to_offset: u8,

    /// CSS length multiplied by each heading depth level.
    #[props(default = "20px".to_string(), into)]
    pub depth_offset: String,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn TableOfContents(props: TableOfContentsProps) -> Element {
    let base = attributes!(nav {
        class: Styles::dx_table_of_contents.to_string(),
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        TableOfContentsPrimitive {
            scroll_spy_options: props.scroll_spy_options,
            initial_data: props.initial_data,
            min_depth_to_offset: props.min_depth_to_offset,
            depth_offset: props.depth_offset,
            attributes: merged,
        }
    }
}
