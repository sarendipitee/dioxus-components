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
                default_view: ScheduleView::Week,
                events: sample_events(),
                week_view: ScheduleWeekViewConfig {
                    time_grid: ScheduleTimeGridConfig {
                        start_hour: 7,
                        end_hour: 18,
                        slot_minutes: 30,
                        with_default_header: true,
                    },
                    ..ScheduleWeekViewConfig::default()
                },
            }
        }
    }
}
