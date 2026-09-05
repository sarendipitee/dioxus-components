use dioxus::prelude::*;
use dioxus_components::tree_view::{
    TreeView, TreeViewItem, TreeViewItemContent, TreeViewItemGroup, TreeViewSelectionMode,
};
use dioxus_icons::lucide::{File, Folder};

#[component]
pub fn Demo() -> Element {
    let mut selected = use_signal(|| Some(vec!["readme".to_string()]));
    let selected_label = use_memo(move || {
        selected()
            .and_then(|items| items.first().cloned())
            .unwrap_or_else(|| "Nothing selected".to_string())
    });

    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            gap: "1rem",
            width: "100%",
            max_width: "24rem",

            TreeView {
                id: "tree-view",
                aria_label: "File explorer",
                selection_mode: TreeViewSelectionMode::Single,
                selected,
                on_selected_change: move |items| selected.set(Some(items)),

                TreeViewItem {
                    id: "src",
                    default_expanded: true,
                    posinset: 1,
                    setsize: 2,
                    TreeViewItemContent {
                        "data-testid": "tree-src",
                        icon: rsx! { Folder { size: "1rem", stroke: "currentColor" } },
                        "src"
                    }
                    TreeViewItemGroup {
                        TreeViewItem {
                            id: "components",
                            default_expanded: true,
                            posinset: 1,
                            setsize: 2,
                            TreeViewItemContent {
                                "data-testid": "tree-components",
                                icon: rsx! { Folder { size: "1rem", stroke: "currentColor" } },
                                "components"
                            }
                            TreeViewItemGroup {
                                TreeViewItem {
                                    id: "button",
                                    posinset: 1,
                                    setsize: 2,
                                    TreeViewItemContent {
                                        "data-testid": "tree-button",
                                        icon: rsx! { File { size: "1rem", stroke: "currentColor" } },
                                        "button.rs"
                                    }
                                }
                                TreeViewItem {
                                    id: "tree-view-file",
                                    posinset: 2,
                                    setsize: 2,
                                    TreeViewItemContent {
                                        icon: rsx! { File { size: "1rem", stroke: "currentColor" } },
                                        "tree_view.rs"
                                    }
                                }
                            }
                        }
                        TreeViewItem {
                            id: "lib",
                            posinset: 2,
                            setsize: 2,
                            TreeViewItemContent {
                                icon: rsx! { File { size: "1rem", stroke: "currentColor" } },
                                "lib.rs"
                            }
                        }
                    }
                }
                TreeViewItem {
                    id: "readme",
                    posinset: 2,
                    setsize: 2,
                    TreeViewItemContent {
                        "data-testid": "tree-readme",
                        icon: rsx! { File { size: "1rem", stroke: "currentColor" } },
                        "README.md"
                    }
                }
            }

            p {
                margin: "0",
                color: "var(--fg-muted)",
                "Selected: ",
                code { {selected_label} }
            }
        }
    }
}
