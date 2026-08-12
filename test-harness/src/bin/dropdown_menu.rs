use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayProvider;

use dioxus_components::DioxusComponentsStyles;

#[path = "../../../preview/src/components/dropdown_menu/demos/checkable/mod.rs"]
mod demo_checkable;

#[path = "../../../preview/src/components/dropdown_menu/demos/filterable/mod.rs"]
mod demo_filterable;

#[path = "../../../preview/src/components/dropdown_menu/demos/main/mod.rs"]
mod demo_main;

#[path = "../../../preview/src/components/dropdown_menu/demos/nested_submenus/mod.rs"]
mod demo_nested_submenus;

#[path = "../../../preview/src/components/dropdown_menu/demos/presentation/mod.rs"]
mod demo_presentation;

#[path = "../../../preview/src/components/dropdown_menu/demos/structure/mod.rs"]
mod demo_structure;

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
        "checkable" => rsx! { demo_checkable::Demo {} },
        "filterable" => rsx! { demo_filterable::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        "nested_submenus" => rsx! { demo_nested_submenus::Demo {} },
        "presentation" => rsx! { demo_presentation::Demo {} },
        "structure" => rsx! { demo_structure::Demo {} },
        _ => rsx! { demo_main::Demo {} },
    }
}
