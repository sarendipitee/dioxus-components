use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::tabs::*;
use dioxus::prelude::*;
use dioxus_primitives::tabs::TabsOrientation;

#[component]
pub fn Demo() -> Element {

    rsx! {
        div { display: "grid", gap: "1rem",

            Tabs { default_value: "overview", width: "100%",
                TabList { aria_label: "Automatic tabs demo",
                    TabTrigger { value: "overview", index: 0usize, "Overview" }
                    TabTrigger { value: "metrics", index: 1usize, "Metrics" }
                    TabTrigger { value: "files", index: 2usize, "Files" }
                }
                TabContent { index: 0usize, value: "overview", "Overview content" }
                TabContent { index: 1usize, value: "metrics", "Metrics content" }
                TabContent { index: 2usize, value: "files", "Files content" }
            }
        }
    }
}
