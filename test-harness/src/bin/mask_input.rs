use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayProvider;

#[path = "../../../preview/src/components/mask_input/demos/behavior/mod.rs"]
mod demo_behavior;

#[path = "../../../preview/src/components/mask_input/demos/main/mod.rs"]
mod demo_main;

#[path = "../../../preview/src/components/mask_input/demos/tokens/mod.rs"]
mod demo_tokens;

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
        "behavior" => rsx! { demo_behavior::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        "tokens" => rsx! { demo_tokens::Demo {} },
        _ => rsx! { demo_behavior::Demo {} },
    }
}
