#[path = "../demo_support.rs"]
mod demo_support;

use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonSize, ButtonVariant};
use dioxus_components::data_table::{
    DataTable, DataTablePageInfo, DataTableRowSelectionState,
    DataTableState, DataTableStateChange, DataTableStateMode,
};

use demo_support::{order_columns, order_row_id, order_rows, table_state, CLIENT_PAGE_SIZE};

/// Renders a DataTable with built-in row selection enabled.
#[component]
pub fn Demo() -> Element {
    let mut state = use_signal(|| {
        let mut state = table_state(CLIENT_PAGE_SIZE);
        state.row_selection = DataTableRowSelectionState::Explicit { rows: Vec::new() };
        state
    });
    let rows = order_rows();

    let selection_summary = move || match &state().row_selection {
        DataTableRowSelectionState::Explicit { rows } if rows.is_empty() => "No rows selected".to_string(),
        DataTableRowSelectionState::Explicit { rows } => format!("{count} rows selected", count = rows.len()),
        DataTableRowSelectionState::AllMatching { except, .. } if except.is_empty() => {
            "All matching rows selected".to_string()
        }
        DataTableRowSelectionState::AllMatching { except, .. } => {
            format!("All matching rows selected, except {count} rows", count = except.len())
        }
    };

    let select_first_five = {
        let row_ids: Vec<String> = rows.iter().take(5).map(|row| row.order.clone()).collect();
        move |_| {
            state.set(DataTableState {
                row_selection: DataTableRowSelectionState::Explicit { rows: row_ids.clone() },
                ..state()
            });
        }
    };

    let clear_selection = move |_| {
        state.set(DataTableState {
            row_selection: DataTableRowSelectionState::Explicit { rows: Vec::new() },
            ..state()
        });
    };

    rsx! {
        DataTable {
            page_info: DataTablePageInfo::known_total(rows.len() as u64),
            items: rows,
            columns: order_columns(),
            state_mode: DataTableStateMode::Controlled { state: state() },
            on_state_change: move |change: DataTableStateChange| state.set(change.next_state),
            show_selection: true,
            row_id: Callback::new(order_row_id),
            empty_message: "No selectable orders match this view",
            toolbar_right: rsx! {
                div { class: "dx-demo-toolbar-right", display: "flex", gap: "0.5rem", align_items: "center",
                    span { class: "text-sm text-muted", "{selection_summary()}" }
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        r#type: "button",
                        onclick: select_first_five,
                        "Select first 5"
                    }
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        r#type: "button",
                        onclick: clear_selection,
                        "Clear selection"
                    }
                }
            },
        }
    }
}
