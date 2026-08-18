use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayManager;

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
        OverlayManager {
            div { id: "dx-preview-block-root", style: "min-height: 100vh;",
                BlockView {}
            }
        }
    }
}

#[component]
fn BlockView() -> Element {
    #[allow(unused_mut)]
    let mut hash = use_signal(|| {
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

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;
        if let Some(window) = web_sys::window() {
            let closure = Closure::<dyn FnMut()>::new(move || {
                if let Some(win) = web_sys::window() {
                    let h = win.location().hash().unwrap_or_default();
                    hash.set(h.trim_start_matches('#').to_string());
                }
            });
            let _ = window
                .add_event_listener_with_callback("hashchange", closure.as_ref().unchecked_ref());
            closure.forget();
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
