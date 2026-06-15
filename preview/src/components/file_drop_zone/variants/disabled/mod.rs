use dioxus::prelude::*;
use dioxus_components::FileDropZone;

#[component]
pub fn Demo() -> Element {
    rsx! {
        FileDropZone {
            disabled: true,
            p { "This drop zone is disabled" }
        }
    }
}
