//! Data-oriented tree state and node models.

use std::collections::HashSet;

use dioxus::prelude::*;

use crate::tree_view::TreeViewSelectionMode;

/// A data node rendered by a data-oriented tree.
///
/// `id` is the stable identity used for keys, selection, expansion, and ARIA
/// relationships. `data` is application-owned content passed to the node
/// renderer. The tree does not make assumptions about how it is displayed.
#[derive(Clone, PartialEq)]
pub struct TreeNode<T> {
    /// Stable node identity.
    pub id: String,
    /// Application-owned node data.
    pub data: T,
    /// Nested child nodes.
    pub children: Vec<Self>,
}

impl<T> TreeNode<T> {
    /// Create a node without children.
    pub fn new(id: impl Into<String>, data: T) -> Self {
        Self {
            id: id.into(),
            data,
            children: Vec::new(),
        }
    }

    /// Create a leaf node.
    pub fn leaf(id: impl Into<String>, data: T) -> Self {
        Self::new(id, data)
    }

    /// Create a node with children.
    pub fn branch(
        id: impl Into<String>,
        data: T,
        children: impl IntoIterator<Item = Self>,
    ) -> Self {
        Self {
            id: id.into(),
            data,
            children: children.into_iter().collect(),
        }
    }

    /// Add child nodes to this node.
    pub fn with_children(mut self, children: impl IntoIterator<Item = Self>) -> Self {
        self.children = children.into_iter().collect();
        self
    }
}

/// Context passed to a data-oriented tree node renderer.
#[derive(Clone, PartialEq)]
pub struct TreeNodeRenderProps<T: Clone + PartialEq + 'static> {
    /// The current node.
    pub node: TreeNode<T>,
    /// One-based depth of the node in the tree.
    pub level: usize,
    /// Whether the node is currently expanded.
    pub expanded: bool,
    /// Whether the node has child nodes.
    pub has_children: bool,
    /// Whether the node is currently selected.
    pub selected: bool,
    /// Controller for reading and changing tree state.
    pub tree: TreeController,
}

/// Options used to initialize [`TreeController`] with [`use_tree`].
#[derive(Clone, PartialEq)]
pub struct UseTreeOptions {
    /// Selection behavior for the tree.
    pub selection_mode: TreeViewSelectionMode,
    /// Nodes that should be expanded on first render.
    pub initial_expanded: Vec<String>,
    /// Nodes that should be selected on first render.
    pub initial_selected: Vec<String>,
    /// Whether keyboard focus wraps at the first and last visible nodes.
    pub roving_loop: bool,
}

impl Default for UseTreeOptions {
    fn default() -> Self {
        Self {
            selection_mode: TreeViewSelectionMode::None,
            initial_expanded: Vec::new(),
            initial_selected: Vec::new(),
            roving_loop: true,
        }
    }
}

/// Cloneable state controller returned by [`use_tree`].
#[derive(Clone, Copy, PartialEq)]
pub struct TreeController {
    expanded: Signal<HashSet<String>>,
    set_expanded_state: Callback<(String, bool)>,
    set_expanded_values: Callback<HashSet<String>>,
    selected: Signal<Option<Vec<String>>>,
    set_selected_state: Callback<Vec<String>>,
    selection_mode: Signal<TreeViewSelectionMode>,
    roving_loop: Signal<bool>,
}

impl TreeController {
    /// Returns the signal containing the expanded node IDs.
    pub fn expanded_signal(&self) -> Signal<HashSet<String>> {
        self.expanded
    }

    /// Returns the signal containing the selected node IDs.
    pub fn selected_signal(&self) -> Signal<Option<Vec<String>>> {
        self.selected
    }

    /// Returns the signal containing the tree selection mode.
    pub fn selection_mode_signal(&self) -> Signal<TreeViewSelectionMode> {
        self.selection_mode
    }

    /// Returns the signal controlling roving keyboard focus.
    pub fn roving_loop_signal(&self) -> Signal<bool> {
        self.roving_loop
    }

    /// Returns whether a node is expanded.
    pub fn is_expanded(&self, id: &str) -> bool {
        self.expanded.read().contains(id)
    }

    /// Sets a node's expanded state.
    pub fn set_expanded(&self, id: impl Into<String>, expanded: bool) {
        self.set_expanded_state.call((id.into(), expanded));
    }

    /// Toggles a node and returns its next expanded state.
    pub fn toggle(&self, id: impl Into<String>) -> bool {
        let id = id.into();
        let expanded = !self.is_expanded(&id);
        self.set_expanded(id, expanded);
        expanded
    }

    /// Expands every branch in the supplied tree data.
    pub fn expand_all<T>(&self, nodes: &[TreeNode<T>]) {
        let mut expanded = HashSet::new();
        collect_branch_ids(nodes, &mut expanded);
        self.set_expanded_values.call(expanded);
    }

    /// Collapses every node.
    pub fn collapse_all(&self) {
        self.set_expanded_values.call(HashSet::new());
    }

    /// Returns whether a node is selected.
    pub fn is_selected(&self, id: &str) -> bool {
        self.selected
            .read()
            .as_ref()
            .is_some_and(|selected| selected.iter().any(|selected| selected == id))
    }

    /// Sets the selected node IDs.
    pub fn set_selected(&self, selected: Vec<String>) {
        self.set_selected_state.call(selected);
    }

    /// Clears the current selection.
    pub fn clear_selection(&self) {
        self.set_selected(Vec::new());
    }

    /// Selects a node according to the configured selection mode.
    pub fn select(&self, id: impl Into<String>) {
        let id = id.into();
        let mode = (self.selection_mode)();
        if mode == TreeViewSelectionMode::None {
            return;
        }

        let mut selected = self.selected.read().clone().unwrap_or_default();
        match mode {
            TreeViewSelectionMode::None => return,
            TreeViewSelectionMode::Single => {
                selected.clear();
                selected.push(id);
            }
            TreeViewSelectionMode::Multiple => {
                if let Some(index) = selected.iter().position(|selected| selected == &id) {
                    selected.remove(index);
                } else {
                    selected.push(id);
                }
            }
        }
        self.set_selected(selected);
    }
}

/// Create a data-oriented tree state controller.
pub fn use_tree(options: UseTreeOptions) -> TreeController {
    use_hook(|| {
        let expanded: Signal<HashSet<String>> =
            Signal::new(options.initial_expanded.into_iter().collect());
        let selected: Signal<Option<Vec<String>>> = Signal::new(Some(options.initial_selected));
        let set_expanded_state = Callback::new({
            move |(id, is_expanded): (String, bool)| {
                let mut state = expanded;
                let mut values = state.write();
                if is_expanded {
                    values.insert(id);
                } else {
                    values.remove(&id);
                }
            }
        });
        let set_expanded_values = Callback::new({
            move |values: HashSet<String>| {
                let mut state = expanded;
                state.set(values);
            }
        });
        let set_selected_state = Callback::new({
            move |values: Vec<String>| {
                let mut state = selected;
                state.set(Some(values));
            }
        });

        TreeController {
            expanded,
            set_expanded_state,
            set_expanded_values,
            selected,
            set_selected_state,
            selection_mode: Signal::new(options.selection_mode),
            roving_loop: Signal::new(options.roving_loop),
        }
    })
}

fn collect_branch_ids<T>(nodes: &[TreeNode<T>], expanded: &mut HashSet<String>) {
    for node in nodes {
        if !node.children.is_empty() {
            expanded.insert(node.id.clone());
            collect_branch_ids(&node.children, expanded);
        }
    }
}
