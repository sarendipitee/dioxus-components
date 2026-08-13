use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayProvider;

use dioxus_components::DioxusComponentsStyles;
use dioxus_components_themes::DEFAULT_CSS;

#[path = "../../../preview/src/components/data_table/demos/controlled/mod.rs"]
mod demo_controlled;

#[path = "../../../preview/src/components/data_table/demos/density/mod.rs"]
mod demo_density;

#[path = "../../../preview/src/components/data_table/demos/expansion/mod.rs"]
mod demo_expansion;

#[path = "../../../preview/src/components/data_table/demos/main/mod.rs"]
mod demo_main;

#[path = "../../../preview/src/components/data_table/demos/selectable/mod.rs"]
mod demo_selectable;

#[path = "../../../preview/src/components/data_table/demos/server_backed/mod.rs"]
mod demo_server_backed;

#[path = "../../../preview/src/components/data_table/demos/virtualized/mod.rs"]
mod demo_virtualized;

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
        "controlled" => rsx! { demo_controlled::Demo {} },
        "density" => rsx! { demo_density::Demo {} },
        "expansion" => rsx! { demo_expansion::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        "selectable" => rsx! { demo_selectable::Demo {} },
        "server_backed" => rsx! { demo_server_backed::Demo {} },
        "virtualized" => rsx! { demo_virtualized::Demo {} },
        _ => rsx! { demo_main::Demo {} },
    }
}
