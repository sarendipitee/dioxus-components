use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayManager;

use dioxus_components::DioxusComponentsStyles;
use dioxus_components_themes::DEFAULT_CSS;

#[path = "../../../preview/src/components/input/demos/composition/mod.rs"]
mod demo_composition;

#[path = "../../../preview/src/components/input/demos/loading/mod.rs"]
mod demo_loading;

#[path = "../../../preview/src/components/input/demos/main/mod.rs"]
mod demo_main;

#[path = "../../../preview/src/components/input/demos/sections/mod.rs"]
mod demo_sections;

#[path = "../../../preview/src/components/input/demos/states/mod.rs"]
mod demo_states;

#[path = "../../../preview/src/components/input/demos/variants/mod.rs"]
mod demo_variants;

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
        "composition" => rsx! { demo_composition::Demo {} },
        "loading" => rsx! { demo_loading::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        "sections" => rsx! { demo_sections::Demo {} },
        "states" => rsx! { demo_states::Demo {} },
        "variants" => rsx! { demo_variants::Demo {} },
        _ => rsx! { demo_main::Demo {} },
    }
}
