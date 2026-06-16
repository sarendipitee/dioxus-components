#[path = "../demo_support.rs"]
mod demo_support;

use dioxus::prelude::*;
use dioxus_components::data_table::{
    DataTable, DataTableExpandedRowContext, DataTablePageInfo, DataTableStateMode,
};

use demo_support::{
    format_currency, order_columns, order_row_id, order_rows, table_state, OrderRow,
    CLIENT_PAGE_SIZE,
};

#[css_module("./style.css")]
struct Styles;

/// Renders a DataTable with expandable row detail content.
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
            empty_message: "No expandable orders match",
            expanded_row: Callback::new(|ctx: DataTableExpandedRowContext<OrderRow>| {
                let row = ctx.row.item;
                let priority = if row.priority {
                    "Priority handling"
                } else {
                    "Standard handling"
                };
                rsx! {
                    div { class: Styles::dx_data_table_expansion_detail,
                        div { class: Styles::dx_data_table_expansion_summary,
                            div { class: Styles::dx_data_table_expansion_summary_main,
                                span { class: Styles::dx_data_table_expansion_eyebrow, "Fulfillment detail" }
                                strong { class: Styles::dx_data_table_expansion_title, "{row.order} for {row.customer}" }
                            }
                            span { class: Styles::dx_data_table_expansion_status, "{row.status}" }
                        }
                        dl { class: Styles::dx_data_table_expansion_facts,
                            div {
                                dt { "Order value" }
                                dd { "{format_currency(row.total)}" }
                            }
                            div {
                                dt { "Last update" }
                                dd { "{row.updated}" }
                            }
                            div {
                                dt { "Handling" }
                                dd { "{priority}" }
                            }
                        }
                    }
                }
            }),
        }
    }
}
