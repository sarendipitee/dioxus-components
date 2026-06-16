#[path = "../demo_support.rs"]
mod demo_support;

use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonSize, ButtonVariant};
use dioxus_components::data_table::{
    DataTable, DataTablePageInfo, DataTableStateChange, DataTableStateMode,
};

use demo_support::{
    fake_server_rows, order_columns, order_row_id, table_state, ServerStatus, SERVER_PAGE_SIZE,
};

/// Renders a manual table backed by a preview-local fake server.
#[component]
pub fn Demo() -> Element {
    let mut state = use_signal(|| table_state(SERVER_PAGE_SIZE));
    let mut server_status = use_signal(|| ServerStatus::Ready);
    let response = fake_server_rows(&state());
    let rows = match server_status() {
        ServerStatus::Ready => response.rows,
        ServerStatus::Loading | ServerStatus::Error => Vec::new(),
    };
    let error = (server_status() == ServerStatus::Error).then_some(
        "Fake orders endpoint returned 503. Retry keeps the same DataTableState query.".to_string(),
    );

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 0.75rem;",
            div {
                style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.5rem; padding: 0.625rem 0.75rem; border: 1px solid var(--border); border-radius: var(--radius); background: var(--surface-muted);",
                span {
                    style: "color: var(--surface-muted-fg); font-size: 0.8125rem; font-weight: 500; margin-right: 0.25rem;",
                    "Simulate server response"
                }
                StateButton {
                    label: "Resolve",
                    active: server_status() == ServerStatus::Ready,
                    onclick: move |_| server_status.set(ServerStatus::Ready),
                }
                StateButton {
                    label: "Loading",
                    active: server_status() == ServerStatus::Loading,
                    onclick: move |_| server_status.set(ServerStatus::Loading),
                }
                StateButton {
                    label: "Error",
                    active: server_status() == ServerStatus::Error,
                    onclick: move |_| server_status.set(ServerStatus::Error),
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    onclick: move |_| server_status.set(ServerStatus::Ready),
                    "Retry"
                }
            }
            DataTable {
                items: rows,
                columns: order_columns(),
                page_info: DataTablePageInfo::known_total(response.total_count as u64),
                state_mode: DataTableStateMode::Controlled { state: state() },
                on_state_change: move |change: DataTableStateChange| {
                    server_status.set(ServerStatus::Loading);
                    state.set(change.next_state);
                },
                row_id: Callback::new(order_row_id),
                manual_pagination: true,
                manual_sorting: true,
                manual_filtering: true,
                loading: server_status() == ServerStatus::Loading,
                error,
                empty_message: "Server returned no orders",
                empty_hint: "Try another search, filter, sort, or page.",
                toolbar_left: rsx! {
                    span { style: "color: var(--surface-muted-fg); font-size: 0.875rem;", "Server mode" }
                },
                table_settings: rsx! {
                    span { style: "color: var(--surface-muted-fg); font-size: 0.8125rem;", "Manual pagination, sorting, and filtering" }
                },
            }
        }
    }
}

#[component]
fn StateButton(label: &'static str, active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        Button {
            variant: if active {
                ButtonVariant::Default
            } else {
                ButtonVariant::Outline
            },
            size: ButtonSize::Sm,
            onclick,
            "{label}"
        }
    }
}
