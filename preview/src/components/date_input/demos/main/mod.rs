use dioxus::prelude::*;
use dioxus_components::date_input::*;
use dioxus_primitives::calendar::DateRange;
use time::{macros::date, Date};

fn format_date(date: Option<Date>) -> String {
    date.map(|date| date.to_string()).unwrap_or_default()
}

#[component]
pub fn Demo() -> Element {
    let mut selected_date = use_signal(|| Some(date!(2024 - 05 - 15)));
    let mut selected_range = use_signal(|| None::<DateRange>);
    let mut due_date_callback_count = use_signal(|| 0_u32);

    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 28rem;",
            div { "data-testid": "due-date-field",

                DateInput {
                label: rsx! { "Due date" },
                description: rsx! { "Single-date input composition." },
                clearable: true,
                selected_date,
                on_value_change: move |value| {
                    selected_date.set(value);
                    due_date_callback_count += 1;
                },
            }
            output { "data-testid": "due-date-value", "{format_date(selected_date())}" }
                }

            output { "data-testid": "due-date-callback-count", "{due_date_callback_count}" }
            div { "data-testid": "required-date-field",

                DateInput {
                label: rsx! { "Required date" },
                required: true,
                error: rsx! { "A date is required" },
            }
                }

            div { "data-testid": "disabled-date-field",

                DateInput {
                label: rsx! { "Disabled date" },
                selected_date: Some(date!(2024 - 06 - 20)),
                disabled: true,
            }
                }

            div { "data-testid": "readonly-date-field",

                DateInput {
                label: rsx! { "Read-only date" },
                selected_date: Some(date!(2024 - 07 - 25)),
                read_only: true,
            }
                }

            DateRangePickerInput {
                label: rsx! { "Booking range" },
                selected_range: selected_range(),
                on_range_change: move |value| selected_range.set(value),
                month_count: 2,
            }
        }
    }
}
