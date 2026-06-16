use dioxus::prelude::*;
use dioxus_components::checkbox::*;
use dioxus_primitives::checkbox::CheckboxState;

#[component]
pub fn Demo() -> Element {
    let mut state = use_signal(|| CheckboxState::Indeterminate);

    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            Checkbox {
                name: "partial-selection",
                label: "Select all visible rows",
                description: "Some rows are selected. Toggle to select or clear the full set.",
                checked: Some(state()),
                on_checked_change: move |next| state.set(next),
            }
        }
    }
}
