# Events data

Events are plain `ScheduleEvent` values passed to the `events` prop. The model covers timed, all-day, multi-day, colored, and recurring events.

## Event fields

- `id` — stable identifier used to track the event across interactions.
- `title` — text shown by the default renderer.
- `start` / `end` — `PrimitiveDateTime` bounds; spanning multiple days renders a multi-day event.
- `all_day` — render in the all-day row instead of the time grid.
- `color` — optional token exposed as `data-color` for styling.
- `description` — optional text exposed as an accessible title.
- `recurrence` — optional `ScheduleRecurrence` rule (see the Recurring events page).
- `drag_disabled` / `resize_disabled` — opt individual events out of drag/drop or resize.

```rust
ScheduleEvent {
    id: "planning".to_string(),
    title: "Planning sync".to_string(),
    start: PrimitiveDateTime::new(date!(2026 - 05 - 12), time!(09:00)),
    end: PrimitiveDateTime::new(date!(2026 - 05 - 12), time!(10:00)),
    all_day: false,
    color: Some("blue".to_string()),
    description: Some("Weekly planning.".to_string()),
    recurrence: None,
    drag_disabled: false,
    resize_disabled: false,
}
```
