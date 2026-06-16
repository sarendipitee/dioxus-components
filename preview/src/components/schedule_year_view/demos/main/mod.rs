use crate::components::schedule_year_view::*;
use dioxus::prelude::*;
#[path = "../../../schedule/demos/demo_support.rs"]
mod demo_support;
use demo_support::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "padding: 20px;",
            Schedule {
                default_date: sample_date(),
                default_view: ScheduleView::Year,
                events: sample_events(),
            }
        }
    }
}
