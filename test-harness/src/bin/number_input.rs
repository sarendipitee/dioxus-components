use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayProvider;

use dioxus_components::DioxusComponentsStyles;
use dioxus_components_themes::DEFAULT_CSS;

#[path = "../../../preview/src/components/number_input/demos/constraints/mod.rs"]
mod demo_constraints;

#[path = "../../../preview/src/components/number_input/demos/controls/mod.rs"]
mod demo_controls;

#[path = "../../../preview/src/components/number_input/demos/formatting/mod.rs"]
mod demo_formatting;

#[path = "../../../preview/src/components/number_input/demos/main/mod.rs"]
mod demo_main;

#[path = "../../../preview/src/components/number_input/demos/states/mod.rs"]
mod demo_states;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        DioxusComponentsStyles {},
        document::Style { {DEFAULT_CSS} }
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
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
        "constraints" => rsx! { demo_constraints::Demo {} },
        "controls" => rsx! { demo_controls::Demo {} },
        "formatting" => rsx! { demo_formatting::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        "states" => rsx! { demo_states::Demo {} },
        _ => rsx! { demo_main::Demo {} },
    }
}
