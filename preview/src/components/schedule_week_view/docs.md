# WeekView

The week view renders seven days side by side as time-grid columns. It is the default view; switch to it explicitly with `default_view: ScheduleView::Week` or the controlled `view` prop.

## Configuration

Pass `week_view: ScheduleWeekViewConfig`:

- `time_grid` (`ScheduleTimeGridConfig`) controls the visible hour range, slot size, and per-view header — the same shape used by the day view.
- `first_day_of_week` sets which weekday starts each row.

```rust
Schedule {
    default_view: ScheduleView::Week,
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
```

Multi-day events span the relevant day columns, and all-day events render in the all-day row.
