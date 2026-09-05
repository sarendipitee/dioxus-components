use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonSize, ButtonVariant};
use dioxus_components::tree_view::{
    use_tree, Tree, TreeNode, TreeNodeContent, TreeNodeRenderProps, TreeViewSelectionMode,
    UseTreeOptions,
};
use dioxus_icons::lucide::{File, Folder, FolderOpen};

#[derive(Clone, Copy, PartialEq)]
enum FileKind {
    File,
    Folder,
}

#[derive(Clone, PartialEq)]
struct FileEntry {
    name: &'static str,
    kind: FileKind,
}

impl FileEntry {
    fn file(name: &'static str) -> Self {
        Self {
            name,
            kind: FileKind::File,
        }
    }

    fn folder(name: &'static str) -> Self {
        Self {
            name,
            kind: FileKind::Folder,
        }
    }
}

#[component]
pub fn Demo() -> Element {
    let data = use_signal(file_tree);
    let tree = use_tree(UseTreeOptions {
        selection_mode: TreeViewSelectionMode::Single,
        initial_expanded: vec!["src".to_string(), "components".to_string()],
        initial_selected: vec!["readme".to_string()],
        ..Default::default()
    });
    let selected = tree.selected_signal();
    let selected_label = use_memo(move || {
        selected
            .read()
            .as_ref()
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

            Tree {
                id: "tree-data-view",
                aria_label: "Data-oriented file explorer",
                data,
                tree,
                render_node: move |ctx: TreeNodeRenderProps<FileEntry>| {
                    let icon = match (ctx.node.data.kind, ctx.expanded) {
                        (FileKind::Folder, true) => rsx! {
                            FolderOpen { size: "1rem", stroke: "currentColor" }
                        },
                        (FileKind::Folder, false) => rsx! {
                            Folder { size: "1rem", stroke: "currentColor" }
                        },
                        (FileKind::File, _) => rsx! {
                            File { size: "1rem", stroke: "currentColor" }
                        },
                    };

                    TreeNodeContent::with_icon(icon, rsx! { "{ctx.node.data.name}" })
                },
            }

            div {
                display: "flex",
                gap: "0.5rem",
                Button {
                    size: ButtonSize::Sm,
                    variant: ButtonVariant::Outline,
                    onclick: move |_| {
                        let nodes = data.read();
                        tree.expand_all(&nodes);
                    },
                    "Expand all"
                }
                Button {
                    size: ButtonSize::Sm,
                    variant: ButtonVariant::Outline,
                    onclick: move |_| tree.collapse_all(),
                    "Collapse all"
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

fn file_tree() -> Vec<TreeNode<FileEntry>> {
    vec![
        TreeNode::branch(
            "src",
            FileEntry::folder("src"),
            vec![
                TreeNode::branch(
                    "components",
                    FileEntry::folder("components"),
                    vec![
                        TreeNode::leaf("button", FileEntry::file("button.rs")),
                        TreeNode::leaf("tree", FileEntry::file("tree_view.rs")),
                    ],
                ),
                TreeNode::leaf("lib", FileEntry::file("lib.rs")),
            ],
        ),
        TreeNode::leaf("readme", FileEntry::file("README.md")),
    ]
}
