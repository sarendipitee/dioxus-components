# Scheduling recurring events in one place

This demo focuses on recurring events that need both flexibility and guardrails. `ScheduleRecurrence` sits on an event so you can define exactly how often it should repeat, and the schedule renderer expands those rules into concrete occurrences for the date window you display.

## Recurrence rule demo

Use the recurrence rule to model real patterns like “every 3rd day,” “every second Thursday,” or “up to 10 occurrences total”:

- `frequency` — `Daily`, `Weekly`, `Monthly`, or `Yearly`.
- `interval` — units between occurrences (e.g. `2` for every other week). Values below one are treated as one.
- `count` — stop after this many occurrences, including the original.
- `until` — stop after occurrences that start past this date/time.

```rust
ScheduleEvent {
    // ...
    recurrence: Some(ScheduleRecurrence {
        frequency: ScheduleRecurrenceFrequency::Weekly,
        interval: 1,
        count: Some(4),
        until: None,
    }),
}
```

## Expansion limit demo

`recurrence_expansion_limit: ScheduleRecurrenceExpansionLimit` bounds how many generated rows can come from one recurring event, which is important when a broad rule would otherwise flood the timeline.

```rust
Schedule {
    events: my_events,
    recurrence_expansion_limit: ScheduleRecurrenceExpansionLimit { max_occurrences: 8 },
}
```
