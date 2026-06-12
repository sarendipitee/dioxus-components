# DataTable Architecture Specification

## Purpose

`DataTable` is the canonical table system for rendering, controlling, and orchestrating structured row data in `dioxus-components`.

The component must support both client-side and server-backed workflows through the same table state model. Server-backed behavior must be first-class, not an app-specific afterthought.

## Acceptance Summary

An implementation satisfies this specification only if:

- Table behavior is represented by `DataTableState`.
- Pagination, sorting, filtering, global search, column state, row selection, and expansion all have defined state slots.
- `DataTable` renders rows from typed `items` and typed `DataTableColumn<T>` definitions.
- Column definitions include accessors, renderers, and feature metadata.
- Cell and header renderers receive typed context objects, not raw row values only.
- Server-backed tables use the same `DataTableState` and column definitions as client-side tables.
- Built-in sorting, filtering, pagination, visibility, and selection behavior do not depend on generic caller-rendered toolbar escapes.
- Row identity is stable and caller-configurable.
- State updates are emitted as complete next-state values with action metadata.
- Accessibility semantics are defined for table structure and interactive controls.
- Preview examples and tests demonstrate controlled, uncontrolled, and server-backed usage.

## Layering

`primitives/` owns reusable unstyled table behavior if behavior is factored below the styled component. This includes state transition helpers, row model derivation, and accessibility behavior when those pieces are shared below the styled component.

`dioxus-components/` owns:

- Public table state types.
- Public column and row metadata types.
- The styled `DataTable`.
- Optional first-class server-backed wrapper APIs.

`preview/` owns:

- Documentation.
- Demos.
- Fake server fixtures.
- Visual and interaction examples.

No preview-only module may define canonical table behavior or reusable table styles.

## Core State Model

`DataTableState` is the single source of truth for table behavior.

```rust
pub struct DataTableState {
    pub pagination: DataTablePaginationState,
    pub sorting: Vec<DataTableSortState>,
    pub filters: Vec<DataTableFilterState>,
    pub global_filter: Option<String>,
    pub column_visibility: Vec<DataTableColumnVisibilityState>,
    pub column_order: Vec<String>,
    pub column_pinning: DataTableColumnPinningState,
    pub column_sizing: Vec<DataTableColumnSizeState>,
    pub row_selection: DataTableRowSelectionState,
    pub expanded_rows: Vec<DataTableRowId>,
}
```

All state updates emitted by table controls must produce a complete `DataTableState`.

State updates should also identify why the update happened.

```rust
pub struct DataTableStateChange {
    pub next_state: DataTableState,
    pub action: DataTableAction,
}

pub enum DataTableAction {
    SetPage { page: u32 },
    SetPageSize { page_size: u64 },
    SetSorting { sorting: Vec<DataTableSortState> },
    SetFilter { column: String, value: Option<DataTableFilterValue> },
    SetGlobalFilter { value: Option<String> },
    SetColumnVisibility { column: String, visible: bool },
    SetColumnOrder { columns: Vec<String> },
    SetColumnPinning { pinning: DataTableColumnPinningState },
    SetColumnSize { column: String, width: Option<f64> },
    SetRowSelection { selection: DataTableRowSelectionState },
    SetExpandedRows { rows: Vec<DataTableRowId> },
    ResetFilters,
    ResetColumnState,
}

pub type DataTableQueryFingerprint = String;
```

Requirements:

- `DataTableState` remains the canonical state API.
- `on_state_change` must receive `DataTableStateChange`.
- `DataTableStateChange.next_state` is the complete next state value and is the primary controlled-state value callers store.
- Feature-specific callbacks may exist as convenience hooks, but they must not replace `DataTableState` or become the primary state API.
- State transitions must be derived from the latest known state, not from stale closure-captured values.
- Column resize may emit live updates, committed updates, or both, but the distinction must be explicit.
- `DataTableQueryFingerprint` is derived from canonical JSON serialization of canonicalized `filters` and `global_filter`. It must not include pagination or sorting.

## State Canonicalization

The public state shape may use vectors for serialization and deterministic display order, but implementations must canonicalize state before deriving rows or rendering controls.

Requirements:

- Duplicate row ids in `row_selection` and `expanded_rows` are collapsed deterministically.
- Duplicate column ids in sorting, filters, visibility, order, pinning, and sizing are resolved deterministically.
- State entries that reference unknown columns are ignored for rendering and client-side transforms.
- Unknown column entries may be preserved when emitting state so callers can round-trip externally stored table state, but they must not break rendering.
- Column ids in `column_order` that are missing from `columns` are ignored for rendering.
- Columns missing from `column_order` render after ordered columns in their definition order.
- A column cannot be pinned left and right at the same time; the implementation must resolve conflicts deterministically.
- Internal lookup structures may use maps or sets, but externally emitted state must remain stable and deterministic.

## Row Identity

Every row must have a stable identity.

```rust
pub type DataTableRowId = String;
```

`DataTable` must accept a caller-provided row id strategy:

```rust
row_id: Callback<DataTableRowIdentityContext<T>, DataTableRowId>
```

Row index is not an acceptable identity for selection, expansion, pinning, virtualization, server-backed pagination, sorted data, or filtered data.

```rust
pub struct DataTableRowIdentityContext<T> {
    pub item: Rc<T>,
    pub source_index: usize,
}
```

Requirements:

- Row ids must be stable across sorting, filtering, pagination, and server refetches.
- For server-backed tables, row ids must be unique across the logical dataset, not only the current page.
- Duplicate row ids are invalid input. Implementations must define a deterministic failure mode, such as a debug assertion plus documented last-wins or first-wins behavior in release builds.
- The row index may be exposed for display context, but it must not be used as the default identity.
- Row callbacks receive `Rc<T>` so the table can pass owned Dioxus callback inputs without cloning whole row values.
- Public table APIs that store items require `T: 'static`. Additional bounds must be local to features that need them, not global defaults.

## Pagination

```rust
pub struct DataTablePaginationState {
    pub page: u32,
    pub page_size: u64,
}
```

Requirements:

- Page numbers are one-based.
- Page size is part of `DataTableState`.
- Page changes emit a complete `DataTableState`.
- Page size changes emit a complete `DataTableState`.
- Page size changes reset page to `1`.
- Filter and global-search changes reset page to `1`.
- Known total count mode must be supported.
- Unknown total count mode must be supported for cursor-like or indeterminate server APIs.
- `DataTablePageInfo` is the source of truth for whether previous and next page targets are known.
- Pagination controls must support first, previous, next, and last when the target page is knowable.
- Last-page controls must be disabled or omitted when total count is unknown.
- Manual/server-backed pagination and client-side pagination must use the same state shape.

## Sorting

```rust
pub struct DataTableSortState {
    pub column: String,
    pub direction: DataTableSortDirection,
}

pub enum DataTableSortDirection {
    Ascending,
    Descending,
}
```

Requirements:

- Sorting state is ordered and supports multi-column sorting.
- Sortable columns opt in through column metadata.
- Sortable columns must provide either an accessor value with built-in comparison support or a custom client-side comparator.
- Header controls must reflect sortable state.
- Sort toggles must emit a complete `DataTableState`.
- Manual/server-backed sorting and client-side sorting must use the same state shape.
- The table must expose enough metadata for accessible sort indicators.

## Filtering

```rust
pub struct DataTableFilterState {
    pub column: String,
    pub value: DataTableFilterValue,
}

pub enum DataTableFilterValue {
    Text(String),
    Option(String),
    Multiple(Vec<String>),
    Boolean(bool),
    Range {
        min: Option<String>,
        max: Option<String>,
    },
}
```

Requirements:

- Column filters are keyed by column id.
- Global search is represented separately from column filters.
- Filterable columns declare filter metadata.
- Filter metadata must define the filter kind, option labels when applicable, and how string state values are parsed or compared for client-side filtering.
- Custom client-side filter predicates must be supported for columns whose value type is not covered by built-in filters.
- Filter changes emit a complete `DataTableState`.
- Filter changes reset pagination to page `1`.
- Manual/server-backed filtering and client-side filtering must use the same state shape.
- Manual filtering means `DataTable` emits filter state changes but does not filter `items` locally.

## Column State

```rust
pub struct DataTableColumnVisibilityState {
    pub column: String,
    pub visible: bool,
}

pub struct DataTableColumnPinningState {
    pub left: Vec<String>,
    pub right: Vec<String>,
}

pub struct DataTableColumnSizeState {
    pub column: String,
    pub width: Option<f64>,
}
```

Requirements:

- Column visibility is stateful.
- Column order is stateful.
- Column pinning is stateful.
- Column sizing is stateful.
- Hidden columns must not render header cells or body cells.
- Reordered columns must render headers and cells in the same order.
- Pinned columns must preserve accessible table semantics.
- Duplicate column keys are invalid input.
- State that references missing columns must not render phantom headers or cells.

## Row State

```rust
pub enum DataTableRowSelectionState {
    Explicit {
        rows: Vec<DataTableRowId>,
    },
    AllMatching {
        query: DataTableQueryFingerprint,
        except: Vec<DataTableRowId>,
    },
}
```

Requirements:

- Row selection is represented by row ids.
- Explicit selection stores selected row ids.
- `AllMatching` represents all rows in the logical filtered dataset identified by `query`, except the listed row ids. It exists for server-backed bulk workflows where not every selected id is loaded.
- Expanded rows are represented by row ids.
- Row selection changes emit a complete `DataTableState`.
- Row expansion changes emit a complete `DataTableState`.
- Explicit selection and expansion must survive sorting, filtering, pagination, and server refetches when row ids remain present.
- `AllMatching` selection must be cleared when filters or global search change and the new `DataTableQueryFingerprint` does not match the stored `query`.
- Sorting and pagination changes must not change the `DataTableQueryFingerprint`.
- Selection and expansion state for row ids not present in the current rendered page may remain in `DataTableState`.
- Built-in controls must distinguish selecting all rows on the current page from selecting all rows in the filtered logical dataset.

## Column Values

Built-in sorting, filtering, global search, and default display use an erased value model so one table can store heterogeneous column values in one `Vec<DataTableColumn<T>>`.

```rust
pub enum DataTableValue {
    Text(String),
    Number(f64),
    Boolean(bool),
    DateTime(String),
    Empty,
}
```

Requirements:

- `DataTableValue` is the normalized value model for built-in client-side behavior.
- `Number(f64)` values that are `NaN` sort after all non-NaN numbers in ascending order and before all non-NaN numbers in descending order.
- `NaN` does not match numeric range filters.
- `DateTime` values must use a documented sortable string format, preferably ISO 8601.
- Custom comparators and custom filter predicates may use the original row item when `DataTableValue` is insufficient.
- Built-in default cell rendering uses `DataTableValue` unless a custom cell renderer is provided.

## Column Model

```rust
pub struct DataTableColumn<T> {
    pub id: String,
    pub header: DataTableColumnHeader,
    pub accessor: DataTableColumnAccessor<T>,
    pub width: Option<DataTableColumnWidth>,
    pub min_width: Option<f64>,
    pub max_width: Option<f64>,
    pub cell: Option<Callback<DataTableCellContext<T>, Element>>,
    pub sortable: Option<DataTableColumnSorting<T>>,
    pub filter: Option<DataTableColumnFilter<T>>,
    pub searchable: bool,
    pub hideable: bool,
    pub resizable: bool,
    pub pinnable: bool,
    pub meta: Option<DataTableColumnMeta>,
}

pub enum DataTableColumnAccessor<T> {
    Accessor(Callback<Rc<T>, DataTableValue>),
    DisplayOnly,
}

pub enum DataTableColumnWidth {
    Px(f64),
    Css(String),
}

pub enum DataTableColumnHeader {
    Label(String),
    Custom(Callback<DataTableColumnHeaderContext, Element>),
}

pub enum DataTableColumnSorting<T> {
    BuiltIn,
    Custom(Callback<DataTableSortCompareContext<T>, std::cmp::Ordering>),
}

pub struct DataTableSortCompareContext<T> {
    pub left: DataTableRow<T>,
    pub right: DataTableRow<T>,
    pub direction: DataTableSortDirection,
}

pub enum DataTableColumnFilter<T> {
    Text,
    Select { options: Vec<DataTableFilterOption> },
    MultiSelect { options: Vec<DataTableFilterOption> },
    Boolean,
    Range { kind: DataTableRangeFilterKind },
    Custom(Callback<DataTableFilterPredicateContext<T>, bool>),
}

pub struct DataTableFilterOption {
    pub value: String,
    pub label: String,
}

pub enum DataTableRangeFilterKind {
    Number,
    Text,
    DateTime,
}

pub struct DataTableFilterPredicateContext<T> {
    pub row: DataTableRow<T>,
    pub column_id: String,
    pub value: DataTableValue,
    pub filter: DataTableFilterValue,
}

pub struct DataTableColumnMeta {
    pub description: Option<String>,
    pub align: Option<DataTableColumnAlign>,
    pub class: Option<String>,
    pub data_attributes: Vec<(String, String)>,
}

pub enum DataTableColumnAlign {
    Start,
    Center,
    End,
}
```

Requirements:

- `id` is the stable column id used by state.
- Column ids must be unique.
- `accessor` provides the semantic value used by built-in sorting, filtering, search, and default cell rendering.
- Display-only columns may omit semantic values, but they cannot use built-in sorting or filtering unless they provide custom behavior.
- `cell` renders cell content for one typed row context. If omitted, the table renders the accessor value with a default display strategy.
- `width`, `min_width`, `max_width`, and `DataTableColumnSizeState.width` are pixel-based for numeric sizing. `Css` width is accepted only as an initial style hint and must be normalized before resize state is emitted.
- `searchable` controls whether the column participates in global search.
- Column metadata is for presentation and application-specific annotations. Built-in feature availability is controlled by explicit fields such as `sortable`, `filter`, `hideable`, `resizable`, and `pinnable`.
- Built-in controls for sorting, filtering, visibility, sizing, and pinning must derive from explicit column feature fields and `DataTableState`.
- Custom header rendering must receive enough context to preserve built-in state and accessibility.
- Column callbacks receive owned context values containing `Rc<T>` rows so they are compatible with Dioxus callback ownership.
- `DataTableColumnSorting<T>` must define either built-in sorting over `DataTableValue` or a custom comparator over row contexts.
- `DataTableColumnFilter<T>` must define the filter UI kind and either built-in filtering over `DataTableValue` or a custom predicate over row contexts.

## Row and Render Contexts

Renderers must receive explicit context objects so application authors do not need to close over table internals.

```rust
pub struct DataTableRow<T> {
    pub id: DataTableRowId,
    pub item: Rc<T>,
    pub source_index: usize,
    pub display_index: usize,
    pub selected: bool,
    pub expanded: bool,
}

pub struct DataTableCellContext<T> {
    pub row: DataTableRow<T>,
    pub column_id: String,
    pub value: DataTableValue,
    pub state: DataTableState,
    pub actions: DataTableActions,
}

pub struct DataTableColumnHeaderContext {
    pub column_id: String,
    pub state: DataTableState,
    pub sorting: Option<DataTableSortDirection>,
    pub actions: DataTableActions,
}

pub struct DataTableActions {
    pub update_state: Callback<DataTableAction, ()>,
    pub toggle_row_selected: Callback<DataTableRowId, ()>,
    pub toggle_row_expanded: Callback<DataTableRowId, ()>,
    pub toggle_all_page_rows_selected: Callback<(), ()>,
    pub toggle_all_matching_rows_selected: Callback<(), ()>,
    pub reset_filters: Callback<(), ()>,
    pub reset_column_state: Callback<(), ()>,
}
```

Requirements:

- Context must expose row id, row data, source index, display index, column id, relevant state, and action helpers.
- Display index is allowed for UI display only; it must not become row identity.
- `update_state(DataTableAction)` is the generic helper for sorting, filtering, visibility, pinning, sizing, and other full-state transitions.
- Named action helpers are required for common row selection, row expansion, reset filters, and reset column state workflows.
- Contexts own cheap metadata and clone `Rc<T>` row handles. They must not require cloning full row values.

## Column Authoring Ergonomics

The public API should include a builder or helper so callers can define columns without manually constructing every field.

Target shape:

```rust
let column = DataTableColumnHelper::<User>::new();

let columns = vec![
    column
        .accessor("email", |user| DataTableValue::Text(user.email.clone()))
        .header("Email")
        .sortable()
        .filter_text()
        .cell(|ctx| rsx! { "{ctx.value}" }),
    column
        .display("actions")
        .header("Actions")
        .cell(|ctx| rsx! { /* row actions */ }),
];
```

Requirements:

- The helper must make the common case concise.
- The helper must not hide the underlying `DataTableColumn` semantics.
- Accessor columns should make built-in sort/filter/search setup straightforward.
- Display-only columns should be explicit.

## DataTable API

`DataTable` must be state-first.

```rust
DataTable {
    columns,
    items,
    page_info,
    state_mode,
    on_state_change,
    row_id,
    manual_pagination,
    manual_sorting,
    manual_filtering,
    loading,
    error,
}
```

Requirements:

- `items` contains the rows available to render for the active data mode.
- `columns` defines rendering and feature metadata.
- `page_info` describes total count and known navigation availability.
- `state_mode` chooses controlled or uncontrolled state.
- `on_state_change` receives `DataTableStateChange`.
- `row_id` maps row data to stable row ids.
- Manual mode flags determine whether `DataTable` transforms rows locally or only emits state changes.
- `loading` and `error` are presentational state derived from the caller or server-backed wrapper.
- Slots may be offered for toolbar, empty state, loading state, error state, bulk actions, row actions, pagination override, and column visibility controls.
- Built-in features must remain available without requiring callers to provide those slots.

## Controlled and Uncontrolled Usage

Controlled usage:

```rust
DataTable {
    state_mode: DataTableStateMode::Controlled { state },
    on_state_change,
}
```

Uncontrolled usage:

```rust
DataTable {
    state_mode: DataTableStateMode::Uncontrolled { default_state },
    on_state_change,
}
```

State mode:

```rust
pub enum DataTableStateMode {
    Controlled { state: DataTableState },
    Uncontrolled { default_state: Option<DataTableState> },
}
```

Requirements:

- Controlled and uncontrolled APIs must both use `DataTableState`.
- Mixed partial-control APIs are not valid and are prevented by `DataTableStateMode`.
- Uncontrolled state changes may still notify `on_state_change`.
- Controlled state must never be mutated internally without emitting `on_state_change`.
- In uncontrolled mode, `on_state_change` must fire after the next internal state is derived and before or during the same render update cycle. The exact timing must be documented.

## Client-Side Mode

Client-side mode derives displayed rows from `items` and `DataTableState`.

Requirements:

- Client-side pagination uses `state.pagination`.
- Client-side sorting uses `state.sorting`.
- Client-side filtering uses `state.filters` and `state.global_filter`.
- Derived row order and visibility must be deterministic.
- Client-side mode must not use a different column model.
- Client-side row derivation order is:

```text
items -> row ids -> filtering/global search -> sorting -> pagination -> row contexts
```

- Column derivation order is:

```text
columns -> canonical column state -> visibility -> order -> pinning -> sizing
```

- `manual_filtering` skips local filtering and global search filtering.
- `manual_sorting` skips local sorting.
- `manual_pagination` skips local pagination.
- Mixed manual modes are valid only if the behavior remains deterministic. For example, manual pagination with client-side sorting sorts only the provided `items`, not the whole remote dataset.
- `page_info.total_count` is derived from filtered client-side rows in pure client mode and supplied by callers in manual/server-backed pagination mode.

## Server-Backed Mode

Server-backed mode uses `DataTableState` as fetch input.

```rust
pub struct ServerDataTableData<T> {
    pub items: Vec<T>,
    pub page_info: DataTablePageInfo,
}

pub enum DataTableTotalCount {
    Known(u64),
    Unknown,
}

pub struct DataTablePageInfo {
    pub total_count: DataTableTotalCount,
    pub has_next_page: Option<bool>,
    pub has_previous_page: Option<bool>,
}

pub struct DataTableFetchError {
    pub message: String,
}

pub type DataTableFetcher<T> = Arc<
    dyn Fn(DataTableState) -> Pin<Box<dyn Future<Output = Result<ServerDataTableData<T>, DataTableFetchError>>>>
        + 'static,
>;
```

Target wrapper shape:

```rust
ServerDataTable {
    columns,
    state_mode,
    on_state_change,
    row_id,
    fetcher,
}
```

The semantic fetcher contract is:

```rust
fetcher(DataTableState) -> Future<Output = Result<ServerDataTableData<T>, DataTableFetchError>>
```

Requirements:

- Server-backed tables use the same `DataTableState` as client-side tables.
- Server-backed tables use the same `DataTableColumn<T>` definitions as client-side tables.
- Pagination, sorting, filtering, and search changes trigger refetches.
- The wrapper tracks loading state.
- The wrapper tracks error state.
- The wrapper supports retry.
- The wrapper ignores stale responses using a deterministic latest-request-wins mechanism, such as a monotonically increasing request id.
- Retry must use the latest table state unless the API explicitly exposes retrying the failed request state.
- The wrapper must define whether loading preserves previous rows or clears rows; preserving previous rows is preferred for pagination, sorting, and filtering transitions.
- The wrapper must define whether filter and global-search refetches are debounced and how callers configure the debounce.
- The public error type is `DataTableFetchError`. Applications may map their own errors into it.
- The wrapper must not know about HTTP, URL query params, router state, or application request types.
- Applications map `DataTableState` into their own request contracts.

## Accessibility

Requirements:

- Render semantic `<table>`, `<thead>`, `<tbody>`, `<tr>`, `<th>`, and `<td>` elements.
- The implementation must use native table semantics by default. If a future grid mode is added, it must be a separate documented mode.
- Header cells must use `scope="col"`.
- Sortable headers must expose accessible sort state with `aria-sort`.
- Interactive header controls must be keyboard accessible.
- Pagination controls must be keyboard accessible and labelled.
- Loading state must expose appropriate busy/live semantics.
- Error state must expose alert semantics.
- Selection controls must expose selected state.
- Expansion controls must expose expanded state.
- Column resize, row expansion, row selection, and sortable header controls must have documented keyboard behavior.

## Styling

Requirements:

- Canonical reusable styles live in `dioxus-components/src/components/data_table/style.css`.
- Demo-only styling lives in `preview/`.
- State styling must use data attributes or stable class hooks.
- Styling must not depend on preview-only theme tokens.
- Responsive behavior must preserve table readability and control accessibility.

## Rejected API Shapes

These are not acceptable primary APIs:

```rust
current_page: Option<u32>
on_page_change: Option<EventHandler<u32>>
page_size: u64
sort_column: Option<String>
sort_direction: Option<DataTableSortDirection>
on_sort_change: Option<EventHandler<_>>
filters: Vec<_>
on_filters_change: Option<EventHandler<_>>
selected_rows: Vec<_>
on_selected_rows_change: Option<EventHandler<_>>
visible_columns: Vec<_>
on_visible_columns_change: Option<EventHandler<_>>
```

These split the table into unrelated prop islands. Implementations must use `DataTableState` and full-state updates instead.

`header_controls` may be offered for custom toolbar content. It must not be the mechanism for built-in sorting, filtering, column visibility, or selection behavior.

Primary APIs must also avoid raw renderer-only column definitions that make sorting, filtering, and server mapping impossible without duplicate metadata.

## Documentation Requirements

Preview docs must include:

- Basic client-side table.
- Controlled table state example.
- Uncontrolled table state example.
- Server-backed table example using a fake async data source.
- Server-backed table example with pagination, sorting, filtering, loading, error, retry, and stale-response handling.
- Sorting example.
- Filtering example.
- Custom filter example.
- Pagination and page-size example.
- Row selection example.
- Bulk action example that distinguishes current-page selection from logical-dataset selection.
- Column visibility example.
- Column ordering, pinning, and sizing examples.
- Custom cell renderer example.
- Column helper or builder example.

Docs must explain how applications map `DataTableState` into their own server request types without requiring a component-level query type.

## Test Requirements

Required validation:

- `cargo check -p dioxus-components`
- `cargo check -p preview`
- `scripts/preview-web.sh build`
- Relevant stylelint command for changed CSS.

Behavior coverage must include:

- Controlled state updates emit complete `DataTableState`.
- Uncontrolled state updates mutate internal state and notify callbacks.
- Pagination state changes.
- Sorting state changes.
- Filtering state changes.
- Server-backed refetch on state changes.
- Stale server responses are ignored.
- Row selection remains keyed by row id.
- Duplicate row ids and duplicate column ids are handled according to the documented invalid-input behavior.
- State entries for unknown columns do not break rendering.
- Client-side row derivation follows the specified filtering, sorting, and pagination order.
- Mixed manual modes follow the documented local-transform behavior.
- Column visibility affects headers and cells consistently.
- Column order, pinning, and sizing affect headers and cells consistently.
- Keyboard interaction for built-in controls.
- Selection persists across pagination and server refetches when row ids remain in the logical dataset.
- Custom cell, header, sort, and filter contexts expose the documented state and action helpers.
