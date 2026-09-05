//! Accessible hierarchical tree view primitives.

use std::collections::HashMap;

use crate::dioxus_elements::Key;
use crate::focus::{use_focus_control_disabled, use_focus_entry_disabled, FocusState};
use crate::{use_controlled, use_unique_id};
use dioxus::prelude::*;

/// Selection behavior for a [`TreeView`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum TreeViewSelectionMode {
    /// Items are not selectable.
    #[default]
    None,
    /// Selecting an item clears the previous selection.
    Single,
    /// Selecting an item toggles it without clearing other selected items.
    Multiple,
}

#[derive(Clone, Copy, PartialEq)]
struct TreeViewContext {
    next_id: Signal<usize>,
    indices: Signal<HashMap<String, usize>>,
    focus: FocusState,
    disabled: ReadSignal<bool>,
    selection_mode: ReadSignal<TreeViewSelectionMode>,
    selected: Memo<Vec<String>>,
    set_selected: Callback<Vec<String>>,
}

impl TreeViewContext {
    fn register_item(&mut self, id: String) -> usize {
        if let Some(index) = self.indices.peek().get(&id).copied() {
            return index;
        }

        let mut next_id = self.next_id.write();
        let index = *next_id;
        *next_id += 1;
        self.indices.write().insert(id, index);
        index
    }

    fn is_disabled(&self) -> bool {
        (self.disabled)()
    }

    fn is_selected(&self, id: &str) -> bool {
        (self.selected)().iter().any(|selected| selected == id)
    }

    fn select(&self, id: String) -> Option<bool> {
        let mode = (self.selection_mode)();
        let mut selected = (self.selected)();

        match mode {
            TreeViewSelectionMode::None => return None,
            TreeViewSelectionMode::Single => {
                selected.clear();
                selected.push(id.clone());
            }
            TreeViewSelectionMode::Multiple => {
                if let Some(index) = selected.iter().position(|selected| selected == &id) {
                    selected.remove(index);
                } else {
                    selected.push(id.clone());
                }
            }
        }

        let is_selected = selected.iter().any(|selected| selected == &id);
        self.set_selected.call(selected);
        Some(is_selected)
    }
}

#[derive(Clone, Copy, PartialEq)]
struct TreeViewItemContext {
    index: usize,
    id: ReadSignal<String>,
    parent_index: Option<usize>,
    level: usize,
    expanded: Memo<bool>,
    set_expanded: Callback<bool>,
    disabled: ReadSignal<bool>,
    has_children: Signal<bool>,
    group_id: Signal<String>,
    posinset: Option<usize>,
    setsize: Option<usize>,
    on_select: Callback<bool>,
}

impl TreeViewItemContext {
    fn is_disabled(&self) -> bool {
        (self.disabled)()
    }

    fn has_children(&self) -> bool {
        (self.has_children)()
    }

    fn is_expanded(&self) -> bool {
        (self.expanded)()
    }
}

/// Props for the [`TreeView`] component.
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

/// An accessible hierarchical tree with roving keyboard focus.
#[component]
pub fn TreeView(props: TreeViewProps) -> Element {
    let (selected, set_selected) = use_controlled(
        props.selected,
        props.default_selected,
        props.on_selected_change,
    );
    let focus = crate::focus::use_focus_provider(props.roving_loop);
    let mut ctx = use_context_provider(|| TreeViewContext {
        next_id: Signal::new(0),
        indices: Signal::new(HashMap::new()),
        focus,
        disabled: props.disabled,
        selection_mode: props.selection_mode,
        selected,
        set_selected,
    });

    rsx! {
        div {
            id: props.id,
            role: "tree",
            aria_disabled: (ctx.disabled)(),
            aria_multiselectable: if (ctx.selection_mode)() == TreeViewSelectionMode::Multiple {
                Some("true")
            } else {
                None
            },
            "data-disabled": (ctx.disabled)(),
            onfocusout: move |_| ctx.focus.set_focus(None),
            ..props.attributes,
            {props.children}
        }
    }
}

/// Props for a tree item. Nest child items inside [`TreeViewItemGroup`].
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

/// An item in a [`TreeView`].
#[component]
pub fn TreeViewItem(props: TreeViewItemProps) -> Element {
    let mut tree_ctx: TreeViewContext = use_context();
    let parent = try_use_context::<TreeViewItemContext>();
    let item_id = props.id;
    let index = use_hook(|| tree_ctx.register_item(item_id.cloned()));
    let parent_index = parent.map(|parent| parent.index);
    let level = parent.map_or(1, |parent| parent.level + 1);
    let (expanded, set_expanded) = use_controlled(
        props.expanded,
        props.default_expanded.cloned(),
        props.on_expanded_change,
    );
    let has_children = use_signal(|| false);
    let group_id = use_unique_id();
    let item = use_context_provider(|| TreeViewItemContext {
        index,
        id: item_id,
        parent_index,
        level,
        expanded,
        set_expanded,
        disabled: props.disabled,
        has_children,
        group_id,
        posinset: props.posinset,
        setsize: props.setsize,
        on_select: props.on_select,
    });

    let disabled = move || tree_ctx.is_disabled() || item.is_disabled();
    let index_signal = use_signal(|| index);
    use_focus_entry_disabled(tree_ctx.focus, index_signal, disabled);

    rsx! {
        div {
            role: "none",
            "data-state": if item.is_expanded() { "open" } else { "closed" },
            "data-level": item.level,
            "data-has-children": item.has_children(),
            "data-disabled": disabled(),
            ..props.attributes,
            {props.children}
        }
    }
}

/// Props for the focusable row of a [`TreeViewItem`].
#[derive(Props, Clone, PartialEq)]
pub struct TreeViewItemContentProps {
    /// Additional attributes for the focusable treeitem row.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The item's label and optional custom content.
    pub children: Element,
}

/// The focusable, selectable row for a [`TreeViewItem`].
#[component]
pub fn TreeViewItemContent(props: TreeViewItemContentProps) -> Element {
    let mut tree_ctx: TreeViewContext = use_context();
    let item: TreeViewItemContext = use_context();
    let disabled = move || tree_ctx.is_disabled() || item.is_disabled();
    let index = use_signal(|| item.index);
    let onmounted = use_focus_control_disabled(tree_ctx.focus, index, disabled);
    let selected = use_memo(move || tree_ctx.is_selected(&(item.id)()));
    let tab_index = use_memo(move || {
        let _ = tree_ctx.focus.items_revision();
        if disabled() {
            "-1"
        } else if tree_ctx.focus.recent_focus_or_default() == item.index {
            "0"
        } else {
            "-1"
        }
    });

    rsx! {
        div {
            id: item.id,
            role: "treeitem",
            tabindex: tab_index,
            aria_level: item.level,
            aria_posinset: item.posinset,
            aria_setsize: item.setsize,
            aria_expanded: if item.has_children() {
                Some(if item.is_expanded() { "true" } else { "false" })
            } else {
                None
            },
            aria_selected: if (tree_ctx.selection_mode)() == TreeViewSelectionMode::None {
                None
            } else if selected() {
                Some("true")
            } else {
                Some("false")
            },
            aria_disabled: if disabled() { Some("true") } else { None },
            aria_controls: if item.has_children() {
                Some((item.group_id)())
            } else {
                None
            },
            "data-selected": selected(),
            "data-expanded": item.is_expanded(),
            "data-disabled": disabled(),
            onmounted,
            onfocus: move |_| tree_ctx.focus.set_focus(Some(item.index)),
            onclick: move |_| {
                if disabled() {
                    return;
                }
                if let Some(is_selected) = tree_ctx.select((item.id)()) {
                    item.on_select.call(is_selected);
                }
                if item.has_children() {
                    item.set_expanded.call(!item.is_expanded());
                }
            },
            onkeydown: move |event: Event<KeyboardData>| {
                if disabled() {
                    return;
                }

                let mut prevent_default = true;
                match event.key() {
                    Key::ArrowDown => tree_ctx.focus.focus_next(),
                    Key::ArrowUp => tree_ctx.focus.focus_prev(),
                    Key::Home => tree_ctx.focus.focus_first(),
                    Key::End => tree_ctx.focus.focus_last(),
                    Key::ArrowRight if item.has_children() && !item.is_expanded() => {
                        item.set_expanded.call(true);
                    }
                    Key::ArrowRight => tree_ctx.focus.focus_next(),
                    Key::ArrowLeft if item.has_children() && item.is_expanded() => {
                        item.set_expanded.call(false);
                    }
                    Key::ArrowLeft => {
                        if let Some(parent_index) = item.parent_index {
                            tree_ctx.focus.set_focus(Some(parent_index));
                        }
                    }
                    Key::Enter => {
                        if let Some(is_selected) = tree_ctx.select((item.id)()) {
                            item.on_select.call(is_selected);
                        }
                    }
                    Key::Character(ref key) if key == " " => {
                        if let Some(is_selected) = tree_ctx.select((item.id)()) {
                            item.on_select.call(is_selected);
                        }
                    }
                    _ => prevent_default = false,
                }

                if prevent_default {
                    event.prevent_default();
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}

/// Props for the nested group of a [`TreeViewItem`].
#[derive(Props, Clone, PartialEq)]
pub struct TreeViewItemGroupProps {
    /// Additional attributes for the group element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// Nested tree items.
    pub children: Element,
}

/// A nested group rendered when its parent item is expanded.
#[component]
pub fn TreeViewItemGroup(props: TreeViewItemGroupProps) -> Element {
    let item: TreeViewItemContext = use_context();
    let mut has_children = item.has_children;
    use_hook(move || has_children.set(true));

    if !item.is_expanded() {
        return rsx! {};
    }

    rsx! {
        div {
            id: item.group_id,
            role: "group",
            aria_labelledby: item.id,
            ..props.attributes,
            {props.children}
        }
    }
}
