use dioxus_components::split_pane::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut resizable = use_signal(|| true);

    let resizing_status = if resizable() {
        "Resizing enabled"
    } else {
        "Resizing disabled"
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 0.75rem; width: 100%; max-width: 760px;",
            button {
                onclick: move |_| resizable.toggle(),
                if resizable() { "Disable resizing" } else { "Enable resizing" }
            }
            output { aria_live: "polite", "{resizing_status}" }
            div {
                "data-testid": "constraints-split-pane",
                style: "height: 260px;",
                SplitPane {
                    aria_label: "Constrained workspace",
                    direction: SplitPaneDirection::Horizontal,
                    resizable,
                    Pane {
                        default_size: SplitPaneSize::px(220.0),
                        min_size: SplitPaneSize::px(160.0),
                        max_size: SplitPaneSize::px(320.0),
                        Panel { title: "Fixed Range", body: "This pane stays between 160px and 320px." }
                    }
                    SplitPaneDivider { aria_label: "Resize constrained panes" }
                    Pane {
                        min_size: SplitPaneSize::percent(30.0),
                        Panel { title: "Flexible", body: "The neighboring pane absorbs the remaining width." }
                    }
                }
            }
        }
    }
}

#[component]
fn Panel(title: &'static str, body: &'static str) -> Element {
    rsx! {
        div {
            style: "height: 100%; box-sizing: border-box; padding: 1rem;",
            h3 { style: "margin: 0 0 0.5rem; font-size: var(--text-md);", "{title}" }
            p { style: "margin: 0; color: var(--fg);", "{body}" }
        }
    }
}
