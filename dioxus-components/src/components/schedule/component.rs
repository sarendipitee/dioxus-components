use dioxus::prelude::*;
use crate::component_styles;
use dioxus_icons::lucide::{ChevronLeft, ChevronRight};
use dioxus_primitives::schedule::{self, ScheduleClassNames as PrimitiveScheduleClassNames};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
use time::Date;

use crate::components::button::{Button, ButtonSize, ButtonVariant};

pub use dioxus_primitives::schedule::{
    add_months, shift_date, today, use_schedule, use_schedule_context, ScheduleAllDaySlotClick,
    ScheduleClassNames, ScheduleContext, ScheduleDateChange, ScheduleDayClick,
    ScheduleDayViewConfig, ScheduleDropDestination, ScheduleEvent, ScheduleEventClick,
    ScheduleEventCreate, ScheduleEventCreateSource, ScheduleEventDrag, ScheduleEventDrop,
    ScheduleEventRenderContext, ScheduleEventResize, ScheduleExternalDrop, ScheduleHeaderContext,
    ScheduleLabels, ScheduleLayout, ScheduleMobileMonthViewConfig, ScheduleMode,
    ScheduleMonthViewConfig, ScheduleProps, ScheduleRecurrence, ScheduleRecurrenceExpansionLimit,
    ScheduleRecurrenceFrequency, ScheduleResizeEdge, ScheduleSlotRangeSelection, ScheduleState,
    ScheduleTimeGridConfig, ScheduleTimeSlotClick, ScheduleView, ScheduleViewChange,
    ScheduleWeekViewConfig, ScheduleYearViewConfig, UseScheduleConfig,
};

#[component_styles("./style.css")]
struct Styles;

fn append_class(existing: String, class_name: &str) -> String {
    if existing.trim().is_empty() {
        class_name.to_string()
    } else {
        format!("{} {}", class_name, existing.trim())
    }
}

fn schedule_class_names(class_names: PrimitiveScheduleClassNames) -> PrimitiveScheduleClassNames {
    PrimitiveScheduleClassNames {
        desktop_view: append_class(class_names.desktop_view, &Styles::dx_schedule_desktop),
        mobile_view: append_class(class_names.mobile_view, &Styles::dx_schedule_mobile),
        day_view: append_class(class_names.day_view, &Styles::dx_schedule_time_view),
        week_view: append_class(class_names.week_view, &Styles::dx_schedule_time_view),
        month_view: append_class(class_names.month_view, &Styles::dx_schedule_month_view),
        year_view: append_class(class_names.year_view, &Styles::dx_schedule_year_view),
        mobile_month_view: append_class(
            class_names.mobile_month_view,
            &Styles::dx_schedule_mobile_month_view,
        ),
        time_slot: append_class(class_names.time_slot, &Styles::dx_schedule_time_slot),
        all_day_slot: append_class(class_names.all_day_slot, &Styles::dx_schedule_all_day_slot),
        month_day: append_class(class_names.month_day, &Styles::dx_schedule_month_day),
        event: append_class(class_names.event, &Styles::dx_schedule_event),
    }
}

/// Props for the styled [`ScheduleHeader`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ScheduleHeaderProps {
    /// Reactive current date from the primitive's internal state.
    pub date: ReadSignal<Date>,
    /// Localised labels for navigation controls.
    pub labels: ScheduleLabels,
    /// Called when the previous range button is clicked.
    pub on_previous: Callback<MouseEvent>,
    /// Called when the next range button is clicked.
    pub on_next: Callback<MouseEvent>,
    /// Called when the today button is clicked.
    pub on_today: Callback<MouseEvent>,
}

/// Styled schedule header with Previous / Today / Next navigation.
#[component]
pub fn ScheduleHeader(props: ScheduleHeaderProps) -> Element {
    let date = props.date;
    let on_previous = props.on_previous;
    let on_next = props.on_next;
    let on_today = props.on_today;
    rsx! {
        header { "data-schedule-header": true,
            div { "data-schedule-nav": true,
                span { "data-schedule-title": true,
                    span { {date().month().to_string()} }
                    span { {date().year().to_string()} }
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Icon,
                    "aria-label": props.labels.previous.clone(),
                    onclick: move |event| on_previous.call(event),
                    ChevronLeft { size: "1rem" }
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    onclick: move |event| on_today.call(event),
                    {props.labels.today.clone()}
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Icon,
                    "aria-label": props.labels.next.clone(),
                    onclick: move |event| on_next.call(event),
                    ChevronRight { size: "1rem" }
                }
            }
        }
    }
}

/// Props for [`ScheduleViewSwitcher`].
#[derive(Props, Clone, PartialEq)]
pub struct ScheduleViewSwitcherProps {
    /// Shared schedule state from [`use_schedule`].
    pub state: ScheduleState,
    /// Localised labels for view controls.
    #[props(default)]
    pub labels: ScheduleLabels,
}

/// Styled schedule view switcher that can be placed anywhere in the layout.
#[component]
pub fn ScheduleViewSwitcher(props: ScheduleViewSwitcherProps) -> Element {
    let button = |target: ScheduleView, label: String| {
        let active = (props.state.view)() == target;
        rsx! {
            Button {
                variant: if active { ButtonVariant::Secondary } else { ButtonVariant::Ghost },
                "aria-pressed": active,
                "data-schedule-view-button": target.as_str(),
                "data-active": active,
                onclick: move |_| props.state.set_view.call(target),
                {label}
            }
        }
    };

    rsx! {
        div {
            class: Styles::dx_schedule_view_switcher,
            role: "group",
            "aria-label": "Schedule views",
            "data-schedule-view-controls": true,
            {button(ScheduleView::Day, props.labels.day.clone())}
            {button(ScheduleView::Week, props.labels.week.clone())}
            {button(ScheduleView::Month, props.labels.month.clone())}
            {button(ScheduleView::Year, props.labels.year.clone())}
        }
    }
}

/// Styled schedule wrapper that applies reusable component styles to the primitive schedule.
#[component]
pub fn Schedule(mut props: schedule::ScheduleProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_schedule,
    });

    props.attributes = merge_attributes(vec![base, props.attributes]);
    props.class_names = schedule_class_names(props.class_names);

    if props.with_default_header && props.header.is_none() && props.render_header.is_none() {
        props.with_default_header = false;
        props.render_header = Some(Callback::new(|ctx: ScheduleHeaderContext| {
            rsx! {
                ScheduleHeader {
                    date: ctx.date,
                    labels: ctx.labels,
                    on_previous: ctx.on_previous,
                    on_next: ctx.on_next,
                    on_today: ctx.on_today,
                }
            }
        }));
    }

    rsx! {
        schedule::Schedule { ..props }
    }
}
