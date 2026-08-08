use dioxus::prelude::*;

use dioxus_components::badge::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { display: "flex", align_items: "center", gap: "1rem",
            Badge { "Primary" }
            Badge { variant: BadgeVariant::Secondary, "Secondary" }
            Badge { variant: BadgeVariant::Destructive, "Destructive" }
            Badge { variant: BadgeVariant::Outline, "Outline" }
            Badge {
                background: "var(--success);",
                color: "var(--success-fg)",
                "data-testid": "verified-badge",
                id: "verified-status",
                "aria-label": "Verified status",
                VerifiedIcon {}
                "Verified"
            }
        
        }
    }
}
