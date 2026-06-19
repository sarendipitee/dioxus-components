# DayView

The Day view is the component page used to inspect one date at a glance. It turns a single day into a vertical schedule with time-labeled rows so overlapping tasks and exact durations stay visually separated.

In the preview below, use this view when you want to verify how the component handles narrow windows (for example, business-hour focused calendars, support shifts, or quick drill-down from week/month modes). Enter it with `default_view: ScheduleView::Day` (or a controlled `view` prop), or by selecting a date from another view.

## Time Grid controls in this demo

This page demonstrates day-level configuration through `day_view: ScheduleDayViewConfig`, which is forwarded into `ScheduleTimeGridConfig`:

- `start_hour` / `end_hour` bound the visible hours.
- `slot_minutes` sets the per-row slot size (for example, `30` for half-hour increments).
- `with_default_header` toggles the built-in day header so you can compare compact vs. labeled layouts.

```rust
Schedule {
    default_view: ScheduleView::Day,
    day_view: ScheduleDayViewConfig {
        time_grid: ScheduleTimeGridConfig {
            start_hour: 8,
            end_hour: 18,
            slot_minutes: 30,
            with_default_header: true,
        },
    },
}
```

In this view, timed events are rendered directly inside the day’s slot rows, and all-day events are lifted into the dedicated row above the grid, which makes this mode especially useful for checking day-level collisions and agenda density.
