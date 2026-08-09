use dioxus::prelude::*;

#[path = "../../../preview/src/components/dialog/demos/form/mod.rs"]
mod demo_form;

#[path = "../../../preview/src/components/dialog/demos/main/mod.rs"]
mod demo_main;

#[path = "../../../preview/src/components/dialog/demos/nested/mod.rs"]
mod demo_nested;

#[path = "../../../preview/src/components/dialog/demos/scrollable/mod.rs"]
mod demo_scrollable;

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
        "form" => rsx! { demo_form::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        "nested" => rsx! { demo_nested::Demo {} },
        "scrollable" => rsx! { demo_scrollable::Demo {} },
        _ => rsx! { demo_form::Demo {} },
    }
}
