use dioxus::prelude::*;
use dioxus_components::schedule::*;
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
                events: sample_events(),
                mode: ScheduleMode::Static,
                with_events_drag_and_drop: true,
                with_event_resize: true,
                with_drag_slot_select: true,
            }
        }
    }
}
