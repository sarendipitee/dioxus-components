use crate::components::schedule_day_view::*;
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
                default_view: ScheduleView::Day,
                events: sample_events(),
                day_view: ScheduleDayViewConfig {
                    time_grid: ScheduleTimeGridConfig {
                        start_hour: 8,
                        end_hour: 18,
                        slot_minutes: 30,
                        with_default_header: true,
                    },
                },
            }
        }
    }
}
