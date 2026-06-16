use std::cmp::Ordering;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};

use crate::component_styles;
use dioxus::prelude::*;
use dioxus_icons::lucide::{ArrowDown, ArrowUp, ArrowUpDown, Plus, Search, SlidersHorizontal, X};
use dioxus_primitives::checkbox::CheckboxState;
use dioxus_primitives::r#virtual::types::VirtualItem;
use dioxus_primitives::r#virtual::{
    compute_measurements, get_total_size, get_virtual_items, resize_item, set_scroll_offset,
    set_viewport_size, VirtualizerState, VirtualizerStateStoreExt,
};
use dioxus_primitives::TextOrElement;

use crate::components::{
    Button, ButtonSize, ButtonVariant, Checkbox, ComboboxEmpty, ComboboxOption,
    MultiSelect as ComboboxMultiSelect, Pagination, PaginationContent, PaginationFirst,
    PaginationItem, PaginationLast, PaginationNext, PaginationPrevious, PopoverContent,
    PopoverRoot, PopoverTrigger, Select, SelectOption, Skeleton, TextInput, Tooltip,
    TooltipContent, TooltipTrigger,
};
use crate::input::InputWrapper;

#[component_styles("./style.css")]
struct Styles;

/// Stable row identifier used by table state.
pub type DataTableRowId = String;

/// Stable fingerprint for the active filtered query.
pub type DataTableQueryFingerprint = String;

/// Complete canonical state for `DataTable` behavior.
#[derive(Clone, Debug, PartialEq)]
pub struct DataTableState {
    /// Current one-based page and page size.
    pub pagination: DataTablePaginationState,
    /// Ordered sorting state.
    pub sorting: Vec<DataTableSortState>,
    /// Column filter state.
    pub filters: Vec<DataTableFilterState>,
    /// Optional global search filter.
    pub global_filter: Option<String>,
    /// Column visibility overrides.
    pub column_visibility: Vec<DataTableColumnVisibilityState>,
    /// Preferred column display order.
    pub column_order: Vec<String>,
    /// Column pinning state.
    pub column_pinning: DataTableColumnPinningState,
    /// Column sizing overrides.
    pub column_sizing: Vec<DataTableColumnSizeState>,
    /// Row selection state.
    pub row_selection: DataTableRowSelectionState,
    /// Expanded row identifiers.
    pub expanded_rows: Vec<DataTableRowId>,
}

impl Default for DataTableState {
    fn default() -> Self {
        Self {
            pagination: DataTablePaginationState {
                page: 1,
                page_size: 25,
            },
            sorting: Vec::new(),
            filters: Vec::new(),
            global_filter: None,
            column_visibility: Vec::new(),
            column_order: Vec::new(),
            column_pinning: DataTableColumnPinningState::default(),
            column_sizing: Vec::new(),
            row_selection: DataTableRowSelectionState::default(),
            expanded_rows: Vec::new(),
        }
    }
}

/// Pagination state for the canonical table model.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DataTablePaginationState {
    /// One-based page number.
    pub page: u32,
    /// Number of rows represented by each page.
    pub page_size: u64,
}

/// One sorted column entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataTableSortState {
    /// Column id being sorted.
    pub column: String,
    /// Sort direction for the column.
    pub direction: DataTableSortDirection,
}

/// Sort direction for a sorted column.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataTableSortDirection {
    /// Ascending sort order.
    Ascending,
    /// Descending sort order.
    Descending,
}

/// One column filter entry.
#[derive(Clone, Debug, PartialEq)]
pub struct DataTableFilterState {
    /// Column id being filtered.
    pub column: String,
    /// Filter value for the column.
    pub value: DataTableFilterValue,
}

/// Serializable filter value used by table state.
#[derive(Clone, Debug, PartialEq)]
pub enum DataTableFilterValue {
    /// Text filter.
    Text(String),
    /// Single option filter.
    Option(String),
    /// Multiple option filter.
    Multiple(Vec<String>),
    /// Boolean filter.
    Boolean(bool),
    /// Range filter with optional string bounds.
    Range {
        /// Optional minimum value.
        min: Option<String>,
        /// Optional maximum value.
        max: Option<String>,
    },
}

/// One column visibility override.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataTableColumnVisibilityState {
    /// Column id.
    pub column: String,
    /// Whether the column is visible.
    pub visible: bool,
}

/// Column pinning state.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DataTableColumnPinningState {
    /// Column ids pinned to the left side.
    pub left: Vec<String>,
    /// Column ids pinned to the right side.
    pub right: Vec<String>,
}

/// One column sizing override.
#[derive(Clone, Debug, PartialEq)]
pub struct DataTableColumnSizeState {
    /// Column id.
    pub column: String,
    /// Pixel width override. `None` clears the override.
    pub width: Option<f64>,
}

/// Row selection state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DataTableRowSelectionState {
    /// Explicitly selected row ids.
    Explicit {
        /// Selected row ids.
        rows: Vec<DataTableRowId>,
    },
    /// All rows matching a query fingerprint except listed row ids.
    AllMatching {
        /// Query fingerprint represented by the selection.
        query: DataTableQueryFingerprint,
        /// Excluded row ids.
        except: Vec<DataTableRowId>,
    },
}

impl Default for DataTableRowSelectionState {
    fn default() -> Self {
        Self::Explicit { rows: Vec::new() }
    }
}

/// Complete state update emitted by `DataTable`.
#[derive(Clone, Debug, PartialEq)]
pub struct DataTableStateChange {
    /// Complete next canonical state.
    pub next_state: DataTableState,
    /// Action that produced the next state.
    pub action: DataTableAction,
}

/// Canonical state action for table controls and renderer contexts.
#[derive(Clone, Debug, PartialEq)]
pub enum DataTableAction {
    /// Set the one-based page.
    SetPage { page: u32 },
    /// Set page size and reset page to one.
    SetPageSize { page_size: u64 },
    /// Replace sorting state.
    SetSorting { sorting: Vec<DataTableSortState> },
    /// Set or remove a column filter.
    SetFilter {
        /// Column id.
        column: String,
        /// New filter value, or `None` to remove it.
        value: Option<DataTableFilterValue>,
    },
    /// Set global filter.
    SetGlobalFilter { value: Option<String> },
    /// Set a column visibility override.
    SetColumnVisibility { column: String, visible: bool },
    /// Set column order.
    SetColumnOrder { columns: Vec<String> },
    /// Set column pinning.
    SetColumnPinning {
        pinning: DataTableColumnPinningState,
    },
    /// Set or clear a column pixel width.
    SetColumnSize { column: String, width: Option<f64> },
    /// Replace row selection state.
    SetRowSelection {
        /// New selection state.
        selection: DataTableRowSelectionState,
    },
    /// Replace expanded row ids.
    SetExpandedRows { rows: Vec<DataTableRowId> },
    /// Clear column filters and global filter.
    ResetFilters,
    /// Reset column visibility, order, pinning, and sizing.
    ResetColumnState,
}

/// Controlled or uncontrolled canonical state mode.
#[derive(Clone, Debug, PartialEq)]
pub enum DataTableStateMode {
    /// Caller owns the full table state.
    Controlled { state: DataTableState },
    /// Table owns state initialized from `default_state` when provided.
    Uncontrolled {
        /// Optional initial state for the uncontrolled table.
        default_state: Option<DataTableState>,
    },
}

impl Default for DataTableStateMode {
    fn default() -> Self {
        Self::Uncontrolled {
            default_state: None,
        }
    }
}

/// Total row count mode for pagination.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataTableTotalCount {
    /// Total count is known.
    Known(u64),
    /// Total count is unknown.
    Unknown,
}

/// Pagination metadata for the currently supplied items.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DataTablePageInfo {
    /// Total row count mode.
    pub total_count: DataTableTotalCount,
    /// Whether a next page is known to exist.
    pub has_next_page: Option<bool>,
    /// Whether a previous page is known to exist.
    pub has_previous_page: Option<bool>,
}

impl DataTablePageInfo {
    /// Creates page info for a known total count.
    pub fn known_total(total_count: u64) -> Self {
        Self {
            total_count: DataTableTotalCount::Known(total_count),
            has_next_page: None,
            has_previous_page: None,
        }
    }
}

impl Default for DataTablePageInfo {
    fn default() -> Self {
        Self::known_total(0)
    }
}

/// Normalized column value used by built-in table features and default cells.
#[derive(Clone, Debug, PartialEq)]
pub enum DataTableValue {
    /// Text value.
    Text(String),
    /// Numeric value.
    Number(f64),
    /// Boolean value.
    Boolean(bool),
    /// Sortable datetime string, preferably ISO 8601.
    DateTime(String),
    /// Empty value.
    Empty,
}

impl std::fmt::Display for DataTableValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(value) | Self::DateTime(value) => f.write_str(value),
            Self::Number(value) => write!(f, "{value}"),
            Self::Boolean(value) => write!(f, "{value}"),
            Self::Empty => Ok(()),
        }
    }
}

/// Semantic accessor for a data table column.
#[derive(Clone, PartialEq)]
pub enum DataTableColumnAccessor<T: Clone + PartialEq + 'static = String> {
    /// Accesses a normalized value from a row.
    Accessor(Callback<Rc<T>, DataTableValue>),
    /// Column has no semantic value.
    DisplayOnly,
}

/// Column width configuration.
#[derive(Clone, Debug, PartialEq)]
pub enum DataTableColumnWidth {
    /// Pixel width.
    Px(f64),
    /// CSS width hint.
    Css(String),
}

/// Header renderer for a column.
///
/// This is an alias for [`TextOrElement`] parameterised with [`DataTableColumnHeaderContext`].
/// Use `TextOrElement::Text(label)` for plain text headers and `TextOrElement::Render(cb)` for
/// custom renderers.
pub type DataTableColumnHeader = TextOrElement<DataTableColumnHeaderContext>;

/// Sorting support for a column.
#[derive(Clone, PartialEq)]
pub enum DataTableColumnSorting<T: Clone + PartialEq + 'static = String> {
    /// Built-in sorting over `DataTableValue`.
    BuiltIn,
    /// Custom comparator over row contexts.
    Custom(Callback<DataTableSortCompareContext<T>, Ordering>),
}

/// Context passed to a custom sort comparator.
#[derive(Clone, PartialEq)]
pub struct DataTableSortCompareContext<T: Clone + PartialEq + 'static = String> {
    /// Left row.
    pub left: DataTableRow<T>,
    /// Right row.
    pub right: DataTableRow<T>,
    /// Sort direction being applied.
    pub direction: DataTableSortDirection,
}

/// Filter support for a column.
#[derive(Clone, PartialEq)]
pub enum DataTableColumnFilter<T: Clone + PartialEq + 'static = String> {
    /// Text filter.
    Text,
    /// Single-select filter.
    Select { options: Vec<DataTableFilterOption> },
    /// Multi-select filter.
    MultiSelect { options: Vec<DataTableFilterOption> },
    /// Boolean filter.
    Boolean,
    /// Range filter.
    Range { kind: DataTableRangeFilterKind },
    /// Custom predicate over row context.
    Custom(Callback<DataTableFilterPredicateContext<T>, bool>),
}

/// Option used by select-style column filters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataTableFilterOption {
    /// Stored option value.
    pub value: String,
    /// Display label.
    pub label: String,
}

/// Range filter value kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataTableRangeFilterKind {
    /// Numeric range.
    Number,
    /// Text range.
    Text,
    /// Datetime string range.
    DateTime,
}

/// Context passed to a custom filter predicate.
#[derive(Clone, PartialEq)]
pub struct DataTableFilterPredicateContext<T: Clone + PartialEq + 'static = String> {
    /// Row being tested.
    pub row: DataTableRow<T>,
    /// Column id being filtered.
    pub column_id: String,
    /// Column value for the row.
    pub value: DataTableValue,
    /// Filter value from canonical state.
    pub filter: DataTableFilterValue,
}

/// Presentation and application metadata for a column.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DataTableColumnMeta {
    /// Optional accessible or UI description.
    pub description: Option<String>,
    /// Optional text alignment.
    pub align: Option<DataTableColumnAlign>,
    /// Optional CSS class hook.
    pub class: Option<String>,
    /// Stable data attributes as name/value pairs.
    pub data_attributes: Vec<(String, String)>,
}

/// Column alignment metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataTableColumnAlign {
    /// Start aligned.
    Start,
    /// Center aligned.
    Center,
    /// End aligned.
    End,
}

/// Describes a single `DataTable` column and its feature metadata.
#[derive(Clone, PartialEq)]
pub struct DataTableColumn<T: Clone + PartialEq + 'static = String> {
    /// Stable identifier for the column.
    pub id: String,
    /// Header label or custom header renderer.
    pub header: DataTableColumnHeader,
    /// Semantic value accessor for built-in features.
    pub accessor: DataTableColumnAccessor<T>,
    /// Initial width configuration.
    pub width: Option<DataTableColumnWidth>,
    /// Minimum pixel width for resizing.
    pub min_width: Option<f64>,
    /// Maximum pixel width for resizing.
    pub max_width: Option<f64>,
    /// Optional typed cell renderer.
    pub cell: Option<Callback<DataTableCellContext<T>, Element>>,
    /// Optional sorting support.
    pub sortable: Option<DataTableColumnSorting<T>>,
    /// Optional filtering support.
    pub filter: Option<DataTableColumnFilter<T>>,
    /// Whether this column participates in global search.
    pub searchable: bool,
    /// Whether this column can be hidden.
    pub hideable: bool,
    /// Whether this column can be resized.
    pub resizable: bool,
    /// Whether this column can be pinned.
    pub pinnable: bool,
    /// Optional presentation metadata.
    pub meta: Option<DataTableColumnMeta>,
}

impl<T: Clone + PartialEq + 'static> DataTableColumn<T> {
    /// Creates a display-only column with a label and typed cell renderer.
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        cell: impl Into<Callback<DataTableCellContext<T>, Element>>,
    ) -> Self {
        let label = label.into();
        Self {
            id: id.into(),
            header: TextOrElement::Text(label),
            accessor: DataTableColumnAccessor::DisplayOnly,
            width: None,
            min_width: None,
            max_width: None,
            cell: Some(cell.into()),
            sortable: None,
            filter: None,
            searchable: false,
            hideable: true,
            resizable: false,
            pinnable: false,
            meta: None,
        }
    }

    /// Creates a display-only column with a CSS width hint and typed cell renderer.
    pub fn with_width(
        id: impl Into<String>,
        label: impl Into<String>,
        width: impl Into<String>,
        cell: impl Into<Callback<DataTableCellContext<T>, Element>>,
    ) -> Self {
        Self {
            width: Some(DataTableColumnWidth::Css(width.into())),
            ..Self::new(id, label, cell)
        }
    }

    /// Creates an accessor column with a label.
    pub fn accessor(
        id: impl Into<String>,
        label: impl Into<String>,
        accessor: impl Into<Callback<Rc<T>, DataTableValue>>,
    ) -> Self {
        let label = label.into();
        Self {
            id: id.into(),
            header: TextOrElement::Text(label),
            accessor: DataTableColumnAccessor::Accessor(accessor.into()),
            width: None,
            min_width: None,
            max_width: None,
            cell: None,
            sortable: None,
            filter: None,
            searchable: true,
            hideable: true,
            resizable: false,
            pinnable: false,
            meta: None,
        }
    }

    /// Sets a custom header renderer.
    pub fn header(mut self, header: impl Into<DataTableColumnHeader>) -> Self {
        self.header = header.into();
        self
    }

    /// Sets a typed cell renderer.
    pub fn cell(mut self, cell: impl Into<Callback<DataTableCellContext<T>, Element>>) -> Self {
        self.cell = Some(cell.into());
        self
    }

    /// Enables built-in sorting for this column.
    pub fn sortable(mut self) -> Self {
        self.sortable = Some(DataTableColumnSorting::BuiltIn);
        self
    }

    /// Enables custom sorting for this column.
    pub fn sort_with(
        mut self,
        compare: impl Into<Callback<DataTableSortCompareContext<T>, Ordering>>,
    ) -> Self {
        self.sortable = Some(DataTableColumnSorting::Custom(compare.into()));
        self
    }

    /// Enables a text column filter.
    pub fn filter_text(mut self) -> Self {
        self.filter = Some(DataTableColumnFilter::Text);
        self
    }

    /// Enables a single-select column filter.
    pub fn filter_select(mut self, options: Vec<DataTableFilterOption>) -> Self {
        self.filter = Some(DataTableColumnFilter::Select { options });
        self
    }

    /// Enables a multi-select column filter.
    pub fn filter_multi_select(mut self, options: Vec<DataTableFilterOption>) -> Self {
        self.filter = Some(DataTableColumnFilter::MultiSelect { options });
        self
    }

    /// Enables a boolean column filter.
    pub fn filter_boolean(mut self) -> Self {
        self.filter = Some(DataTableColumnFilter::Boolean);
        self
    }

    /// Enables a range column filter.
    pub fn filter_range(mut self, kind: DataTableRangeFilterKind) -> Self {
        self.filter = Some(DataTableColumnFilter::Range { kind });
        self
    }

    /// Enables a custom column filter predicate.
    pub fn filter_with(
        mut self,
        predicate: impl Into<Callback<DataTableFilterPredicateContext<T>, bool>>,
    ) -> Self {
        self.filter = Some(DataTableColumnFilter::Custom(predicate.into()));
        self
    }

    /// Enables this column for global search.
    pub fn searchable(mut self) -> Self {
        self.searchable = true;
        self
    }

    /// Enables this column for built-in visibility controls.
    pub fn hideable(mut self) -> Self {
        self.hideable = true;
        self
    }

    /// Sets the column text alignment for header and body cells.
    pub fn align(mut self, align: DataTableColumnAlign) -> Self {
        self.meta
            .get_or_insert_with(DataTableColumnMeta::default)
            .align = Some(align);
        self
    }

    /// Sets presentation metadata for this column.
    pub fn meta(mut self, meta: DataTableColumnMeta) -> Self {
        self.meta = Some(meta);
        self
    }
}

/// Helper for concise typed `DataTableColumn` construction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DataTableColumnHelper<T: Clone + PartialEq + 'static = String> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: Clone + PartialEq + 'static> DataTableColumnHelper<T> {
    /// Creates a typed column helper.
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }

    /// Creates an accessor column with a default header derived from the id.
    pub fn accessor(
        self,
        id: impl Into<String>,
        accessor: impl Into<Callback<Rc<T>, DataTableValue>>,
    ) -> DataTableColumn<T> {
        let id = id.into();
        DataTableColumn::accessor(id.clone(), id, accessor)
    }

    /// Creates a display-only column with a default header derived from the id.
    pub fn display(self, id: impl Into<String>) -> DataTableColumn<T> {
        let id = id.into();
        DataTableColumn {
            id: id.clone(),
            header: TextOrElement::Text(id.clone()),
            accessor: DataTableColumnAccessor::DisplayOnly,
            width: None,
            min_width: None,
            max_width: None,
            cell: None,
            sortable: None,
            filter: None,
            searchable: false,
            hideable: true,
            resizable: false,
            pinnable: false,
            meta: None,
        }
    }
}

/// Render context for deriving a row id.
#[derive(Clone, PartialEq)]
pub struct DataTableRowIdentityContext<T: Clone + PartialEq + 'static = String> {
    /// Row item.
    pub item: Rc<T>,
    /// Index in the supplied `items` vector.
    pub source_index: usize,
}

/// Renderable row context.
#[derive(Clone, PartialEq)]
pub struct DataTableRow<T: Clone + PartialEq + 'static = String> {
    /// Stable row id.
    pub id: DataTableRowId,
    /// Row item.
    pub item: Rc<T>,
    /// Index in the supplied `items` vector.
    pub source_index: usize,
    /// Index in the rendered rows.
    pub display_index: usize,
    /// Whether the row is selected.
    pub selected: bool,
    /// Whether the row is expanded.
    pub expanded: bool,
}

/// Cell renderer context.
#[derive(Clone, PartialEq)]
pub struct DataTableCellContext<T: Clone + PartialEq + 'static = String> {
    /// Row context.
    pub row: DataTableRow<T>,
    /// Column id.
    pub column_id: String,
    /// Normalized column value.
    pub value: DataTableValue,
    /// Complete canonical state.
    pub state: DataTableState,
    /// State action helpers.
    pub actions: DataTableActions,
}

/// Expanded row renderer context.
#[derive(Clone, PartialEq)]
pub struct DataTableExpandedRowContext<T: Clone + PartialEq + 'static = String> {
    /// Row context.
    pub row: DataTableRow<T>,
    /// Complete canonical state.
    pub state: DataTableState,
    /// State action helpers.
    pub actions: DataTableActions,
}

/// Row click handler context.
#[derive(Clone, PartialEq)]
pub struct DataTableRowClickContext<T: Clone + PartialEq + 'static = String> {
    /// Row context.
    pub row: DataTableRow<T>,
    /// Complete canonical state.
    pub state: DataTableState,
    /// State action helpers.
    pub actions: DataTableActions,
}

/// Header renderer context.
#[derive(Clone, PartialEq)]
pub struct DataTableColumnHeaderContext {
    /// Column id.
    pub column_id: String,
    /// Complete canonical state.
    pub state: DataTableState,
    /// Current sorting direction for the column.
    pub sorting: Option<DataTableSortDirection>,
    /// State action helpers.
    pub actions: DataTableActions,
}

/// Table action callbacks exposed to render contexts.
#[derive(Clone, PartialEq)]
pub struct DataTableActions {
    /// Applies a canonical action.
    pub update_state: Callback<DataTableAction, ()>,
    /// Toggles one row's selected state.
    pub toggle_row_selected: Callback<DataTableRowId, ()>,
    /// Toggles one row's expanded state.
    pub toggle_row_expanded: Callback<DataTableRowId, ()>,
    /// Toggles selection for rendered page rows.
    pub toggle_all_page_rows_selected: Callback<(), ()>,
    /// Toggles selection for all rows matching the query.
    pub toggle_all_matching_rows_selected: Callback<(), ()>,
    /// Clears filters and global filter.
    pub reset_filters: Callback<(), ()>,
    /// Resets column visibility, order, pinning, and sizing.
    pub reset_column_state: Callback<(), ()>,
}

/// Row virtualization configuration for [`DataTable`].
///
/// When supplied through [`DataTableProps::virtualization`], the table renders only the rows in (or
/// near) the scroll viewport plus an overscan buffer, reusing the shared virtualizer primitive.
/// Built-in pagination is disabled while virtualization is active because the whole filtered and
/// sorted row set is scrolled through a single bounded viewport.
///
/// Row expansion (`expanded_row`) is not supported together with virtualization; expanded detail
/// rows are ignored while virtualization is active.
#[derive(Clone, Debug, PartialEq)]
pub struct DataTableVirtualization {
    /// Estimated row height in pixels used before a row has been measured. For best scroll
    /// stability set this close to the real row height.
    pub estimated_row_height: u32,
    /// Number of buffer rows rendered above and below the viewport.
    pub overscan: usize,
    /// Optional CSS length bounding the scroll viewport height (for example `"32rem"`). When
    /// `None`, bound the surface height via CSS instead so the body can scroll.
    pub max_height: Option<String>,
}

impl Default for DataTableVirtualization {
    fn default() -> Self {
        Self {
            estimated_row_height: 48,
            overscan: 8,
            max_height: Some("32rem".to_string()),
        }
    }
}

/// Props for the canonical styled `DataTable`.
#[derive(Props, Clone, PartialEq)]
pub struct DataTableProps<T: Clone + PartialEq + 'static = String> {
    /// Items rendered into body rows.
    pub items: Vec<T>,
    /// Column definitions used to render the table header and body cells.
    pub columns: Vec<DataTableColumn<T>>,
    /// Page information for known or unknown total pagination.
    #[props(default)]
    pub page_info: DataTablePageInfo,
    /// Controlled or uncontrolled canonical state mode.
    #[props(default)]
    pub state_mode: DataTableStateMode,
    /// Called with a complete next state and the action that produced it.
    #[props(default)]
    pub on_state_change: Option<EventHandler<DataTableStateChange>>,
    /// Maps row data to a stable row id.
    pub row_id: Callback<DataTableRowIdentityContext<T>, DataTableRowId>,
    /// When true, pagination only emits state changes and does not slice `items`.
    #[props(default)]
    pub manual_pagination: bool,
    /// When true, sorting only emits state changes and does not sort `items`.
    #[props(default)]
    pub manual_sorting: bool,
    /// When true, filtering only emits state changes and does not filter `items`.
    #[props(default)]
    pub manual_filtering: bool,
    /// Transitional known total count. Prefer `page_info`.
    #[props(default)]
    pub total_count: Option<u64>,
    /// Transitional page size. Prefer `state_mode.state.pagination.page_size`.
    #[props(default)]
    pub page_size: Option<u64>,
    /// Transitional controlled current page. Prefer `state_mode`.
    #[props(default)]
    pub current_page: Option<u32>,
    /// Transitional page callback. Prefer `on_state_change`.
    #[props(default)]
    pub on_page_change: Option<EventHandler<u32>>,
    /// Hides the pagination footer when true.
    #[props(default)]
    pub hide_pagination: bool,
    /// Shows the loading state when true.
    #[props(default)]
    pub loading: bool,
    /// Message rendered in the loading state.
    #[props(default = "Loading table data...".to_string())]
    pub loading_message: String,
    /// Error message rendered when loading failed.
    #[props(default)]
    pub error: Option<String>,
    /// Message rendered when there are no rows.
    #[props(default = "No results found".to_string())]
    pub empty_message: String,
    /// Optional supporting message rendered below `empty_message`.
    #[props(default)]
    pub empty_hint: Option<String>,
    /// Optional controls rendered above the table on the right.
    #[props(default)]
    pub header_controls: Option<Element>,
    /// Optional content rendered at the left edge of the toolbar.
    #[props(default)]
    pub toolbar_left: Option<Element>,
    /// Optional content rendered at the right edge of the toolbar before table settings.
    #[props(default)]
    pub toolbar_right: Option<Element>,
    /// Optional extra content rendered inside the table settings dropdown.
    #[props(default)]
    pub table_settings: Option<Element>,
    /// Optional content rendered in a detail row below expanded rows.
    #[props(default)]
    pub expanded_row: Option<Callback<DataTableExpandedRowContext<T>, Element>>,
    /// Called when a body row is clicked.
    #[props(default)]
    pub on_row_click: Option<Callback<DataTableRowClickContext<T>, ()>>,
    /// Whether to render the built-in row selection checkbox column.
    #[props(default)]
    pub show_selection: bool,
    /// Enables row virtualization. When set, only rows near the viewport render, built-in
    /// pagination is disabled, and the table body scrolls within a bounded surface. Row expansion
    /// is not supported while virtualization is active.
    #[props(default)]
    pub virtualization: Option<DataTableVirtualization>,
    /// Additional attributes applied to the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Canonical styled data table that renders rows from items and column definitions.
#[component]
pub fn DataTable<T: Clone + PartialEq + 'static>(props: DataTableProps<T>) -> Element {
    let initial_state = initial_state_from_props(&props);
    let initial_page_size = initial_state.pagination.page_size;
    let uncontrolled_state = use_signal(|| initial_state);
    let is_controlled = matches!(props.state_mode, DataTableStateMode::Controlled { .. });
    let virtualization = props.virtualization.clone();
    let is_virtualized = virtualization.is_some();
    let virtual_container_id = use_data_table_id();
    let active_state = match &props.state_mode {
        DataTableStateMode::Controlled { state } => merge_transitional_state(state.clone(), &props),
        DataTableStateMode::Uncontrolled { .. } => {
            merge_transitional_state(uncontrolled_state(), &props)
        }
    };
    let page_info = page_info_from_props(&props);
    let operation_columns = canonicalize_operation_columns(&props.columns);
    let canonical_columns = canonicalize_columns(&props.columns, &active_state);
    let rows = derive_rows(
        &props.items,
        &props.row_id,
        &canonical_columns,
        &active_state,
        DataTableManualModes {
            // Virtualization scrolls the whole row set, so the built-in page slice is skipped.
            pagination: props.manual_pagination || is_virtualized,
            sorting: props.manual_sorting,
            filtering: props.manual_filtering,
        },
    );
    let toolbar_model = DataTableToolbarModel::new(
        &operation_columns,
        &canonical_columns,
        &props.columns,
        props.error.as_ref(),
        props.loading,
        rows.len(),
    );
    let page_targets = page_targets(&active_state.pagination, page_info);
    let show_pagination = !props.hide_pagination && !is_virtualized && page_targets.show;
    let page_size_options = page_size_options(active_state.pagination.page_size);
    let mut page_size_value = use_signal(|| Some(initial_page_size));
    use_effect({
        let page_size = active_state.pagination.page_size;
        move || {
            if *page_size_value.peek() != Some(page_size) {
                page_size_value.set(Some(page_size));
            }
        }
    });

    let emit_action = Callback::new({
        let on_state_change = props.on_state_change;
        let on_page_change = props.on_page_change;
        let mut uncontrolled_state = uncontrolled_state;
        let active_state = active_state.clone();
        move |action: DataTableAction| {
            let base_state = active_state.clone();
            let next_state = apply_action(&base_state, action.clone());
            if !is_controlled {
                uncontrolled_state.set(next_state.clone());
            }
            if let DataTableAction::SetPage { page } = action {
                if let Some(on_page_change) = on_page_change {
                    on_page_change.call(page);
                }
            }
            if let Some(on_state_change) = on_state_change {
                on_state_change.call(DataTableStateChange { next_state, action });
            }
        }
    });

    let actions = data_table_actions(
        emit_action,
        active_state.clone(),
        rows.iter().map(|row| row.id.clone()).collect(),
    );
    let body_model = DataTableBodyModel::new(
        &rows,
        &canonical_columns,
        &active_state,
        props.loading,
        props.error.as_ref(),
        &props.loading_message,
        &props.empty_message,
        props.empty_hint.as_deref(),
        // Row expansion is unsupported while virtualized; dropping it here keeps the expansion
        // column, control colspan, and state-row colspan consistent.
        if is_virtualized {
            None
        } else {
            props.expanded_row.as_ref()
        },
        props.on_row_click.as_ref(),
        props.show_selection,
    );
    let pagination_model = DataTablePaginationViewModel {
        page_info,
        page_targets,
        page_size_options,
    };

    let surface_style = virtualization
        .as_ref()
        .and_then(|config| config.max_height.clone())
        .map(|max_height| format!("max-height: {max_height};"));
    // Virtualized bodies hold only a window of rows, so expose the full count to assistive tech
    // (header row included) via aria-rowcount/aria-rowindex.
    let aria_rowcount = is_virtualized.then(|| (rows.len() + 1).to_string());

    rsx! {
        div {
            class: Styles::dx_data_table,
            "data-slot": "data-table",
            ..props.attributes,
            if toolbar_model
                .should_render(
                    props.toolbar_left.is_some(),
                    props.toolbar_right.is_some(),
                    props.header_controls.is_some(),
                    props.table_settings.is_some(),
                )
            {
                {
                    render_toolbar(
                        props.toolbar_left,
                        props.toolbar_right,
                        props.header_controls,
                        props.table_settings,
                        toolbar_model.inline_error,
                        &operation_columns,
                        &canonical_columns,
                        &active_state,
                        &actions,
                        &toolbar_model,
                    )
                }
            }
            div {
                class: Styles::dx_data_table_surface,
                "data-slot": "data-table-surface",
                id: is_virtualized.then(|| virtual_container_id.clone()),
                "data-virtualized": is_virtualized.then_some("true"),
                style: surface_style,
                table {
                    class: Styles::dx_data_table_base,
                    "data-slot": "data-table-table",
                    "aria-rowcount": aria_rowcount.clone(),
                    thead {
                        class: Styles::dx_data_table_head,
                        "data-slot": "data-table-header",
                        tr { "aria-rowindex": is_virtualized.then_some("1"),
                            if body_model.show_selection {
                                th {
                                    class: Styles::dx_data_table_selection_head_cell,
                                    scope: "col",
                                    Checkbox {
                                        checked: Some(
                                            if body_model.selection.all_page_rows_selected {
                                                CheckboxState::Checked
                                            } else {
                                                CheckboxState::Unchecked
                                            },
                                        ),
                                        disabled: body_model.selection.row_count == 0,
                                        "aria-label": if is_virtualized { "Select all rows" } else { "Select all rows on this page" },
                                        on_checked_change: {
                                            let actions = actions.clone();
                                            move |_| actions.toggle_all_page_rows_selected.call(())
                                        },
                                    }
                                }
                            }
                            if body_model.has_expansion {
                                th {
                                    class: Styles::dx_data_table_expansion_head_cell,
                                    scope: "col",
                                    "aria-label": "Row expansion controls",
                                }
                            }
                            for column in canonical_columns.iter() {
                                th {
                                    class: Styles::dx_data_table_head_cell,
                                    "data-column-key": column.id.as_str(),
                                    "data-align": column_align(column),
                                    width: column_width(column, &active_state).as_deref(),
                                    scope: "col",
                                    aria_sort: aria_sort(column, &active_state).unwrap_or("none"),
                                    {render_header(column, &active_state, &actions)}
                                }
                            }
                        }
                    }
                    if is_virtualized && matches!(body_model.status, DataTableBodyStatus::Rows) {
                        DataTableVirtualBody {
                            container_id: virtual_container_id.clone(),
                            rows: rows.clone(),
                            columns: canonical_columns.clone(),
                            state: active_state.clone(),
                            actions: actions.clone(),
                            config: virtualization.clone().unwrap_or_default(),
                            show_selection: props.show_selection,
                            on_row_click: props.on_row_click,
                        }
                    } else {
                        {render_table_body(&body_model, &actions)}
                    }
                }
            }
            if show_pagination {
                {render_pagination(&pagination_model, &active_state, &actions, page_size_value)}
            }
        }
    }
}

struct DataTableToolbarModel<'a> {
    has_filter_controls: bool,
    has_search_controls: bool,
    has_visibility_controls: bool,
    inline_error: Option<&'a String>,
}

impl<'a> DataTableToolbarModel<'a> {
    fn new<T: Clone + PartialEq + 'static>(
        operation_columns: &[DataTableColumn<T>],
        visible_columns: &[DataTableColumn<T>],
        configured_columns: &[DataTableColumn<T>],
        error: Option<&'a String>,
        loading: bool,
        row_count: usize,
    ) -> Self {
        Self {
            has_filter_controls: operation_columns
                .iter()
                .any(|column| column.filter.is_some()),
            has_search_controls: visible_columns.iter().any(|column| column.searchable),
            has_visibility_controls: configured_columns.iter().any(|column| column.hideable),
            inline_error: error.filter(|_| !loading && row_count > 0),
        }
    }

    fn should_render(
        &self,
        has_toolbar_left: bool,
        has_toolbar_right: bool,
        has_header_controls: bool,
        has_table_settings: bool,
    ) -> bool {
        has_toolbar_left
            || has_toolbar_right
            || has_header_controls
            || has_table_settings
            || self.inline_error.is_some()
            || self.has_filter_controls
            || self.has_search_controls
            || self.has_visibility_controls
    }
}

struct DataTableSelectionViewModel {
    row_count: usize,
    all_page_rows_selected: bool,
}

struct DataTableBodyModel<'a, T: Clone + PartialEq + 'static> {
    rows: &'a [DataTableDerivedRow<T>],
    columns: &'a [DataTableColumn<T>],
    state: &'a DataTableState,
    status: DataTableBodyStatus<'a>,
    selection: DataTableSelectionViewModel,
    has_expansion: bool,
    show_selection: bool,
    control_colspan: usize,
    expanded_row: Option<&'a Callback<DataTableExpandedRowContext<T>, Element>>,
    on_row_click: Option<&'a Callback<DataTableRowClickContext<T>, ()>>,
}

impl<'a, T: Clone + PartialEq + 'static> DataTableBodyModel<'a, T> {
    #[allow(clippy::too_many_arguments)]
    fn new(
        rows: &'a [DataTableDerivedRow<T>],
        columns: &'a [DataTableColumn<T>],
        state: &'a DataTableState,
        loading: bool,
        error: Option<&'a String>,
        loading_message: &'a str,
        empty_message: &'a str,
        empty_hint: Option<&'a str>,
        expanded_row: Option<&'a Callback<DataTableExpandedRowContext<T>, Element>>,
        on_row_click: Option<&'a Callback<DataTableRowClickContext<T>, ()>>,
        show_selection: bool,
    ) -> Self {
        let row_count = rows.len();
        let active_query = query_fingerprint(state);
        let all_page_rows_selected = row_count > 0
            && rows
                .iter()
                .all(|row| is_row_selected(&state.row_selection, &row.id, &active_query));
        let has_expansion = expanded_row.is_some();
        let control_columns = usize::from(show_selection) + usize::from(has_expansion);
        let control_colspan = columns.len().max(1) + control_columns;
        let status = DataTableBodyStatus::from_parts(
            loading,
            error,
            row_count,
            loading_message,
            empty_message,
            empty_hint,
        );

        Self {
            rows,
            columns,
            state,
            status,
            selection: DataTableSelectionViewModel {
                row_count,
                all_page_rows_selected,
            },
            has_expansion,
            show_selection,
            control_colspan,
            expanded_row,
            on_row_click,
        }
    }
}

enum DataTableBodyStatus<'a> {
    Loading {
        message: &'a str,
    },
    Error {
        message: &'a str,
    },
    Empty {
        message: &'a str,
        hint: Option<&'a str>,
    },
    Rows,
}

impl<'a> DataTableBodyStatus<'a> {
    fn from_parts(
        loading: bool,
        error: Option<&'a String>,
        row_count: usize,
        loading_message: &'a str,
        empty_message: &'a str,
        empty_hint: Option<&'a str>,
    ) -> Self {
        if loading {
            Self::Loading {
                message: loading_message,
            }
        } else if let Some(error) = error.filter(|_| row_count == 0) {
            Self::Error {
                message: error.as_str(),
            }
        } else if row_count == 0 {
            Self::Empty {
                message: empty_message,
                hint: empty_hint,
            }
        } else {
            Self::Rows
        }
    }
}

struct DataTablePaginationViewModel {
    page_info: DataTablePageInfo,
    page_targets: PageTargets,
    page_size_options: Vec<u64>,
}

fn initial_state_from_props<T: Clone + PartialEq + 'static>(
    props: &DataTableProps<T>,
) -> DataTableState {
    merge_transitional_pagination_state(
        match &props.state_mode {
            DataTableStateMode::Controlled { state } => state.clone(),
            DataTableStateMode::Uncontrolled {
                default_state: Some(state),
            } => state.clone(),
            DataTableStateMode::Uncontrolled {
                default_state: None,
            } => DataTableState::default(),
        },
        props.page_size,
        props.current_page,
    )
}

fn merge_transitional_state<T: Clone + PartialEq + 'static>(
    state: DataTableState,
    props: &DataTableProps<T>,
) -> DataTableState {
    merge_transitional_pagination_state(state, props.page_size, props.current_page)
}

fn merge_transitional_pagination_state(
    mut state: DataTableState,
    page_size: Option<u64>,
    current_page: Option<u32>,
) -> DataTableState {
    if let Some(page_size) = page_size {
        state.pagination.page_size = page_size.max(1);
    }
    if let Some(current_page) = current_page {
        state.pagination.page = current_page.max(1);
    }
    state
}

fn page_info_from_props<T: Clone + PartialEq + 'static>(
    props: &DataTableProps<T>,
) -> DataTablePageInfo {
    if let Some(total_count) = props.total_count {
        DataTablePageInfo::known_total(total_count)
    } else {
        props.page_info
    }
}

fn canonicalize_columns<T: Clone + PartialEq + 'static>(
    columns: &[DataTableColumn<T>],
    state: &DataTableState,
) -> Vec<DataTableColumn<T>> {
    let unique = canonicalize_operation_columns(columns);
    let mut hidden = HashSet::new();
    for entry in &state.column_visibility {
        if !entry.visible {
            hidden.insert(entry.column.as_str());
        } else {
            hidden.remove(entry.column.as_str());
        }
    }

    let mut remaining = unique
        .into_iter()
        .filter(|column| !hidden.contains(column.id.as_str()))
        .collect::<Vec<_>>();
    let mut ordered = Vec::new();
    for id in &state.column_order {
        if let Some(index) = remaining.iter().position(|column| column.id == *id) {
            ordered.push(remaining.remove(index));
        }
    }
    ordered.extend(remaining);

    let pinning = canonicalize_pinning(&state.column_pinning);
    let mut left_pinned = Vec::new();
    for id in &pinning.left {
        if let Some(index) = ordered.iter().position(|column| column.id == *id) {
            left_pinned.push(ordered.remove(index));
        }
    }
    let mut right_pinned = Vec::new();
    for id in &pinning.right {
        if let Some(index) = ordered.iter().position(|column| column.id == *id) {
            right_pinned.push(ordered.remove(index));
        }
    }
    left_pinned.extend(ordered);
    left_pinned.extend(right_pinned);
    left_pinned
}

fn canonicalize_operation_columns<T: Clone + PartialEq + 'static>(
    columns: &[DataTableColumn<T>],
) -> Vec<DataTableColumn<T>> {
    let mut unique = Vec::new();
    let mut known = HashSet::new();
    for column in columns {
        let is_new = known.insert(column.id.clone());
        debug_assert!(is_new, "duplicate DataTable column id: {}", column.id);
        if !is_new {
            continue;
        }
        unique.push(column.clone());
    }
    unique
}

fn canonicalize_pinning(pinning: &DataTableColumnPinningState) -> DataTableColumnPinningState {
    let mut seen = HashSet::new();
    let left = pinning
        .left
        .iter()
        .filter(|id| seen.insert((*id).clone()))
        .cloned()
        .collect::<Vec<_>>();
    let right = pinning
        .right
        .iter()
        .filter(|id| seen.insert((*id).clone()))
        .cloned()
        .collect::<Vec<_>>();
    DataTableColumnPinningState { left, right }
}

fn derive_rows<T: Clone + PartialEq + 'static>(
    items: &[T],
    row_id: &Callback<DataTableRowIdentityContext<T>, DataTableRowId>,
    columns: &[DataTableColumn<T>],
    state: &DataTableState,
    manual_modes: DataTableManualModes,
) -> Vec<DataTableDerivedRow<T>> {
    let rows = build_base_rows(items, row_id, state);
    let rows = cache_row_values(rows, columns);
    let rows = apply_row_model_stages(rows, columns, state, manual_modes);
    assign_display_indexes(rows)
}

#[derive(Clone, PartialEq)]
struct DataTableDerivedRow<T: Clone + PartialEq + 'static = String> {
    row: DataTableRow<T>,
    values: Vec<DataTableValue>,
}

impl<T: Clone + PartialEq + 'static> std::ops::Deref for DataTableDerivedRow<T> {
    type Target = DataTableRow<T>;

    fn deref(&self) -> &Self::Target {
        &self.row
    }
}

fn build_base_rows<T: Clone + PartialEq + 'static>(
    items: &[T],
    row_id: &Callback<DataTableRowIdentityContext<T>, DataTableRowId>,
    state: &DataTableState,
) -> Vec<DataTableRow<T>> {
    let expanded = state.expanded_rows.iter().collect::<HashSet<_>>();
    let mut ids = HashSet::new();
    items
        .iter()
        .enumerate()
        .filter_map(|(source_index, item)| {
            let item = Rc::new(item.clone());
            let id = row_id.call(DataTableRowIdentityContext {
                item: item.clone(),
                source_index,
            });
            // Duplicate row ids are invalid input. Debug builds fail fast; release builds keep
            // the first row for deterministic selection and expansion semantics.
            let is_new = ids.insert(id.clone());
            debug_assert!(is_new, "duplicate DataTable row id: {id}");
            if !is_new {
                return None;
            }
            let selected = is_row_selected(&state.row_selection, &id, &query_fingerprint(state));
            let expanded = expanded.contains(&id);
            Some(DataTableRow {
                id,
                item,
                source_index,
                display_index: source_index,
                selected,
                expanded,
            })
        })
        .collect()
}

fn cache_row_values<T: Clone + PartialEq + 'static>(
    rows: Vec<DataTableRow<T>>,
    columns: &[DataTableColumn<T>],
) -> Vec<DataTableDerivedRow<T>> {
    rows.into_iter()
        .map(|row| {
            let values = columns
                .iter()
                .map(|column| column_value(column, row.item.clone()))
                .collect();
            DataTableDerivedRow { row, values }
        })
        .collect()
}

fn apply_row_model_stages<T: Clone + PartialEq + 'static>(
    mut rows: Vec<DataTableDerivedRow<T>>,
    columns: &[DataTableColumn<T>],
    state: &DataTableState,
    manual_modes: DataTableManualModes,
) -> Vec<DataTableDerivedRow<T>> {
    if !manual_modes.filtering {
        rows = filter_rows(rows, columns, state);
    }
    if !manual_modes.sorting {
        sort_rows(&mut rows, columns, state);
    }
    if !manual_modes.pagination {
        rows = paginate_rows(rows, &state.pagination);
    }
    rows
}

fn assign_display_indexes<T: Clone + PartialEq + 'static>(
    mut rows: Vec<DataTableDerivedRow<T>>,
) -> Vec<DataTableDerivedRow<T>> {
    for (display_index, row) in rows.iter_mut().enumerate() {
        row.row.display_index = display_index;
    }
    rows
}

fn is_row_selected(
    selection: &DataTableRowSelectionState,
    row_id: &str,
    active_query: &str,
) -> bool {
    match selection {
        DataTableRowSelectionState::Explicit { rows } => rows.iter().any(|id| id == row_id),
        DataTableRowSelectionState::AllMatching { query, except } => {
            query == active_query && !except.iter().any(|id| id == row_id)
        }
    }
}

#[derive(Clone, Copy)]
struct DataTableManualModes {
    pagination: bool,
    sorting: bool,
    filtering: bool,
}

fn filter_rows<T: Clone + PartialEq + 'static>(
    rows: Vec<DataTableDerivedRow<T>>,
    columns: &[DataTableColumn<T>],
    state: &DataTableState,
) -> Vec<DataTableDerivedRow<T>> {
    if state.filters.is_empty()
        && state
            .global_filter
            .as_deref()
            .unwrap_or_default()
            .is_empty()
    {
        return rows;
    }

    rows.into_iter()
        .filter(|row| {
            let column_filters_match = state.filters.iter().all(|filter| {
                columns
                    .iter()
                    .find(|column| column.id == filter.column)
                    .map(|column| filter_matches(column, row, columns, &filter.value))
                    .unwrap_or(true)
            });
            let global_filter_matches = state
                .global_filter
                .as_ref()
                .filter(|value| !value.is_empty())
                .map(|global_filter| {
                    let needle = global_filter.to_lowercase();
                    columns
                        .iter()
                        .filter(|column| column.searchable)
                        .any(|column| {
                            cached_column_value(column, row, columns)
                                .to_string()
                                .to_lowercase()
                                .contains(&needle)
                        })
                })
                .unwrap_or(true);
            column_filters_match && global_filter_matches
        })
        .collect()
}

fn filter_matches<T: Clone + PartialEq + 'static>(
    column: &DataTableColumn<T>,
    row: &DataTableDerivedRow<T>,
    columns: &[DataTableColumn<T>],
    filter: &DataTableFilterValue,
) -> bool {
    let value = cached_column_value(column, row, columns);
    match (&column.filter, filter) {
        (Some(DataTableColumnFilter::Custom(predicate)), filter) => {
            predicate.call(DataTableFilterPredicateContext {
                row: row.row.clone(),
                column_id: column.id.clone(),
                value,
                filter: filter.clone(),
            })
        }
        (_, DataTableFilterValue::Text(filter)) => value
            .to_string()
            .to_lowercase()
            .contains(&filter.to_lowercase()),
        (_, DataTableFilterValue::Option(filter)) => value.to_string() == *filter,
        (_, DataTableFilterValue::Multiple(filters)) => {
            let value = value.to_string();
            filters.iter().any(|filter| *filter == value)
        }
        (_, DataTableFilterValue::Boolean(filter)) => {
            matches!(value, DataTableValue::Boolean(value) if value == *filter)
        }
        (_, DataTableFilterValue::Range { min, max }) => {
            let value = value.to_string();
            min.as_ref()
                .map(|min| value.as_str() >= min.as_str())
                .unwrap_or(true)
                && max
                    .as_ref()
                    .map(|max| value.as_str() <= max.as_str())
                    .unwrap_or(true)
        }
    }
}

fn sort_rows<T: Clone + PartialEq + 'static>(
    rows: &mut [DataTableDerivedRow<T>],
    columns: &[DataTableColumn<T>],
    state: &DataTableState,
) {
    let sorts = state
        .sorting
        .iter()
        .filter_map(|sort| {
            columns
                .iter()
                .find(|column| column.id == sort.column)
                .map(|column| (sort, column))
        })
        .collect::<Vec<_>>();
    if sorts.is_empty() {
        return;
    }

    rows.sort_by(|left, right| {
        for (sort, column) in &sorts {
            let ordering = match &column.sortable {
                Some(DataTableColumnSorting::Custom(compare)) => {
                    compare.call(DataTableSortCompareContext {
                        left: left.row.clone(),
                        right: right.row.clone(),
                        direction: sort.direction,
                    })
                }
                Some(DataTableColumnSorting::BuiltIn) => compare_values(
                    &cached_column_value(column, left, columns),
                    &cached_column_value(column, right, columns),
                    sort.direction,
                ),
                None => Ordering::Equal,
            };
            if ordering != Ordering::Equal {
                return ordering;
            }
        }
        left.source_index.cmp(&right.source_index)
    });
}

fn compare_values(
    left: &DataTableValue,
    right: &DataTableValue,
    direction: DataTableSortDirection,
) -> Ordering {
    let ordering = match (left, right) {
        (DataTableValue::Number(left), DataTableValue::Number(right)) => {
            compare_numbers(*left, *right)
        }
        (DataTableValue::Boolean(left), DataTableValue::Boolean(right)) => left.cmp(right),
        (DataTableValue::Empty, DataTableValue::Empty) => Ordering::Equal,
        (DataTableValue::Empty, _) => Ordering::Greater,
        (_, DataTableValue::Empty) => Ordering::Less,
        _ => left.to_string().cmp(&right.to_string()),
    };
    match direction {
        DataTableSortDirection::Ascending => ordering,
        DataTableSortDirection::Descending => ordering.reverse(),
    }
}

fn compare_numbers(left: f64, right: f64) -> Ordering {
    match (left.is_nan(), right.is_nan()) {
        (true, true) => Ordering::Equal,
        (true, false) => Ordering::Greater,
        (false, true) => Ordering::Less,
        (false, false) => left.partial_cmp(&right).unwrap_or(Ordering::Equal),
    }
}

fn paginate_rows<T: Clone + PartialEq + 'static>(
    rows: Vec<DataTableDerivedRow<T>>,
    pagination: &DataTablePaginationState,
) -> Vec<DataTableDerivedRow<T>> {
    let page_size = pagination.page_size.max(1) as usize;
    let page = pagination.page.max(1) as usize;
    let start = page.saturating_sub(1).saturating_mul(page_size);
    rows.into_iter().skip(start).take(page_size).collect()
}

fn cached_column_value<T: Clone + PartialEq + 'static>(
    column: &DataTableColumn<T>,
    row: &DataTableDerivedRow<T>,
    columns: &[DataTableColumn<T>],
) -> DataTableValue {
    columns
        .iter()
        .position(|candidate| candidate.id == column.id)
        .and_then(|index| row.values.get(index))
        .cloned()
        .unwrap_or_else(|| column_value(column, row.item.clone()))
}

fn render_table_body<T: Clone + PartialEq + 'static>(
    model: &DataTableBodyModel<T>,
    actions: &DataTableActions,
) -> Element {
    rsx! {
        tbody { "data-slot": "data-table-body",
            match &model.status {
                DataTableBodyStatus::Loading { message } => {
                    render_loading_row(model.control_colspan, message)
                }
                DataTableBodyStatus::Error { message } => {
                    render_error_row(model.control_colspan, message)
                }
                DataTableBodyStatus::Empty { message, hint } => {
                    render_empty_row(model.control_colspan, message, *hint)
                }
                DataTableBodyStatus::Rows => render_data_rows(model, actions),
            }
        }
    }
}

fn render_loading_row(colspan: usize, message: &str) -> Element {
    rsx! {
        tr {
            td { class: Styles::dx_data_table_state_cell, colspan: "{colspan}",
                div {
                    class: Styles::dx_data_table_loading,
                    aria_busy: "true",
                    aria_live: "polite",
                    span { "{message}" }
                    Skeleton {}
                    Skeleton {}
                    Skeleton {}
                }
            }
        }
    }
}

fn render_error_row(colspan: usize, message: &str) -> Element {
    rsx! {
        tr {
            td { class: Styles::dx_data_table_state_cell, colspan: "{colspan}",
                div { class: Styles::dx_data_table_error, role: "alert", "{message}" }
            }
        }
    }
}

fn render_empty_row(colspan: usize, message: &str, hint: Option<&str>) -> Element {
    rsx! {
        tr {
            td { class: Styles::dx_data_table_state_cell, colspan: "{colspan}",
                div { class: Styles::dx_data_table_empty,
                    p { class: Styles::dx_data_table_empty_title, "{message}" }
                    if let Some(hint) = hint {
                        p { class: Styles::dx_data_table_empty_hint, "{hint}" }
                    }
                }
            }
        }
    }
}

fn render_data_rows<T: Clone + PartialEq + 'static>(
    model: &DataTableBodyModel<T>,
    actions: &DataTableActions,
) -> Element {
    rsx! {
        for row in model.rows.iter() {
            {render_data_row(model, row, actions)}
            if row.expanded {
                {render_expanded_row(model, row, actions)}
            }
        }
    }
}

fn render_data_row<T: Clone + PartialEq + 'static>(
    model: &DataTableBodyModel<T>,
    row: &DataTableDerivedRow<T>,
    actions: &DataTableActions,
) -> Element {
    rsx! {
        tr {
            class: Styles::dx_data_table_row,
            "data-row-id": row.id.as_str(),
            "aria-selected": row.selected.to_string(),
            "data-clickable": model.on_row_click.is_some().then_some("true"),
            onclick: {
                let actions = actions.clone();
                let on_row_click = model.on_row_click.cloned();
                let row = row.row.clone();
                let state = model.state.clone();
                move |_| {
                    if let Some(on_row_click) = on_row_click {
                        on_row_click.call(DataTableRowClickContext {
                            row: row.clone(),
                            state: state.clone(),
                            actions: actions.clone(),
                        });
                    }
                }
            },
            if model.show_selection {
                td {
                    class: Styles::dx_data_table_selection_cell,
                    onclick: move |event: MouseEvent| event.stop_propagation(),
                    Checkbox {
                        checked: Some(if row.selected { CheckboxState::Checked } else { CheckboxState::Unchecked }),
                        "aria-label": "Select row",
                        on_checked_change: {
                            let actions = actions.clone();
                            let row_id = row.id.clone();
                            move |_| actions.toggle_row_selected.call(row_id.clone())
                        },
                    }
                }
            }
            if model.has_expansion {
                td {
                    class: Styles::dx_data_table_expansion_cell,
                    onclick: move |event: MouseEvent| event.stop_propagation(),
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        class: Styles::dx_data_table_icon_button,
                        r#type: "button",
                        "aria-expanded": row.expanded.to_string(),
                        "aria-label": "Toggle row expansion",
                        onclick: {
                            let actions = actions.clone();
                            let row_id = row.id.clone();
                            move |_| actions.toggle_row_expanded.call(row_id.clone())
                        },
                        if row.expanded {
                            "-"
                        } else {
                            "+"
                        }
                    }
                }
            }
            for column in model.columns.iter() {
                td {
                    class: Styles::dx_data_table_cell,
                    "data-column-key": column.id.as_str(),
                    "data-align": column_align(column),
                    {render_cell(column, row, model.columns, model.state, actions)}
                }
            }
        }
    }
}

fn render_expanded_row<T: Clone + PartialEq + 'static>(
    model: &DataTableBodyModel<T>,
    row: &DataTableDerivedRow<T>,
    actions: &DataTableActions,
) -> Element {
    let Some(expanded_row) = model.expanded_row else {
        return rsx! {};
    };

    rsx! {
        tr {
            class: Styles::dx_data_table_expanded_row,
            "data-row-id": row.id.as_str(),
            "data-expanded-row": "true",
            td {
                class: Styles::dx_data_table_expanded_cell,
                colspan: "{model.control_colspan}",
                {
                    expanded_row
                        .call(DataTableExpandedRowContext {
                            row: row.row.clone(),
                            state: model.state.clone(),
                            actions: actions.clone(),
                        })
                }
            }
        }
    }
}

/// Generate a process-unique id for the virtualized scroll container.
fn use_data_table_id() -> String {
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

    use_hook(|| {
        let id = NEXT_ID.fetch_add(1, AtomicOrdering::Relaxed);
        format!("dx-data-table-{id}")
    })
}

/// Props for the internal virtualized table body component.
#[derive(Props, Clone, PartialEq)]
struct DataTableVirtualBodyProps<T: Clone + PartialEq + 'static> {
    /// Id of the scroll surface that owns `scrollTop`/`clientHeight`.
    container_id: String,
    /// Full filtered and sorted row set scrolled through the viewport.
    rows: Vec<DataTableDerivedRow<T>>,
    /// Visible columns rendered for each row.
    columns: Vec<DataTableColumn<T>>,
    /// Active canonical state forwarded to cell renderers.
    state: DataTableState,
    /// Table action callbacks forwarded to cell renderers.
    actions: DataTableActions,
    /// Virtualization configuration.
    config: DataTableVirtualization,
    /// Whether to render the built-in row selection checkbox column.
    show_selection: bool,
    /// Called when a body row is clicked.
    on_row_click: Option<Callback<DataTableRowClickContext<T>, ()>>,
}

/// Virtualized `<tbody>` that renders only rows near the viewport.
///
/// Reuses the shared virtualizer primitive (`dioxus_primitives::r#virtual`) for range,
/// measurement, and scrollbar-stability logic. Table layout is preserved by rendering spacer rows
/// with the offset height above and below the rendered window instead of transforming rows.
#[component]
fn DataTableVirtualBody<T: Clone + PartialEq + 'static>(
    props: DataTableVirtualBodyProps<T>,
) -> Element {
    let DataTableVirtualBodyProps {
        container_id,
        rows,
        columns,
        state,
        actions,
        config,
        show_selection,
        on_row_click,
    } = props;

    // Expansion is unsupported while virtualized.
    let colspan = columns.len().max(1) + usize::from(show_selection);
    let row_count = rows.len();
    let estimated = config.estimated_row_height.max(1);
    let overscan = config.overscan;

    let state_store: Store<VirtualizerState> = use_store(VirtualizerState::new);

    // Mirror the row count into a signal so the measurements memo recomputes when the row set
    // changes (rows arrive as a plain prop rather than a reactive signal).
    let mut count_signal = use_signal(|| row_count);
    use_effect(use_reactive!(|row_count| {
        if *count_signal.peek() != row_count {
            count_signal.set(row_count);
        }
    }));

    // Measurements recompute when the row count or measured sizes change.
    let measurements: Memo<Vec<VirtualItem>> = use_memo(move || {
        let count = count_signal();
        let isc = state_store.item_size_cache();
        let cache = isc.read();
        let estimate = move |_: usize| estimated;
        compute_measurements(count, &cache, Some(&estimate as &dyn Fn(usize) -> u32))
    });

    // Bridge scroll/viewport changes from the surface container into the virtualizer store.
    use_effect({
        let container_id = container_id.clone();
        move || {
            let script = r#"
                const container = document.getElementById(await dioxus.recv());
                if (!container) return;

                let scrollEndTimer = null;
                let lastOffset = null;
                let lastViewport = null;
                let lastIsScrolling = null;

                function publish(isScrolling) {
                    const scroll = Math.round(container.scrollTop);
                    const viewport = Math.min(container.clientHeight, window.innerHeight) || 600;
                    if (
                        scroll === lastOffset &&
                        viewport === lastViewport &&
                        isScrolling === lastIsScrolling
                    ) {
                        return;
                    }
                    lastOffset = scroll;
                    lastViewport = viewport;
                    lastIsScrolling = isScrolling;
                    dioxus.send([scroll, viewport, isScrolling]);
                }

                function onScroll() {
                    if (scrollEndTimer !== null) {
                        clearTimeout(scrollEndTimer);
                    }
                    publish(true);
                    scrollEndTimer = setTimeout(() => {
                        scrollEndTimer = null;
                        publish(false);
                    }, 600);
                }

                publish(false);

                container.addEventListener("scroll", onScroll, { passive: true });
                window.addEventListener("resize", () => publish(false), { passive: true });

                await dioxus.recv();
                if (scrollEndTimer !== null) clearTimeout(scrollEndTimer);
                container.removeEventListener("scroll", onScroll);
            "#;
            let mut eval = document::eval(script);
            let _ = eval.send(container_id.clone());

            let container_id = container_id.clone();
            spawn(async move {
                while let Ok((offset, viewport, is_scrolling)) =
                    eval.recv::<(u32, u32, bool)>().await
                {
                    let correction = {
                        let m = measurements.peek();
                        set_scroll_offset(&state_store, &m, offset, is_scrolling)
                    };
                    set_viewport_size(&state_store, viewport);

                    if let Some(delta) = correction {
                        let new_scroll = (offset as i32 + delta).max(0) as u32;
                        sync_table_scroll(container_id.clone(), new_scroll).await;
                        state_store.scroll_offset().set(new_scroll);
                    }
                }
            });
        }
    });

    // Resize callback measures each rendered row and feeds the virtualizer size cache.
    let on_row_resize = use_callback({
        let container_id = container_id.clone();
        move |(index, measured): (usize, u32)| {
            let m = measurements.peek();
            let adjustment = resize_item(&state_store, &m, index, measured);
            drop(m);

            if let Some(delta) = adjustment {
                let current = *state_store.scroll_offset().peek();
                let new_scroll = (current as i32 + delta).max(0) as u32;
                let container_id = container_id.clone();
                spawn(async move {
                    sync_table_scroll(container_id, new_scroll).await;
                });
            }
        }
    });

    let m = measurements.read();
    let virtual_items = get_virtual_items(&state_store, &m, overscan);
    let total_height = get_total_size(&state_store, &m);
    let top_offset = virtual_items.first().map(VirtualItem::start).unwrap_or(0);
    let bottom_offset =
        total_height.saturating_sub(virtual_items.last().map(VirtualItem::end).unwrap_or(0));

    rsx! {
        tbody { "data-slot": "data-table-body", "data-virtualized": "true",
            if top_offset > 0 {
                tr { "aria-hidden": "true", "data-virtual-spacer": "top",
                    td {
                        colspan: "{colspan}",
                        style: "height: {top_offset}px; padding: 0; border: 0;",
                    }
                }
            }
            for item in virtual_items.iter() {
                if let Some(row) = rows.get(item.index()) {
                    {
                        render_virtual_data_row(
                            &columns,
                            &state,
                            &actions,
                            row,
                            on_row_resize,
                            show_selection,
                            on_row_click.clone(),
                        )
                    }
                }
            }
            if bottom_offset > 0 {
                tr { "aria-hidden": "true", "data-virtual-spacer": "bottom",
                    td {
                        colspan: "{colspan}",
                        style: "height: {bottom_offset}px; padding: 0; border: 0;",
                    }
                }
            }
        }
    }
}

/// Render a single virtualized data row with measurement and selection wired up.
fn render_virtual_data_row<T: Clone + PartialEq + 'static>(
    columns: &[DataTableColumn<T>],
    state: &DataTableState,
    actions: &DataTableActions,
    row: &DataTableDerivedRow<T>,
    on_resize: Callback<(usize, u32)>,
    show_selection: bool,
    on_row_click: Option<Callback<DataTableRowClickContext<T>, ()>>,
) -> Element {
    let index = row.display_index;
    rsx! {
        tr {
            key: "{row.id}",
            class: Styles::dx_data_table_row,
            "data-row-id": row.id.as_str(),
            "data-virtual-index": "{index}",
            // Header occupies row 1, so data rows start at row 2 in the full (unvirtualized) grid.
            "aria-rowindex": "{index + 2}",
            "aria-selected": row.selected.to_string(),
            "data-clickable": on_row_click.is_some().then_some("true"),
            onresize: move |event: Event<ResizeData>| {
                let height = event
                    .data()
                    .get_border_box_size()
                    .map(|size| size.height)
                    .unwrap_or(0.0);
                let measured = height.max(1.0).round() as u32;
                on_resize.call((index, measured));
            },
            onclick: {
                let actions = actions.clone();
                let row = row.row.clone();
                let state = state.clone();
                move |_| {
                    if let Some(on_row_click) = on_row_click {
                        on_row_click.call(DataTableRowClickContext {
                            row: row.clone(),
                            state: state.clone(),
                            actions: actions.clone(),
                        });
                    }
                }
            },
            if show_selection {
                td {
                    class: Styles::dx_data_table_selection_cell,
                    onclick: move |event: MouseEvent| event.stop_propagation(),
                    Checkbox {
                        checked: Some(if row.selected { CheckboxState::Checked } else { CheckboxState::Unchecked }),
                        "aria-label": "Select row",
                        on_checked_change: {
                            let actions = actions.clone();
                            let row_id = row.id.clone();
                            move |_| actions.toggle_row_selected.call(row_id.clone())
                        },
                    }
                }
            }
            for column in columns.iter() {
                td {
                    class: Styles::dx_data_table_cell,
                    "data-column-key": column.id.as_str(),
                    "data-align": column_align(column),
                    {render_cell(column, row, columns, state, actions)}
                }
            }
        }
    }
}

/// Imperatively set the scroll container's `scrollTop` (used to apply scroll corrections).
async fn sync_table_scroll(container_id: String, scroll_top: u32) {
    let eval = document::eval(
        r#"
        const id = await dioxus.recv();
        const targetScroll = await dioxus.recv();
        const container = document.getElementById(id);
        if (container) {
            container.scrollTop = targetScroll;
        }
        "#,
    );
    let _ = eval.send(container_id);
    let _ = eval.send(scroll_top);
}

fn render_pagination(
    model: &DataTablePaginationViewModel,
    state: &DataTableState,
    actions: &DataTableActions,
    mut page_size_value: Signal<Option<u64>>,
) -> Element {
    let first_page = {
        let actions = actions.clone();
        move |_| {
            actions
                .update_state
                .call(DataTableAction::SetPage { page: 1 })
        }
    };
    let previous_page = {
        let actions = actions.clone();
        let previous = model.page_targets.previous;
        move |_| {
            if let Some(page) = previous {
                actions.update_state.call(DataTableAction::SetPage { page });
            }
        }
    };
    let next_page = {
        let actions = actions.clone();
        let next = model.page_targets.next;
        move |_| {
            if let Some(page) = next {
                actions.update_state.call(DataTableAction::SetPage { page });
            }
        }
    };
    let last_page = {
        let actions = actions.clone();
        let last = model.page_targets.last;
        move |_| {
            if let Some(page) = last {
                actions.update_state.call(DataTableAction::SetPage { page });
            }
        }
    };

    rsx! {
        div {
            class: Styles::dx_data_table_pagination,
            "data-slot": "data-table-pagination",
            span { class: Styles::dx_data_table_page_summary,
                {pagination_summary(&state.pagination, model.page_info)}
            }
            div { class: Styles::dx_data_table_page_actions,
                label { class: Styles::dx_data_table_page_size,
                    Select::<u64> {
                        class: Styles::dx_data_table_select,
                        placeholder: state.pagination.page_size.to_string(),
                        value: Some(page_size_value.into()),
                        "aria-label": "Rows per page",
                        on_value_change: {
                            let actions = actions.clone();
                            move |value: Option<u64>| {
                                if let Some(page_size) = value {
                                    page_size_value.set(Some(page_size));
                                    actions
                                        .update_state
                                        .call(DataTableAction::SetPageSize {
                                            page_size,
                                        });
                                }
                            }
                        },
                        for (index , page_size) in model.page_size_options.iter().enumerate() {
                            SelectOption::<u64> {
                                index,
                                value: *page_size,
                                text_value: page_size.to_string(),
                                "{page_size}"
                            }
                        }
                    }
                }
                Pagination { class: Styles::dx_data_table_page_nav,
                    PaginationContent {
                        PaginationItem {
                            PaginationFirst {
                                disabled: model.page_targets.first.is_none(),
                                onclick: first_page,
                            }
                        }
                        PaginationItem {
                            PaginationPrevious {
                                disabled: model.page_targets.previous.is_none(),
                                onclick: previous_page,
                            }
                        }
                        PaginationItem {
                            PaginationNext {
                                disabled: model.page_targets.next.is_none(),
                                onclick: next_page,
                            }
                        }
                        PaginationItem {
                            PaginationLast {
                                disabled: model.page_targets.last.is_none(),
                                onclick: last_page,
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_header<T: Clone + PartialEq + 'static>(
    column: &DataTableColumn<T>,
    state: &DataTableState,
    actions: &DataTableActions,
) -> Element {
    let sorting = state
        .sorting
        .iter()
        .find(|sort| sort.column == column.id)
        .map(|sort| sort.direction);
    match &column.header {
        TextOrElement::Text(label) => rsx! {
            {render_label_header(column, label, sorting, state, actions)}
        },
        TextOrElement::Element(el) => el.clone(),
        TextOrElement::Render(cb) => cb.call(DataTableColumnHeaderContext {
            column_id: column.id.clone(),
            state: state.clone(),
            sorting,
            actions: actions.clone(),
        }),
    }
}

fn render_label_header<T: Clone + PartialEq + 'static>(
    column: &DataTableColumn<T>,
    label: &str,
    sorting: Option<DataTableSortDirection>,
    state: &DataTableState,
    actions: &DataTableActions,
) -> Element {
    if column.sortable.is_some() {
        let next_sorting = next_sorting_for_column(state, &column.id);
        let aria_label = match sorting {
            Some(DataTableSortDirection::Ascending) => format!("Sort {label} descending"),
            Some(DataTableSortDirection::Descending) => format!("Clear {label} sorting"),
            None => format!("Sort {label} ascending"),
        };
        rsx! {
            span { class: Styles::dx_data_table_sort_header,
                span { class: Styles::dx_data_table_head_label, "{label}" }
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    class: Styles::dx_data_table_sort_button,
                    r#type: "button",
                    "aria-label": aria_label,
                    onclick: {
                        let actions = actions.clone();
                        move |_| {
                            actions
                                .update_state
                                .call(DataTableAction::SetSorting {
                                    sorting: next_sorting.clone(),
                                });
                        }
                    },
                    span {
                        class: Styles::dx_data_table_sort_indicator,
                        aria_hidden: "true",
                        {render_sort_indicator(sorting)}
                    }
                }
            }
        }
    } else {
        rsx! {
            span { class: Styles::dx_data_table_head_label, "{label}" }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn render_toolbar<T: Clone + PartialEq + 'static>(
    toolbar_left: Option<Element>,
    toolbar_right: Option<Element>,
    header_controls: Option<Element>,
    table_settings: Option<Element>,
    inline_error: Option<&String>,
    all_columns: &[DataTableColumn<T>],
    filter_columns: &[DataTableColumn<T>],
    state: &DataTableState,
    actions: &DataTableActions,
    model: &DataTableToolbarModel,
) -> Element {
    rsx! {
        div {
            class: Styles::dx_data_table_toolbar,
            "data-slot": "data-table-toolbar",
            if let Some(toolbar_left) = toolbar_left {
                div {
                    class: Styles::dx_data_table_toolbar_slot,
                    "data-slot": "data-table-toolbar-left",
                    {toolbar_left}
                }
            }
            div { class: Styles::dx_data_table_toolbar_main,
                if model.has_search_controls {
                    {render_global_search(state, actions)}
                }
                if model.has_filter_controls {
                    {render_filter_menu(filter_columns, state, actions)}
                }
                if model.has_filter_controls {
                    {render_active_filters(filter_columns, state, actions)}
                }
            }
            div { class: Styles::dx_data_table_toolbar_aside,
                if let Some(error) = inline_error {
                    div {
                        class: Styles::dx_data_table_inline_error,
                        role: "status",
                        "{error}"
                    }
                }
                if let Some(toolbar_right) = toolbar_right {
                    div {
                        class: Styles::dx_data_table_toolbar_slot,
                        "data-slot": "data-table-toolbar-right",
                        {toolbar_right}
                    }
                }
                if let Some(header_controls) = header_controls {
                    div {
                        class: Styles::dx_data_table_header_controls,
                        "data-slot": "data-table-header-controls",
                        {header_controls}
                    }
                }
                if model.has_visibility_controls || table_settings.is_some() {
                    {
                        render_table_settings(
                            all_columns,
                            state,
                            actions,
                            table_settings,
                            model.has_visibility_controls,
                        )
                    }
                }
            }
        }
    }
}

fn render_global_search(state: &DataTableState, actions: &DataTableActions) -> Element {
    rsx! {
        TextInput {
            class: Styles::dx_data_table_search_input,
            r#type: "search",
            value: state.global_filter.as_deref().unwrap_or_default(),
            placeholder: "Search",
            left_section: rsx! {
                Search { size: "14", "aria-hidden": "true" }
            },
            "aria-label": "Search table",
            oninput: {
                let actions = actions.clone();
                move |event: FormEvent| {
                    let value = event.value();
                    actions
                        .update_state
                        .call(DataTableAction::SetGlobalFilter {
                            value: (!value.is_empty()).then_some(value),
                        });
                }
            },
        }
    }
}

fn render_filter_menu<T: Clone + PartialEq + 'static>(
    columns: &[DataTableColumn<T>],
    state: &DataTableState,
    actions: &DataTableActions,
) -> Element {
    rsx! {
        PopoverRoot { class: Styles::dx_data_table_filter_menu,
            PopoverTrigger { class: "dx--trigger", "aria-label": "Add filter",
                Button { variant: ButtonVariant::Outline,
                    Plus { size: "14", "aria-hidden": "true" }
                    "Filter"
                }
            }
            PopoverContent { class: Styles::dx_data_table_filter_options,
                for column in columns.iter().filter(|column| column.filter.is_some()) {
                    {render_filter_control(column, state, actions)}
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    disabled: state.filters.is_empty() && state.global_filter.is_none(),
                    onclick: {
                        let actions = actions.clone();
                        move |_| actions.reset_filters.call(())
                    },
                    "Reset filters"
                }
            }
        }
    }
}

fn render_active_filters<T: Clone + PartialEq + 'static>(
    columns: &[DataTableColumn<T>],
    state: &DataTableState,
    actions: &DataTableActions,
) -> Element {
    rsx! {
        div {
            class: Styles::dx_data_table_active_filters,
            "aria-label": "Active table filters",
            if let Some(global_filter) = state
                .global_filter
                .as_ref()
                .filter(|value| !value.is_empty())
            {
                {
                    render_filter_chip(
                        "Search".to_string(),
                        global_filter.clone(),
                        {
                            let actions = actions.clone();
                            Callback::new(move |_| {
                                actions
                                    .update_state
                                    .call(DataTableAction::SetGlobalFilter {
                                        value: None,
                                    });
                            })
                        },
                    )
                }
            }
            for filter in state.filters.iter() {
                {
                    render_filter_chip(
                        filter_label(columns, &filter.column),
                        filter_value_label(&filter.value),
                        {
                            let actions = actions.clone();
                            let column_id = filter.column.clone();
                            Callback::new(move |_| {
                                actions
                                    .update_state
                                    .call(DataTableAction::SetFilter {
                                        column: column_id.clone(),
                                        value: None,
                                    });
                            })
                        },
                    )
                }
            }
        }
    }
}

fn render_filter_chip(
    label: String,
    value: String,
    on_remove: Callback<MouseEvent, ()>,
) -> Element {
    rsx! {
        Button {
            class: Styles::dx_data_table_filter_chip,
            variant: ButtonVariant::Outline,
            r#type: "button",
            "aria-label": "Remove {label} filter",
            onclick: move |event| on_remove.call(event),
            span { class: Styles::dx_data_table_filter_chip_label, "{label}" }
            span { class: Styles::dx_data_table_filter_chip_value, "{value}" }
            X { size: "14", "aria-hidden": "true" }
        }
    }
}

fn render_table_settings<T: Clone + PartialEq + 'static>(
    columns: &[DataTableColumn<T>],
    state: &DataTableState,
    actions: &DataTableActions,
    table_settings: Option<Element>,
    has_visibility_controls: bool,
) -> Element {
    rsx! {
        PopoverRoot { class: Styles::dx_data_table_column_menu,
            Tooltip {
                TooltipTrigger {
                    r#as: move |tooltip_attrs: Vec<Attribute>| {
                        rsx! {
                            PopoverTrigger { attributes: tooltip_attrs,
                                Button { variant: ButtonVariant::Outline, "aria-label": "Table settings",
                                    SlidersHorizontal { size: "18", "aria-hidden": "true" }
                                }
                            }
                        }
                    },
                }
                TooltipContent { "Table settings" }
            }
            PopoverContent { class: Styles::dx_data_table_settings,
                if let Some(table_settings) = table_settings {
                    div {
                        class: Styles::dx_data_table_settings_slot,
                        "data-slot": "data-table-settings",
                        {table_settings}
                    }
                }
                if has_visibility_controls {
                    div { class: Styles::dx_data_table_column_options,
                        div { class: Styles::dx_data_table_column_options_header, "Columns" }
                        for column in columns.iter().filter(|column| column.hideable) {
                            label { class: Styles::dx_data_table_option,
                                Checkbox {
                                    // label: column_label(column),
                                    checked: Some(
                                        if is_column_visible(column, state) {
                                            CheckboxState::Checked
                                        } else {
                                            CheckboxState::Unchecked
                                        },
                                    ),
                                    "aria-label": "Toggle column visibility",
                                    on_checked_change: {
                                        let actions = actions.clone();
                                        let column_id = column.id.clone();
                                        move |checked| {
                                            actions
                                                .update_state
                                                .call(DataTableAction::SetColumnVisibility {
                                                    column: column_id.clone(),
                                                    visible: checked == CheckboxState::Checked,
                                                });
                                        }
                                    },
                                }
                                {column_label(column)}
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_filter_control<T: Clone + PartialEq + 'static>(
    column: &DataTableColumn<T>,
    state: &DataTableState,
    actions: &DataTableActions,
) -> Element {
    let label = column_label(column);
    let filter = active_filter(column, state);
    let Some(column_filter) = column.filter.as_ref() else {
        return rsx! {};
    };

    let control = match column_filter {
        DataTableColumnFilter::Text | DataTableColumnFilter::Custom(_) => {
            let value = match filter {
                Some(DataTableFilterValue::Text(value)) => value.clone(),
                _ => String::new(),
            };
            rsx! {
                TextInput {
                    class: Styles::dx_data_table_input,
                    value,
                    "aria-label": "Filter column",
                    oninput: {
                        let actions = actions.clone();
                        let column_id = column.id.clone();
                        move |event: FormEvent| {
                            let value = event.value();
                            actions
                                .update_state
                                .call(DataTableAction::SetFilter {
                                    column: column_id.clone(),
                                    value: (!value.is_empty())
                                        .then_some(DataTableFilterValue::Text(value)),
                                });
                        }
                    },
                }
            }
        }
        DataTableColumnFilter::Select { options } => {
            let value = match filter {
                Some(DataTableFilterValue::Option(value)) => value.clone(),
                _ => String::new(),
            };
            rsx! {
                Select::<String> {
                    class: Styles::dx_data_table_select,
                    value: Some(ReadSignal::new(Signal::new(Some(value)))),
                    "aria-label": "Filter column",
                    on_value_change: {
                        let actions = actions.clone();
                        let column_id = column.id.clone();
                        move |value: Option<String>| {
                            actions
                                .update_state
                                .call(DataTableAction::SetFilter {
                                    column: column_id.clone(),
                                    value: value
                                        .filter(|value| !value.is_empty())
                                        .map(DataTableFilterValue::Option),
                                });
                        }
                    },
                    SelectOption::<String> {
                        index: 0usize,
                        value: String::new(),
                        text_value: "All",
                        "All"
                    }
                    for (index , option) in options.iter().enumerate() {
                        SelectOption::<String> {
                            index: index + 1,
                            value: option.value.clone(),
                            text_value: option.label.clone(),
                            "{option.label}"
                        }
                    }
                }
            }
        }
        DataTableColumnFilter::MultiSelect { options } => {
            let values = match filter {
                Some(DataTableFilterValue::Multiple(values)) => values.clone(),
                _ => Vec::new(),
            };
            let option_labels = options
                .iter()
                .map(|option| (option.value.clone(), option.label.clone()))
                .collect::<Vec<_>>();

            rsx! {
                ComboboxMultiSelect::<String> {
                    values: ReadSignal::new(Signal::new(Some(values))),
                    placeholder: "(All)",
                    render_value: move |value: String| {
                        let label = option_labels
                            .iter()
                            .find(|(option_value, _)| option_value == &value)
                            .map(|(_, label)| label.clone())
                            .unwrap_or(value);
                        rsx! { "{label}" }
                    },
                    on_values_change: {
                        let actions = actions.clone();
                        let column_id = column.id.clone();
                        move |values: Vec<String>| {
                            actions
                                .update_state
                                .call(DataTableAction::SetFilter {
                                    column: column_id.clone(),
                                    value: (!values.is_empty())
                                        .then_some(DataTableFilterValue::Multiple(values)),
                                });
                        }
                    },
                    ComboboxEmpty { "No options found." }
                    for (index , option) in options.iter().enumerate() {
                        ComboboxOption::<String> {
                            index,
                            value: option.value.clone(),
                            text_value: option.label.clone(),
                            "{option.label}"
                        }
                    }
                }
            }
        }
        DataTableColumnFilter::Boolean => {
            let value = match filter {
                Some(DataTableFilterValue::Boolean(true)) => "true",
                Some(DataTableFilterValue::Boolean(false)) => "false",
                _ => "",
            };
            rsx! {
                Select::<String> {
                    class: Styles::dx_data_table_select,
                    value: Some(ReadSignal::new(Signal::new(Some(value.to_string())))),
                    "aria-label": "Filter column",
                    on_value_change: {
                        let actions = actions.clone();
                        let column_id = column.id.clone();
                        move |event_value: Option<String>| {
                            let value = match event_value.as_deref() {
                                Some("true") => Some(DataTableFilterValue::Boolean(true)),
                                Some("false") => Some(DataTableFilterValue::Boolean(false)),
                                _ => None,
                            };
                            actions
                                .update_state
                                .call(DataTableAction::SetFilter {
                                    column: column_id.clone(),
                                    value,
                                });
                        }
                    },
                    SelectOption::<String> {
                        index: 0usize,
                        value: String::new(),
                        text_value: "All",
                        "All"
                    }
                    SelectOption::<String> {
                        index: 1usize,
                        value: "true".to_string(),
                        text_value: "True",
                        "True"
                    }
                    SelectOption::<String> {
                        index: 2usize,
                        value: "false".to_string(),
                        text_value: "False",
                        "False"
                    }
                }
            }
        }
        DataTableColumnFilter::Range { kind } => {
            let (min, max) = match filter {
                Some(DataTableFilterValue::Range { min, max }) => (
                    min.clone().unwrap_or_default(),
                    max.clone().unwrap_or_default(),
                ),
                _ => (String::new(), String::new()),
            };
            let input_type = match kind {
                DataTableRangeFilterKind::Number => "number",
                DataTableRangeFilterKind::Text => "text",
                DataTableRangeFilterKind::DateTime => "datetime-local",
            };
            rsx! {
                div { class: Styles::dx_data_table_range,
                    TextInput {
                        class: Styles::dx_data_table_input,
                        r#type: input_type,
                        value: min.clone(),
                        placeholder: "Min",
                        "aria-label": "Minimum filter value",
                        oninput: {
                            let actions = actions.clone();
                            let column_id = column.id.clone();
                            let max = max.clone();
                            move |event: FormEvent| {
                                actions
                                    .update_state
                                    .call(DataTableAction::SetFilter {
                                        column: column_id.clone(),
                                        value: range_filter_value(event.value(), max.clone()),
                                    });
                            }
                        },
                    }
                    TextInput {
                        class: Styles::dx_data_table_input,
                        r#type: input_type,
                        value: max,
                        placeholder: "Max",
                        "aria-label": "Maximum filter value",
                        oninput: {
                            let actions = actions.clone();
                            let column_id = column.id.clone();
                            let min = min.clone();
                            move |event: FormEvent| {
                                actions
                                    .update_state
                                    .call(DataTableAction::SetFilter {
                                        column: column_id.clone(),
                                        value: range_filter_value(min.clone(), event.value()),
                                    });
                            }
                        },
                    }
                }
            }
        }
    };

    rsx! {
        InputWrapper { class: Styles::dx_data_table_filter, label, {control} }
    }
}

fn render_cell<T: Clone + PartialEq + 'static>(
    column: &DataTableColumn<T>,
    row: &DataTableDerivedRow<T>,
    columns: &[DataTableColumn<T>],
    state: &DataTableState,
    actions: &DataTableActions,
) -> Element {
    let value = cached_column_value(column, row, columns);
    if let Some(cell) = &column.cell {
        cell.call(DataTableCellContext {
            row: row.row.clone(),
            column_id: column.id.clone(),
            value,
            state: state.clone(),
            actions: actions.clone(),
        })
    } else {
        rsx! { "{value}" }
    }
}

fn column_value<T: Clone + PartialEq + 'static>(
    column: &DataTableColumn<T>,
    item: Rc<T>,
) -> DataTableValue {
    match &column.accessor {
        DataTableColumnAccessor::Accessor(accessor) => accessor.call(item),
        DataTableColumnAccessor::DisplayOnly => DataTableValue::Empty,
    }
}

fn column_width<T: Clone + PartialEq + 'static>(
    column: &DataTableColumn<T>,
    state: &DataTableState,
) -> Option<String> {
    for size in state.column_sizing.iter().rev() {
        if size.column == column.id {
            return size.width.map(|width| format!("{width}px"));
        }
    }
    match &column.width {
        Some(DataTableColumnWidth::Px(width)) => Some(format!("{width}px")),
        Some(DataTableColumnWidth::Css(width)) => Some(width.clone()),
        None => None,
    }
}

fn column_align<T: Clone + PartialEq + 'static>(
    column: &DataTableColumn<T>,
) -> Option<&'static str> {
    match column.meta.as_ref().and_then(|meta| meta.align) {
        Some(DataTableColumnAlign::Start) => Some("start"),
        Some(DataTableColumnAlign::Center) => Some("center"),
        Some(DataTableColumnAlign::End) => Some("end"),
        None => None,
    }
}

fn aria_sort<T: Clone + PartialEq + 'static>(
    column: &DataTableColumn<T>,
    state: &DataTableState,
) -> Option<&'static str> {
    state
        .sorting
        .iter()
        .find(|sort| sort.column == column.id)
        .map(|sort| match sort.direction {
            DataTableSortDirection::Ascending => "ascending",
            DataTableSortDirection::Descending => "descending",
        })
}

fn column_label<T: Clone + PartialEq + 'static>(column: &DataTableColumn<T>) -> String {
    match &column.header {
        TextOrElement::Text(label) => label.clone(),
        TextOrElement::Element(_) | TextOrElement::Render(_) => column.id.clone(),
    }
}

fn filter_label<T: Clone + PartialEq + 'static>(
    columns: &[DataTableColumn<T>],
    column_id: &str,
) -> String {
    columns
        .iter()
        .find(|column| column.id == column_id)
        .map(column_label)
        .unwrap_or_else(|| column_id.to_string())
}

fn filter_value_label(value: &DataTableFilterValue) -> String {
    match value {
        DataTableFilterValue::Text(value) | DataTableFilterValue::Option(value) => value.clone(),
        DataTableFilterValue::Multiple(values) => values.join(", "),
        DataTableFilterValue::Boolean(true) => "Yes".to_string(),
        DataTableFilterValue::Boolean(false) => "No".to_string(),
        DataTableFilterValue::Range { min, max } => match (min, max) {
            (Some(min), Some(max)) => format!("{min} - {max}"),
            (Some(min), None) => format!("From {min}"),
            (None, Some(max)) => format!("To {max}"),
            (None, None) => String::new(),
        },
    }
}

fn is_column_visible<T: Clone + PartialEq + 'static>(
    column: &DataTableColumn<T>,
    state: &DataTableState,
) -> bool {
    state
        .column_visibility
        .iter()
        .rev()
        .find(|visibility| visibility.column == column.id)
        .map(|visibility| visibility.visible)
        .unwrap_or(true)
}

fn active_filter<'a, T: Clone + PartialEq + 'static>(
    column: &DataTableColumn<T>,
    state: &'a DataTableState,
) -> Option<&'a DataTableFilterValue> {
    state
        .filters
        .iter()
        .rev()
        .find(|filter| filter.column == column.id)
        .map(|filter| &filter.value)
}

fn next_sorting_for_column(state: &DataTableState, column_id: &str) -> Vec<DataTableSortState> {
    let current = state
        .sorting
        .iter()
        .find(|sort| sort.column == column_id)
        .map(|sort| sort.direction);
    let mut sorting = state
        .sorting
        .iter()
        .filter(|sort| sort.column != column_id)
        .cloned()
        .collect::<Vec<_>>();
    match current {
        None => sorting.insert(
            0,
            DataTableSortState {
                column: column_id.to_string(),
                direction: DataTableSortDirection::Ascending,
            },
        ),
        Some(DataTableSortDirection::Ascending) => sorting.insert(
            0,
            DataTableSortState {
                column: column_id.to_string(),
                direction: DataTableSortDirection::Descending,
            },
        ),
        Some(DataTableSortDirection::Descending) => {}
    }
    sorting
}

fn render_sort_indicator(sorting: Option<DataTableSortDirection>) -> Element {
    match sorting {
        Some(DataTableSortDirection::Ascending) => rsx! {
            ArrowUp { size: "12px", stroke: "currentColor" }
        },
        Some(DataTableSortDirection::Descending) => rsx! {
            ArrowDown { size: "12px", stroke: "currentColor" }
        },
        None => rsx! {
            ArrowUpDown { size: "12px", stroke: "currentColor" }
        },
    }
}

fn range_filter_value(min: String, max: String) -> Option<DataTableFilterValue> {
    let min = (!min.is_empty()).then_some(min);
    let max = (!max.is_empty()).then_some(max);
    if min.is_none() && max.is_none() {
        None
    } else {
        Some(DataTableFilterValue::Range { min, max })
    }
}

fn data_table_actions(
    update_state: Callback<DataTableAction, ()>,
    state: DataTableState,
    page_row_ids: Vec<DataTableRowId>,
) -> DataTableActions {
    DataTableActions {
        update_state,
        toggle_row_selected: Callback::new({
            let update_state = update_state;
            let state = state.clone();
            move |row_id| {
                update_state.call(DataTableAction::SetRowSelection {
                    selection: toggle_row_selection(&state.row_selection, row_id),
                })
            }
        }),
        toggle_row_expanded: Callback::new({
            let update_state = update_state;
            let state = state.clone();
            move |row_id| {
                let mut rows = state.expanded_rows.clone();
                if let Some(index) = rows.iter().position(|id| *id == row_id) {
                    rows.remove(index);
                } else {
                    rows.push(row_id);
                }
                update_state.call(DataTableAction::SetExpandedRows { rows })
            }
        }),
        toggle_all_page_rows_selected: Callback::new({
            let update_state = update_state;
            let state = state.clone();
            let page_row_ids = page_row_ids.clone();
            move |_| {
                update_state.call(DataTableAction::SetRowSelection {
                    selection: toggle_page_selection(
                        &state.row_selection,
                        &page_row_ids,
                        &query_fingerprint(&state),
                    ),
                })
            }
        }),
        toggle_all_matching_rows_selected: Callback::new({
            let update_state = update_state;
            let state = state.clone();
            move |_| {
                let query = query_fingerprint(&state);
                update_state.call(DataTableAction::SetRowSelection {
                    selection: match &state.row_selection {
                        DataTableRowSelectionState::AllMatching {
                            query: active,
                            except,
                        } if *active == query && except.is_empty() => {
                            DataTableRowSelectionState::Explicit { rows: Vec::new() }
                        }
                        _ => DataTableRowSelectionState::AllMatching {
                            query,
                            except: Vec::new(),
                        },
                    },
                })
            }
        }),
        reset_filters: Callback::new({
            let update_state = update_state;
            move |_| update_state.call(DataTableAction::ResetFilters)
        }),
        reset_column_state: Callback::new(move |_| {
            update_state.call(DataTableAction::ResetColumnState)
        }),
    }
}

fn toggle_row_selection(
    selection: &DataTableRowSelectionState,
    row_id: DataTableRowId,
) -> DataTableRowSelectionState {
    match selection {
        DataTableRowSelectionState::Explicit { rows } => {
            let mut rows = rows.clone();
            if let Some(index) = rows.iter().position(|id| *id == row_id) {
                rows.remove(index);
            } else {
                rows.push(row_id);
            }
            DataTableRowSelectionState::Explicit {
                rows: dedupe_strings(rows),
            }
        }
        DataTableRowSelectionState::AllMatching { query, except } => {
            let mut except = except.clone();
            if let Some(index) = except.iter().position(|id| *id == row_id) {
                except.remove(index);
            } else {
                except.push(row_id);
            }
            DataTableRowSelectionState::AllMatching {
                query: query.clone(),
                except: dedupe_strings(except),
            }
        }
    }
}

fn toggle_page_selection(
    selection: &DataTableRowSelectionState,
    page_row_ids: &[DataTableRowId],
    active_query: &str,
) -> DataTableRowSelectionState {
    let all_page_rows_selected = page_row_ids
        .iter()
        .all(|row_id| is_row_selected(selection, row_id, active_query));
    match selection {
        DataTableRowSelectionState::Explicit { rows } => {
            let mut rows = rows.clone();
            if all_page_rows_selected {
                rows.retain(|row_id| !page_row_ids.iter().any(|page_id| page_id == row_id));
            } else {
                rows.extend(page_row_ids.iter().cloned());
            }
            DataTableRowSelectionState::Explicit {
                rows: dedupe_strings(rows),
            }
        }
        DataTableRowSelectionState::AllMatching { query, except } => {
            let mut except = except.clone();
            if all_page_rows_selected {
                except.extend(page_row_ids.iter().cloned());
            } else {
                except.retain(|row_id| !page_row_ids.iter().any(|page_id| page_id == row_id));
            }
            DataTableRowSelectionState::AllMatching {
                query: query.clone(),
                except: dedupe_strings(except),
            }
        }
    }
}

fn apply_action(state: &DataTableState, action: DataTableAction) -> DataTableState {
    let mut next = state.clone();
    match action {
        DataTableAction::SetPage { page } => {
            next.pagination.page = page.max(1);
        }
        DataTableAction::SetPageSize { page_size } => {
            next.pagination.page = 1;
            next.pagination.page_size = page_size.max(1);
        }
        DataTableAction::SetSorting { sorting } => {
            next.sorting = dedupe_by_column(sorting);
        }
        DataTableAction::SetFilter { column, value } => {
            next.pagination.page = 1;
            next.filters.retain(|filter| filter.column != column);
            if let Some(value) = value {
                next.filters.push(DataTableFilterState { column, value });
            }
            clear_stale_all_matching_selection(&mut next);
        }
        DataTableAction::SetGlobalFilter { value } => {
            next.pagination.page = 1;
            next.global_filter = value;
            clear_stale_all_matching_selection(&mut next);
        }
        DataTableAction::SetColumnVisibility { column, visible } => {
            next.column_visibility
                .retain(|visibility| visibility.column != column);
            next.column_visibility
                .push(DataTableColumnVisibilityState { column, visible });
            clear_all_matching_selection(&mut next);
        }
        DataTableAction::SetColumnOrder { columns } => {
            next.column_order = dedupe_strings(columns);
        }
        DataTableAction::SetColumnPinning { pinning } => {
            next.column_pinning = canonicalize_pinning(&pinning);
        }
        DataTableAction::SetColumnSize { column, width } => {
            next.column_sizing.retain(|size| size.column != column);
            next.column_sizing
                .push(DataTableColumnSizeState { column, width });
        }
        DataTableAction::SetRowSelection { selection } => {
            next.row_selection = canonicalize_selection(selection);
        }
        DataTableAction::SetExpandedRows { rows } => {
            next.expanded_rows = dedupe_strings(rows);
        }
        DataTableAction::ResetFilters => {
            next.pagination.page = 1;
            next.filters.clear();
            next.global_filter = None;
            clear_stale_all_matching_selection(&mut next);
        }
        DataTableAction::ResetColumnState => {
            next.column_visibility.clear();
            next.column_order.clear();
            next.column_pinning = DataTableColumnPinningState::default();
            next.column_sizing.clear();
            clear_all_matching_selection(&mut next);
        }
    }
    next
}

fn clear_all_matching_selection(state: &mut DataTableState) {
    if matches!(
        state.row_selection,
        DataTableRowSelectionState::AllMatching { .. }
    ) {
        state.row_selection = DataTableRowSelectionState::Explicit { rows: Vec::new() };
    }
}

fn clear_stale_all_matching_selection(state: &mut DataTableState) {
    let query = query_fingerprint(state);
    if matches!(
        &state.row_selection,
        DataTableRowSelectionState::AllMatching { query: active, .. } if *active != query
    ) {
        state.row_selection = DataTableRowSelectionState::Explicit { rows: Vec::new() };
    }
}

fn dedupe_by_column(states: Vec<DataTableSortState>) -> Vec<DataTableSortState> {
    let mut seen = HashSet::new();
    states
        .into_iter()
        .filter(|state| seen.insert(state.column.clone()))
        .collect()
}

fn dedupe_strings(values: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    values
        .into_iter()
        .filter(|value| seen.insert(value.clone()))
        .collect()
}

fn canonicalize_selection(selection: DataTableRowSelectionState) -> DataTableRowSelectionState {
    match selection {
        DataTableRowSelectionState::Explicit { rows } => DataTableRowSelectionState::Explicit {
            rows: dedupe_strings(rows),
        },
        DataTableRowSelectionState::AllMatching { query, except } => {
            DataTableRowSelectionState::AllMatching {
                query,
                except: dedupe_strings(except),
            }
        }
    }
}

fn query_fingerprint(state: &DataTableState) -> DataTableQueryFingerprint {
    let mut filters = canonicalize_filters(state.filters.clone());
    filters.sort_by(|left, right| left.column.cmp(&right.column));

    let mut output = String::from("{\"filters\":[");
    for (index, filter) in filters.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"column\":");
        output.push_str(&json_string(&filter.column));
        output.push_str(",\"value\":");
        output.push_str(&filter_value_json(&filter.value));
        output.push('}');
    }
    output.push_str("],\"global_filter\":");
    match &state.global_filter {
        Some(global_filter) => output.push_str(&json_string(global_filter)),
        None => output.push_str("null"),
    }
    output.push('}');
    output
}

fn canonicalize_filters(filters: Vec<DataTableFilterState>) -> Vec<DataTableFilterState> {
    let mut seen = HashSet::new();
    filters
        .into_iter()
        .filter(|filter| seen.insert(filter.column.clone()))
        .collect()
}

fn filter_value_json(value: &DataTableFilterValue) -> String {
    match value {
        DataTableFilterValue::Text(value) => {
            format!("{{\"type\":\"text\",\"value\":{}}}", json_string(value))
        }
        DataTableFilterValue::Option(value) => {
            format!("{{\"type\":\"option\",\"value\":{}}}", json_string(value))
        }
        DataTableFilterValue::Multiple(values) => {
            let mut values = values.clone();
            values.sort();
            format!(
                "{{\"type\":\"multiple\",\"value\":[{}]}}",
                values
                    .iter()
                    .map(|value| json_string(value))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
        DataTableFilterValue::Boolean(value) => {
            format!("{{\"type\":\"boolean\",\"value\":{value}}}")
        }
        DataTableFilterValue::Range { min, max } => format!(
            "{{\"type\":\"range\",\"min\":{},\"max\":{}}}",
            min.as_deref()
                .map(json_string)
                .unwrap_or_else(|| "null".to_string()),
            max.as_deref()
                .map(json_string)
                .unwrap_or_else(|| "null".to_string())
        ),
    }
}

fn json_string(value: &str) -> String {
    let mut output = String::from("\"");
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\u{08}' => output.push_str("\\b"),
            '\u{0C}' => output.push_str("\\f"),
            character if character.is_control() => {
                output.push_str(&format!("\\u{:04x}", character as u32));
            }
            character => output.push(character),
        }
    }
    output.push('"');
    output
}

#[derive(Clone, Copy)]
struct PageTargets {
    show: bool,
    first: Option<u32>,
    previous: Option<u32>,
    next: Option<u32>,
    last: Option<u32>,
}

fn page_targets(
    pagination: &DataTablePaginationState,
    page_info: DataTablePageInfo,
) -> PageTargets {
    let page = pagination.page.max(1);
    match page_info.total_count {
        DataTableTotalCount::Known(total_count) => {
            let total_pages = total_pages(total_count, pagination.page_size);
            let previous = (page > 1).then_some(page - 1);
            let next = (page < total_pages).then_some(page + 1);
            PageTargets {
                show: total_pages > 1,
                first: (page > 1).then_some(1),
                previous,
                next,
                last: (page < total_pages).then_some(total_pages),
            }
        }
        DataTableTotalCount::Unknown => {
            let previous_known = page_info.has_previous_page.unwrap_or(page > 1);
            let next_known = page_info.has_next_page.unwrap_or(false);
            PageTargets {
                show: previous_known || next_known,
                first: previous_known.then_some(1),
                previous: (previous_known && page > 1).then_some(page - 1),
                next: next_known.then_some(page + 1),
                last: None,
            }
        }
    }
}

fn pagination_summary(
    pagination: &DataTablePaginationState,
    page_info: DataTablePageInfo,
) -> String {
    match page_info.total_count {
        DataTableTotalCount::Known(total_count) => {
            let total_pages = total_pages(total_count, pagination.page_size);
            format!(
                "Page {} of {} - {} total",
                pagination.page.max(1).min(total_pages),
                total_pages,
                total_count
            )
        }
        DataTableTotalCount::Unknown => format!("Page {} - total unknown", pagination.page.max(1)),
    }
}

fn total_pages(total_count: u64, page_size: u64) -> u32 {
    if total_count == 0 {
        return 1;
    }

    let page_size = page_size.max(1);
    total_count.div_ceil(page_size).min(u32::MAX as u64) as u32
}

fn page_size_options(active_page_size: u64) -> Vec<u64> {
    let active_page_size = active_page_size.max(1);
    let mut options = vec![10, 25, 50, 1000];
    if !options.contains(&active_page_size) {
        options.push(active_page_size);
        options.sort_unstable();
    }
    options
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::rc::Rc;

    use super::*;

    fn filtered_state() -> DataTableState {
        DataTableState {
            pagination: DataTablePaginationState {
                page: 3,
                page_size: 10,
            },
            sorting: vec![DataTableSortState {
                column: "total".to_string(),
                direction: DataTableSortDirection::Descending,
            }],
            filters: vec![DataTableFilterState {
                column: "status".to_string(),
                value: DataTableFilterValue::Option("paid".to_string()),
            }],
            global_filter: Some("north".to_string()),
            ..DataTableState::default()
        }
    }

    #[test]
    fn page_size_changes_reset_to_first_page() {
        let next = apply_action(
            &filtered_state(),
            DataTableAction::SetPageSize { page_size: 50 },
        );

        assert_eq!(next.pagination.page, 1);
        assert_eq!(next.pagination.page_size, 50);
    }

    #[test]
    fn page_size_options_keep_default_sizes_unique() {
        assert_eq!(page_size_options(25), vec![10, 25, 50, 1000]);
    }

    #[test]
    fn page_size_options_include_active_custom_size() {
        assert_eq!(page_size_options(3), vec![3, 10, 25, 50, 1000]);
    }

    #[test]
    fn filter_changes_reset_page_and_clear_stale_all_matching_selection() {
        let mut state = filtered_state();
        state.row_selection = DataTableRowSelectionState::AllMatching {
            query: query_fingerprint(&state),
            except: Vec::new(),
        };

        let next = apply_action(
            &state,
            DataTableAction::SetFilter {
                column: "status".to_string(),
                value: Some(DataTableFilterValue::Option("failed".to_string())),
            },
        );

        assert_eq!(next.pagination.page, 1);
        assert_eq!(
            next.row_selection,
            DataTableRowSelectionState::Explicit { rows: Vec::new() }
        );
    }

    #[test]
    fn column_visibility_changes_clear_stale_all_matching_selection() {
        let mut state = filtered_state();
        state.row_selection = DataTableRowSelectionState::AllMatching {
            query: query_fingerprint(&state),
            except: Vec::new(),
        };

        let next = apply_action(
            &state,
            DataTableAction::SetColumnVisibility {
                column: "status".to_string(),
                visible: false,
            },
        );

        assert_eq!(
            next.row_selection,
            DataTableRowSelectionState::Explicit { rows: Vec::new() }
        );
    }

    #[test]
    fn reset_column_state_clears_stale_all_matching_selection() {
        let mut state = filtered_state();
        state.column_visibility = vec![DataTableColumnVisibilityState {
            column: "status".to_string(),
            visible: false,
        }];
        state.row_selection = DataTableRowSelectionState::AllMatching {
            query: query_fingerprint(&state),
            except: Vec::new(),
        };

        let next = apply_action(&state, DataTableAction::ResetColumnState);

        assert_eq!(
            next.row_selection,
            DataTableRowSelectionState::Explicit { rows: Vec::new() }
        );
    }

    #[test]
    fn merged_transitional_pagination_survives_later_actions() {
        let state = DataTableState {
            pagination: DataTablePaginationState {
                page: 1,
                page_size: 25,
            },
            ..DataTableState::default()
        };
        let merged = merge_transitional_pagination_state(state, Some(50), Some(3));
        let next = apply_action(
            &merged,
            DataTableAction::SetSorting {
                sorting: vec![DataTableSortState {
                    column: "rank".to_string(),
                    direction: DataTableSortDirection::Ascending,
                }],
            },
        );

        assert_eq!(next.pagination.page, 3);
        assert_eq!(next.pagination.page_size, 50);
    }

    #[test]
    fn query_fingerprint_excludes_pagination_and_sorting() {
        let mut left = filtered_state();
        let mut right = left.clone();
        right.pagination = DataTablePaginationState {
            page: 99,
            page_size: 100,
        };
        right.sorting = vec![DataTableSortState {
            column: "customer".to_string(),
            direction: DataTableSortDirection::Ascending,
        }];

        assert_eq!(query_fingerprint(&left), query_fingerprint(&right));

        left.global_filter = Some("south".to_string());
        assert_ne!(query_fingerprint(&left), query_fingerprint(&right));
    }

    #[test]
    fn query_fingerprint_canonicalizes_filter_order() {
        let mut left = DataTableState {
            filters: vec![
                DataTableFilterState {
                    column: "status".to_string(),
                    value: DataTableFilterValue::Option("paid".to_string()),
                },
                DataTableFilterState {
                    column: "customer".to_string(),
                    value: DataTableFilterValue::Text("ada".to_string()),
                },
            ],
            ..DataTableState::default()
        };
        let mut right = left.clone();
        right.filters.reverse();

        assert_eq!(query_fingerprint(&left), query_fingerprint(&right));

        left.filters[0].value = DataTableFilterValue::Option("failed".to_string());
        assert_ne!(query_fingerprint(&left), query_fingerprint(&right));
    }

    #[test]
    fn row_selection_is_deduped_deterministically() {
        let next = apply_action(
            &DataTableState::default(),
            DataTableAction::SetRowSelection {
                selection: DataTableRowSelectionState::Explicit {
                    rows: vec!["a".to_string(), "b".to_string(), "a".to_string()],
                },
            },
        );

        assert_eq!(
            next.row_selection,
            DataTableRowSelectionState::Explicit {
                rows: vec!["a".to_string(), "b".to_string()]
            }
        );
    }

    #[test]
    fn pinning_canonicalization_removes_right_side_conflicts() {
        let next = apply_action(
            &DataTableState::default(),
            DataTableAction::SetColumnPinning {
                pinning: DataTableColumnPinningState {
                    left: vec!["id".to_string(), "id".to_string(), "name".to_string()],
                    right: vec!["name".to_string(), "total".to_string(), "total".to_string()],
                },
            },
        );

        assert_eq!(next.column_pinning.left, vec!["id", "name"]);
        assert_eq!(next.column_pinning.right, vec!["total"]);
    }

    #[test]
    fn known_total_page_targets_include_last_page() {
        let targets = page_targets(
            &DataTablePaginationState {
                page: 2,
                page_size: 10,
            },
            DataTablePageInfo::known_total(35),
        );

        assert!(targets.show);
        assert_eq!(targets.first, Some(1));
        assert_eq!(targets.previous, Some(1));
        assert_eq!(targets.next, Some(3));
        assert_eq!(targets.last, Some(4));
    }

    #[test]
    fn unknown_total_page_targets_do_not_include_last_page() {
        let targets = page_targets(
            &DataTablePaginationState {
                page: 2,
                page_size: 10,
            },
            DataTablePageInfo {
                total_count: DataTableTotalCount::Unknown,
                has_previous_page: Some(true),
                has_next_page: Some(true),
            },
        );

        assert!(targets.show);
        assert_eq!(targets.first, Some(1));
        assert_eq!(targets.previous, Some(1));
        assert_eq!(targets.next, Some(3));
        assert_eq!(targets.last, None);
    }

    #[derive(Clone, PartialEq)]
    struct TestItem {
        visible: String,
        hidden: String,
        rank: f64,
    }

    fn test_row_id(ctx: DataTableRowIdentityContext<TestItem>) -> DataTableRowId {
        format!("row-{}", ctx.source_index)
    }

    fn test_columns() -> Vec<DataTableColumn<TestItem>> {
        vec![
            DataTableColumn::accessor(
                "visible",
                "Visible",
                Callback::new(|item: Rc<TestItem>| DataTableValue::Text(item.visible.clone())),
            )
            .searchable()
            .filter_text(),
            DataTableColumn::accessor(
                "hidden",
                "Hidden",
                Callback::new(|item: Rc<TestItem>| DataTableValue::Text(item.hidden.clone())),
            )
            .searchable()
            .filter_text()
            .hideable(),
            DataTableColumn::accessor(
                "rank",
                "Rank",
                Callback::new(|item: Rc<TestItem>| DataTableValue::Number(item.rank)),
            )
            .sortable()
            .hideable(),
        ]
    }

    #[test]
    fn hidden_columns_do_not_participate_in_local_filtering_or_search() {
        let mut dom = VirtualDom::new(|| {
            let items = vec![
                TestItem {
                    visible: "Ada".to_string(),
                    hidden: "internal".to_string(),
                    rank: 2.0,
                },
                TestItem {
                    visible: "Grace".to_string(),
                    hidden: "match me".to_string(),
                    rank: 1.0,
                },
            ];
            let state = DataTableState {
                filters: vec![DataTableFilterState {
                    column: "hidden".to_string(),
                    value: DataTableFilterValue::Text("match".to_string()),
                }],
                global_filter: Some("match".to_string()),
                column_visibility: vec![DataTableColumnVisibilityState {
                    column: "hidden".to_string(),
                    visible: false,
                }],
                ..DataTableState::default()
            };
            let columns = canonicalize_columns(&test_columns(), &state);

            let rows = derive_rows(
                &items,
                &Callback::new(test_row_id),
                &columns,
                &state,
                DataTableManualModes {
                    pagination: false,
                    sorting: false,
                    filtering: false,
                },
            );

            assert_eq!(rows.len(), 2);
            rsx! {}
        });
        dom.rebuild_in_place();
    }

    #[test]
    fn hidden_columns_do_not_participate_in_local_sorting() {
        let mut dom = VirtualDom::new(|| {
            let items = vec![
                TestItem {
                    visible: "First".to_string(),
                    hidden: "z".to_string(),
                    rank: 2.0,
                },
                TestItem {
                    visible: "Second".to_string(),
                    hidden: "a".to_string(),
                    rank: 1.0,
                },
            ];
            let state = DataTableState {
                sorting: vec![DataTableSortState {
                    column: "rank".to_string(),
                    direction: DataTableSortDirection::Ascending,
                }],
                column_visibility: vec![DataTableColumnVisibilityState {
                    column: "rank".to_string(),
                    visible: false,
                }],
                ..DataTableState::default()
            };
            let columns = canonicalize_columns(&test_columns(), &state);

            let rows = derive_rows(
                &items,
                &Callback::new(test_row_id),
                &columns,
                &state,
                DataTableManualModes {
                    pagination: false,
                    sorting: false,
                    filtering: false,
                },
            );

            assert_eq!(rows[0].item.visible, "First");
            assert_eq!(rows[1].item.visible, "Second");
            rsx! {}
        });
        dom.rebuild_in_place();
    }

    #[test]
    fn accessor_values_are_reused_across_filter_sort_and_render_value_lookup() {
        let mut dom = VirtualDom::new(|| {
            let access_count = Rc::new(Cell::new(0));
            let columns = vec![{
                let access_count = access_count.clone();
                DataTableColumn::accessor(
                    "visible",
                    "Visible",
                    Callback::new(move |item: Rc<TestItem>| {
                        access_count.set(access_count.get() + 1);
                        DataTableValue::Text(item.visible.clone())
                    }),
                )
                .searchable()
                .filter_text()
                .sortable()
            }];
            let items = vec![
                TestItem {
                    visible: "beta".to_string(),
                    hidden: String::new(),
                    rank: 0.0,
                },
                TestItem {
                    visible: "alpha".to_string(),
                    hidden: String::new(),
                    rank: 0.0,
                },
                TestItem {
                    visible: "gamma".to_string(),
                    hidden: String::new(),
                    rank: 0.0,
                },
            ];
            let state = DataTableState {
                sorting: vec![DataTableSortState {
                    column: "visible".to_string(),
                    direction: DataTableSortDirection::Ascending,
                }],
                global_filter: Some("a".to_string()),
                ..DataTableState::default()
            };

            let rows = derive_rows(
                &items,
                &Callback::new(test_row_id),
                &columns,
                &state,
                DataTableManualModes {
                    pagination: false,
                    sorting: false,
                    filtering: false,
                },
            );

            assert_eq!(access_count.get(), items.len());
            assert_eq!(rows[0].item.visible, "alpha");
            assert_eq!(
                cached_column_value(&columns[0], &rows[0], &columns),
                DataTableValue::Text("alpha".to_string())
            );
            assert_eq!(access_count.get(), items.len());
            rsx! {}
        });
        dom.rebuild_in_place();
    }
}
