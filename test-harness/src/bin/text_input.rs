use dioxus::prelude::*;

#[path = "../../../preview/src/components/text_input/demos/description/mod.rs"]
mod demo_description;

#[path = "../../../preview/src/components/text_input/demos/error/mod.rs"]
mod demo_error;

#[path = "../../../preview/src/components/text_input/demos/main/mod.rs"]
mod demo_main;

#[path = "../../../preview/src/components/text_input/demos/sections/mod.rs"]
mod demo_sections;

#[path = "../../../preview/src/components/text_input/demos/size/mod.rs"]
mod demo_size;

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
        div { id: "dx-preview-block-root", style: "min-height: 100vh;",
            BlockView {}
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
        "description" => rsx! { demo_description::Demo {} },
        "error" => rsx! { demo_error::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        "sections" => rsx! { demo_sections::Demo {} },
        "size" => rsx! { demo_size::Demo {} },
        _ => rsx! { demo_description::Demo {} },
    }
}
