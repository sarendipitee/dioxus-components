# Refactor: shared `TextOrElement` prop type

## Goal

Several components reinvent the same enum pattern: a prop that accepts a static
`&str`, an owned `String`, **or** a Dioxus `Element`, wired through `#[props(into)]`
plus a set of `From` impls. Unify these into **one shared type** so the pattern is
defined once and reused across the library.

## Locked decisions (from product owner)

| Decision | Choice |
| --- | --- |
| Name | **`TextOrElement`** |
| Home crate | **`primitives`** (crate root, alongside `ContentSide` / `ContentAlign`) |
| DataTable handling | **Full unify** — `DataTableColumnHeader` is reworked around the shared type, including its context-callback case |

## Current state (verified)

### Pattern A — `InputContent` / `InputContentValue` (the canonical pattern)
- Defined in [`input/component.rs:150-212`](../../dioxus-components/src/components/input/component.rs).
- `InputContent` = `struct { content: Option<InputContentValue> }`, derives `Clone, Default, PartialEq`.
- `InputContentValue` (private enum, derives `Clone, PartialEq`): `Text(String)`, `Element(Element)`.
- `From` impls: `String`, `&str`, `Element`, `Option<Element>`, `Option<String>`.
- Helpers: `into_element(self) -> Option<Element>` (renders `Text` → `rsx!{ "{text}" }`, `Element` passthrough), `is_some()`.
- Type alias `pub type InputLabel = InputContent;` (line 212).
- Consumed via `#[props(default, into)]` on ~12 props: `InputFieldText`, `InputWrapper`, `InputBase`, `TextInput` (`label` / `description` / `error` each) and reused by `Checkbox` (`label` / `description` / `error`).

### Pattern B — `DataTableColumnHeader` (partial match + a callback variant)
- Defined in [`data_table/component.rs:336-343`](../../dioxus-components/src/components/data_table/component.rs).
- Enum variants: `Label(Cow<'static, str>)`, `Custom(Callback<DataTableColumnHeaderContext, Element>)`.
- `From` impls: `String`, `&'static str` (both → `Label`). **No `Element` support.**
- Not a `#[props(into)]` prop — set through the builder `DataTableColumn::header(impl Into<DataTableColumnHeader>)` (`:537-540`).
- Internal construction sites: `:482`, `:520`, `:670` (`DataTableColumnHeader::Label(... .into())`).

### Workspace
- Crate graph: `primitives → dioxus-components → preview` (+ `test-harness`, `dioxus-attributes`).
- `primitives/src/lib.rs` holds unprefixed generic enums at the crate root (`ContentSide:300`, `ContentAlign:324`). No `shared`/`common` module exists; **add the type at the `primitives` crate root** to match precedent.
- `primitives` already depends on Dioxus, so `Element` and `Callback` are available there.

## Target design

### The shared type (in `primitives`)

> **Derive bound.** A `#[derive(Clone, PartialEq)]` on a generic enum makes the
> impls conditional on `Ctx: Clone + PartialEq`. This holds for both concrete uses:
> `()` and `DataTableColumnHeaderContext` (derives `Clone, PartialEq` at
> [`data_table/component.rs:740`](../../dioxus-components/src/components/data_table/component.rs)).
> No extra work needed, but new context types must derive both.

```rust
/// Content that can be supplied to a component prop as plain text, a rendered
/// `Element`, or — when render context is needed — a callback that produces an
/// `Element` from that context.
///
/// The `Ctx` parameter defaults to `()` so the common case is simply
/// `TextOrElement` and works with `#[props(into)]`.
#[derive(Clone, PartialEq)]
pub enum TextOrElement<Ctx: 'static = ()> {
    /// Plain text content.
    Text(String),
    /// A pre-rendered element.
    Element(Element),
    /// A renderer invoked with context to produce an element.
    Render(Callback<Ctx, Element>),
}

impl<Ctx: 'static> From<String> for TextOrElement<Ctx> { /* Text */ }
impl<Ctx: 'static> From<&str>   for TextOrElement<Ctx> { /* Text(value.to_string()) */ }
impl<Ctx: 'static> From<Element> for TextOrElement<Ctx> { /* Element */ }
impl<Ctx: 'static> From<Callback<Ctx, Element>> for TextOrElement<Ctx> { /* Render */ }
```

Rendering helper for the **no-context** case (`Ctx = ()`):

```rust
impl TextOrElement<()> {
    /// Render to an element. `Render` callbacks are invoked with `()`.
    pub fn into_element(self) -> Element {
        match self {
            TextOrElement::Text(text) => rsx! { "{text}" },
            TextOrElement::Element(el) => el,
            TextOrElement::Render(cb) => cb.call(()),
        }
    }
}
```

For the **with-context** case (DataTable), the consumer matches and calls
`Render(cb)` with its own context value (mirrors how `Custom` is used today).

### Optional / "empty" content story

`InputContent` exists today only to add `Default` (= empty) + `is_some()` + the
`Option<…>` `From` impls on top of the value enum. Under full unification the
value type becomes `TextOrElement<()>`. Two viable shapes — **recommend (1)**,
fall back to (2) only if Dioxus `into` ergonomics force it:

1. **Standardize optional content on `Option<TextOrElement>`.**
   - Optional props become `#[props(default, into)] label: Option<TextOrElement>` (`Default` = `None`).
   - Provide the orphan-rule-legal `From` impls so literals still work through `into`:
     `From<&str>`, `From<String>`, `From<Element>`, `From<Option<String>>`, `From<Option<Element>>` for `Option<TextOrElement>`.
   - `is_some()` → `Option::is_some`; rendering → `opt.map(TextOrElement::into_element)`.
   - Remove `InputContent` / `InputContentValue` / `InputLabel`.
2. **Keep a thin shared wrapper** `pub struct OptionalContent(Option<TextOrElement>)` (or retain `InputContent` as a newtype) wrapping the shared value type, preserving today's `Default`/`is_some`/`into_element` ergonomics verbatim while still deduping the underlying value enum.

> The choice between (1) and (2) hinges on whether `#[props(default, into)]` plays
> cleanly with `Option<TextOrElement>` and the orphan `From` impls. This MUST be
> settled by a compile check early (see Phase 0), because it determines whether
> the ~12 Input prop declarations + Checkbox change type or stay as a wrapper.

### `DataTableColumnHeader` rework (full unify)

- Replace the bespoke enum with `TextOrElement<DataTableColumnHeaderContext>`.
  - `Label(Cow)` → `Text(String)` (see Cow tradeoff below).
  - `Custom(Callback<Ctx, Element>)` → `Render(Callback<Ctx, Element>)` (the generic `Render` variant).
- `DataTableColumn::header` field (`:445`) + builder (`:537-540`) become `TextOrElement<DataTableColumnHeaderContext>` / `impl Into<…>`.
- Update the three internal construction sites (`:482`, `:520`, `:670`) from `DataTableColumnHeader::Label(..)` to `TextOrElement::Text(..)`.

**All match sites that touch the header enum (these were the validator's blocking gaps — the enum is matched in two distinct paths, render AND text-extraction):**
- **Render path** — `render_header` (`:2235-2245`) and `render_label_header` (`:2248`, signature `label: &str`): draws the header. Today `Label` columns get the auto sort-button wrapper when `column.sortable.is_some()`; `Custom` passes through bare. Must add an `Element` arm.
- **Text-extraction path** — `column_label` (`:2883-2888`) returns a `String` used by the filter menu, column-visibility menu (`:2560`, `:2575`), and `filter_label` (`:2890`, consumed at `:2461`). Today: `Label(label) => label.to_string()`, `Custom(_) => column.id.clone()`. Must add an `Element` arm.
- Note: `:2538` has a pre-existing commented-out `// label: column_label(column),` — dead, leave as-is.

**Design decisions for the NEW `Element` variant in DataTable (must be explicit — these are real choices, not mechanical arms):**
1. **`column_label` text fallback:** `Element(_)` has no extractable text → fall back to `column.id.clone()`, same as `Render(_)`/today's `Custom(_)`. (Safe extension of the existing fallback.)
2. **Sort-button affordance in `render_header`:** **Recommend** only `Text` headers receive the automatic sort-button wrapper (today's `Label` behavior); `Element` and `Render` pass through bare (today's `Custom` behavior), making the consumer responsible for any sort affordance. Rationale: preserves current `Custom` semantics and avoids wrapping arbitrary author-supplied elements. If a sortable `Element` header should still get the affordance, that is a deliberate enhancement — call it out at review, do not silently add it.

- `DataTableColumnHeader` (the name) is either removed or kept as a type alias
  `pub type DataTableColumnHeader = TextOrElement<DataTableColumnHeaderContext>;`
  to minimize churn — **recommend the alias** so existing references compile. Note: any
  external/preview code constructing `DataTableColumnHeader::Custom(..)` becomes `::Render(..)` (breaking) — caught in the Wave 3 sweep.

## Key tradeoffs & decisions

- **`Cow<'static, str>` → `String`.** DataTable currently avoids an allocation for
  static headers. Unifying on `String` costs one small allocation per column header
  (headers are few, built once) in exchange for a single shared type. Accepted as
  part of "full unify." If this is later unwanted, `TextOrElement` could store
  `Cow<'static, str>` instead of `String` — but that ripples into `InputContent`
  semantics and the `From` impls, so keep `String` unless profiling objects.
- **Generic with default param.** `TextOrElement<()>` is the ergonomic common case;
  `TextOrElement<SomeCtx>` serves context-driven renderers. The default param keeps
  every existing call-site writing just `TextOrElement`.
- **Naming churn.** Keeping `InputLabel` and `DataTableColumnHeader` as aliases (at
  least transitionally) shrinks the diff and keeps the public API recognizable.

## Open risks — must verify at implementation (compile-time)

1. **`#[props(into)]` + `Option<TextOrElement>` + orphan `From` impls** actually
   compile and let bare `"literal"` / `String` / `Element` flow through. Decides
   Option-story (1) vs (2). *Verify first (Phase 0).*
2. **Generic enum as a concrete prop type.** Using `TextOrElement` (= `TextOrElement<()>`)
   under `#[props(into)]` should be fine (concrete type), but confirm Dioxus `Props`
   derive is happy. *Low risk — confirm by compiling one prop.*
3. **`Callback<(), Element>` satisfies `Clone + PartialEq`.** Already true in this
   codebase (`DataTableColumnHeader::Custom` holds a `Callback` inside a `Clone +
   PartialEq` builder type), so this is effectively pre-verified; reconfirm after move.
4. **`rsx!` availability in `primitives`.** `into_element` uses `rsx!`; confirm the
   macro/imports resolve in `primitives` (it's a Dioxus crate, expected fine).

## Migration plan (waves)

**Phase 0 — De-risk + lock scope (single small slice, blocking).**
- Add `TextOrElement` to `primitives/src/lib.rs` with `From` impls + `into_element`.
- Prove the Option-story: add the type, wire **one** Input prop (e.g. `InputWrapper::label`)
  to the chosen optional shape, and compile. Resolve risks #1–#4 here.
- **Scope lock (closes the validator's incomplete-inventory caveat):** grep the workspace
  (`primitives`, `dioxus-components`, `preview`) for other ad-hoc str/String/Element prop
  enums and for `InputContentValue` / `DataTableColumnHeader::{Label,Custom}` usages,
  confirming the inventory below is complete before committing the wave split. Spot-checks
  already cleared `badge` (uses `children: Element`) and `avatar` (typed props).
- Output: confirmed optional-content shape (1 or 2), confirmed full call-site set, green compile of the spike.

**Wave 1 — Input/Checkbox migration (one owner: `dioxus-components/src/components/input/*` + `checkbox/*`).**
- Re-export `TextOrElement` from `dioxus-components` (via `primitives` re-export or `pub use`).
- Replace `InputContentValue` with `TextOrElement`; apply the chosen optional shape across the ~12 Input props + 3 Checkbox props; keep `InputLabel` as an alias if retained.
- Update `into_element`/`is_some` call-sites inside Input/Checkbox bodies.
- **Internal helper signatures (depend on the Phase-0 option choice — flagged by validator):**
  `build_input_field_text_state` (`:227-232`, takes `&InputContent`), `element_label`
  (`:214-216`, returns `InputLabel`), and the `InputFieldTextState` type. Under option (1)
  these become `&Option<TextOrElement>` / `Option<TextOrElement>`; under option (2) they keep
  the wrapper type. Migrate accordingly.
- Verify: targeted build + Input/Checkbox tests (`input/component.rs:742-809` harness).

**Wave 2 — DataTable migration (different owner: `dioxus-components/src/components/data_table/*`).**
- Disjoint files from Wave 1 → may run **concurrently** with Wave 1.
- Swap `DataTableColumnHeader` for `TextOrElement<DataTableColumnHeaderContext>` (or alias), `Cow`→`String`, `Custom`→`Render`; update builder + sites `:482/:520/:670` + header rendering.
- Verify: targeted build + DataTable tests + relevant Playwright (`playwright/*data_table*` if present).

**Wave 3 — Cleanup & sweep (after 1 & 2 land).**
- Remove dead types/aliases not kept intentionally; ensure `primitives` re-export path is the single source.
- Grep for any remaining `InputContentValue`, `DataTableColumnHeader::{Label,Custom}`, stray re-definitions.
- Full workspace build + clippy + test + Playwright.

## Call-site inventory

- **Input props (`#[props(default, into)]`)**: `input/component.rs:265-267, 344/347/350, 530/533/536, 603/606/609`.
- **Input internal helpers**: `element_label` `:214-216`, `build_input_field_text_state` `:227-232`, `InputFieldTextState` type, plus `into_element`/`is_some` body uses `:198-208, 278-284`.
- **Checkbox props**: `checkbox/component.rs:19, 22, 25`.
- **Input tests/harness**: `input/component.rs:742-750, 763-772, 800-809`.
- **DataTable internal construction**: `data_table/component.rs:482, 520, 670`; builder `:537-540`; field `:445`.
- **DataTable enum match sites**: `render_header` `:2235-2245`, `render_label_header` `:2248`, `column_label` `:2883-2888`, `filter_label` `:2890` (used at `:2461`), column-visibility menu `:2560, :2575`.
- **Preview call-sites**: pass string literals / elements through `#[props(into)]`; because the `From<&str>` / `From<Element>` impls are preserved, these are **source-compatible and need no change** (confirm during Wave 3 sweep). The only breaking external shape is anyone constructing `DataTableColumnHeader::Custom(..)` directly → becomes `::Render(..)`.

## Verification strategy

- Phase 0: compile spike (resolves the four risks).
- Per wave: targeted `cargo build` + relevant unit tests for the owned component(s); run owning-area Playwright specs where they exist.
- Final (Wave 3): workspace `cargo build`, `cargo clippy`, `cargo test`, full Playwright run.

## Out of scope

- Changing component visual behavior or HTML structure.
- Migrating any prop that is *not* part of the str/String/Element pattern.
- Performance work beyond the noted `Cow`→`String` tradeoff.
