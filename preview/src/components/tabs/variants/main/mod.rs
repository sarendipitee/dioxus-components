use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::tabs::*;
use dioxus::prelude::*;
use dioxus_primitives::tabs::TabsOrientation;

const VARIANT_OPTIONS: &[(TabsVariant, &str)] = &[
    (TabsVariant::Default, "Default"),
    (TabsVariant::Outline, "Outline"),
    (TabsVariant::Pills, "Pills"),
    (TabsVariant::Ghost, "Ghost"),
    (TabsVariant::None, "None"),
];

const ORIENTATION_OPTIONS: &[(TabsOrientation, &str)] = &[
    (TabsOrientation::Horizontal, "Horizontal"),
    (TabsOrientation::Vertical, "Vertical"),
];

#[component]
pub fn Demo() -> Element {
    let mut variant = use_signal(|| TabsVariant::Default);
    let mut orientation = use_signal(|| TabsOrientation::Horizontal);
    let is_vertical = move || orientation() == TabsOrientation::Vertical;

    rsx! {
        div { display: "grid", gap: "1rem",

            div { display: "grid", gap: "1rem", margin_bottom: "2rem",

                span { font_weight: "600", "Variant" }
                div {
                    display: "flex",
                    flex_wrap: "wrap",
                    align_items: "center",
                    gap: "0.5rem",

                    for (value , label) in VARIANT_OPTIONS {
                        Button {
                            key: "variant-{label}",
                            variant: if variant() == *value { ButtonVariant::Primary } else { ButtonVariant::Outline },
                            size: ButtonSize::Sm,
                            onclick: move |_| variant.set(*value),
                            "{label}"
                        }
                    }
                }

                span { font_weight: "600", "Orientation" }
                div {
                    display: "flex",
                    flex_wrap: "wrap",
                    align_items: "center",
                    gap: "0.5rem",

                    for (value , label) in ORIENTATION_OPTIONS {
                        Button {
                            key: "orientation-{label}",
                            variant: if orientation() == *value { ButtonVariant::Primary } else { ButtonVariant::Outline },
                            size: ButtonSize::Sm,
                            onclick: move |_| orientation.set(*value),
                            "{label}"
                        }
                    }
                }
            }

            Tabs {
                default_value: "overview",
                variant: variant(),
                orientation: Some(orientation()),
                width: "100%",
                max_width: if is_vertical() { "48rem" } else { "100%" },
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
