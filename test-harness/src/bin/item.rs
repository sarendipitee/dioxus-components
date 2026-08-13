use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayProvider;

use dioxus_components::DioxusComponentsStyles;
use dioxus_components_themes::DEFAULT_CSS;

#[path = "../../../preview/src/components/item/demos/group/mod.rs"]
mod demo_group;

#[path = "../../../preview/src/components/item/demos/image/mod.rs"]
mod demo_image;

#[path = "../../../preview/src/components/item/demos/main/mod.rs"]
mod demo_main;

#[path = "../../../preview/src/components/item/demos/size/mod.rs"]
mod demo_size;

#[path = "../../../preview/src/components/item/demos/variant/mod.rs"]
mod demo_variant;

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
        "group" => rsx! { demo_group::Demo {} },
        "image" => rsx! { demo_image::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        "size" => rsx! { demo_size::Demo {} },
        "variant" => rsx! { demo_variant::Demo {} },
        _ => rsx! { demo_main::Demo {} },
    }
}
