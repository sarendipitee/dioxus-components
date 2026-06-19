#[path = "../demo_support.rs"]
mod demo_support;

use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonSize, ButtonVariant};
use dioxus_components::data_table::{
    DataTable, DataTablePageInfo, DataTableStateChange, DataTableStateMode,
};

use demo_support::{order_columns, order_row_id, order_rows, table_state, CLIENT_PAGE_SIZE};

/// Renders a DataTable whose complete state is owned by the parent.
#[component]
pub fn Demo() -> Element {
    let mut state = use_signal(|| table_state(CLIENT_PAGE_SIZE));
    let rows = order_rows();

    rsx! {
        Button {
            variant: ButtonVariant::Outline,
            onclick: move |_| state.set(table_state(CLIENT_PAGE_SIZE)),
            "Reset state"
        }
        DataTable {
            page_info: DataTablePageInfo::known_total(rows.len() as u64),
            items: rows,
            columns: order_columns(),
            state_mode: DataTableStateMode::Controlled { state: state() },
            on_state_change: move |change: DataTableStateChange| state.set(change.next_state),
            row_id: Callback::new(order_row_id),
            empty_message: "No controlled rows match",
            toolbar_right: rsx! {
            },
        }
    }
}
