use dioxus::prelude::*;
use dioxus_components::FileDropZone;
use dioxus_primitives::file_drop_zone::AcceptedFile;

#[component]
pub fn Demo() -> Element {
    let mut selected = use_signal(|| None::<String>);

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 1rem; width: 100%;",
            FileDropZone {
                // `multiple` defaults to false: only a single file is accepted.
                on_accepted: move |files: Vec<AcceptedFile>| {
                    selected.set(files.into_iter().next().map(|f| f.name));
                },
                p { "Drop a single file here or click to select" }
            }
            if let Some(name) = selected.read().as_ref() {
                div { "Selected: {name}" }
            }
        }
    }
}
