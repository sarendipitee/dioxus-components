use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayManager;

use dioxus_components::DioxusComponentsStyles;
use dioxus_components_themes::DEFAULT_CSS;

#[path = "../../../preview/src/components/dropdown_menu/demos/checkable/mod.rs"]
mod demo_checkable;

#[path = "../../../preview/src/components/dropdown_menu/demos/filterable/mod.rs"]
mod demo_filterable;

#[path = "../../../preview/src/components/dropdown_menu/demos/main/mod.rs"]
mod demo_main;

#[path = "../../../preview/src/components/dropdown_menu/demos/nested_submenus/mod.rs"]
mod demo_nested_submenus;

#[path = "../../../preview/src/components/dropdown_menu/demos/presentation/mod.rs"]
mod demo_presentation;

#[path = "../../../preview/src/components/dropdown_menu/demos/structure/mod.rs"]
mod demo_structure;

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
        "checkable" => rsx! { demo_checkable::Demo {} },
        "filterable" => rsx! { demo_filterable::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        "nested_submenus" => rsx! { demo_nested_submenus::Demo {} },
        "presentation" => rsx! { demo_presentation::Demo {} },
        "structure" => rsx! { demo_structure::Demo {} },
        _ => rsx! { demo_main::Demo {} },
    }
}
