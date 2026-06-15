use crate::components::schedule::*;
use dioxus::prelude::*;
#[path = "../demo_support.rs"]
mod demo_support;
use demo_support::*;

#[component]
pub fn Demo() -> Element {
    let events = vec![
        ScheduleEvent {
            id: "summit".to_string(),
            title: "Spring summit".to_string(),
            start: sample_date_time(2026, 5, 14, 10, 0),
            end: sample_date_time(2026, 5, 16, 12, 0),
            all_day: false,
            color: Some("orange".to_string()),
            description: Some("Multi-day customer summit spanning three days.".to_string()),
            recurrence: None,
            drag_disabled: false,
            resize_disabled: false,
        },
        ScheduleEvent {
            id: "workshop".to_string(),
            title: "Engineering workshop".to_string(),
            start: sample_date_time(2026, 5, 13, 14, 0),
            end: sample_date_time(2026, 5, 14, 17, 0),
            all_day: false,
            color: Some("blue".to_string()),
            description: Some("Two-day hands-on workshop for engineering team.".to_string()),
            recurrence: None,
            drag_disabled: false,
            resize_disabled: false,
        },
    ];

    rsx! {
        div { style: "padding: 20px;",
            Schedule {
                default_date: sample_date(),
                default_view: ScheduleView::Week,
                events,
            }
        }
    }
}
