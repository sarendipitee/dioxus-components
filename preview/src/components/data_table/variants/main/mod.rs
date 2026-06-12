#[path = "../demo_support.rs"]
mod demo_support;

use dioxus::prelude::*;
use dioxus_components::data_table::{DataTable, DataTablePageInfo, DataTableStateMode};

use demo_support::{order_columns, order_row_id, order_rows, table_state, CLIENT_PAGE_SIZE};

/// Renders the default uncontrolled DataTable example.
#[component]
pub fn Demo() -> Element {
    let rows = order_rows();

    rsx! {
        DataTable {
            page_info: DataTablePageInfo::known_total(rows.len() as u64),
            items: rows,
            columns: order_columns(),
            state_mode: DataTableStateMode::Uncontrolled {
                default_state: Some(table_state(CLIENT_PAGE_SIZE)),
            },
            row_id: Callback::new(order_row_id),
            empty_message: "No orders match this view".to_string(),
            empty_hint: Some("Try clearing search, filters, or sorting.".to_string()),
        }
    }
}
