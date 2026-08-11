use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayProvider;

#[path = "../../../preview/src/components/slider/demos/dynamic_range/mod.rs"]
mod demo_dynamic_range;

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
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/dx-components-theme.css"),
        }
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
        "dynamic_range" => rsx! { demo_dynamic_range::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        "range" => rsx! { demo_range::Demo {} },
        _ => rsx! { demo_main::Demo {} },
    }
}
