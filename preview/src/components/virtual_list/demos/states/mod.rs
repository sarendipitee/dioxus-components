use dioxus::prelude::*;
use dioxus_components::virtual_list::*;

#[component]
pub fn Demo() -> Element {
    let mut count = use_signal(|| 40usize);

    rsx! {
        div {
            id: "virtual-list-states-demo",
            "data-testid": "virtual-list-states-demo",
            class: "dx-virtual-list-states-demo",
            style { r#"
                .virtual-list-states {{
                  position: relative;
                  max-height: 20rem;
                  overflow-y: auto;
                  contain: layout paint;
                  border: 1px solid var(--surface-border);
                  border-radius: 0.625rem;
                }}
                .virtual-list-states-row {{
                  box-sizing: border-box;
                  height: 44px;
                  padding: 0.75rem 0.9rem;
                  border-bottom: 1px solid var(--surface-border);
                }}
            "# }
            div { class: "dx-virtual-list-states-controls",
                button {
                    r#type: "button",
                    onclick: move |_| count.set(0),
                    "Clear list"
                }
                button {
                    r#type: "button",
                    onclick: move |_| count.set(40),
                    "Reset list"
                }
                output {
                    "data-testid": "virtual-list-states-count",
                    aria_live: "polite",
                    "{count()} items"
                }
            }
            VirtualList {
                count,
                buffer: 4usize,
                estimate_size: |_| 44u32,
                id: "virtual-list-states",
                "data-testid": "virtual-list-states",
                aria_label: "Virtual list state fixture",
                class: "virtual-list-states",
                render_item: move |idx: usize| rsx! {
                    div {
                        key: "{idx}",
                        class: "virtual-list-states-row",
                        "data-testid": "virtual-list-state-row",
                        "Item {idx + 1}"
                    }
                },
            }
        }
    }
}
