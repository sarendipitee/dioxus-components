use dioxus::prelude::*;
use dioxus_components::tabs::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Tabs {
            default_value: "account",
            variant: TabsVariant::Pills,
            roving_loop: false,
            "data-testid": "tabs-nonloop-root",
            width: "100%",
            TabList {
                aria_label: "Settings tabs demo",
                "data-testid": "tabs-nonloop-list",
                TabTrigger { value: "account", index: 0usize, "Account" }
                TabTrigger { value: "profile", index: 1usize, "Profile" }
                TabTrigger { value: "notifications", index: 2usize, "Notifications" }
                TabTrigger { value: "security", index: 3usize, "Security" }
            }
            TabContent { index: 0usize, value: "account", "Account settings" }
            TabContent { index: 1usize, value: "profile", "Profile settings" }
            TabContent { index: 2usize, value: "notifications", "Notification preferences" }
            TabContent { index: 3usize, value: "security", "Security settings" }
        }
    }
}
