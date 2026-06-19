# Schedule Month View

The Month View component renders a full month calendar grid suitable for high-density schedule snapshots. It is the main landing view for calendar-style overviews, where each visible cell maps to one calendar day and event rows are compressed to keep the grid readable while still showing day-level activity.

Use this demo to verify how monthly planning behaves when weeks shift start days, when headers are toggled on/off, and when multi-day items flow across the grid.

## Configuration

Pass `month_view: ScheduleMonthViewConfig`:

- `with_default_header` toggles the per-view header.
- `first_day_of_week` sets which weekday starts each row.
- `event_limit` controls how many events are shown in a day cell before truncation.
- `show_outside_days` controls whether leading/trailing dates from adjacent months remain visible.

```rust
Schedule {
    default_view: ScheduleView::Month,
    month_view: ScheduleMonthViewConfig::default(),
}
```

Timed and all-day events are rendered as compact day-cell entries by default, while multi-day events maintain continuity by spanning across the affected days. This lets you confirm both density management and layout consistency without leaving the month canvas.
