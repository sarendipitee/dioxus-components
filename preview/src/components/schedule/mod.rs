pub use dioxus_components::schedule::*;
#[path = "demos/demo_support.rs"]
mod demo_support;

pub(crate) use demo_support::{
    apply_demo_event_drop,
    apply_demo_event_resize,
    french_labels,
    sample_date,
    sample_events,
    workday_time_grid,
};
