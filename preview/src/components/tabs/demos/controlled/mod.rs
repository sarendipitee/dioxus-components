use crate::components::tabs::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut controlled = use_signal(|| Some("overview".to_string()));

    rsx! {
        div {
            display: "grid",
            gap: "0.5rem",

            p {
                margin: "0",
                "Selected: {controlled().clone().unwrap_or_else(|| \"none\".to_string())}"
            }
            Tabs {
                value: Some(ReadSignal::from(controlled)),
                on_value_change: move |next| controlled.set(next),
                keep_mounted: true,
                allow_tab_deactivation: true,
                variant: TabsVariant::Ghost,
                width: "100%",
                TabList {
                    aria_label: "Controlled tabs demo",
                    scrollable: true,
                    TabTrigger { value: "overview", index: 0usize, "Overview" }
                    TabTrigger { value: "metrics", index: 1usize, "Metrics" }
                    TabTrigger { value: "files", index: 2usize, "Files" }
                    TabTrigger { value: "activity", index: 3usize, "Activity" }
                    TabTrigger { value: "settings", index: 4usize, "Settings" }
                    TabTrigger { value: "history", index: 5usize, "History" }
                }
                TabContent {
                    index: 0usize,
                    value: "overview",
                    div { id: "kept-overview-panel", "Overview panel stays mounted" }
                }
                TabContent {
                    index: 1usize,
                    value: "metrics",
                    div { id: "kept-metrics-panel", "Metrics panel stays mounted" }
                }
                TabContent { index: 2usize, value: "files", "Files panel stays mounted" }
                TabContent {
                    index: 3usize,
                    value: "activity",
                    "Activity panel stays mounted"
                }
                TabContent {
                    index: 4usize,
                    value: "settings",
                    "Settings panel stays mounted"
                }
                TabContent {
                    index: 5usize,
                    value: "history",
                    "History panel stays mounted"
                }
            }
        }
    }
}
