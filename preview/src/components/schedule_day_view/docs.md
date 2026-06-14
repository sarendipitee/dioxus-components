# DayView

The day view renders a single day as a vertical time grid. Switch to it with `default_view: ScheduleView::Day` (or the controlled `view` prop), or by selecting a day from another view.

## Time Grid

Pass `day_view: ScheduleDayViewConfig` to configure the grid through `ScheduleTimeGridConfig`:

- `start_hour` / `end_hour` bound the visible hours.
- `slot_minutes` sets the slot size (e.g. `30` for half-hour slots).
- `with_default_header` toggles the per-view header.

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

Timed events render in their slots, while all-day events appear in the all-day row above the grid.
