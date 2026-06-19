# Data Table

This component page is a focused demonstration of a production-style data grid: a component that lets users browse, search, and act on large typed record sets without reinventing table behavior each time.

The `DataTable` demos in this page are designed around operational use cases (invoices, tickets, exports, audit rows) where users need one surface that can sort quickly, filter by column, jump pages, and preserve interaction state across rerenders.

## State Model

`DataTableState` is the contract that describes the table’s complete behavior at any moment. It tracks pagination, sorting, column filters, global search, visibility, order, pinning, column sizing, selection, and expansion state.

- `DataTableStateMode::Uncontrolled` keeps state inside the table and starts from an optional default state. This is the right choice for local, client-only datasets.
- `DataTableStateMode::Controlled` moves state ownership to the parent and emits `DataTableStateChange` with both the next state and the triggering action.
- Manual modes preserve the same state schema while disabling built-in sort/filter/pagination transforms so the parent can compute the visible rows itself (for example, via an API call).

This page demonstrates all three patterns side by side so you can compare where control belongs in your app.

## Rows And Columns

- `items` is the typed row source passed to the table.
- `row_id` maps each row to a stable identifier used for selection, expansion, and stable keyed rendering.
- `DataTableColumn<T>` defines a column’s ID, header, accessor, metadata, and optional typed renderer.
- `DataTableColumnHelper<T>` provides a concise way to declare strongly-typed columns for a specific row model.
- Accessors return `DataTableValue`, which powers built-in search/sort/filter behavior and default rendering.
- Cell renderers receive `DataTableCellContext<T>`, which includes the row context, normalized value, current state, and actions.
- `expanded_row` enables detail rendering beneath a row using the built-in expansion column.
- `on_row_click` receives `DataTableRowClickContext<T>` whenever a body row is selected.
- `show_selection: true` turns on the built-in selection checkbox column. Omit it if you provide custom selection UI from `toolbar_left`, `toolbar_right`, or cell content.
- Custom header callbacks receive `DataTableColumnHeaderContext` with the column ID, active sort direction, and action helpers.

## DataTableColumn API

`DataTableColumn<T>` is the primary column model for table behavior and rendering decisions.

### Core fields

- `id: String`
  - Stable identifier used across ordering, pinning, sorting, filtering, visibility, and state tracking.
- `header: DataTableColumnHeader`
  - Text header label or custom header render callback.
- `accessor: DataTableColumnAccessor<T>`
  - Usually `Accessor` for built-in `DataTableValue` behavior, or `DisplayOnly`.
- `width: Option<DataTableColumnWidth>`
- `min_width: Option<f64>`
- `max_width: Option<f64>`
- `cell: Option<Callback<DataTableCellContext<T>, Element>>`
  - Optional renderer for per-cell custom markup; otherwise the default renderer uses `DataTableValue`.
- `sortable: Option<DataTableColumnSorting<T>>`
- `filter: Option<DataTableColumnFilter<T>>`
- `searchable: bool`
- `resizable: bool`
- `pinnable: bool`
- `meta: Option<DataTableColumnMeta>`

### Additional behavior fields

- `text_align: Option<DataTableColumnTextAlign>`
  - Default: `None`
  - Explicitly controls alignment for headers and body (`start`, `center`, `end`).
- `toggleable: bool`
  - Default: `false`
  - Whether the user can include this column in the visibility toggle list.
- `default_toggle: bool`
  - Default: `false`
  - If `true` and `hidden` is also `true`, the column begins in the default-on position.
- `title_class_name: Option<String>`
  - Optional extra class for plain-text headers.
- `title_style: Option<String>`
  - Optional inline style for plain-text headers.
- `hidden: bool`
  - Default: `false`
  - When `true`, the column is excluded from output unless state overrides show it.
- `hidden_content: bool`
  - Default: `false`
  - Keeps column structure while rendering an empty content cell.

### Notes

- Use `text_align` for alignment. `align(...)` remains available and maps to both `text_align` and metadata for compatibility.
- `toggleable` drives visibility controls. A column is visible by default unless state explicitly hides it.
- Columns hidden by default are omitted during canonicalization unless `column_visibility` state brings them back in.

## Client And Server Usage

Use uncontrolled mode when the full row set already exists in memory. The table applies built-in search, filtering, sorting, selection, and pagination directly to `items`.

Use controlled mode when surrounding app logic needs to observe or persist grid state. The parent owns `DataTableState` and applies `change.next_state` from `on_state_change`.

Use manual mode for server-backed workflows. The table continues emitting `DataTableStateChange`, while the parent sends those changes to the server and passes back a new `items` page plus updated `page_info`.

## Virtualization

Set `virtualization` to a `DataTableVirtualization` to render only rows near the viewport plus an overscan buffer. This is the preferred pattern for large datasets when you want smooth scrolling without loading a small subset manually.

- While virtualization is active, built-in pagination is disabled and the filtered/sorted row set scrolls inside a bounded sticky-header viewport.
- `estimated_row_height` seeds initial row positions before measurement; pick a value close to expected row height for stable first paint.
- `overscan` determines how many extra rows render above and below the viewport.
- `max_height` limits the scroll viewport with an explicit size token; set `None` if your parent layout controls height.
- Column widths stay fixed and aligned through scrolling.
- Expansion rows from `expanded_row` are not supported with virtualization and are intentionally omitted while virtual mode is active.

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

## Preview Demos

This page includes targeted demos so you can validate behavior quickly:

- `main` shows the default uncontrolled client-side table with standard searching, filtering, sorting, and paging.
- `controlled` shows parent-owned state with a manual reset action and state-driven rendering.
- `server_backed` simulates a backend query cycle with manual pagination, sorting, filtering, search, loading, error, and retry states.
- `expansion` shows row-level detail rendering through `expanded_row`.
- `selectable` demonstrates built-in checkbox selection and external state visibility.
- `virtualized` renders a 5,000-row local set inside a bounded scroll window using row virtualization.
- `density` demonstrates density presets plus inline CSS variable overrides.

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

The styled table ships with a single toolbar row that is designed for list-management workflows: global search, removable filter chips, a `+ Filter` menu, settings, sortable headers, column visibility, optional selection, optional expansion, and pagination controls.

External controls can be merged through `toolbar_left` and `toolbar_right`, while `table_settings` lets you append project-specific options to the settings dropdown. `header_controls` remains available as a compatibility path for right-side injection.

## Density

`DataTableDensity` controls the table’s baseline spacing profile:

- `DataTableDensity::Compact` tightens rhythm for dense administrative screens.
- `DataTableDensity::Default` uses the balanced baseline spacing.
- `DataTableDensity::Comfortable` increases row breathing room for readability-heavy screens.

For precise spacing control, override the CSS variables directly. Both x/y axis-specific pairs are supported:

- `--dx-data-table-cell-padding-x`
- `--dx-data-table-cell-padding-y`
- `--dx-data-table-head-cell-padding-x`
- `--dx-data-table-head-cell-padding-y`
- `--dx-data-table-selection-cell-padding-x`
- `--dx-data-table-selection-cell-padding-y`

The defaults resolve against `var(--space)` so density changes remain theme-aware.

Example:

```rust
DataTable {
    // ...
    density: DataTableDensity::Compact,
}
```

```rust
DataTable {
    // ...
    style: "--dx-data-table-cell-padding-x: 0.8rem; --dx-data-table-cell-padding-y: 0.45rem; --dx-data-table-selection-cell-padding-x: 0.8rem; --dx-data-table-selection-cell-padding-y: 0.55rem;",
}
```
