# Events data for the schedule demos

This page documents the exact `ScheduleEvent` shape that powers the schedule examples in this section. Each field below maps to a concrete rendering or interaction behavior you can observe in the demos: timed placement, all-day row placement, color accents, recurrence expansion, drag affordances, and resize affordances.

## Event fields

- `id` — a stable identifier that lets the renderer track an event when selections, hover states, or drag interactions are applied.
- `title` — the label text drawn by the default event renderer.
- `start` / `end` — `PrimitiveDateTime` bounds used to place events on the day grid; if the end is on a later day, the event appears across multiple day slots.
- `all_day` — when true, the event is placed in the all-day area instead of the timed grid.
- `color` — optional style token surfaced as `data-color` for custom schedule theming.
- `description` — optional text used for accessibility metadata such as the rendered title/tooltip.
- `recurrence` — optional `ScheduleRecurrence` rule (see the recurring events demo), which expands source events into repeated occurrences.
- `drag_disabled` / `resize_disabled` — toggles per-event interaction so specific items can be locked while others remain movable.

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
