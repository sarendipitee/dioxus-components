# Schedule

Use this page to wire a real calendar workflow: one source of truth for date/view state, multiple visualized views, and interaction callbacks that let apps create, move, and resize time-based entries without ad-hoc state hacks. The Schedule demos focus on practical usage patterns—shared navigation state, controlled view/date updates, and event actions you can reuse in booking, planning, and operations dashboards.

## Usage

```rust
let schedule = use_schedule(UseScheduleConfig {
    default_date: sample_date(),
    default_view: ScheduleView::Week,
    ..UseScheduleConfig::default()
});

rsx! {
    ScheduleViewSwitcher {
        state: schedule,
    }

    Schedule {
        state: schedule,
        events: sample_events(),
    }
}
```

This is the “single source of truth” demo: the `use_schedule` hook owns the active date and view, then both the switcher and calendar body read from that same state. The pattern is ideal when the schedule must sit inside a larger toolbar, sidebar, or filter ribbon and stay in sync.

```rust
let schedule = use_schedule(UseScheduleConfig {
    default_view: ScheduleView::Week,
    ..UseScheduleConfig::default()
});

rsx! {
    button { onclick: move |_| schedule.set_view.call(ScheduleView::Day), "Day" }
    button { onclick: move |_| schedule.set_view.call(ScheduleView::Week), "Week" }

    Schedule {
        state: schedule,
        events: sample_events(),
    }
}
```

This variant demonstrates fully custom controls. You can replace the built-in switcher with your own navigation controls (tabs, segmented buttons, dropdowns) while still delegating event rendering and grid logic to `Schedule`.

The legacy prop-based API remains available if your migration path still depends on uncontrolled props:

```rust
Schedule {
    default_date: sample_date(),
    default_view: ScheduleView::Week,
    events: sample_events(),
}
```

## Views

The shared state flow is the reason to use `use_schedule` in dashboard contexts: any component can change the selected view and every `Schedule`/helper that subscribes to that state updates together. For isolated integrations, `default_view` keeps the calendar self-managed, while `view` plus `on_view_change` gives you a controlled model in the parent. Supported values are `ScheduleView::Day`, `ScheduleView::Week`, `ScheduleView::Month`, and `ScheduleView::Year`. The year layout is treated as a drill-down entry point into month views for quick jump-to-month workflows.

See the DayView, WeekView, MonthView, YearView, and MobileMonthView pages for per-view tuning examples.

## Controlled State

When you need external state synchronization, bind `date` with `on_date_change` to make route transitions, data re-fetches, or analytics tracking follow calendar movement. Pair `view` with `on_view_change` to keep URL-based deep links, tab state, and persisted layout in lockstep with what users see.

## Events And Recurrence

Feed `events` with your `ScheduleEvent` payloads and use `recurrence_expansion_limit` to bound repeated occurrences for performance and UX predictability. This is the section to reference when your domain model uses rules like “every weekday at 9am” or “weekly recurring standups” and you need a stable expansion window in the UI.

See the Events data and Recurring events pages for full schema and recurrence behavior details.

## Interactions

Enable `with_events_drag_and_drop`, `with_drag_slot_select`, and `with_event_resize` to expose direct-manipulation editing in your planner UI. Those flags surface payloads for time-slot and all-day interactions, event drag cycles, external drops, slot selections, and resize commits so you can map behavior to business logic (approval, validation, and persistence).

`can_drag_event` and `can_resize_event` let you gate editing constraints per event or workflow (for example, locking confirmed bookings while still allowing tentative ones to move).

Use `on_event_create` for built-in event creation flows. Timed slot clicks provide a `ScheduleEventCreate` with normalized `start`/`end`. Timed drag selections also emit one create payload for the selected range, while all-day click interactions emit full-day ranges. The payload also carries `date`, `all_day`, `view`, and `source` (`TimeSlotClick`, `TimeSlotDrag`, `AllDaySlotClick`, or `DayClick`) so you can attach business-specific creation rules without combining callbacks manually.

## Responsive Layout

Use `layout: ScheduleLayout::Responsive` for a single component that serves desktop planners and compact mobile timelines. On smaller widths, this mode swaps to the mobile month presentation while keeping the year view path available, which is useful for touch-first check-in and field operations screens.

See the MobileMonthView page for mobile-only rendering behaviors.

## Custom Rendering And Header

Use `render_event_body` to inject your own event card layout (title, metadata, status badges, or conflict hints) while preserving the underlying schedule interactions. The default header only includes primary date navigation; if your application already has global controls, `with_default_header: false` removes it cleanly, and `header` swaps in a fully custom header block.

Per-view config structs expose their own `with_default_header` toggle too, which lets you keep default controls on some views while replacing them on others.

## Styling

The primitive’s styling hooks include a runtime `radius` token, stateful `data-*` attributes, and `ScheduleClassNames` for desktop/mobile containers, day/week/month/year/mobile-month surfaces, slots, days, and events. This makes it practical to enforce design-system-level look and feel without forking interaction code.

## Localization And Labels

Set `locale` and `ScheduleLabels` when the schedule needs in-app language control, date-format conventions, or workspace-specific wording. Labels cover navigation text, view titles, all-day badges, and empty-state slot text so translated calendars remain understandable with minimal glue code.

## Static Mode

Use `mode: ScheduleMode::Static` for read-heavy screens where selection and navigation are still required (for inspecting workload, reporting, or auditing) but drag/drop and resize must stay disabled regardless of interaction props.

## Accessibility

The primitive renders semantic controls for navigation and selection: buttons for dates, months, and slots, plus an accessible `ScheduleViewSwitcher` that marks active view state. If you build custom controls or custom event rendering, preserve explicit labels, focus order, and selected-state semantics; provide meaningful titles/descriptions so screen reader users get the same scheduling clarity as mouse users.

## Component Structure

```rust
Schedule {
    default_date: sample_date(),
    default_view: ScheduleView::Week,
    layout: ScheduleLayout::Responsive,
    mode: ScheduleMode::Default,
    events: sample_events(),
    labels: ScheduleLabels::default(),
    with_events_drag_and_drop: true,
    with_drag_slot_select: true,
    with_event_resize: true,
}
```

