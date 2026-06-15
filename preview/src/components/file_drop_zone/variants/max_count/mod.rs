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
                multiple: true,
                max_files: Some(3),
                on_accepted: move |files: Vec<AcceptedFile>| {
                    accepted.set(files.into_iter().map(|f| f.name).collect());
                },
                on_rejected: move |files: Vec<FileRejection>| {
                    rejected.set(files);
                },
                p { "Drop up to 3 files here or click to select" }
            }
            div { "{accepted.read().len()} of 3 accepted" }
            if !accepted.read().is_empty() {
                ul {
                    for name in accepted.read().iter() {
                        li { key: "{name}", "{name}" }
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
