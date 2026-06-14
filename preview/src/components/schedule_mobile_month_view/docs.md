# MobileMonthView

The mobile month view is a compact month presentation rendered by the responsive layout. Set `layout: ScheduleLayout::Responsive` to render both the desktop and mobile containers; CSS switches to the mobile month view at small widths while keeping the year view available.

## Configuration

Pass `mobile_month_view: ScheduleMobileMonthViewConfig`:

- `with_default_header` toggles the per-view header.

```rust
Schedule {
    default_view: ScheduleView::Week,
    layout: ScheduleLayout::Responsive,
    mobile_month_view: ScheduleMobileMonthViewConfig::default(),
}
```

Resize the preview (or open it on a narrow screen) to see the layout switch to the mobile month presentation.
