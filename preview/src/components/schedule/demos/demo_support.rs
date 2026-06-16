use crate::components::schedule::*;
use time::macros::{date, time};
use time::{Date, PrimitiveDateTime};

/// Returns the shared reference date used by schedule preview demos.
pub(super) fn sample_date() -> Date {
    date!(2026 - 05 - 12)
}

/// Returns the shared sample events rendered by schedule preview demos.
pub(super) fn sample_events() -> Vec<ScheduleEvent> {
    vec![
        ScheduleEvent {
            id: "planning".to_string(),
            title: "Planning sync".to_string(),
            start: sample_date_time(2026, 5, 12, 9, 0),
            end: sample_date_time(2026, 5, 12, 10, 0),
            all_day: false,
            color: Some("blue".to_string()),
            description: Some("Weekly planning with design and engineering.".to_string()),
            recurrence: None,
            drag_disabled: false,
            resize_disabled: false,
        },
        ScheduleEvent {
            id: "review".to_string(),
            title: "Design review".to_string(),
            start: sample_date_time(2026, 5, 12, 9, 30),
            end: sample_date_time(2026, 5, 12, 11, 0),
            all_day: false,
            color: Some("pink".to_string()),
            description: Some("Review interaction polish for the release candidate.".to_string()),
            recurrence: None,
            drag_disabled: false,
            resize_disabled: false,
        },
        ScheduleEvent {
            id: "lunch".to_string(),
            title: "Team lunch".to_string(),
            start: sample_date_time(2026, 5, 13, 12, 0),
            end: sample_date_time(2026, 5, 13, 13, 0),
            all_day: false,
            color: Some("green".to_string()),
            description: Some("Casual lunch with the product team.".to_string()),
            recurrence: None,
            drag_disabled: false,
            resize_disabled: false,
        },
        ScheduleEvent {
            id: "offsite".to_string(),
            title: "Leadership offsite".to_string(),
            start: sample_date_time(2026, 5, 15, 0, 0),
            end: sample_date_time(2026, 5, 16, 0, 0),
            all_day: true,
            color: Some("purple".to_string()),
            description: Some("All-day offsite shown in the all-day row.".to_string()),
            recurrence: None,
            drag_disabled: false,
            resize_disabled: false,
        },
        ScheduleEvent {
            id: "standup".to_string(),
            title: "Daily standup".to_string(),
            start: sample_date_time(2026, 5, 12, 10, 30),
            end: sample_date_time(2026, 5, 12, 10, 45),
            all_day: false,
            color: Some("teal".to_string()),
            description: Some("Recurring daily delivery sync.".to_string()),
            recurrence: Some(ScheduleRecurrence {
                frequency: ScheduleRecurrenceFrequency::Daily,
                interval: 1,
                count: Some(5),
                until: None,
            }),
            drag_disabled: false,
            resize_disabled: false,
        },
        ScheduleEvent {
            id: "office-hours".to_string(),
            title: "Office hours".to_string(),
            start: sample_date_time(2026, 5, 11, 15, 0),
            end: sample_date_time(2026, 5, 11, 17, 0),
            all_day: false,
            color: Some("yellow".to_string()),
            description: Some("Recurring weekly support window.".to_string()),
            recurrence: Some(ScheduleRecurrence {
                frequency: ScheduleRecurrenceFrequency::Weekly,
                interval: 1,
                count: Some(4),
                until: None,
            }),
            drag_disabled: false,
            resize_disabled: false,
        },
    ]
}

/// Returns a workday-oriented time grid configuration used by several preview demos.
pub(super) fn workday_time_grid() -> ScheduleTimeGridConfig {
    ScheduleTimeGridConfig {
        with_default_header: true,
        start_hour: 8,
        end_hour: 18,
        slot_minutes: 30,
    }
}

/// Returns localized French labels used by the internationalized preview demo.
pub(super) fn french_labels() -> ScheduleLabels {
    ScheduleLabels {
        previous: "Précédent".to_string(),
        next: "Suivant".to_string(),
        today: "Aujourd’hui".to_string(),
        day: "Jour".to_string(),
        week: "Semaine".to_string(),
        month: "Mois".to_string(),
        year: "Année".to_string(),
        all_day: "Toute la journée".to_string(),
        empty_slot: "Aucun événement".to_string(),
    }
}

/// Applies a dropped event payload to the preview-owned sample events.
pub(super) fn apply_demo_event_drop(
    events: &mut Vec<ScheduleEvent>,
    payload: &ScheduleEventDrop,
    _limit: ScheduleRecurrenceExpansionLimit,
) {
    let mut updated = payload.event.clone();
    updated.start = payload.new_start;
    updated.end = payload.new_end;
    updated.all_day = payload.destination == ScheduleDropDestination::AllDay;
    updated.recurrence = None;
    replace_demo_event(events, &payload.event, updated);
}

/// Applies a resize payload to the preview-owned sample events.
pub(super) fn apply_demo_event_resize(
    events: &mut Vec<ScheduleEvent>,
    payload: &ScheduleEventResize,
    _limit: ScheduleRecurrenceExpansionLimit,
) {
    let mut updated = payload.event.clone();
    updated.start = payload.new_start;
    updated.end = payload.new_end;
    updated.recurrence = None;
    replace_demo_event(events, &payload.event, updated);
}

fn replace_demo_event(
    events: &mut Vec<ScheduleEvent>,
    original: &ScheduleEvent,
    updated: ScheduleEvent,
) {
    if let Some(index) = events.iter().position(|event| {
        event.id == original.id && event.start == original.start && event.end == original.end
    }) {
        events[index] = updated;
        return;
    }

    if let Some(index) = events.iter().position(|event| event.id == original.id) {
        events[index] = updated;
        return;
    }

    events.push(updated);
}

pub(super) fn sample_date_time(year: i32, month: u8, day: u8, hour: u8, minute: u8) -> PrimitiveDateTime {
    PrimitiveDateTime::new(
        Date::from_calendar_date(
            year,
            month.try_into().expect("valid preview month"),
            day,
        )
        .expect("valid preview date"),
        time!(00:00)
            .replace_hour(hour)
            .expect("valid preview hour")
            .replace_minute(minute)
            .expect("valid preview minute"),
    )
}
