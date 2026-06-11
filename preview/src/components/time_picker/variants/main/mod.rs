use super::super::component::*;
use dioxus::prelude::*;
use dioxus_primitives::time_picker::{
    TimePickerFormat, TimePickerSteps, TimePickerType, TimePickerValue,
};

use time::{macros::time, Time};

#[component]
pub fn Demo() -> Element {
    let mut selected_time = use_signal(|| Some(time!(09:30)));
    let mut simple_time = use_signal(|| Some(time!(14:45)));
    let mut empty_time = use_signal(|| None::<Time>);
    let mut compact_time = use_signal(|| Some(time!(07:15)));
    let mut outline_time = use_signal(|| Some(time!(18:00)));
    let mut error_time = use_signal(|| Some(time!(22:30)));
    let mut duration = use_signal(|| {
        Some(TimePickerValue::Duration {
            hours: 36,
            minutes: 15,
            seconds: 0,
        })
    });

    rsx! {
        div {
            style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(18rem, 1fr)); gap: 1rem; max-width: 54rem;",
            TimePicker {
                label: "Appointment time",
                description: "Dropdown, seconds, 12-hour mode, presets, and clearing",
                selected_time: selected_time(),
                on_value_change: move |v: Option<Time>| {
                    tracing::info!("Selected time changed: {:?}", v);
                    selected_time.set(v);
                },
                with_seconds: true,
                format: TimePickerFormat::TwelveHour,
                am_pm_labels: ("am".to_string(), "pm".to_string()),
                min_time: time!(08:00),
                max_time: time!(18:00),
                steps: TimePickerSteps {
                    hours: 1,
                    minutes: 15,
                    seconds: 15,
                },
                clearable: true,
                with_dropdown: true,
                max_dropdown_content_height: "10rem",
                presets: vec![
                    TimePickerPreset::new("Morning", time!(09:00)),
                    TimePickerPreset::new("Lunch", time!(12:30)),
                    TimePickerPreset::new("Afternoon", time!(15:00)),
                ],
                preset_groups: vec![TimePickerPresetGroup::new(
                    "Generated",
                    get_time_range(time!(16:00), time!(17:00), 30 * 60),
                )],
            }
            TimePicker {
                label: "Basic time",
                description: "24-hour input without dropdown",
                selected_time: simple_time(),
                on_value_change: move |v: Option<Time>| simple_time.set(v),
            }
            TimePicker {
                label: "Empty clearable",
                description: "Clear button appears only after a value is set",
                selected_time: empty_time(),
                on_value_change: move |v: Option<Time>| empty_time.set(v),
                clearable: true,
                with_dropdown: true,
                presets: vec![
                    TimePickerPreset::new("Start", time!(08:00)),
                    TimePickerPreset::new("End", time!(17:00)),
                ],
            }
            TimePicker {
                label: "Small ghost",
                description: "Small size with custom radius",
                selected_time: compact_time(),
                on_value_change: move |v: Option<Time>| compact_time.set(v),
                size: TimePickerSize::Sm,
                variant: TimePickerVariant::Ghost,
                radius: "999px",
                clearable: true,
            }
            TimePicker {
                label: "Outline dropdown",
                description: "Right section content with dropdown options",
                selected_time: outline_time(),
                on_value_change: move |v: Option<Time>| outline_time.set(v),
                variant: TimePickerVariant::Outline,
                with_dropdown: true,
                clearable: true,
                right_section: rsx! { span { "UTC" } },
                clear_section_mode: TimePickerClearSectionMode::Both,
                dropdown_width: "18rem",
            }
            TimePicker {
                label: "Validation state",
                description: "Large size with visible error text",
                selected_time: error_time(),
                on_value_change: move |v: Option<Time>| error_time.set(v),
                size: TimePickerSize::Lg,
                clearable: true,
                error: "Choose a time before 21:00",
            }
            TimePicker {
                label: "Disabled",
                description: "Non-interactive time field",
                selected_time: Some(time!(11:00)),
                disabled: true,
                with_dropdown: true,
                clearable: true,
            }
            TimePicker {
                label: "Elapsed time",
                description: "Duration mode with seconds and padded hours",
                selected_value: duration(),
                on_picker_value_change: move |v| duration.set(v),
                picker_type: TimePickerType::Duration,
                with_seconds: true,
                min_hours_digits: 3,
                clearable: true,
            }
        }
    }
}
