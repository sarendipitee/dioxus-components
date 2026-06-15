use dioxus::prelude::*;
use dioxus_components::button::Button;
use dioxus_components::FileDropZone;
use dioxus_primitives::file_drop_zone::AcceptedFile;

#[component]
pub fn Demo() -> Element {
    let mut files = use_signal(Vec::<String>::new);
    // A monotonic token: bumping it asks the zone to open its file picker.
    let mut open_request = use_signal(|| None::<u64>);
    let mut token = use_signal(|| 0u64);

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 1rem; width: 100%;",
            Button {
                onclick: move |_| {
                    token += 1;
                    open_request.set(Some(token()));
                },
                "Choose files"
            }
            FileDropZone {
                // Disable click-to-open so the external button is the only trigger.
                activate_on_click: false,
                open_request: ReadSignal::new(open_request),
                on_accepted: move |accepted: Vec<AcceptedFile>| {
                    files.set(accepted.into_iter().map(|f| f.name).collect());
                },
                p { "Drag files here, or use the button above" }
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
