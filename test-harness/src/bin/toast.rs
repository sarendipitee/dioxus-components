use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayManager;

use dioxus_components::DioxusComponentsStyles;
use dioxus_components_themes::DEFAULT_CSS;

#[path = "../../../preview/src/components/toast/demos/custom_duration/mod.rs"]
mod demo_custom_duration;

#[path = "../../../preview/src/components/toast/demos/loading/mod.rs"]
mod demo_loading;

#[path = "../../../preview/src/components/toast/demos/main/mod.rs"]
mod demo_main;

#[path = "../../../preview/src/components/toast/demos/permanent/mod.rs"]
mod demo_permanent;

#[path = "../../../preview/src/components/toast/demos/with_action/mod.rs"]
mod demo_with_action;

#[path = "../../../preview/src/components/toast/demos/with_description/mod.rs"]
mod demo_with_description;

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
        "custom_duration" => rsx! { demo_custom_duration::Demo {} },
        "loading" => rsx! { demo_loading::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        "permanent" => rsx! { demo_permanent::Demo {} },
        "with_action" => rsx! { demo_with_action::Demo {} },
        "with_description" => rsx! { demo_with_description::Demo {} },
        _ => rsx! { demo_main::Demo {} },
    }
}
