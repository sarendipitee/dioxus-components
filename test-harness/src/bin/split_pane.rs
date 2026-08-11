use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayProvider;

#[path = "../../../preview/src/components/split_pane/demos/constraints/mod.rs"]
mod demo_constraints;

#[path = "../../../preview/src/components/split_pane/demos/controlled/mod.rs"]
mod demo_controlled;

#[path = "../../../preview/src/components/split_pane/demos/custom_divider/mod.rs"]
mod demo_custom_divider;

#[path = "../../../preview/src/components/split_pane/demos/main/mod.rs"]
mod demo_main;

#[path = "../../../preview/src/components/split_pane/demos/multi_pane/mod.rs"]
mod demo_multi_pane;

#[path = "../../../preview/src/components/split_pane/demos/nested/mod.rs"]
mod demo_nested;

#[path = "../../../preview/src/components/split_pane/demos/persistence/mod.rs"]
mod demo_persistence;

#[path = "../../../preview/src/components/split_pane/demos/snap/mod.rs"]
mod demo_snap;

#[path = "../../../preview/src/components/split_pane/demos/vertical/mod.rs"]
mod demo_vertical;

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
        "constraints" => rsx! { demo_constraints::Demo {} },
        "controlled" => rsx! { demo_controlled::Demo {} },
        "custom_divider" => rsx! { demo_custom_divider::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        "multi_pane" => rsx! { demo_multi_pane::Demo {} },
        "nested" => rsx! { demo_nested::Demo {} },
        "persistence" => rsx! { demo_persistence::Demo {} },
        "snap" => rsx! { demo_snap::Demo {} },
        "vertical" => rsx! { demo_vertical::Demo {} },
        _ => rsx! { demo_main::Demo {} },
    }
}
