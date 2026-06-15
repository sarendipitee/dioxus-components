use dioxus::prelude::*;
use dioxus_components::FileDropZone;
use dioxus_primitives::file_drop_zone::{
    AcceptedFile, AcceptedFileType, FileDropZoneAccept, FileRejection,
};

#[component]
pub fn Demo() -> Element {
    let mut accepted = use_signal(Vec::<String>::new);
    let mut rejected = use_signal(Vec::<FileRejection>::new);

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 1rem; width: 100%;",
            FileDropZone {
                // PDFs only, and no larger than 512 KB.
                accept: FileDropZoneAccept::Types(vec![AcceptedFileType {
                    mime: Some("application/pdf".into()),
                    extensions: vec!["pdf".into()],
                }]),
                max_size: Some(512 * 1024),
                multiple: true,
                on_accepted: move |files: Vec<AcceptedFile>| {
                    accepted.set(files.into_iter().map(|f| f.name).collect());
                },
                on_rejected: move |files: Vec<FileRejection>| {
                    rejected.set(files);
                },
                p { "Drop PDF files here or click to select" }
                p { style: "font-size: 0.85em; opacity: 0.7;", "PDF only, max 512 KB" }
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
                                strong { "{rejection.name}" }
                                ul {
                                    for error in rejection.errors.iter() {
                                        li { key: "{error.code.as_str()}",
                                            "[{error.code.as_str()}] {error.message}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
