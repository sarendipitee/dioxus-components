use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayProvider;

use dioxus_components::DioxusComponentsStyles;

#[path = "../../../preview/src/components/accordion/demos/customization/mod.rs"]
mod demo_customization;

#[path = "../../../preview/src/components/accordion/demos/disabled/mod.rs"]
mod demo_disabled;

#[path = "../../../preview/src/components/accordion/demos/horizontal/mod.rs"]
mod demo_horizontal;

#[path = "../../../preview/src/components/accordion/demos/main/mod.rs"]
mod demo_main;

#[path = "../../../preview/src/components/accordion/demos/multiple/mod.rs"]
mod demo_multiple;

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
        "customization" => rsx! { demo_customization::Demo {} },
        "disabled" => rsx! { demo_disabled::Demo {} },
        "horizontal" => rsx! { demo_horizontal::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        "multiple" => rsx! { demo_multiple::Demo {} },
        _ => rsx! { demo_main::Demo {} },
    }
}
