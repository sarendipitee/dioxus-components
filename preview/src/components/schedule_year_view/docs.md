# ScheduleYearView

`ScheduleYearView` is the annual planning surface of the schedule component.  
It renders all twelve months of the active year as a compact, high-level grid so users can spot activity patterns by month before moving deeper.

Use `default_view: ScheduleView::Year` to open the schedule at this level.  
When a month tile is selected, the schedule transitions into that month by switching the view to `ScheduleView::Month`.

## Configuration

Control the annual shell with `year_view: ScheduleYearViewConfig`:

- `with_default_header` toggles the per-view header.

```rust
Schedule {
    default_view: ScheduleView::Year,
    year_view: ScheduleYearViewConfig::default(),
}
```

Days with events are highlighted inside each month tile, so the component works as a true “year-at-a-glance” signal rather than just a navigation grid.
