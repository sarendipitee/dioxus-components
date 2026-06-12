#![allow(dead_code)]

use std::{cmp::Ordering, rc::Rc};

use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonVariant};
use dioxus_components::badge::Badge;
use dioxus_components::data_table::{
    DataTableCellContext, DataTableColumn, DataTableColumnAlign, DataTableColumnHelper,
    DataTableFilterOption, DataTableFilterState, DataTableFilterValue, DataTablePaginationState,
    DataTableRowIdentityContext, DataTableSortDirection, DataTableState, DataTableValue,
};

#[css_module("/src/components/data_table/variants/demo.css")]
struct Styles;

pub(super) const CLIENT_PAGE_SIZE: u64 = 10;
pub(super) const SERVER_PAGE_SIZE: u64 = 3;
pub(super) const ORDER_ROW_COUNT: usize = 200;

#[derive(Clone, PartialEq)]
pub(super) struct OrderRow {
    pub(super) order: String,
    pub(super) customer: String,
    pub(super) status: String,
    pub(super) total: f64,
    pub(super) updated: String,
    pub(super) updated_sort: String,
    pub(super) priority: bool,
}

pub(super) fn order_rows() -> Vec<OrderRow> {
    let customers = [
        "Northwind Supply",
        "Acme Retail",
        "Pine Street Labs",
        "Juniper Health",
        "Blue Mesa Foods",
        "Atlas Design Co.",
        "Delta Field Ops",
        "Summit Electric",
        "Redwood Logistics",
        "Cedar Grove Market",
        "Harbor Medical",
        "Apex Fabrication",
    ];
    let statuses = ["Paid", "Packing", "Backorder", "Review", "Refunded"];

    (0..ORDER_ROW_COUNT)
        .map(|index| {
            let order_number = 1042_u32.saturating_sub(index as u32);
            let customer = customers[index % customers.len()];
            let status = statuses[(index * 7) % statuses.len()];
            let total = 240.0 + ((index * 379) % 8900) as f64 + ((index % 4) as f64 * 0.25);
            let minutes_ago = 2 + index * 17;
            let day = 18 - (minutes_ago / 1440).min(17);
            let minute_of_day: usize = 12 * 60 + 58;
            let updated_minutes = minute_of_day.saturating_sub(minutes_ago % 720);
            let hour = updated_minutes / 60;
            let minute = updated_minutes % 60;

            OrderRow {
                order: format!("SO-{order_number:04}"),
                customer: customer.to_string(),
                status: status.to_string(),
                total,
                updated: relative_time(minutes_ago),
                updated_sort: format!("2026-02-{day:02}T{hour:02}:{minute:02}:00Z"),
                priority: index % 3 == 0 || total > 7_500.0,
            }
        })
        .collect()
}

/// Generates `count` order rows with unique, stable ids for large/virtualized datasets.
pub(super) fn order_rows_with_count(count: usize) -> Vec<OrderRow> {
    let customers = [
        "Northwind Supply",
        "Acme Retail",
        "Pine Street Labs",
        "Juniper Health",
        "Blue Mesa Foods",
        "Atlas Design Co.",
        "Delta Field Ops",
        "Summit Electric",
        "Redwood Logistics",
        "Cedar Grove Market",
        "Harbor Medical",
        "Apex Fabrication",
    ];
    let statuses = ["Paid", "Packing", "Backorder", "Review", "Refunded"];

    (0..count)
        .map(|index| {
            let customer = customers[index % customers.len()];
            let status = statuses[(index * 7) % statuses.len()];
            let total = 240.0 + ((index * 379) % 8900) as f64 + ((index % 4) as f64 * 0.25);
            let minutes_ago = 2 + index * 17;

            OrderRow {
                // Index-based id guarantees uniqueness across arbitrarily large datasets.
                order: format!("SO-{index:05}"),
                customer: customer.to_string(),
                status: status.to_string(),
                total,
                updated: relative_time(minutes_ago),
                updated_sort: format!("2026-02-18T00:{:02}:00Z", index % 60),
                priority: index % 3 == 0 || total > 7_500.0,
            }
        })
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ServerStatus {
    Ready,
    Loading,
    Error,
}

pub(super) struct ServerRows {
    pub(super) rows: Vec<OrderRow>,
    pub(super) total_count: usize,
}

pub(super) fn order_columns() -> Vec<DataTableColumn<OrderRow>> {
    let column = DataTableColumnHelper::<OrderRow>::new();

    vec![
        column
            .clone()
            .accessor(
                "order",
                Callback::new(|row: Rc<OrderRow>| DataTableValue::Text(row.order.to_string())),
            )
            .header("Order")
            .cell(Callback::new(|ctx: DataTableCellContext<OrderRow>| {
                let row = ctx.row.item;
                rsx! {
                    Button {
                        variant: ButtonVariant::Link,
                        class: "{Styles::dx_data_table_row_selector} data-table__row-selector",
                        r#type: "button",
                        onclick: move |_| ctx.actions.toggle_row_selected.call(ctx.row.id.clone()),
                        "{row.order}"
                    }
                }
            }))
            .sortable(),
        column
            .clone()
            .accessor(
                "customer",
                Callback::new(|row: Rc<OrderRow>| DataTableValue::Text(row.customer.to_string())),
            )
            .header("Customer")
            .cell(Callback::new(|ctx: DataTableCellContext<OrderRow>| {
                rsx! { "{ctx.row.item.customer}" }
            }))
            .sortable()
            .filter_text(),
        column
            .clone()
            .accessor(
                "status",
                Callback::new(|row: Rc<OrderRow>| DataTableValue::Text(row.status.to_string())),
            )
            .header("Status")
            .cell(Callback::new(|ctx: DataTableCellContext<OrderRow>| {
                rsx! { Badge { "{ctx.value}" } }
            }))
            .sortable()
            .filter_multi_select(status_options()),
        column
            .clone()
            .accessor(
                "total",
                Callback::new(|row: Rc<OrderRow>| DataTableValue::Number(row.total)),
            )
            .header("Total")
            .cell(Callback::new(|ctx: DataTableCellContext<OrderRow>| {
                rsx! { "{format_currency(ctx.row.item.total)}" }
            }))
            .align(DataTableColumnAlign::End)
            .sortable(),
        column
            .clone()
            .accessor(
                "priority",
                Callback::new(|row: Rc<OrderRow>| DataTableValue::Boolean(row.priority)),
            )
            .header("Priority")
            .cell(Callback::new(|ctx: DataTableCellContext<OrderRow>| {
                let label = if ctx.row.item.priority { "Yes" } else { "No" };
                rsx! { "{label}" }
            }))
            .filter_boolean(),
        column
            .clone()
            .accessor(
                "updated",
                Callback::new(|row: Rc<OrderRow>| {
                    DataTableValue::DateTime(row.updated_sort.to_string())
                }),
            )
            .header("Updated")
            .cell(Callback::new(|ctx: DataTableCellContext<OrderRow>| {
                rsx! { span { style: "color: var(--surface-muted-fg);", "{ctx.row.item.updated}" } }
            }))
            .sortable(),
    ]
}

pub(super) fn table_state(page_size: u64) -> DataTableState {
    DataTableState {
        pagination: DataTablePaginationState { page: 1, page_size },
        ..DataTableState::default()
    }
}

pub(super) fn order_row_id(ctx: DataTableRowIdentityContext<OrderRow>) -> String {
    ctx.item.order.to_string()
}

pub(super) fn fake_server_rows(state: &DataTableState) -> ServerRows {
    let mut rows = order_rows();
    rows.retain(|row| server_matches(row, state));

    for sort in state.sorting.iter().rev() {
        rows.sort_by(|left, right| {
            let ordering = compare_server_column(left, right, &sort.column);
            match sort.direction {
                DataTableSortDirection::Ascending => ordering,
                DataTableSortDirection::Descending => ordering.reverse(),
            }
        });
    }

    let total_count = rows.len();
    let page_size = state.pagination.page_size.max(1) as usize;
    let start = state.pagination.page.saturating_sub(1) as usize * page_size;
    let rows = rows.into_iter().skip(start).take(page_size).collect();

    ServerRows { rows, total_count }
}

fn status_options() -> Vec<DataTableFilterOption> {
    ["Paid", "Packing", "Backorder", "Review", "Refunded"]
        .into_iter()
        .map(|status| DataTableFilterOption {
            value: status.to_string(),
            label: status.to_string(),
        })
        .collect()
}

fn server_matches(row: &OrderRow, state: &DataTableState) -> bool {
    if let Some(search) = &state.global_filter {
        let query = search.trim().to_lowercase();
        if !query.is_empty()
            && [
                row.order.as_str(),
                row.customer.as_str(),
                row.status.as_str(),
                row.updated.as_str(),
            ]
                .into_iter()
                .any(|value| value.to_lowercase().contains(&query))
        {
            return false;
        }
    }

    state
        .filters
        .iter()
        .all(|filter| server_filter_matches(row, filter))
}

fn server_filter_matches(row: &OrderRow, filter: &DataTableFilterState) -> bool {
    match (filter.column.as_str(), &filter.value) {
        ("customer", DataTableFilterValue::Text(value)) => row
            .customer
            .to_lowercase()
            .contains(&value.trim().to_lowercase()),
        ("status", DataTableFilterValue::Multiple(values)) => {
            values.iter().any(|value| value == &row.status)
        }
        ("priority", DataTableFilterValue::Boolean(value)) => row.priority == *value,
        _ => true,
    }
}

fn compare_server_column(left: &OrderRow, right: &OrderRow, column: &str) -> Ordering {
    match column {
        "order" => left.order.cmp(&right.order),
        "customer" => left.customer.cmp(&right.customer),
        "status" => left.status.cmp(&right.status),
        "total" => left
            .total
            .partial_cmp(&right.total)
            .unwrap_or(Ordering::Equal),
        "updated" => left.updated_sort.cmp(&right.updated_sort),
        _ => Ordering::Equal,
    }
}

pub(super) fn format_currency(value: f64) -> String {
    format!("${value:.2}")
}

fn relative_time(minutes_ago: usize) -> String {
    match minutes_ago {
        0..=59 => format!("{minutes_ago}m ago"),
        60..=1439 => format!("{}h ago", minutes_ago / 60),
        _ => format!("{}d ago", minutes_ago / 1440),
    }
}
