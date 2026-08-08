use dioxus::prelude::*;
use dioxus_components::tooltip::*;
use dioxus_primitives::ContentSide;

#[component]
pub fn Demo() -> Element {
    let mut controlled_open = use_signal(|| Some(false));
    let mut callback_count = use_signal(|| 0usize);

    rsx! {
        div { style: "display: flex; flex-direction: column; align-items: flex-start; gap: 1rem; padding: 2rem;",
            Tooltip {
                "data-testid": "tooltip-root",
                "data-tooltip-root": "main",
                TooltipTrigger {
                    "data-testid": "tooltip-trigger",
                    "data-tooltip-trigger": "main",
                    "Ordinary tooltip trigger"
                }
                TooltipContent {
                    "data-testid": "tooltip-content",
                    "data-tooltip-content": "main",
                    side: ContentSide::Left,
                    "Ordinary tooltip content"
                }
            }

            Tooltip {
                disabled: true,
                TooltipTrigger {
                    "data-testid": "disabled-trigger",
                    "Disabled tooltip trigger"
                }
                TooltipContent { "Disabled tooltip content" }
            }

            Tooltip {
                open: controlled_open(),
                on_open_change: move |is_open| {
                    controlled_open.set(Some(is_open));
                    callback_count += 1;
                },
                TooltipTrigger {
                    "data-testid": "controlled-trigger",
                    "Controlled tooltip trigger"
                }
                TooltipContent {
                    "data-testid": "controlled-content",
                    "Controlled tooltip content"
                }
            }
            output {
                "data-testid": "controlled-state",
                {if controlled_open().unwrap_or(false) { "open" } else { "closed" }}
            }
            output {
                "data-testid": "controlled-callback-count",
                "{callback_count()}"
            }
            button {
                r#type: "button",
                "data-testid": "controlled-toggle",
                onclick: move |_| controlled_open.set(Some(!controlled_open().unwrap_or(false))),
                "Toggle controlled tooltip"
            }
        }
    }
}
