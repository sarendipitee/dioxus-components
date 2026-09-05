use crate::component_styles;
use dioxus::prelude::*;
use dioxus_icons::lucide::ChevronRight;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use dioxus_primitives::tree_view;

#[component_styles("./style.css")]
struct Styles;

pub use dioxus_primitives::tree::{
    use_tree, TreeController, TreeNode, TreeNodeRenderProps, UseTreeOptions,
};
pub use dioxus_primitives::tree_view::TreeViewSelectionMode;

/// The styled content returned by a data-oriented tree node renderer.
#[derive(Clone, PartialEq)]
pub struct TreeNodeContent {
    /// Optional leading icon rendered in the tree item's icon slot.
    pub icon: Option<Element>,
    /// Label or custom content rendered in the tree item's label slot.
    pub content: Element,
}

impl TreeNodeContent {
    /// Create node content without an icon.
    pub fn new(content: Element) -> Self {
        Self {
            icon: None,
            content,
        }
    }

    /// Create node content with a leading icon.
    pub fn with_icon(icon: Element, content: Element) -> Self {
        Self {
            icon: Some(icon),
            content,
        }
    }
}

/// Props for the styled [`TreeView`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TreeViewProps {
    /// The ID of the tree root.
    #[props(default)]
    pub id: Option<String>,
    /// Whether the tree and all of its items are disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,
    /// Selection behavior for tree items.
    #[props(default)]
    pub selection_mode: ReadSignal<TreeViewSelectionMode>,
    /// Controlled selected item IDs.
    #[props(default)]
    pub selected: ReadSignal<Option<Vec<String>>>,
    /// Initially selected item IDs when `selected` is not provided.
    #[props(default)]
    pub default_selected: Vec<String>,
    /// Whether focus wraps from the last item to the first item.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,
    /// Additional attributes for the tree root.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// Tree items.
    pub children: Element,
    /// Callback fired after the selected item IDs change.
    #[props(default)]
    pub on_selected_change: Callback<Vec<String>>,
}

/// Props for a styled [`TreeViewItem`].
#[derive(Props, Clone, PartialEq)]
pub struct TreeViewItemProps {
    /// Stable item ID used for identity, selection, and the rendered treeitem ID.
    pub id: ReadSignal<String>,
    /// Whether this item is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,
    /// Whether this item is expanded initially.
    #[props(default)]
    pub default_expanded: ReadSignal<bool>,
    /// Controlled expanded state.
    #[props(default)]
    pub expanded: ReadSignal<Option<bool>>,
    /// Callback fired when the expanded state changes.
    #[props(default)]
    pub on_expanded_change: Callback<bool>,
    /// Optional one-based position among the item's siblings.
    #[props(default)]
    pub posinset: Option<usize>,
    /// Optional number of siblings at the item's level.
    #[props(default)]
    pub setsize: Option<usize>,
    /// Callback fired when this item is selected. The argument is its new selected state.
    #[props(default)]
    pub on_select: Callback<bool>,
    /// Additional attributes for the item wrapper.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The item's content and optional [`TreeViewItemGroup`].
    pub children: Element,
}

/// Props for the styled focusable row of a [`TreeViewItem`].
#[derive(Props, Clone, PartialEq)]
pub struct TreeViewItemContentProps {
    /// Optional leading icon.
    #[props(default)]
    pub icon: Option<Element>,
    /// Additional attributes for the focusable treeitem row.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The item's label and optional custom content.
    pub children: Element,
}

/// Props for a styled nested group of a [`TreeViewItem`].
#[derive(Props, Clone, PartialEq)]
pub struct TreeViewItemGroupProps {
    /// Additional attributes for the group element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// Nested tree items.
    pub children: Element,
}

/// Props for a data-oriented [`Tree`].
#[derive(Props, Clone, PartialEq)]
pub struct TreeProps<T: Clone + PartialEq + 'static> {
    /// Hierarchical data to render.
    pub data: ReadSignal<Vec<TreeNode<T>>>,
    /// State controller returned by [`use_tree`].
    pub tree: TreeController,
    /// The ID of the tree root.
    #[props(default)]
    pub id: Option<String>,
    /// Whether the tree and all of its items are disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,
    /// Additional attributes for the tree root.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// Render each node's icon and content inside the accessible tree item row.
    pub render_node: Callback<TreeNodeRenderProps<T>, TreeNodeContent>,
}

/// A styled accessible hierarchical tree.
#[component]
pub fn TreeView(props: TreeViewProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_tree_view,
        "data-slot": "tree-view",
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        tree_view::TreeView {
            id: props.id,
            disabled: props.disabled,
            selection_mode: props.selection_mode,
            selected: props.selected,
            default_selected: props.default_selected,
            roving_loop: props.roving_loop,
            on_selected_change: props.on_selected_change,
            attributes,
            {props.children}
        }
    }
}

/// Render hierarchical data with a [`TreeController`].
#[component]
pub fn Tree<T: Clone + PartialEq + 'static>(props: TreeProps<T>) -> Element {
    let tree = props.tree;
    let nodes = props.data.read().clone();
    let setsize = nodes.len();

    rsx! {
        TreeView {
            id: props.id,
            disabled: props.disabled,
            selection_mode: tree.selection_mode_signal(),
            selected: tree.selected_signal(),
            roving_loop: tree.roving_loop_signal(),
            on_selected_change: move |selected| tree.set_selected(selected),
            attributes: props.attributes,
            for (index, node) in nodes.into_iter().enumerate() {
                TreeNodeView {
                    key: "{node.id}",
                    node,
                    tree,
                    render_node: props.render_node,
                    level: 1,
                    posinset: index + 1,
                    setsize,
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct TreeNodeViewProps<T: Clone + PartialEq + 'static> {
    node: TreeNode<T>,
    tree: TreeController,
    render_node: Callback<TreeNodeRenderProps<T>, TreeNodeContent>,
    level: usize,
    posinset: usize,
    setsize: usize,
}

#[component]
fn TreeNodeView<T: Clone + PartialEq + 'static>(props: TreeNodeViewProps<T>) -> Element {
    let tree = props.tree;
    let node = props.node;
    let node_id = node.id.clone();
    let expanded = use_memo(move || Some(tree.is_expanded(&node_id)));
    let node_id = node.id.clone();
    let on_expanded_change = Callback::new(move |expanded: bool| {
        tree.set_expanded(node_id.clone(), expanded);
    });
    let has_children = !node.children.is_empty();
    let child_setsize = node.children.len();
    let rendered_node = props.render_node.call(TreeNodeRenderProps {
        node: node.clone(),
        level: props.level,
        expanded: tree.is_expanded(&node.id),
        has_children,
        selected: tree.is_selected(&node.id),
        tree,
    });

    rsx! {
        TreeViewItem {
            id: node.id.clone(),
            expanded,
            on_expanded_change,
            posinset: props.posinset,
            setsize: props.setsize,
            TreeViewItemContent {
                icon: rendered_node.icon,
                {rendered_node.content}
            }
            if has_children {
                TreeViewItemGroup {
                    for (index, child) in node.children.into_iter().enumerate() {
                        TreeNodeView {
                            key: "{child.id}",
                            node: child,
                            tree,
                            render_node: props.render_node,
                            level: props.level + 1,
                            posinset: index + 1,
                            setsize: child_setsize,
                        }
                    }
                }
            }
        }
    }
}

/// A styled tree item.
#[component]
pub fn TreeViewItem(props: TreeViewItemProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_tree_view_item,
        "data-slot": "tree-item",
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        tree_view::TreeViewItem {
            id: props.id,
            disabled: props.disabled,
            default_expanded: props.default_expanded,
            expanded: props.expanded,
            on_expanded_change: props.on_expanded_change,
            posinset: props.posinset,
            setsize: props.setsize,
            on_select: props.on_select,
            attributes,
            {props.children}
        }
    }
}

/// The styled focusable row of a tree item.
#[component]
pub fn TreeViewItemContent(props: TreeViewItemContentProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_tree_view_item_content,
        "data-slot": "tree-item-content",
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        tree_view::TreeViewItemContent {
            attributes,
            span {
                class: Styles::dx_tree_view_indicator,
                "aria-hidden": "true",
                ChevronRight { size: "1rem", stroke: "currentColor" }
            }
            if let Some(icon) = props.icon {
                span {
                    class: Styles::dx_tree_view_icon,
                    "aria-hidden": "true",
                    {icon}
                }
            }
            span {
                class: Styles::dx_tree_view_label,
                {props.children}
            }
        }
    }
}

/// A styled nested item group.
#[component]
pub fn TreeViewItemGroup(props: TreeViewItemGroupProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_tree_view_group,
        "data-slot": "tree-item-group",
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        tree_view::TreeViewItemGroup {
            attributes,
            {props.children}
        }
    }
}
