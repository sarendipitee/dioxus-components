use dioxus::prelude::*;
use dioxus_components::split_pane::*;

#[component]
pub fn Demo() -> Element {
    let mut last_size =
        use_signal(|| "Drag the divider or focus it and use arrow keys".to_string());
    let mut resize_start = use_signal(|| "Resize has not started".to_string());
    let mut resize_end = use_signal(|| "Resize has not ended".to_string());
    let mut resize_start_count = use_signal(|| 0_u32);
    let mut resize_end_count = use_signal(|| 0_u32);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 0.75rem; width: 100%; max-width: 760px;",
            div { style: "font-size: var(--text-sm); color: var(--fg);", "{last_size}" }
            output {
                "data-testid": "split-pane-resize-start",
                aria_live: "polite",
                "{resize_start}"
            }
            output {
                "data-testid": "split-pane-resize-end",
                aria_live: "polite",
                "{resize_end}"
            }
            div {
                style: "height: 260px;",
                SplitPane {
                    aria_label: "Primary workspace",
                    direction: SplitPaneDirection::Horizontal,
                    step: 24.0,
                    on_resize_start: move |event: SplitPaneResizeEvent| {
                        resize_start_count += 1;
                        resize_start.set(format!(
                            "Resize started: {} ({})",
                            resize_source(event.source),
                            resize_start_count(),
                        ));
                    },
                    on_resize: move |event: SplitPaneResizeEvent| {
                        if let Some(size) = event.sizes.first() {
                            last_size.set(format!("Left pane: {}", format_size(size)));
                        }
                    },
                    on_resize_end: move |event: SplitPaneResizeEvent| {
                        resize_end_count += 1;
                        resize_end.set(format!(
                            "Resize ended: {} ({})",
                            resize_source(event.source),
                            resize_end_count(),
                        ));
                    },
                    Pane {
                        aria_label: "Navigator pane",
                        default_size: SplitPaneSize::percent(35.0),
                        min_size: SplitPaneSize::px(160.0),
                        DemoPanel { title: "Navigator", body: "Project tree, filters, and quick actions." }
                    }
                    SplitPaneDivider { aria_label: "Resize navigator and preview" }
                    Pane {
                        aria_label: "Preview pane",
                        min_size: SplitPaneSize::px(220.0),
                        DemoPanel { title: "Preview", body: "Resizable content region using pointer and keyboard input." }
                    }
                }
            }
        }
    }
}

#[component]
fn DemoPanel(title: &'static str, body: &'static str) -> Element {
    rsx! {
        section {
            style: "height: 100%; box-sizing: border-box; padding: 1rem; display: flex; flex-direction: column; gap: 0.5rem;",
            h3 { style: "margin: 0; font-size: var(--text-md);", "{title}" }
            p { style: "margin: 0; color: var(--fg); line-height: 1.4;", "{body}" }
        }
    }
}

fn format_size(size: &SplitPaneSize) -> String {
    match size {
        SplitPaneSize::Px(px) => format!("{px:.0}px"),
        SplitPaneSize::Percent(percent) => format!("{percent:.0}%"),
    }
}

fn resize_source(source: SplitPaneResizeSource) -> &'static str {
    match source {
        SplitPaneResizeSource::Pointer => "Pointer",
        SplitPaneResizeSource::Keyboard => "Keyboard",
    }
}
