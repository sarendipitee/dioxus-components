# MobileMonthView

`MobileMonthView` is the compact mobile rendering mode for the `Schedule` component. It turns the month grid into a touch-oriented layout with tighter spacing and smaller cell geometry so calendar browsing stays usable on narrow screens.

Use it by enabling the responsive schedule layout and providing a mobile month configuration. In responsive mode, the schedule renders both desktop and mobile containers, then switches to this view when the viewport is narrow enough; the year view remains available at larger breakpoints.

## Configuration

The page is controlled through `mobile_month_view: ScheduleMobileMonthViewConfig` in the schedule config.

- `with_default_header` controls whether each month card keeps the built-in header (typically used for demo clarity and quick month context).

```rust
Schedule {
    default_view: ScheduleView::Week,
    layout: ScheduleLayout::Responsive,
    mobile_month_view: ScheduleMobileMonthViewConfig::default(),
}
```

The docs demo below shows the default compact month presentation first, then lets you verify the viewport behavior by resizing the preview to a phone width. It demonstrates the exact moment the component switches from desktop layout to `MobileMonthView`, including how the built-in header can be shown or hidden via config.
