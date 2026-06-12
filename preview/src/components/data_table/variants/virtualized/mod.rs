#[path = "../demo_support.rs"]
mod demo_support;

use dioxus::prelude::*;
use dioxus_components::data_table::{
    DataTable, DataTablePageInfo, DataTableStateMode, DataTableVirtualization,
};

use demo_support::{order_columns, order_row_id, order_rows_with_count, table_state};

const VIRTUAL_ROW_COUNT: usize = 5_000;

/// Renders a virtualized DataTable that scrolls a large row set through a bounded viewport.
#[component]
pub fn Demo() -> Element {
    let rows = order_rows_with_count(VIRTUAL_ROW_COUNT);

    rsx! {
        DataTable {
            page_info: DataTablePageInfo::known_total(rows.len() as u64),
            items: rows,
            columns: order_columns(),
            state_mode: DataTableStateMode::Uncontrolled {
                default_state: Some(table_state(VIRTUAL_ROW_COUNT as u64)),
            },
            row_id: Callback::new(order_row_id),
            virtualization: Some(DataTableVirtualization {
                estimated_row_height: 49,
                overscan: 10,
                max_height: Some("32rem".to_string()),
            }),
            empty_message: "No orders match this view".to_string(),
            empty_hint: Some("Try clearing search, filters, or sorting.".to_string()),
        }
    }
}
