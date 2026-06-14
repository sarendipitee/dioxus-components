use crate::components::schedule_events::*;
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
                events: sample_events(),
            }
        }
    }
}
