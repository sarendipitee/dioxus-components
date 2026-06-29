#[path = "../demo_support.rs"]
mod demo_support;

use dioxus::prelude::*;
use dioxus_components::data_table::{DataTable, DataTableDensity, DataTablePageInfo, DataTableStateMode};

use demo_support::{order_columns, order_row_id, order_rows, table_state, CLIENT_PAGE_SIZE};

/// Renders DataTable with different density presets and direct CSS variable overrides.
#[component]
pub fn Demo() -> Element {
    let rows = order_rows();

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 1.5rem;",
            DataTable {
                page_info: DataTablePageInfo::known_total(rows.len() as u64),
                items: rows.clone(),
                columns: order_columns(),
                density: DataTableDensity::Compact,
                state_mode: DataTableStateMode::Uncontrolled {
                    default_state: Some(table_state(CLIENT_PAGE_SIZE)),
                },
                row_id: Callback::new(order_row_id),
                empty_message: "No compact-orders",
            }
            DataTable {
                page_info: DataTablePageInfo::known_total(rows.len() as u64),
                items: rows.clone(),
                columns: order_columns(),
                density: DataTableDensity::Default,
                state_mode: DataTableStateMode::Uncontrolled {
                    default_state: Some(table_state(CLIENT_PAGE_SIZE)),
                },
                row_id: Callback::new(order_row_id),
                empty_message: "No default orders",
            }
            DataTable {
                page_info: DataTablePageInfo::known_total(rows.len() as u64),
                items: rows,
                columns: order_columns(),
                density: DataTableDensity::Comfortable,
                state_mode: DataTableStateMode::Uncontrolled {
                    default_state: Some(table_state(CLIENT_PAGE_SIZE)),
                },
                row_id: Callback::new(order_row_id),
                style: "--dx-data-table-cell-padding-x: var(--space); --dx-data-table-cell-padding-y: 0.9rem; --dx-data-table-selection-cell-padding-x: var(--space); --dx-data-table-selection-cell-padding-y: 0.9rem;",
                empty_message: "No custom-density orders",
                toolbar_left: rsx! {
                    span { style: "color: var(--surface-muted-fg); font-size: var(--text-sm);", "Custom inline/block CSS overrides" }
                },
            }
        }
    }
}
