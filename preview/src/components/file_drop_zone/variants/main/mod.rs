use dioxus::prelude::*;
use dioxus_components::FileDropZone;
use dioxus_primitives::file_drop_zone::AcceptedFile;

#[component]
pub fn Demo() -> Element {
    let mut files = use_signal(Vec::<String>::new);

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 1rem; width: 100%;",
            FileDropZone {
                on_accepted: move |accepted: Vec<AcceptedFile>| {
                    files.set(accepted.into_iter().map(|f| f.name).collect());
                },
                p { "Drop files here or click to select" }
            }
            if !files.read().is_empty() {
                div {
                    p { "{files.read().len()} file(s) selected:" }
                    ul {
                        for name in files.read().iter() {
                            li { key: "{name}", "{name}" }
                        }
                    }
                }
            }
        }
    }
}
