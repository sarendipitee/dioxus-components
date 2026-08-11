use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayProvider;

#[derive(Clone, PartialEq, Debug)]
pub enum Route {
    ComponentDemo {
        name: String,
        demo: String,
        dark_mode: Option<bool>,
    },
}
impl Route {
    pub fn component(name: &str) -> String {
        format!("#/components/{name}")
    }
    pub fn home() -> String {
        "/".to_string()
    }
}

#[path = "../../../preview/src/components/navbar/demos/main/mod.rs"]
mod demo_main;

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
        "main" => rsx! { demo_main::Demo {} },
        _ => rsx! { demo_main::Demo {} },
    }
}
