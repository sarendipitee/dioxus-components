TreeView presents hierarchical content such as files, folders, navigation sections, and outline nodes. Items use the WAI-ARIA tree pattern with roving focus, Arrow/Home/End navigation, optional expansion, and single or multiple selection.

## Component Structure

```rust
TreeView {
    TreeViewItem {
        id: "documents",
        default_expanded: true,
        TreeViewItemContent { "Documents" }
        TreeViewItemGroup {
            TreeViewItem {
                id: "readme",
                TreeViewItemContent { "README.md" }
            }
        }
    }
}
```

`TreeViewItemGroup` is only rendered while its parent is expanded. Keep item IDs stable because they identify selection state and preserve keyboard ordering when branches are reopened.

## Data-oriented tree

Use `use_tree` when the hierarchy is application data rather than hand-written component markup. `Tree` walks nested `TreeNode<T>` values and passes each node to `render_node` with its depth, expansion state, and selection state.

```rust
let tree = use_tree(UseTreeOptions {
    selection_mode: TreeViewSelectionMode::Single,
    ..Default::default()
});
let data = use_signal(|| vec![
    TreeNode::branch(
        "src",
        "src",
        vec![TreeNode::leaf("lib", "lib.rs")],
    ),
]);

rsx! {
    Tree {
        data,
        tree,
        render_node: move |ctx: TreeNodeRenderProps<&'static str>| {
            TreeNodeContent::new(rsx! { "{ctx.node.data}" })
        },
    }
}
```

The node's `id` is its stable identity. Use the `TreeController` from `use_tree` to expand, collapse, select, or clear nodes from application controls.
