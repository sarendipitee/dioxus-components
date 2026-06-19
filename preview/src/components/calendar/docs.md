The Calendar component powers date-driven tasks like reservations, deadlines, and analytics windows by rendering a controlled month view with predictable selection state. In this page’s demos, you can move between months, observe how `view_date` drives the visible grid, and see how `selected_date` captures the active choice. The page also includes a `RangeCalendar` example for contiguous interval selection when a workflow needs start/end dates.

## Component Structure

```rust
Calendar {
    // The active selected date returned to your app when the user picks a day.
    selected_date,
    // Handler that receives the selected date whenever the user clicks a day.
    on_date_change: |date: Option<CalendarDate>| {
        // Triggered on every valid date selection.
        // `date` is the newly chosen date, or None when cleared.
    },
    // Month/year anchor that controls which calendar page is shown.
    view_date,
    // Handler for month navigation and jumps between years.
    on_view_change: |date: CalendarDate| {
        // Called when the user changes month or year.
        // `date` reflects the new visible calendar month.
    },
    // Optional number of month columns to render side-by-side.
    month_count: 1,
}
```

The styled `Calendar` example shows the default single-date interaction model, while `RangeCalendar` shows a date interval flow on the same controlled API. Use this page to compare both patterns before wiring either component into forms, filters, or booking flows.
