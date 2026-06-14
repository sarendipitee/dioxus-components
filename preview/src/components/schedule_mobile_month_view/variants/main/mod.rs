use crate::components::schedule_mobile_month_view::*;
use dioxus::prelude::*;
#[path = "../../../schedule/variants/demo_support.rs"]
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
                events: sample_events(),
                with_events_drag_and_drop: true,
                with_drag_slot_select: true,
            }
        }
    }
}
