# WeekView

The week view is the primary surface for reviewing one full week at a glance. It renders seven day columns and a synchronized hour grid so you can compare back-to-back events, overlapping meetings, and cross-day availability side by side.

In the demo, this component is positioned as the default workspace context: it opens directly with `default_view: ScheduleView::Week` and also supports explicit controlled usage through the `view: ScheduleView::Week` prop.

## Configuration

Pass `week_view: ScheduleWeekViewConfig` to tune each aspect of how the week is laid out:

- `time_grid` (`ScheduleTimeGridConfig`) controls the visible hour range, slot size, and per-view header, and it stays API-compatible with the day view so behavior can be consistent across views.
- `first_day_of_week` controls which weekday begins each page.

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

In practice, multi-day events span across each affected day column, while all-day events stay fixed in the all-day row so your high-level planning lane stays readable even when the grid is dense.
