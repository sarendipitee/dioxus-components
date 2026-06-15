use dioxus::prelude::*;
use dioxus_components::{
    FileDropZone, FileDropZoneAcceptDisplay, FileDropZoneIdle, FileDropZoneRejectDisplay,
};
use dioxus_primitives::file_drop_zone::{
    AcceptedFile, AcceptedFileType, FileDropZoneAccept,
};

#[component]
pub fn Demo() -> Element {
    let mut files = use_signal(Vec::<String>::new);

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 1rem; width: 100%;",
            FileDropZone {
                accept: FileDropZoneAccept::Types(vec![AcceptedFileType {
                    mime: Some("image/*".into()),
                    extensions: vec![],
                }]),
                multiple: true,
                on_accepted: move |accepted: Vec<AcceptedFile>| {
                    files.set(accepted.into_iter().map(|f| f.name).collect());
                },
                FileDropZoneIdle {
                    p { "Drop images here" }
                }
                FileDropZoneAcceptDisplay {
                    p { "Release to upload!" }
                }
                FileDropZoneRejectDisplay {
                    p { "That file is not an image" }
                }
            }
            if !files.read().is_empty() {
                ul {
                    for name in files.read().iter() {
                        li { key: "{name}", "{name}" }
                    }
                }
            }
        }
    }
}
