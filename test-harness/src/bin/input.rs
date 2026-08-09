use dioxus::prelude::*;

#[path = "../../../preview/src/components/input/demos/composition/mod.rs"]
mod demo_composition;

#[path = "../../../preview/src/components/input/demos/loading/mod.rs"]
mod demo_loading;

#[path = "../../../preview/src/components/input/demos/main/mod.rs"]
mod demo_main;

#[path = "../../../preview/src/components/input/demos/sections/mod.rs"]
mod demo_sections;

#[path = "../../../preview/src/components/input/demos/states/mod.rs"]
mod demo_states;

#[path = "../../../preview/src/components/input/demos/variants/mod.rs"]
mod demo_variants;

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
