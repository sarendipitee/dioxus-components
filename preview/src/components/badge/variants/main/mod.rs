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
                variant: BadgeVariant::Secondary,
                style: "background: var(--focus)",
                VerifiedIcon {}
                "Verified"
            }
        }
    }
}
