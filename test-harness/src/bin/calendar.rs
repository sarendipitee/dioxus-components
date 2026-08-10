use dioxus::prelude::*;
use dioxus_primitives::overlay::OverlayProvider;

#[path = "../../../preview/src/components/calendar/demos/internationalized/mod.rs"]
mod demo_internationalized;

#[path = "../../../preview/src/components/calendar/demos/main/mod.rs"]
mod demo_main;

#[path = "../../../preview/src/components/calendar/demos/multi_month/mod.rs"]
mod demo_multi_month;

#[path = "../../../preview/src/components/calendar/demos/range/mod.rs"]
mod demo_range;

#[path = "../../../preview/src/components/calendar/demos/simple/mod.rs"]
mod demo_simple;

#[path = "../../../preview/src/components/calendar/demos/unavailable_dates/mod.rs"]
mod demo_unavailable_dates;

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
        "internationalized" => rsx! { demo_internationalized::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        "multi_month" => rsx! { demo_multi_month::Demo {} },
        "range" => rsx! { demo_range::Demo {} },
        "simple" => rsx! { demo_simple::Demo {} },
        "unavailable_dates" => rsx! { demo_unavailable_dates::Demo {} },
        _ => rsx! { demo_internationalized::Demo {} },
    }
}
