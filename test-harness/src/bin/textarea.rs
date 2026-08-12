use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayProvider;

use dioxus_components::DioxusComponentsStyles;

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
