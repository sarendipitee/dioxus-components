use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayProvider;

#[path = "../../../preview/src/components/combobox/demos/autocomplete/mod.rs"]
mod demo_autocomplete;

#[path = "../../../preview/src/components/combobox/demos/controlled/mod.rs"]
mod demo_controlled;

#[path = "../../../preview/src/components/combobox/demos/disabled/mod.rs"]
mod demo_disabled;

#[path = "../../../preview/src/components/combobox/demos/dynamic/mod.rs"]
mod demo_dynamic;

#[path = "../../../preview/src/components/combobox/demos/main/mod.rs"]
mod demo_main;

#[path = "../../../preview/src/components/combobox/demos/multi_select/mod.rs"]
mod demo_multi_select;

#[path = "../../../preview/src/components/combobox/demos/tags_input/mod.rs"]
mod demo_tags_input;

#[path = "../../../preview/src/components/combobox/demos/virtualized/mod.rs"]
mod demo_virtualized;

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
        "autocomplete" => rsx! { demo_autocomplete::Demo {} },
        "controlled" => rsx! { demo_controlled::Demo {} },
        "disabled" => rsx! { demo_disabled::Demo {} },
        "dynamic" => rsx! { demo_dynamic::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        "multi_select" => rsx! { demo_multi_select::Demo {} },
        "tags_input" => rsx! { demo_tags_input::Demo {} },
        "virtualized" => rsx! { demo_virtualized::Demo {} },
        _ => rsx! { demo_main::Demo {} },
    }
}
