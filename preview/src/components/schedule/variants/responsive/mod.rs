use crate::components::schedule::*;
use dioxus::prelude::*;
#[path = "../demo_support.rs"]
mod demo_support;
use demo_support::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "padding: 20px;",
            Schedule {
                default_date: sample_date(),
                default_view: ScheduleView::Week,
                layout: ScheduleLayout::Responsive,
                with_events_drag_and_drop: true,
                with_drag_slot_select: true,
            }
        }
    }
}
