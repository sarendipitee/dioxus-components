use dioxus::prelude::*;

#[path = "../../../preview/src/components/file_drop_zone/demos/custom_content/mod.rs"]
mod demo_custom_content;

#[path = "../../../preview/src/components/file_drop_zone/demos/disabled/mod.rs"]
mod demo_disabled;

#[path = "../../../preview/src/components/file_drop_zone/demos/images_only/mod.rs"]
mod demo_images_only;

#[path = "../../../preview/src/components/file_drop_zone/demos/loading/mod.rs"]
mod demo_loading;

#[path = "../../../preview/src/components/file_drop_zone/demos/main/mod.rs"]
mod demo_main;

#[path = "../../../preview/src/components/file_drop_zone/demos/max_count/mod.rs"]
mod demo_max_count;

#[path = "../../../preview/src/components/file_drop_zone/demos/max_size/mod.rs"]
mod demo_max_size;

#[path = "../../../preview/src/components/file_drop_zone/demos/open_button/mod.rs"]
mod demo_open_button;

#[path = "../../../preview/src/components/file_drop_zone/demos/rejected/mod.rs"]
mod demo_rejected;

#[path = "../../../preview/src/components/file_drop_zone/demos/single_file/mod.rs"]
mod demo_single_file;

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
        "custom_content" => rsx! { demo_custom_content::Demo {} },
        "disabled" => rsx! { demo_disabled::Demo {} },
        "images_only" => rsx! { demo_images_only::Demo {} },
        "loading" => rsx! { demo_loading::Demo {} },
        "main" => rsx! { demo_main::Demo {} },
        "max_count" => rsx! { demo_max_count::Demo {} },
        "max_size" => rsx! { demo_max_size::Demo {} },
        "open_button" => rsx! { demo_open_button::Demo {} },
        "rejected" => rsx! { demo_rejected::Demo {} },
        "single_file" => rsx! { demo_single_file::Demo {} },
        _ => rsx! { demo_custom_content::Demo {} },
    }
}
