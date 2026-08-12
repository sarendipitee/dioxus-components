use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayProvider;

use dioxus_components::DioxusComponentsStyles;

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
        "composition" => rsx! { demo_composition::Demo {} },
        "loading" => rsx! { demo_loading::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        "sections" => rsx! { demo_sections::Demo {} },
        "states" => rsx! { demo_states::Demo {} },
        "variants" => rsx! { demo_variants::Demo {} },
        _ => rsx! { demo_main::Demo {} },
    }
}
