use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayProvider;

#[path = "../../../preview/src/components/checkbox/demos/disabled/mod.rs"]
mod demo_disabled;

#[path = "../../../preview/src/components/checkbox/demos/element_label/mod.rs"]
mod demo_element_label;

#[path = "../../../preview/src/components/checkbox/demos/indeterminate/mod.rs"]
mod demo_indeterminate;

#[path = "../../../preview/src/components/checkbox/demos/label_description/mod.rs"]
mod demo_label_description;

#[path = "../../../preview/src/components/checkbox/demos/main/mod.rs"]
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
        "disabled" => rsx! { demo_disabled::Demo {} },
        "element_label" => rsx! { demo_element_label::Demo {} },
        "indeterminate" => rsx! { demo_indeterminate::Demo {} },
        "label_description" => rsx! { demo_label_description::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        _ => rsx! { demo_main::Demo {} },
    }
}
