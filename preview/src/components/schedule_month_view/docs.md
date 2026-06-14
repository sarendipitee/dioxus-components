# MonthView

The month view renders a calendar grid for the active month, with each day cell listing its events. Switch to it with `default_view: ScheduleView::Month`, or by selecting a month in the year view.

## Configuration

Pass `month_view: ScheduleMonthViewConfig`:

- `with_default_header` toggles the per-view header.
- `first_day_of_week` sets which weekday starts each row.

```rust
Schedule {
    default_view: ScheduleView::Month,
    month_view: ScheduleMonthViewConfig::default(),
}
```

Timed and all-day events are shown as compact entries inside each day cell, and multi-day events span across cells.
