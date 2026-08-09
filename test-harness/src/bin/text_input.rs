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
    rsx! { demo_main::Demo {} }
}
