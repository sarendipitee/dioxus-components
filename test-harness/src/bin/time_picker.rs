use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayProvider;

use dioxus_components::DioxusComponentsStyles;
use dioxus_components_themes::DEFAULT_CSS;

#[path = "../../../preview/src/components/time_picker/demos/clearable/mod.rs"]
mod demo_clearable;

#[path = "../../../preview/src/components/time_picker/demos/duration/mod.rs"]
mod demo_duration;

#[path = "../../../preview/src/components/time_picker/demos/main/mod.rs"]
mod demo_main;

#[path = "../../../preview/src/components/time_picker/demos/presets/mod.rs"]
mod demo_presets;

#[path = "../../../preview/src/components/time_picker/demos/seconds_12_hour/mod.rs"]
mod demo_seconds_12_hour;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        DioxusComponentsStyles {},
        document::Style { {DEFAULT_CSS} }
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        OverlayProvider {
            div { id: "dx-preview-block-root", style: "min-height: 100vh;",
                BlockView {}
            }
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
        "clearable" => rsx! { demo_clearable::Demo {} },
        "duration" => rsx! { demo_duration::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        "presets" => rsx! { demo_presets::Demo {} },
        "seconds_12_hour" => rsx! { demo_seconds_12_hour::Demo {} },
        _ => rsx! { demo_main::Demo {} },
    }
}
