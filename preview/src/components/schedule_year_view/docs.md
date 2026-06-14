# YearView

The year view renders all twelve months of the active year as compact month grids. Switch to it with `default_view: ScheduleView::Year`. Selecting a month moves the schedule to the month view for that month.

## Configuration

Pass `year_view: ScheduleYearViewConfig`:

- `with_default_header` toggles the per-view header.

```rust
Schedule {
    default_view: ScheduleView::Year,
    year_view: ScheduleYearViewConfig::default(),
}
```

Days that have events are highlighted so the whole year can be scanned at a glance.
