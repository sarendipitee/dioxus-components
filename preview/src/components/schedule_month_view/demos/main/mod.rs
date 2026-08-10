use dioxus::prelude::*;
use dioxus_components::schedule::*;
#[path = "../../../schedule/demos/demo_support.rs"]
mod demo_support;
use demo_support::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "padding: 20px;",
            Schedule {
                default_date: sample_date(),
                default_view: ScheduleView::Month,
                events: sample_events(),
            }
        }
    }
}
