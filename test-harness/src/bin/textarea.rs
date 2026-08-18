use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayManager;

use dioxus_components::DioxusComponentsStyles;
use dioxus_components_themes::DEFAULT_CSS;

#[path = "../../../preview/src/components/textarea/demos/autosize/mod.rs"]
mod demo_autosize;

#[path = "../../../preview/src/components/textarea/demos/bottom_section/mod.rs"]
mod demo_bottom_section;

#[path = "../../../preview/src/components/textarea/demos/fade/mod.rs"]
mod demo_fade;

#[path = "../../../preview/src/components/textarea/demos/ghost/mod.rs"]
mod demo_ghost;

#[path = "../../../preview/src/components/textarea/demos/main/mod.rs"]
mod demo_main;

#[path = "../../../preview/src/components/textarea/demos/outline/mod.rs"]
mod demo_outline;

#[path = "../../../preview/src/components/textarea/demos/resize/mod.rs"]
mod demo_resize;

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
        "autosize" => rsx! { demo_autosize::Demo {} },
        "bottom_section" => rsx! { demo_bottom_section::Demo {} },
        "fade" => rsx! { demo_fade::Demo {} },
        "ghost" => rsx! { demo_ghost::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        "outline" => rsx! { demo_outline::Demo {} },
        "resize" => rsx! { demo_resize::Demo {} },
        _ => rsx! { demo_main::Demo {} },
    }
}
