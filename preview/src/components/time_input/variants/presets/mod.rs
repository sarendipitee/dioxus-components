use dioxus::prelude::*;
use dioxus_components::time_input::*;
use dioxus_components::time_picker::{time_range, TimePickerFormat};
use time::{macros::time, Duration, Time};

#[component]
pub fn Demo() -> Element {
    let mut meeting_time = use_signal(|| None::<Time>);
    let mut shift_time = use_signal(|| None::<Time>);

    rsx! {
        div { style: "display: grid; gap: 1rem; max-width: 24rem;",
            TimeInput {
                label: rsx! { "Meeting time" },
                description: rsx! { "Pick a slot or use the column picker." },
                format: TimePickerFormat::TwelveHour,
                selected_time: meeting_time(),
                on_value_change: move |v| meeting_time.set(v),
                presets: vec![time!(09:00), time!(10:30), time!(12:00), time!(14:00), time!(16:30)],
                clearable: true,
            }
            TimeInput {
                label: rsx! { "Shift start" },
                description: rsx! { "Generated every 1.5 hours from 06:00-22:00." },
                selected_time: shift_time(),
                on_value_change: move |v| shift_time.set(v),
                presets: time_range(time!(06:00), time!(22:00), Duration::hours(1) + Duration::minutes(30)),
                clearable: true,
            }
        }
    }
}
