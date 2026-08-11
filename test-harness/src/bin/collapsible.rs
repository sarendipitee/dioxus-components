use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayProvider;

#[path = "../../../preview/src/components/collapsible/demos/controlled/mod.rs"]
mod demo_controlled;

#[path = "../../../preview/src/components/collapsible/demos/default_open/mod.rs"]
mod demo_default_open;

#[path = "../../../preview/src/components/collapsible/demos/disabled/mod.rs"]
mod demo_disabled;

#[path = "../../../preview/src/components/collapsible/demos/inline_actions/mod.rs"]
mod demo_inline_actions;

#[path = "../../../preview/src/components/collapsible/demos/main/mod.rs"]
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
        "controlled" => rsx! { demo_controlled::Demo {} },
        "default_open" => rsx! { demo_default_open::Demo {} },
        "disabled" => rsx! { demo_disabled::Demo {} },
        "inline_actions" => rsx! { demo_inline_actions::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        _ => rsx! { demo_main::Demo {} },
    }
}
