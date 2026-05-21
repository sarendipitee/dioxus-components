use crate::components::tabs::*;
use dioxus::prelude::*;
use dioxus_primitives::tabs::TabsOrientation;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Tabs {
            default_value: "activity",
            orientation: Some(TabsOrientation::Vertical),
            variant: TabsVariant::Default,
            width: "100%",
            TabList {
                aria_label: "Vertical tabs demo",
                TabTrigger { value: "overview", index: 0usize, "Overview" }
                TabTrigger { value: "activity", index: 1usize, "Activity" }
                TabTrigger { value: "files", index: 2usize, "Files" }
            }
            TabContent { index: 0usize, value: "overview", "Overview panel" }
            TabContent { index: 1usize, value: "activity", "Activity panel" }
            TabContent { index: 2usize, value: "files", "Files panel" }
        }
    }
}
