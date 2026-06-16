# Input Unification Architecture Specification

## Purpose

Input unification defines the canonical field-shell architecture for input-like components in this workspace.

The system must let text inputs, picker-backed inputs, selects, combobox-backed inputs, and future text-like controls share one reusable contract for shell geometry, sections, labels, descriptions, errors, disabled state, and clear behavior without collapsing picker surfaces into text-input-shaped APIs.

This specification intentionally follows the Mantine-style split between `Input`, `Input.Wrapper`, `InputBase`, and thin adapters, but it must be implemented against this repository's ownership boundaries and the CSS vars architecture contract.

## Acceptance Summary

An implementation satisfies this specification only if:

- `Input` is the low-level shared shell/surface API.
- `Input.Wrapper` owns field-level wrapper semantics, ids, and aria wiring.
- `InputBase` composes `Input.Wrapper` and `Input`.
- `TextInput` is the public native text-entry adapter and carries the current text-input behavior/API.
- Picker surfaces remain separate from input-composition components:
  - `ColorPicker` vs `ColorInput`
  - `DatePicker` vs `DateInput`
  - `TimePicker` vs `TimeInput`
- `primitives/` owns unstyled behavior, accessibility, state machines, keyboard interaction, and picker or combobox semantics.
- `dioxus-components/` owns the canonical styled reusable input foundation under `dioxus-components/src/components/**`.
- `preview/` owns demos, docs, registry metadata, and import/install surface only.
- Shared input styling consumes the finalized unprefixed CSS recipe-token system and does not introduce a parallel `--dxc-*` token family.
- Wrapper-generated accessibility metadata merges with control-specific accessibility metadata instead of replacing it.
- Clear-button and right-section precedence is centralized and reused.
- Registry/install entries treat `input` as a shared dependency and keep picker surfaces separately installable from input-composition components.
- Migration status, validation requirements, and active blockers remain recorded as part of the living architecture.

## Layering and Ownership

### `primitives/`

`primitives/` owns:

- unstyled behavior
- accessibility behavior
- state machines
- keyboard interaction
- roving focus
- popover, listbox, combobox, date, time, and picker semantics when those behaviors live below the styled layer

`primitives/` must not own reusable styled field chrome.

### `dioxus-components/`

`dioxus-components/` owns:

- the canonical styled input foundation
- reusable styled field-shell CSS
- public styled component APIs for `Input`, `Input.Wrapper`, `InputBase`, `TextInput`, and related adapters
- canonical reusable implementations under `dioxus-components/src/components/**`

The shared input foundation should live under `dioxus-components/src/components/input/` or an equivalent crate-owned canonical location.

### `preview/`

`preview/` owns:

- demos
- docs
- registry metadata
- import/install surface
- visual validation

`preview/src/components/**` is not a second source of truth for canonical styled input implementations.

## Architecture Model

The input system is split into reusable shell composition and component-specific behavior.

### Core Composition

- `Input`: low-level bordered or trigger-like shell
- `Input.Wrapper`: field-level label, description, error, and aria wrapper
- `InputClearButton`: shared clear affordance
- `InputBase`: `Input.Wrapper` plus `Input`
- Thin adapters: `TextInput`, `ColorInput`, `DateInput`, `TimeInput`, `Select`, and combobox-backed inputs

### Surface vs Input Composition

Picker surfaces are the interactive picker UIs. Input variants compose those surfaces with shared field chrome.

Definitions:

- `ColorPicker`: color-picking surface only
- `ColorInput`: `Input.Wrapper` + `Popover` + field trigger + `ColorPicker`
- `DatePicker`: calendar/date-selection surface only
- `DateInput`: field composition around `DatePicker`
- `DateRangePickerInput`: range-specific input composition around date-selection behavior
- `TimePicker`: time-selection surface only
- `TimeInput`: field composition around `TimePicker`

This split also applies to select and combobox-backed inputs: the shared input layer owns the visible shell, while primitives own option state, query state, semantics, and keyboard behavior.

## Public Architecture Contracts

### `Input.Wrapper`

`Input.Wrapper` owns field-level chrome:

- `label`
- `description`
- `error`
- `required` / `with_asterisk`
- wrapper and subpart classes
- generated ids for label, description, and error wiring
- stable state data attributes such as `data-disabled`, `data-error`, and `data-required`

Requirements:

- Render a consistent label, description, input slot, and error structure.
- Expose generated ids to the inner control.
- Merge generated `aria-describedby` values with control-provided values.
- Work for both native inputs and non-native trigger surfaces.

Non-goals:

- picker behavior
- combobox/select behavior
- clear-button visibility policy
- bordered shell rendering

### `Input`

`Input` owns the shared visual shell:

- `variant`
- `size`
- `radius`
- `disabled`
- `error`
- `left_section`
- `right_section`
- section sizing and placement defaults
- state data attributes such as `data-variant`, `data-size`, `data-disabled`, and `data-error`

Requirements:

- Render the bordered input box or trigger surface.
- Apply shared padding, spacing, and section layout.
- Reserve room for sections without component-specific padding recalculation.
- Host arbitrary component-specific inner content.

Non-goals:

- label/description/error rendering
- date, time, select, combobox, or color behavior
- picker state management

### `InputClearButton`

`InputClearButton` owns reusable clear-button markup and styling.

Requirements:

- Provide one consistent clear affordance.
- Standardize size, disabled behavior, aria label, and icon treatment.
- Be reusable by shell callers and higher-level adapters.

### `InputBase`

`InputBase` composes `Input.Wrapper` and `Input`.

Requirements:

- Accept the union of wrapper and shell props.
- Pass wrapper-generated ids and state down to the shell/control.
- Provide a slot for component-specific interactive content.

### Thin Adapters

Higher-level components should stay thin:

- `TextInput`: native text-entry adapter over `InputBase`
- `ColorInput`: input composition over `ColorPicker`
- `DateInput`: input composition over `DatePicker`
- `TimeInput`: input composition over `TimePicker`
- `Select`: trigger shell over select primitives
- `Autocomplete`, `MultiSelect`, `TagsInput`, `PillsInput`, `Combobox`, `VirtualizedCombobox`: shared shell over combobox primitives

Adapters should decide only their component-specific inner content and behavior wiring.

## Shared Prop Model

Shared prop structs should be introduced where Dioxus composition stays clean:

- `InputWrapperProps`
- `InputProps`
- `InputBaseProps`

Recommended public enums:

- `InputVariant`: `Default`, `Filled`, `Unstyled` if still valid after token finalization
- `InputSize`: start from currently proven size variants
- `InputClearSectionMode`: shared extraction of current clear/section precedence behavior when appropriate

Do not pre-create speculative future props. Start with props already proven by existing components and expand only when required by migration.

## Public API Compatibility

The compatibility stance is intentionally conservative for existing text-input consumers.

Requirements:

- `Input` becomes the low-level Mantine-style shell.
- `Input.Wrapper` becomes the field wrapper.
- `InputBase` becomes the wrapper-plus-shell convenience layer.
- `TextInput` becomes the public native text-entry component and inherits the current text-input behavior/API.
- Internal consumers of the old public `Input` text-entry API, including color-related consumers, must migrate to `TextInput`.

Current decided status:

- The architecture does not carry a broad compatibility shim that would prevent `Input` from becoming the low-level shell.
- The migration path is an explicit `Input` to `TextInput` move for old text-entry usage.

## Accessibility Contracts

Field-level accessibility is shared. Control semantics remain local.

Requirements:

- `Input.Wrapper` may own field-wrapper ids for label, description, and error.
- Controls and primitives retain control-specific roles and aria such as `aria-controls`, `aria-expanded`, `aria-activedescendant`, listbox wiring, segment roles, and picker semantics.
- Wrapper-generated `aria-describedby` must merge with control-provided values.
- Error wiring must preserve `aria-invalid` semantics while allowing control-specific attributes.
- Label association must work for native inputs and non-native trigger surfaces.

This prevents shared field chrome from clobbering primitive semantics for select, segmented date/time controls, and combobox-backed inputs.

## Clear and Section Precedence

Right-side affordance precedence is shared behavior.

Canonical slots:

- `left_section`
- `right_section`
- `clear_section`

Requirements:

- If `clearable` is false, no clear affordance is rendered.
- Disabled controls must not allow clearing.
- If `clearable` is true and the value is empty, the clear affordance may hide unless stable layout width is required.
- If both `right_section` and clear are present, preserve current clear precedence semantics:
  - `Both`: render clear plus right section
  - `Clear`: render only clear when visible
  - `RightSection`: render only right section and suppress clear

Compatibility mapping for current behavior must remain explicit if the shared type is renamed later.

## Styling Contract

This architecture depends on the CSS vars refactor contract and must not invent a parallel token system.

Requirements:

- Shared input styling consumes finalized unprefixed recipe tokens.
- Shared input styling must not introduce or depend on `--dxc-*` or `--dxc-input-*` tokens.
- Shared input styling must not rely on legacy palette/status variables remaining available.
- Shared shell styling should read from finalized input, surface, focus, danger, spacing, and radius recipe groups.
- Component-local private variables are acceptable only when they resolve from the shared recipe-token contract.
- Component-specific CSS should keep only behavior-specific layout and visuals.

Current frozen contract for this slice:

- Shared input CSS consumes the unprefixed recipe tokens from the CSS vars plan, including `--input`, `--input-fg`, `--input-border`, `--input-focus-border`, `--input-disabled`, `--danger`, and `--danger-border`.

Suggested shared selectors:

- root wrapper
- label
- required marker
- description
- error
- shell wrapper
- input/surface
- section
- clear button

Suggested shared data attributes:

- `data-disabled`
- `data-error`
- `data-required`
- `data-variant`
- `data-size`
- `data-position="left|right|clear"`

## Registry and Install Contracts

Packaging and installer metadata must treat `input` as the shared installable foundation.

Requirements:

- `input` is the shared dependency for components that import the shared shell foundation.
- Picker surfaces remain separately installable from input-composition components.
- `color_input`, `date_input`, and `time_input` are first-class installable entries distinct from `color_picker`, `date_picker`, and `time_picker`.
- Installer metadata must point at canonical crate-owned styled source rather than preview-local implementation files.

Required dependency shape:

- `color_input` depends on `input`, `popover`, and `color_picker`
- `date_input` depends on `input`, `popover`, `date_picker`, and picker dependencies such as `calendar`
- `time_input` depends on `input`, `popover`, and `time_picker` where applicable, plus existing time-selection dependencies
- `select` adds `input` once migrated
- `combobox` family entries add `input` where shared shell code is imported

Acceptance requirement:

- Installing `input`, `color_input`, `date_input`, `time_input`, `time_picker`, `date_picker`, `color_picker`, `select`, or `combobox` into a fresh downstream project must include the required shared input files from canonical styled source and compile without manual copying.

## Component Matrix

| Component | Role | Shared `input` dependency | Notes |
| --- | --- | --- | --- |
| `input` | Shared shell, wrapper, base composition | N/A | Exports `Input`, `Input.Wrapper`, `InputBase`, `TextInput`, `InputClearButton` |
| `text_input` or compatibility entry | Public text-entry adapter | Yes | Mirrors the current text-input behavior/API through `TextInput` |
| `color_picker` | Picker surface only | No | `Popover` belongs to `ColorInput` |
| `color_input` | Styled field composition | Yes | Depends on `input`, `popover`, `color_picker` |
| `date_picker` | Picker surface only | No | Remains separately installable |
| `date_input` | Styled field composition | Yes | Depends on `input`, `popover`, `date_picker`, `calendar` as applicable |
| `time_picker` | Picker surface only | No | Remains separately installable |
| `time_input` | Styled field composition | Yes | Depends on `input`, `popover`, `time_picker` where applicable |
| `select` | Styled trigger over primitives | Yes | Primitive listbox behavior remains intact |
| `combobox` | Styled combobox-family composition | Yes | Primitive combobox behavior remains intact |

## Implementation Status and Roadmap

### Phase 0: Contract Freeze and Installer Verification

Phase 0 is mandatory before migration work proceeds.

Decided status recorded for the 2026-06-11 foundation slice:

- Source ownership is frozen for this slice: canonical styled implementation lives in `dioxus-components/src/components/input/**`; `preview/src/components/input/**` remains demo/docs/registry/import surface only.
- Token contract is frozen for this slice around unprefixed recipe tokens and must not regress to `--dxc-*` or legacy palette variables.
- Compatibility stance is decided: `Input` is the low-level shell and `TextInput` is the public text-entry adapter.
- Registry/install shape is decided for future `color_input`, `date_input`, and `time_input` entries.
- Dependency shape is decided for those entries.

Open blocker that must remain visible:

- Fresh-install smoke on 2026-06-11 was blocked before install by the local non-interactive environment. `dx` was available, but `dx new "$tmpdir/app" --yes --vcs none` failed with `ERROR dx new: IO error: not a terminal`.

Follow-up status recorded on 2026-06-12:

- The same fresh-install workflow still failed before component install with `dx new: IO error: not a terminal`.
- Approved local substitute smoke was `dx components schema` plus registry dependency checks against `preview/src/components/**/component.json`.
- Fresh interactive-terminal or supported non-interactive `dx new` verification remains required for full downstream install confidence.

### Phase 1: Shared Foundation

Required outputs:

- `Input`
- `Input.Wrapper`
- `InputClearButton`
- `InputBase`
- `TextInput`
- shared enums and prop structs

Acceptance requirements:

- Existing components continue compiling and rendering unchanged until individually migrated.
- The foundation uses the frozen token and ownership contract.
- Manifests add `"input"` only when a component actually imports the shared foundation.

### Phase 2: Text Input Migration

Acceptance requirements:

- Existing text-input examples continue working through the decided `TextInput` path.
- The shared field shell owns common container props.
- Internal text-entry users of the old `Input` API migrate to `TextInput`.

### Phase 3: `TimePicker` / `TimeInput`

Acceptance requirements:

- `TimePicker` becomes time-selection surface and behavior only.
- `TimeInput` becomes the styled field composition.
- `time_input` exists as a separate installable entry.
- Existing clear/section behavior remains unchanged.

### Phase 4: `DatePicker`, `DateInput`, `DateRangePickerInput`

Acceptance requirements:

- `DatePicker` becomes date-selection surface only.
- `DateInput` becomes the single-value field composition.
- `DateRangePickerInput` uses the same shared shell geometry.
- `date_input` exists as a separate installable entry.

### Phase 5: `ColorPicker` / `ColorInput`

Acceptance requirements:

- `ColorPicker` is color-picking surface only.
- `ColorInput` is field composition around `ColorPicker`.
- `color_input` exists as a separate installable entry.

### Phase 6: `Select`

Acceptance requirements:

- `Select` and `SelectMulti` adopt the shared shell contract.
- Select primitives keep listbox state, keyboard behavior, and option semantics.
- Chevron/default trailing affordances move into shared section composition.

### Phase 7: Combobox-Backed Inputs

Affected components:

- `Combobox`
- `VirtualizedCombobox`
- `Autocomplete`
- `MultiSelect`
- `TagsInput`
- `PillsInput`

Acceptance requirements:

- Shared input shell styling replaces duplicated field-shell styling.
- Combobox primitives keep query state, option state, ids, and listbox semantics.
- Installer metadata adds `"input"` where shared shell code is imported.

This phase depends on combobox branch/artifacts being landed or cleanly rebased onto the ownership changes.

### Phase 8: Future Input-Like Components

Future components such as `NumberInput`, `MaskInput`, `Textarea`, and custom-trigger date/time variants should use `InputBase` unless they have a documented reason not to.

## Backward Compatibility

Migration should be incremental rather than one large refactor.

Requirements:

- Preserve old component-specific enums as aliases or conversion wrappers where practical during migration.
- Preserve current `TimePickerClearSectionMode` behavior:
  - `Both`
  - `Clear`
  - `RightSection`
- Preserve existing prop names where practical: `right_section`, `clearable`, `clear_section_mode`, `size`, `radius`, `variant`, `label`, `description`, `error`
- Keep primitive APIs stable unless a real behavior fix requires change.
- Update installer metadata in the same phase that first introduces a shared input dependency.

## Risks and Current Limitations

- Risk: the shared base becomes behavior-aware.
  - Mitigation: keep state, semantics, and behavior in primitives or thin adapters.
- Risk: picker surfaces drift back into input-shaped APIs.
  - Mitigation: keep the surface-vs-input split explicit for color, date, and time.
- Risk: implementation starts against unstable styling contracts.
  - Mitigation: treat CSS vars/source-ownership drift as a blocker.
- Risk: accessibility ids drift during migration.
  - Mitigation: centralize wrapper ids and describedby merging before broad migration.
- Risk: install confidence remains partial while `dx new` is blocked in the current non-interactive environment.
  - Mitigation: keep the blocker recorded and rerun the real fresh-install smoke from an interactive terminal or supported non-interactive path.

## Validation

Use the concrete repo validation contract from `AGENTS.md` and the source plan.

- Styled library build: `cargo check -p dioxus-components`
- Preview build for visual behavior: `scripts/preview-web.sh build`
- Targeted Playwright coverage from `playwright/` for the migrated component family
- CSS validation when CSS changes: from `preview/`, run `npx stylelint "src/**/*.css"`
- Fresh-install smoke workflow: create a temporary fresh Dioxus project, run `dx components add input`, add a minimal use of `TextInput` and `InputBase`, and run the downstream compile check
- Once `color_input`, `date_input`, and `time_input` exist, extend the same workflow with `dx components add color_input date_input time_input` and verify transitive shared-input installation

Recorded validation status that remains relevant to this architecture:

- 2026-06-11 fresh-install smoke: blocked at `dx new "$tmpdir/app" --yes --vcs none` with `ERROR dx new: IO error: not a terminal`
- 2026-06-12 fresh-install smoke rerun: blocked at the same step with `dx new: IO error: not a terminal`
- 2026-06-12 approved local registry/dependency substitute:
  - `dx components schema`
  - `rg -n '"componentDependencies"' preview/src/components/{input,color_input,date_input,time_input,color_picker,date_picker,time_picker,select,combobox}/component.json`
- 2026-06-12 broader completion-slice results from the source plan:
  - `cargo check -p dioxus-components`: passed
  - `scripts/preview-web.sh build`: passed
  - `cd preview && npx stylelint "src/**/*.css"`: passed
  - Playwright preflight was blocked by a non-project `localhost:8080` listener

This specification remains a living architecture document. The migration roadmap may advance, but the layering, ownership, accessibility, styling, packaging, and compatibility contracts above should remain the durable implementation target.
