use crate::components::tabs::*;
use dioxus::prelude::*;
use dioxus_primitives::tabs::TabsActivationMode;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Tabs {
            default_value: "overview",
            activation_mode: TabsActivationMode::Manual,
            variant: TabsVariant::Outline,
            width: "100%",
            TabList {
                aria_label: "Manual tabs demo",
                TabTrigger { value: "overview", index: 0usize, "Overview" }
                TabTrigger { value: "metrics", index: 1usize, "Metrics" }
                TabTrigger {
                    value: "files",
                    index: 2usize,
                    disabled: true,
                    "Files"
                }
            }
            TabContent {
                index: 0usize,
                value: "overview",
                "Overview waits for manual selection"
            }
            TabContent {
                index: 1usize,
                value: "metrics",
                "Metrics waits for manual selection"
            }
            TabContent { index: 2usize, value: "files", "Files are disabled in this demo" }
        }
    }
}
