use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayManager;

use dioxus_components::DioxusComponentsStyles;
use dioxus_components_themes::DEFAULT_CSS;

#[path = "../../../preview/src/components/slider/demos/disabled/mod.rs"]
mod demo_disabled;

#[path = "../../../preview/src/components/slider/demos/dynamic_range/mod.rs"]
mod demo_dynamic_range;

#[path = "../../../preview/src/components/slider/demos/field/mod.rs"]
mod demo_field;

#[path = "../../../preview/src/components/slider/demos/inverted/mod.rs"]
mod demo_inverted;

#[path = "../../../preview/src/components/slider/demos/main/mod.rs"]
mod demo_main;

#[path = "../../../preview/src/components/slider/demos/range/mod.rs"]
mod demo_range;

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
        "disabled" => rsx! { demo_disabled::Demo {} },
        "dynamic_range" => rsx! { demo_dynamic_range::Demo {} },
        "field" => rsx! { demo_field::Demo {} },
        "inverted" => rsx! { demo_inverted::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        "range" => rsx! { demo_range::Demo {} },
        _ => rsx! { demo_main::Demo {} },
    }
}
