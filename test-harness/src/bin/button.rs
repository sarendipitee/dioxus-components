use dioxus::prelude::*;

#[path = "../../../preview/src/components/button/demos/icon/mod.rs"]
mod demo_icon;

#[path = "../../../preview/src/components/button/demos/main/mod.rs"]
mod demo_main;

#[path = "../../../preview/src/components/button/demos/size/mod.rs"]
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
        "icon" => rsx! { demo_icon::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        "size" => rsx! { demo_size::Demo {} },
        _ => rsx! { demo_icon::Demo {} },
    }
}
