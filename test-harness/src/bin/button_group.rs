use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayManager;

use dioxus_components::DioxusComponentsStyles;
use dioxus_components_themes::DEFAULT_CSS;

#[path = "../../../preview/src/components/button_group/demos/dropdown/mod.rs"]
mod demo_dropdown;

#[path = "../../../preview/src/components/button_group/demos/input/mod.rs"]
mod demo_input;

#[path = "../../../preview/src/components/button_group/demos/main/mod.rs"]
mod demo_main;

#[path = "../../../preview/src/components/button_group/demos/nested/mod.rs"]
mod demo_nested;

#[path = "../../../preview/src/components/button_group/demos/popover/mod.rs"]
mod demo_popover;

#[path = "../../../preview/src/components/button_group/demos/separator/mod.rs"]
mod demo_separator;

#[path = "../../../preview/src/components/button_group/demos/split/mod.rs"]
mod demo_split;

#[path = "../../../preview/src/components/button_group/demos/vertical/mod.rs"]
mod demo_vertical;

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
        "dropdown" => rsx! { demo_dropdown::Demo {} },
        "input" => rsx! { demo_input::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        "nested" => rsx! { demo_nested::Demo {} },
        "popover" => rsx! { demo_popover::Demo {} },
        "separator" => rsx! { demo_separator::Demo {} },
        "split" => rsx! { demo_split::Demo {} },
        "vertical" => rsx! { demo_vertical::Demo {} },
        _ => rsx! { demo_main::Demo {} },
    }
}
