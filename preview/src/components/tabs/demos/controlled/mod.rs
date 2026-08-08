use crate::components::tabs::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut controlled = use_signal(|| Some("overview".to_string()));
    let mut callback_count = use_signal(|| 0usize);
    let mut show_reports = use_signal(|| false);
    let selected_label = controlled().clone().unwrap_or_else(|| "none".to_string());

    rsx! {
        div {
            display: "grid",
            gap: "0.5rem",

            button {
                onclick: move |_| {
                    if show_reports() && controlled().as_deref() == Some("reports") {
                        controlled.set(Some("overview".to_string()));
                    }
                    show_reports.set(!show_reports());
                },
                "Add reports tab"
            }
            output {
                "data-testid": "tabs-controlled-output",
                "Selected value: {selected_label}; Callback count: {callback_count()}"
            }
            Tabs {
                "data-testid": "tabs-controlled-root",
                aria_label: "Controlled tabs fixture",
                value: Some(ReadSignal::from(controlled)),
                on_value_change: move |next| {
                    callback_count += 1;
                    controlled.set(next);
                },
                keep_mounted: true,
                allow_tab_deactivation: true,
                variant: TabsVariant::Ghost,
                width: "100%",
                TabList {
                    "data-testid": "tabs-controlled-list",
                    aria_label: "Controlled tabs demo",
                    scrollable: true,
                    TabTrigger {
                        "data-testid": "tabs-controlled-overview-trigger",
                        value: "overview",
                        index: 0usize,
                        "Overview"
                    }
                    TabTrigger { value: "metrics", index: 1usize, "Metrics" }
                    TabTrigger { value: "files", index: 2usize, "Files" }
                    TabTrigger { value: "activity", index: 3usize, "Activity" }
                    TabTrigger { value: "settings", index: 4usize, "Settings" }
                    TabTrigger { value: "history", index: 5usize, "History" }
                    if show_reports() {
                        TabTrigger { value: "reports", index: 6usize, "Reports" }
                    }
                }
                TabContent {
                    "data-testid": "tabs-controlled-overview-panel",
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
                if show_reports() {
                    TabContent {
                        index: 6usize,
                        value: "reports",
                        "Reports panel stays mounted"
                    }
                }
            }
        }
    }
}
