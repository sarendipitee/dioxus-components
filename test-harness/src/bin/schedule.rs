use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayManager;

use dioxus_components::DioxusComponentsStyles;
use dioxus_components_themes::DEFAULT_CSS;

#[path = "../../../preview/src/components/schedule/demos/controlled/mod.rs"]
mod demo_controlled;

#[path = "../../../preview/src/components/schedule/demos/custom_event/mod.rs"]
mod demo_custom_event;

#[path = "../../../preview/src/components/schedule/demos/custom_header/mod.rs"]
mod demo_custom_header;

#[path = "../../../preview/src/components/schedule/demos/drag_and_drop/mod.rs"]
mod demo_drag_and_drop;

#[path = "../../../preview/src/components/schedule/demos/external_drop/mod.rs"]
mod demo_external_drop;

#[path = "../../../preview/src/components/schedule/demos/internationalized/mod.rs"]
mod demo_internationalized;

#[path = "../../../preview/src/components/schedule/demos/main/mod.rs"]
mod demo_main;

#[path = "../../../preview/src/components/schedule/demos/multi_day/mod.rs"]
mod demo_multi_day;

#[path = "../../../preview/src/components/schedule/demos/multi_view/mod.rs"]
mod demo_multi_view;

#[path = "../../../preview/src/components/schedule/demos/resize/mod.rs"]
mod demo_resize;

#[path = "../../../preview/src/components/schedule/demos/slot_selection/mod.rs"]
mod demo_slot_selection;

#[path = "../../../preview/src/components/schedule/demos/static/mod.rs"]
mod demo_static;

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
        "controlled" => rsx! { demo_controlled::Demo {} },
        "custom_event" => rsx! { demo_custom_event::Demo {} },
        "custom_header" => rsx! { demo_custom_header::Demo {} },
        "drag_and_drop" => rsx! { demo_drag_and_drop::Demo {} },
        "external_drop" => rsx! { demo_external_drop::Demo {} },
        "internationalized" => rsx! { demo_internationalized::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        "multi_day" => rsx! { demo_multi_day::Demo {} },
        "multi_view" => rsx! { demo_multi_view::Demo {} },
        "resize" => rsx! { demo_resize::Demo {} },
        "slot_selection" => rsx! { demo_slot_selection::Demo {} },
        "static" => rsx! { demo_static::Demo {} },
        _ => rsx! { demo_main::Demo {} },
    }
}
