use dioxus::prelude::*;

#[path = "../../../preview/src/components/time_input/demos/main/mod.rs"]
mod demo_main;

#[path = "../../../preview/src/components/time_input/demos/presets/mod.rs"]
mod demo_presets;

#[path = "../../../preview/src/components/time_input/demos/with_picker/mod.rs"]
mod demo_with_picker;

#[path = "../../../preview/src/components/time_input/demos/with_seconds/mod.rs"]
mod demo_with_seconds;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/dx-components-theme.css"),
        }
        div { id: "dx-preview-block-root", style: "min-height: 100vh;",
            BlockView {}
        }
    }
}

#[component]
fn BlockView() -> Element {
    let hash = use_signal(|| {
        #[cfg(target_arch = "wasm32")]
        {
            let window = web_sys::window().unwrap();
            let location = window.location();
            let h = location.hash().unwrap_or_default();
            h.trim_start_matches('#').to_string()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            "".to_string()
        }
    });

    match hash().as_str() {
        "main" => rsx! { demo_main::Demo {} },
        "presets" => rsx! { demo_presets::Demo {} },
        "with_picker" => rsx! { demo_with_picker::Demo {} },
        "with_seconds" => rsx! { demo_with_seconds::Demo {} },
        _ => rsx! { demo_main::Demo {} },
    }
}
