use dioxus::prelude::*;
use dioxus_components::date_input::*;
use dioxus_primitives::calendar::DateRange;
use time::Date;

#[component]
pub fn Demo() -> Element {
    let mut selected_date = use_signal(|| None::<Date>);
    let mut selected_range = use_signal(|| None::<DateRange>);

    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 28rem;",
            DateInput {
                label: rsx! { "Due date" },
                description: rsx! { "Single-date input composition." },
                selected_date: selected_date(),
                on_value_change: move |value| selected_date.set(value),
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
