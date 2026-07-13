//! Shared state for the combobox component.

use super::hook::{ComboboxDropdownEventSource, ComboboxIndexTarget, ComboboxStore};
use crate::selectable::{OptionState, RcPartialEqValue, SelectableContext};
use dioxus::prelude::*;

/// The default case-insensitive substring filter.
pub fn default_combobox_filter(query: &str, text: &str) -> bool {
    let query = query.trim().to_lowercase();
    query.is_empty() || text.to_lowercase().contains(&query)
}

#[derive(Clone, Copy, PartialEq)]
pub(super) struct ComboboxContext {
    pub selectable: SelectableContext,
    pub store: ComboboxStore,
    pub query: Memo<String>,
    pub set_query: Callback<String>,
    pub filter: Callback<(String, String), bool>,
    pub portal_open: Signal<bool>,
}

impl ComboboxContext {
    fn matches_query_text(&self, text: String) -> bool {
        self.filter.call((self.query.cloned(), text))
    }

    pub fn set_open(&mut self, open: bool) {
        if open {
            self.selectable.focus_state.set_focus(None);
            self.store
                .open_dropdown(ComboboxDropdownEventSource::Unknown);
        } else {
            self.store
                .close_dropdown(ComboboxDropdownEventSource::Unknown);
        }
    }

    fn predicate_for(&self, query: String) -> impl Fn(&OptionState) -> bool {
        let filter = self.filter;
        move |option| filter.call((query.clone(), option.text_value.clone()))
    }

    fn predicate(&self) -> impl Fn(&OptionState) -> bool {
        self.predicate_for(self.query.cloned())
    }

    pub fn is_visible_text(&self, tab_index: usize, text: String) -> bool {
        let predicate = self.predicate();
        self.selectable
            .options
            .read()
            .iter()
            .find(|option| option.tab_index == tab_index)
            .map_or_else(|| self.matches_query_text(text), predicate)
    }

    pub fn has_visible_options(&self) -> bool {
        self.selectable.options.read().iter().any(self.predicate())
    }

    pub fn open_with_empty_query_and_focus_first(&mut self) {
        let query = String::new();
        self.set_query.call(query.clone());
        let virtual_initial_focus = self
            .store
            .virtual_initial_selection_index(ComboboxIndexTarget::First);
        let initial_focus = virtual_initial_focus.or_else(|| {
            (virtual_initial_focus.is_none())
                .then(|| {
                    self.selectable
                        .first_matching_enabled_index(self.predicate_for(query))
                })
                .flatten()
        });
        if let Some(index) = initial_focus {
            if virtual_initial_focus.is_some() {
                self.store.request_virtual_initial_selection(index);
                self.selectable.initial_focus.set(None);
            } else {
                self.store.request_initial_selection_at(index);
                self.store.resolve_pending_initial_selection_at(index);
                self.selectable.initial_focus.set(Some(index));
            }
        } else {
            self.store
                .request_initial_selection(ComboboxIndexTarget::First);
            self.selectable.initial_focus.set(None);
        }
        self.selectable.focus_state.set_focus(initial_focus);
        self.set_open(true);
    }

    pub fn open_with_empty_query_and_focus_last(&mut self) {
        let query = String::new();
        self.set_query.call(query.clone());
        let virtual_initial_focus = self
            .store
            .virtual_initial_selection_index(ComboboxIndexTarget::Last);
        let initial_focus = virtual_initial_focus.or_else(|| {
            (virtual_initial_focus.is_none())
                .then(|| {
                    self.selectable
                        .last_matching_enabled_index(self.predicate_for(query))
                })
                .flatten()
        });
        if let Some(index) = initial_focus {
            if virtual_initial_focus.is_some() {
                self.store.request_virtual_initial_selection(index);
                self.selectable.initial_focus.set(None);
            } else {
                self.store.request_initial_selection_at(index);
                self.store.resolve_pending_initial_selection_at(index);
                self.selectable.initial_focus.set(Some(index));
            }
        } else {
            self.store
                .request_initial_selection(ComboboxIndexTarget::Last);
            self.selectable.initial_focus.set(None);
        }
        self.selectable.focus_state.set_focus(initial_focus);
        self.set_open(true);
    }

    pub fn focused_option_id(&self) -> Option<String> {
        self.store
            .highlighted_option_index()
            .and_then(|index| {
                self.selectable
                    .options
                    .read()
                    .iter()
                    .find(|option| option.tab_index == index && !option.disabled)
                    .map(|option| option.id.clone())
            })
            .or_else(|| self.selectable.focused_option_id())
    }
    pub fn focus_next_visible(&mut self) {
        self.selectable.focus_next_where(self.predicate());
        if let Some(index) = self.selectable.focus_state.current_focus() {
            self.store.select_option(index);
        }
    }

    pub fn focus_prev_visible(&mut self) {
        self.selectable.focus_prev_where(self.predicate());
        if let Some(index) = self.selectable.focus_state.current_focus() {
            self.store.select_option(index);
        }
    }

    pub fn focus_first_visible(&mut self) {
        self.selectable.focus_first_where(self.predicate());
        if let Some(index) = self.selectable.focus_state.current_focus() {
            self.store.select_option(index);
        }
    }

    pub fn focus_last_visible(&mut self) {
        self.selectable.focus_last_where(self.predicate());
        if let Some(index) = self.selectable.focus_state.current_focus() {
            self.store.select_option(index);
        }
    }

    pub fn select_focused(&mut self) {
        if let Some(index) = self.selectable.focus_state.current_focus() {
            self.submit_index(index, ComboboxDropdownEventSource::Keyboard);
        }
    }

    pub fn submit_index(&mut self, index: usize, source: ComboboxDropdownEventSource) {
        if self.store.select_option(index).is_none() {
            return;
        }
        self.store.submit_selected_option();
        let Some(value) = self
            .selectable
            .options
            .read()
            .iter()
            .find(|option| option.tab_index == index && !option.disabled)
            .map(|option| option.value.clone())
        else {
            return;
        };
        self.selectable.select_value(value);
        if !self.selectable.selection_mode.is_multiple() {
            self.store.close_dropdown(source);
        }
    }
}

/// Root-side registration payload for a materialized virtual option.
#[derive(Clone, PartialEq)]
pub(super) struct ComboboxPortalOptionRegistration {
    pub option: OptionState,
    pub visible: bool,
    pub selected: bool,
}

/// Portal-local read model for combobox list descendants.
#[derive(Clone, PartialEq)]
pub(super) struct ComboboxPortalContext {
    /// Focus and highlight an option through root-owned combobox state.
    pub hover_option: Callback<usize>,
    /// Whether the root combobox is disabled.
    pub root_disabled: bool,
    /// Selected values snapshotted before entering the portal.
    pub selected_values: Vec<RcPartialEqValue>,
    /// Focused option index snapshotted before entering the portal.
    pub focused_index: Option<usize>,
    /// Highlighted option index snapshotted before entering the portal.
    pub highlighted_index: Option<usize>,
    /// Registered root-tree option metadata snapshotted before entering the portal.
    pub options: Vec<OptionState>,
    /// Visible option indices snapshotted before entering the portal.
    pub visible_indices: Option<Vec<usize>>,
    /// Whether any option is visible in the root snapshot.
    pub has_visible_options: bool,
    /// Whether portaled options should register themselves with root state.
    pub register_options: bool,
    /// Register a materialized virtual option through root-owned state.
    pub register_option: Callback<ComboboxPortalOptionRegistration>,
    /// Remove a materialized virtual option through root-owned state.
    pub unregister_option: Callback<ComboboxPortalOptionRegistration>,
    /// Submit an option through the root combobox state.
    pub submit_index_from_mouse: Callback<usize>,
}

impl ComboboxPortalContext {
    /// Returns registered metadata for an option index.
    pub fn option_state(&self, index: usize) -> Option<&OptionState> {
        self.options.iter().find(|option| option.tab_index == index)
    }

    /// Returns whether an option index is visible in the root snapshot.
    pub fn is_visible(&self, index: usize) -> bool {
        self.visible_indices
            .as_ref()
            .map(|indices| indices.contains(&index))
            .unwrap_or(true)
    }

    /// Returns whether the given value is selected in the root state snapshot.
    pub fn is_selected(&self, value: &RcPartialEqValue) -> bool {
        self.selected_values
            .iter()
            .any(|selected| selected == value)
    }

}
