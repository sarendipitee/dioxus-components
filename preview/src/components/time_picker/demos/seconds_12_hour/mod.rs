use dioxus::prelude::*;
use dioxus_components::time_picker::*;
use time::macros::time;

#[component]
pub fn Demo() -> Element {
    let mut selected_time = use_signal(|| Some(time!(09:30:15)));

    rsx! {
        TimePicker {
            selected_time: selected_time(),
            on_value_change: move |v| selected_time.set(v),
            with_seconds: true,
            format: TimePickerFormat::TwelveHour,
            am_pm_labels: ("am".to_string(), "pm".to_string()),
            min_time: time!(08:00:00),
            max_time: time!(18:00:00),
        }
    }
}
