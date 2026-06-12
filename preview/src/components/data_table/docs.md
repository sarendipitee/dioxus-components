The data table component renders typed row data through column definitions and a canonical `DataTableState`. It can own that state for client-side tables or accept controlled state from a parent that maps table actions to local, manual, or server-backed data.

## State Model

- `DataTableState` is the single state object for pagination, sorting, column filters, global search, column visibility, ordering, pinning, sizing, row selection, and expanded rows.
- `DataTableStateMode::Uncontrolled` lets the table manage state internally from an optional default state.
- `DataTableStateMode::Controlled` lets the parent provide the complete state and receive `DataTableStateChange` with both the next state and the action that produced it.
- Manual modes keep the same state contract while disabling built-in pagination, sorting, or filtering transforms so a caller can fetch or derive rows externally.

## Rows And Columns

- `items` supplies typed row data.
- `row_id` maps each row to a stable string identity used by selection, expansion, and keyed rendering.
- `DataTableColumn<T>` describes a stable column id, header, accessor, feature metadata, and optional typed cell renderer.
- `DataTableColumnHelper<T>` keeps column construction concise while preserving the row type.
- Accessors return `DataTableValue`, which powers built-in search, sorting, filtering, and default cell rendering.
- Cell renderers receive `DataTableCellContext<T>` with the row context, normalized value, current state, and table actions.
- `expanded_row` enables the built-in row expansion column and renders detail content below expanded rows.
- `on_row_click` receives `DataTableRowClickContext<T>` when a body row is clicked.
- `show_selection: true` enables the built-in selection checkbox column. Leave it unset to provide custom row selection chrome from cells, `toolbar_left`, or `toolbar_right`.
- Custom headers receive `DataTableColumnHeaderContext` with the column id, current sort direction, state, and actions.

## Client And Server Usage

Use uncontrolled mode for simple client-side datasets. The table applies search, column filters, sorting, selection, and pagination to the supplied `items`.

Use controlled mode when parent code needs to persist or inspect table state. The parent stores `DataTableState` and assigns `change.next_state` from `on_state_change`.

Use manual pagination, sorting, and filtering for server-backed data. The table still emits `DataTableStateChange`; the parent sends that state to the backend, supplies the returned page of `items`, and sets `page_info` from the server response.

## Virtualization

Set `virtualization` to a `DataTableVirtualization` to render only the rows near the viewport plus an overscan buffer, reusing the shared virtualizer primitive. This keeps large client-side datasets responsive without manual paging.

- The whole filtered and sorted row set scrolls within a bounded, sticky-header surface, so built-in pagination is disabled while virtualization is active.
- `estimated_row_height` seeds row positions before measurement; set it close to the real row height for stable scrolling. Rows are measured after mount, so dynamic heights still resolve correctly.
- `overscan` controls how many buffer rows render above and below the viewport.
- `max_height` bounds the scroll viewport (any CSS length); set it to `None` to bound the surface height via CSS instead.
- Virtualization keeps the column layout fixed and rows single-line so widths and the header stay aligned during scrolling.
- Row expansion (`expanded_row`) is not supported together with virtualization; expanded detail rows are ignored while it is active.

```rust
DataTable {
    items: rows,
    columns,
    row_id: Callback::new(order_row_id),
    virtualization: Some(DataTableVirtualization {
        estimated_row_height: 49,
        overscan: 10,
        max_height: Some("32rem".to_string()),
    }),
}
```

## Preview Variants

- `main` shows the default uncontrolled client-side table.
- `controlled` shows a parent-owned `DataTableState` with a reset control.
- `server_backed` shows a preview-local fake server applying manual pagination, sorting, filtering, search, loading, error, and retry states.
- `expansion` shows row detail content rendered through `expanded_row`.
- `virtualized` shows a 5,000-row client-side table virtualized through a bounded viewport.

## Example

```rust
#[derive(Clone, PartialEq)]
struct InvoiceRow {
    id: String,
    customer: String,
    status: String,
    total: f64,
}

let column = DataTableColumnHelper::<InvoiceRow>::new();
let columns = vec![
    column
        .accessor("customer", Callback::new(|row: Rc<InvoiceRow>| {
            DataTableValue::Text(row.customer.clone())
        }))
        .header("Customer")
        .sortable()
        .filter_text(),
    column
        .accessor("status", Callback::new(|row: Rc<InvoiceRow>| {
            DataTableValue::Text(row.status.clone())
        }))
        .header("Status")
        .filter_select(vec![
            DataTableFilterOption {
                value: "paid".to_string(),
                label: "Paid".to_string(),
            },
        ]),
    column
        .accessor("total", Callback::new(|row: Rc<InvoiceRow>| {
            DataTableValue::Number(row.total)
        }))
        .header("Total")
        .cell(Callback::new(|ctx: DataTableCellContext<InvoiceRow>| {
            rsx! { "{ctx.row.item.total}" }
        }))
        .sortable(),
];

let mut state = use_signal(DataTableState::default);

DataTable {
    items: visible_rows,
    columns,
    page_info: DataTablePageInfo::known_total(total_count),
    state_mode: DataTableStateMode::Controlled { state: state() },
    on_state_change: move |change: DataTableStateChange| state.set(change.next_state),
    row_id: Callback::new(|ctx: DataTableRowIdentityContext<InvoiceRow>| ctx.item.id.clone()),
    manual_pagination: true,
    manual_sorting: true,
    manual_filtering: true,
}
```

## Core Controls

The styled table includes a single toolbar row with global search, active removable filter chips, a `+ Filter` menu, table settings, sortable headers, column visibility, optional row selection, optional row expansion, and pagination when the relevant column metadata and state allow them. Loading, empty, and error states are still provided by props so controlled and manual tables can reflect external query status.

Use `toolbar_left` and `toolbar_right` to inject additional controls around the built-in search/filter/settings controls. Use `table_settings` to add extra content to the settings dropdown next to the built-in column visibility controls. `header_controls` remains supported as a compatibility alias for right-side toolbar content.
