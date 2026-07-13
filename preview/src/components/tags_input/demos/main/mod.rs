use dioxus::prelude::*;
use dioxus_components::combobox::TagsInput;

#[component]
pub fn Demo() -> Element {
    let mut values = use_signal(|| Some(vec!["dioxus".to_string(), "components".to_string()]));

    rsx! {
        div {
            TagsInput {
                values,
                on_values_change: move |next| values.set(Some(next)),
                placeholder: "Add tag and press Enter...",
            }
            p {
                "Tags: {values().unwrap_or_default().join(\", \")}"
            }
        }
    }
}
