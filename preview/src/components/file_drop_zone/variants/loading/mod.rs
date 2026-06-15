use dioxus::prelude::*;
use dioxus_components::button::Button;
use dioxus_components::FileDropZone;

#[component]
pub fn Demo() -> Element {
    let mut loading = use_signal(|| true);

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 1rem; width: 100%;",
            FileDropZone {
                loading: loading(),
                p { "Drop files here or click to select" }
            }
            Button {
                onclick: move |_| loading.toggle(),
                if loading() { "Stop loading" } else { "Start loading" }
            }
        }
    }
}
