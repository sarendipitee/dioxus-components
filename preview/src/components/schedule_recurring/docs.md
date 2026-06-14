# Recurring events

Attach a `ScheduleRecurrence` to an event's `recurrence` field to repeat it. The schedule expands recurrences into individual occurrences for the visible range.

## Recurrence rule

`ScheduleRecurrence` controls how an event repeats:

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

## Expansion limit

`recurrence_expansion_limit: ScheduleRecurrenceExpansionLimit` bounds how many occurrences are created per recurring event, guarding against unbounded rules.

```rust
Schedule {
    events: my_events,
    recurrence_expansion_limit: ScheduleRecurrenceExpansionLimit { max_occurrences: 8 },
}
```
