//! Defines the [`Schedule`] component and supporting scheduling data types.

mod components;
mod state;
#[cfg(test)]
mod tests;
mod types;
mod utils;

pub use components::{
    use_schedule, use_schedule_context, Schedule, ScheduleHeader, ScheduleViewButton,
    ScheduleViewSwitcher,
};
pub use types::{
    ScheduleAllDaySlotClick, ScheduleClassNames, ScheduleContext, ScheduleDateChange,
    ScheduleDayClick, ScheduleDayViewConfig, ScheduleDropDestination, ScheduleEvent,
    ScheduleEventClick, ScheduleEventCreate, ScheduleEventCreateSource, ScheduleEventDrag,
    ScheduleEventDrop, ScheduleEventRenderContext, ScheduleEventResize, ScheduleExternalDrop,
    ScheduleHeaderContext, ScheduleLabels, ScheduleLayout, ScheduleMobileMonthViewConfig,
    ScheduleMode, ScheduleMonthViewConfig, ScheduleProps, ScheduleRecurrence,
    ScheduleRecurrenceExpansionLimit, ScheduleRecurrenceFrequency, ScheduleResizeEdge,
    ScheduleSlotRangeSelection, ScheduleState, ScheduleTimeGridConfig, ScheduleTimeSlotClick,
    ScheduleView, ScheduleViewChange, ScheduleWeekViewConfig, ScheduleYearViewConfig,
    UseScheduleConfig,
};
pub use utils::{add_months, shift_date, today};
