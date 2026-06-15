use dioxus::prelude::*;
use dioxus_components::FileDropZone;
use dioxus_primitives::file_drop_zone::{AcceptedFile, FileRejection};

#[component]
pub fn Demo() -> Element {
    let mut accepted = use_signal(Vec::<String>::new);
    let mut rejected = use_signal(Vec::<FileRejection>::new);

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 1rem; width: 100%;",
            FileDropZone {
                // 1 MB limit.
                max_size: Some(1024 * 1024),
                multiple: true,
                on_accepted: move |files: Vec<AcceptedFile>| {
                    accepted.set(files.into_iter().map(|f| f.name).collect());
                },
                on_rejected: move |files: Vec<FileRejection>| {
                    rejected.set(files);
                },
                p { "Drop files here or click to select" }
                p { style: "font-size: 0.85em; opacity: 0.7;", "Files larger than 1 MB are rejected" }
            }
            if !accepted.read().is_empty() {
                div {
                    p { "Accepted:" }
                    ul {
                        for name in accepted.read().iter() {
                            li { key: "{name}", "{name}" }
                        }
                    }
                }
            }
            if !rejected.read().is_empty() {
                div {
                    p { "Rejected:" }
                    ul {
                        for rejection in rejected.read().iter() {
                            li { key: "{rejection.name}",
                                "{rejection.name} - "
                                {rejection.errors.iter().map(|e| e.message.clone()).collect::<Vec<_>>().join(", ")}
                            }
                        }
                    }
                }
            }
        }
    }
}
