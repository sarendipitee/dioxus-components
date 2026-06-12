# Input unification plan

## Goal

Create common composable input foundations so text inputs, color inputs, date inputs, time inputs, selects, combobox-backed inputs, and future controls such as number inputs and masked inputs share sizing, spacing, sections, labels, descriptions, errors, disabled state, and clearable behavior.

The plan should explicitly follow Mantine's composition model because that split unlocks developer flexibility without forcing picker surfaces to masquerade as text inputs. It borrows Mantine's `Input`, `Input.Wrapper`, `InputBase`, and thin adapter model, but it must be implemented against this repo's current ownership boundaries and the CSS variable rewrite contract.

## Sequencing and dependency

This plan is intentionally sequenced after `docs/plans/css-vars-refactor-plan.md`.

Input unification must not invent a parallel styling contract or revive the old one. Implementation should begin only after one of these is true:

- The CSS vars refactor and source-ownership work has landed.
- The CSS vars refactor has not landed yet, but its canonical contracts are finalized enough that input unification can consume them without guessing.

At minimum, this plan depends on these upstream decisions being fixed before implementation starts:

- Canonical styled component ownership remains `dioxus-components/src/components/**`.
- `preview/src/components/**` remains the demo/registry/import surface rather than a second styling source of truth.
- The shared CSS variable contract for field shells is the unprefixed recipe-token system from the CSS vars plan, not a new `--dxc-*` layer and not legacy palette variables.

If those contracts are still moving, pause this work rather than encoding assumptions that will be immediately invalidated.

## Current state

The current input-like components do not share a common visual shell or a consistent picker-surface versus input-composition boundary.

- The styled text-input wrapper currently exposed through preview traces back to the installable styled component implementation, but this plan should treat the crate-owned styled source as canonical.
- There is no explicit `ColorPicker` versus `ColorInput` split yet, so future work needs a first-class architecture definition rather than treating color as an afterthought.
- `time_picker` has the richest field shell today: `label`, `description`, `error`, `variant`, `size`, `radius`, `right_section`, `clearable`, and `clear_section_mode`.
- `date_picker` and `DateRangePickerInput` duplicate input-group shell markup but do not expose the same shared props as `TimePicker`.
- `select` is a styling wrapper around select primitives, with trigger/list styling that does not yet come from a shared field shell.
- The `use-combobox-hook` line of work adds combobox-backed styled wrappers including `Combobox`, `VirtualizedCombobox`, `Autocomplete`, `MultiSelect`, `TagsInput`, and `PillsInput`.
- Combobox styling currently repeats field-surface rules that should eventually come from the shared input foundation rather than each component owning its own shell CSS.

The primitive layer should continue to own behavior, accessibility, state machines, keyboard interaction, roving focus, and popover or combobox behavior for picker surfaces.

The styled component layer should own visual composition and shared component APIs for field chrome and input composition:

- Canonical styled source: `dioxus-components/src/components/**`
- Preview/demo/import surface: `preview/src/components/**`

Any packaging/install assumptions in this plan must follow that ownership boundary. `component.json` and preview metadata are registry/import surface, not the canonical implementation source.

## Phase 0: Pre-implementation gate

Do not start component migration until Phase 0 is complete.

Phase 0 exists to turn the current validation blockers into implementation tasks with recorded outputs rather than leaving them as external prerequisites.

Required Phase 0 tasks:

- Freeze the CSS vars/source ownership contract enough for implementation:
  - Canonical styled component ownership remains `dioxus-components/src/components/**`.
  - `preview/src/components/**` remains the demo/registry/import surface rather than a second styling source of truth.
  - The shared field shell consumes the finalized unprefixed recipe-token system from `docs/plans/css-vars-refactor-plan.md`.
- Verify the actual `dx components add` transitive install smoke workflow for shared dependencies and record the exact command used in this plan before later phases start.
- Lock the public `Input` compatibility path before code edits:
  - `Input` becomes the low-level shell and field surface API.
  - `TextInput` becomes the public native text-entry component and inherits the current text-input behavior/API.
  - Internal consumers that use the current public `Input` text-entry API, including `color_picker`, must migrate to `TextInput` during implementation.
- Lock the picker surface versus input composition split for color, date, and time:
  - `ColorPicker`, `DatePicker`, and `TimePicker` are picker surfaces.
  - `ColorInput`, `DateInput`, and `TimeInput` are first-class input-composition components.
- For combobox phases, require the combobox branch/artifacts to be landed or cleanly rebased onto the ownership changes before that migration phase starts.

Phase 0 acceptance criteria:

- The input token contract is frozen enough that later phases can implement against it without inventing token names or ownership rules.
- The exact fresh-install smoke command/workflow is verified against the real installer path and recorded under `Validation commands`.
- The `Input` to `TextInput` migration path is documented as a decided implementation requirement, not an open question.
- The registry/install shape for `color_input`, `date_input`, and `time_input` is documented as a decided implementation requirement, not an open question.

This gate is mandatory because the main failure mode here is not implementation difficulty. It is locking in the wrong styling contract, the wrong source tree, an accidental public API break, or the wrong picker-versus-input boundary.

## Mantine model to borrow

Mantine separates input concerns into three layers:

- `Input`: low-level visual primitive for the bordered input surface, sizes, radius, variants, disabled/error state, left/right sections, loading, and clear-section plumbing.
- `Input.Wrapper`: field wrapper for label, description, error, required marker, and aria wiring.
- `InputBase`: composition layer that renders `Input.Wrapper` around `Input`.
- Higher-level components such as `TextInput`: thin adapters over `InputBase`.

Important transferable ideas:

- Keep shared box geometry and section layout in one primitive.
- Keep label/description/error and aria relationships in one wrapper.
- Make higher-level components thin adapters where possible.
- Put shared defaults and styling selectors at the base layer.
- Centralize clear/right-section conflict handling.
- Preserve component-specific behavior in the behavior primitive, not in the visual base.
- Treat pickers as picker surfaces first and as input compositions only when they are actually wrapped in a field trigger.

Primary references:

- <https://github.com/mantinedev/mantine/blob/master/packages/%40mantine/core/src/components/TextInput/TextInput.tsx>
- <https://github.com/mantinedev/mantine/blob/master/packages/%40mantine/core/src/components/InputBase/InputBase.tsx>
- <https://github.com/mantinedev/mantine/blob/master/packages/%40mantine/core/src/components/Input/Input.tsx>
- <https://github.com/mantinedev/mantine/blob/master/packages/%40mantine/core/src/components/Input/InputWrapper/InputWrapper.tsx>
- <https://mantine.dev/core/input/>
- <https://mantine.dev/styles/styles-api/>

## Proposed architecture

Introduce a shared input foundation in the canonical styled component layer. Do not move primitive behavior into it.

Architecturally, follow the Mantine split explicitly:

- Picker surfaces are the actual interactive picker UI.
- Input variants compose picker surfaces with shared field chrome.
- The styled input layer should make composition easy, but it must not absorb picker behavior that belongs in primitives.

Definitions for this repo:

- `ColorPicker`: only the color picking surface, such as the RGB square, hue slider, alpha slider, swatches, and any unstyled picker behavior/state/accessibility.
- `ColorInput`: `Input.Wrapper` + `Popover` + input trigger with left-section color preview + `ColorPicker` in the dropdown.
- `DatePicker`: only the calendar/date selection surface and date-selection behavior.
- `DateInput`: `Input.Wrapper` + `Popover` + text-like or segmented date trigger + `DatePicker` in the dropdown.
- `DateRangePickerInput`: range-specific input composition that still delegates calendar behavior to the date picker surface and primitives.
- `TimePicker`: only the time selection surface and time-selection behavior.
- `TimeInput`: `Input.Wrapper` + trigger/input chrome + `TimePicker` surface in dropdown or inline composition as appropriate.

This plan should optimize for nice composition and a clear separation between picker surface and input composition rather than preserving today’s blended shells.

Implementation ownership should follow the CSS vars/source-ownership plan:

- Shared styled source should live under `dioxus-components/src/components/input/` or an equivalent crate-owned canonical location.
- Preview should re-export/import that implementation for demos and registry/install surface purposes.
- Any installer metadata updates should point at the crate-owned styled source as the implementation of record.

The shared foundation must be installable with every component that depends on it. The packaging strategy is:

- Treat `input` as the shared installable input foundation.
- Create separate registry/install entries for `color_input`, `date_input`, and `time_input`.
- Keep picker surfaces separately installable as `color_picker`, `date_picker`, and `time_picker`.
- Add `"input"` to `componentDependencies` for every migrated component that imports shared input modules.
- Add picker/input-specific dependencies explicitly:
  - `color_input` depends on `input`, `popover`, and `color_picker`.
  - `date_input` depends on `input`, `popover`, and `date_picker` plus existing picker dependencies such as `calendar`.
  - `time_input` depends on `input`, `popover`, and `time_picker` if the surface is installed separately, plus existing time-selection dependencies.
- Do not assume preview-local files are the copied implementation source.

Acceptance criterion for the packaging approach: installing `input`, `color_input`, `date_input`, `time_input`, `time_picker`, `date_picker`, `color_picker`, `select`, or `combobox` into a fresh downstream project includes all shared input files from the canonical styled source and compiles without manual file copies.

### `Input.Wrapper`

Owns field-level chrome:

- `label`
- `description`
- `error`
- `required` / `with_asterisk`
- wrapper/root class
- label, description, and error classes
- generated ids for aria relationships
- optional input wrapper order if needed later

Responsibilities:

- Render a consistent label, description, input slot, and error structure.
- Provide stable data attributes such as `data-disabled`, `data-error`, and `data-required`.
- Own label/help/error ids and expose them to the inner control.
- Merge generated `aria-describedby` values with control-provided values instead of clobbering them.

Non-goals:

- It should not own select/date/time/combobox behavior.
- It should not own color/date/time picker behavior or picker popover state.
- It should not decide clear-button visibility.
- It should not own the actual bordered input surface.

### `Input`

Owns the shared input-like visual box:

- `variant`
- `size`
- `radius`
- `disabled`
- `error`
- `left_section`
- `right_section`
- section widths or section placement defaults
- pointer-events behavior for decorative versus interactive sections
- stable data attributes such as `data-variant`, `data-size`, `data-disabled`, `data-error`

Responsibilities:

- Render the bordered/focused input box or trigger surface.
- Apply consistent padding and section spacing.
- Reserve room for sections without each component recalculating padding.
- Expose slots for arbitrary component-specific contents, such as text input elements, time segments, date segments, or select trigger contents.

Non-goals:

- It should not render labels, descriptions, or errors.
- It should not know how to select dates, choose options, or parse time.
- It should not know how to pick colors, manage calendars, or manage time-selection state.

### `InputClearButton`

Owns reusable clear-button markup and styling.

Responsibilities:

- Provide one clear affordance with consistent size, disabled behavior, aria label, and icon.
- Be usable by `Input` callers or higher-level components.

### `InputBase`

Composes `Input.Wrapper` and `Input`.

Responsibilities:

- Accept the union of wrapper props and input props.
- Render `Input.Wrapper` around `Input`.
- Pass aria ids and state from wrapper to shell.
- Provide a slot for the component-specific interactive content.

This is the Dioxus equivalent of Mantine's `InputBase`.

### Thin adapters

Higher-level components should become adapters that decide only their behavior-specific content and then render it through `InputBase` or `Input`.

Examples:

- `TextInput`: raw text entry inside `InputBase`.
- `ColorPicker`: picker surface only, with no shared field chrome baked into it.
- `ColorInput`: input composition that renders the visible field through `InputBase` and places `ColorPicker` inside a popover dropdown.
- `TimePicker`: picker surface only; if an inline time-selection surface is needed, it renders through picker-specific layout without taking ownership of generic field wrapper concerns.
- `TimeInput`: time trigger or text-entry composition through `InputBase`, with clear/dropdown logic using shared clear and section slots while `TimePicker` continues to own selection behavior.
- `DatePicker`: picker surface only, with calendar state and selection behavior remaining local or primitive-owned.
- `DateInput`: date segments or formatted trigger inside `InputBase` or `Input`, while the date picker primitive continues to own calendar state.
- `DateRangePickerInput`: range segments inside the same shell.
- `Select`: select trigger content inside `InputBase` or `Input`, while select primitives continue to own option state, keyboard behavior, and listbox semantics.
- `Combobox` and `VirtualizedCombobox`: combobox search input and chevron rendered through `Input`/`InputBase`, while combobox primitives continue to own target attributes, query state, highlighted option state, and listbox semantics.
- `Autocomplete`: native search input rendered through the shared input container.
- `MultiSelect` and `TagsInput`: pill container rendered through the shared input container, with the inner search field kept visually unstyled inside the pill surface.
- `PillsInput`: reusable pill input surface rendered through `Input`, not through combobox-specific field-shell styling.

## Shared prop model

Create shared prop structs where Dioxus permits composition cleanly:

- `InputWrapperProps` for `label`, `description`, `error`, `required`, `with_asterisk`, and wrapper attributes.
- `InputProps` for `variant`, `size`, `radius`, `disabled`, `error`, sections, and input attributes.
- `InputBaseProps` for the combined high-level API.

Recommended public enums:

- `InputVariant`: `Default`, `Filled`, and `Unstyled`, if those variants still make sense after the CSS vars contract is finalized.
- `InputSize`: start with sizes already represented by `TimePickerSize`.
- `InputClearSectionMode`: extract from `TimePickerClearSectionMode` if it should be shared.

Do not create every future prop upfront. Start with props already proven by `TimePicker` and add only the ones needed to unify `Input`, `DatePicker`, `TimePicker`, `ColorInput`, and `Select`.

## Public API compatibility path

This migration should be conservative for existing `Input` consumers.

Required compatibility stance before implementation:

- `Input` becomes the low-level reusable shell/surface, Mantine-style.
- `Input.Wrapper` is the field-level wrapper for labels, descriptions, errors, and aria wiring.
- `InputBase` is the convenience composition of wrapper + shell.
- `TextInput` is the public native text-entry component and receives the current text-input behavior/API.
- Implementation must migrate internal consumers that currently rely on the public `Input` text-entry API, including `color_picker`, to `TextInput`.
- Do not silently repurpose the public `Input` API in a way that changes behavior for existing callers without an explicit migration stance.

Compatibility note:

- Because `Input` changes semantic role, implementation must choose one of two paths during the first migration phase and document it in release notes:
  - provide a short migration shim only if it can preserve the low-level `Input` API cleanly without compromising the Mantine-style shell role, or
  - treat the rename as a deliberate breaking pre-implementation API migration and move public text-entry consumers to `TextInput`.
- This choice is not deferred beyond implementation start. If a clean shim is not feasible, the plan treats the change as an explicit breaking migration rather than diluting the low-level `Input` surface.

## Accessibility merge rules

The shared wrapper must centralize field-level relationships without stealing primitive-specific accessibility behavior.

Rules:

- `Input.Wrapper` may own label, help, and error ids when the concern is field wrapper semantics or reusable field chrome.
- Primitives and control adapters retain behavior roles and control-specific aria such as `aria-controls`, `aria-expanded`, `aria-activedescendant`, listbox wiring, segment roles, picker semantics, and keyboard behavior.
- Wrapper-generated `aria-describedby` must merge with control-provided `aria-describedby` values rather than replacing them.
- Error state wiring should preserve `aria-invalid` semantics while allowing primitives to add their own required attributes.
- Label association must work for both native inputs and non-native trigger surfaces.
- Unstyled picker behavior, state, and accessibility stay in `primitives/`; styled field chrome and composition stay in `dioxus-components/`.

This split is important for `Select`, color/date/time picker surfaces, segmented controls, and combobox-backed inputs, where field chrome and behavior semantics overlap but should not overwrite each other.

## Clear and section rules

Use consistent precedence for right-side affordances.

Recommended slots:

- `left_section`: optional leading visual or action.
- `right_section`: optional component or user-provided trailing visual/action.
- `clear_section`: shared clear affordance rendered when clearable and the value can be cleared.

Recommended behavior:

- If `clearable` is false, never render the clear affordance.
- If disabled, never allow clear action.
- If `clearable` is true and the value is empty, reserve layout only if the component needs stable width. Otherwise hide the clear affordance.
- If both `right_section` and clear are present, preserve the current `TimePickerClearSectionMode` semantics during migration:
  - `Both`: render the clear affordance and the right section together.
  - `Clear`: render only the clear affordance when it is visible.
  - `RightSection`: render only the right section and suppress the clear affordance.

If the shared type is renamed later, provide an explicit compatibility mapping:

- `Both` maps to shared append/both behavior.
- `Clear` maps to shared replace-clear behavior.
- `RightSection` maps to shared suppress-clear behavior.

This should be implemented once and reused by `ColorInput`, `DateInput`, `TimeInput`, `Select`, and future text-like inputs.

## Styling contract

Define stable shared selectors and data attributes before migration, but do not invent a token contract here.

This plan must consume the finalized CSS vars recipe tokens from `docs/plans/css-vars-refactor-plan.md`. That means:

- Do not introduce or depend on prefixed `--dxc-*` or `--dxc-input-*` theme tokens.
- Do not rely on raw palette compatibility such as `--primary-color-*`, `--secondary-color-*`, `--focused-border-color`, or old status pairs continuing to exist.
- Do not create a parallel field-shell token matrix if the upstream CSS vars plan already defines input, surface, overlay, focus, danger, spacing, and radius recipes.

If exact final token names are still under review when implementation starts, stop and use the finalized CSS-vars recipe tokens once that review is done rather than guessing here.

Expected styling direction once upstream contracts are fixed:

- Shared field shells should read from the finalized unprefixed input/surface/focus/danger recipe groups.
- Shared sizing, radius, and density should read from the finalized role-based control/input spacing tokens.
- Component-local private variables are acceptable for internal layout resolution, but they should resolve from the finalized shared recipe tokens rather than from raw palette values.
- Component-specific CSS should keep only behavior-specific layout, such as date segment spacing, time segment separators, select list styling, combobox options, pills, calendar styling, color picker styling, and dropdown content.

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

## Migration phases

### Phase 0: Freeze contracts and verify installer workflow

Complete the pre-implementation gate as concrete implementation work.

Acceptance criteria:

- The input token and source-ownership contract is frozen enough for implementation and referenced from this plan.
- The real `dx components add` fresh-install smoke workflow is verified and the exact command is recorded in `Validation commands`.
- The public `Input` to `TextInput` migration path is documented, including whether a short shim is feasible or whether the migration is intentionally breaking.
- The registry/install entries for `color_input`, `date_input`, and `time_input` are defined as required outputs, with dependency expectations documented.

Phase 0 record for the Phase 1/2 foundation slice started 2026-06-11:

- Source ownership is frozen for this slice: canonical styled implementation lives in `dioxus-components/src/components/input/**`; `preview/src/components/input/**` is only the demo, docs, and registry/import surface.
- Token contract is frozen for this slice: shared input CSS consumes the unprefixed recipe tokens from the CSS vars plan, including `--input`, `--input-fg`, `--input-border`, `--input-focus-border`, `--input-disabled`, `--danger`, and `--danger-border`. The input foundation must not introduce `--dxc-*` tokens or depend on legacy palette variables.
- Compatibility stance is decided: `Input` is the low-level Mantine-style field shell. `TextInput` is the public native text-entry adapter that carries the old text-entry behavior and event/attribute surface. No broad compatibility shim is added in this slice because it would prevent `Input` from becoming the low-level shell. Direct internal old-`Input` text-entry consumers are migrated to `TextInput` as part of the foundation work.
- Registry/install shape is decided: `input` remains the shared installable foundation. Future `color_input`, `date_input`, and `time_input` entries are separate first-class input-composition entries from `color_picker`, `date_picker`, and `time_picker`.
- Dependency shape is decided: `color_input` depends on `input`, `popover`, and `color_picker`; `date_input` depends on `input`, `popover`, `date_picker`, and picker dependencies such as `calendar`; `time_input` depends on `input`, `popover`, and `time_picker` where applicable plus existing time-selection dependencies.
- Fresh-install smoke workflow is tracked under `Validation commands`. The required workflow is a real fresh downstream project install using `dx components add input` and, once those entries exist, `dx components add color_input date_input time_input`, followed by a downstream compile check.
- Fresh-install smoke result for 2026-06-11: blocked before install by the local non-interactive terminal environment. `dx` is available, but `dx new "$tmpdir/app" --yes --vcs none` exits with `ERROR dx new: IO error: not a terminal`. Rerun the recorded workflow from an interactive terminal or with a non-interactive `dx new` path once available.

Required verification:

- Styled library build: `cargo check -p dioxus-components`
- Preview build for visual behavior: `scripts/preview-web.sh build`
- The exact fresh-install smoke command once verified

### Phase 1: Add shared foundation without changing existing components

Add shared canonical styled-component modules for:

- `Input`
- `Input.Wrapper`
- `InputClearButton`
- `InputBase`
- `TextInput`
- shared enums and prop structs
- shared crate-owned CSS module or style exports

Acceptance criteria:

- Existing components still compile and render unchanged.
- New base components have small examples or tests that prove label, description, error, disabled, section, and size/radius data attributes render.
- The shared foundation uses the Phase 0 token and ownership contract without adding parallel token names.
- Migrated component manifests list `"input"` in `componentDependencies` only once a component is actually migrated to import shared input code, not during the initial foundation phase before any existing component imports change.
- Registry manifests and install metadata are updated consistently once components start importing the shared foundation.
- At least one fresh-install smoke check proves a component depending on the shared foundation includes the shared files through the real transitive install workflow recorded in Phase 0.

Required verification:

- Targeted render coverage for the new base components.
- Targeted browser coverage for representative field-shell states.
- Targeted accessibility coverage for wrapper aria/describedby wiring, label, description, and error relationships.
- The transitive install smoke workflow identified and recorded in Phase 0.

### Phase 2: Migrate generic text input path conservatively

Turn the canonical styled input implementation into the shared low-level field container while preserving an explicit text-input compatibility path.

Acceptance criteria:

- Existing text-input examples keep working through the agreed compatibility strategy.
- The shared field shell owns the common visual container props.
- `TextInput` owns the raw text-entry adapter API.
- The public docs explain the split between low-level `Input`, `Input.Wrapper`, `InputBase`, and `TextInput`.
- Internal consumers of the old public `Input` text-entry API are migrated to `TextInput`.
- No existing `Input` consumer loses behavior silently because the public API was repurposed without the documented migration strategy.

Required verification:

- Targeted render coverage for both the low-level shell and the compatibility text-input path.
- Targeted browser coverage for text-input interaction and states.
- Targeted accessibility coverage for native-input labeling and error wiring.

### Phase 3: Split and migrate `TimePicker` / `TimeInput`

Split the current primitive-rich shell so picker behavior stays primitive or unstyled, while input composition moves into the styled layer.

Acceptance criteria:

- `TimePicker` is defined as the time selection surface and behavior only.
- `TimeInput` is defined as the styled input composition that uses shared field chrome and can host `TimePicker` in dropdown or inline composition.
- `time_input` exists as a first-class registry/install entry separate from `time_picker`.
- `TimePicker` no longer owns generic label/description/error/variant/size/radius/right-section layout.
- Its behavior-specific time segment, dropdown, presets, and time-selection logic remain local or primitive-owned.
- `TimePickerClearSectionMode`, `TimePickerSize`, and `TimePickerVariant` either become shared aliases or migrate to shared input enums with compatibility shims.
- Existing `TimePickerClearSectionMode::Both`, `TimePickerClearSectionMode::Clear`, and `TimePickerClearSectionMode::RightSection` behavior remains unchanged.
- Ownership is explicit: primitive picker behavior in `primitives/`, styled composition in `dioxus-components/`.

Required verification:

- Targeted render coverage for updated markup and section layout.
- Targeted browser coverage for clear/dropdown/focus behavior and popover input composition.
- Targeted accessibility coverage for wrapper ids plus time-control semantics.

### Phase 4: Split and migrate `DatePicker`, `DateInput`, and `DateRangePickerInput`

Define `DatePicker` as the calendar/date selection surface, then use the shared input container for `DateInput` and `DateRangePickerInput`.

Acceptance criteria:

- `DatePicker` is the date selection surface only.
- `DateInput` is the single-value styled input composition that wraps `DatePicker`.
- `date_input` exists as a first-class registry/install entry separate from `date_picker`.
- Single-date and range inputs share the same shell geometry as the rest of the field family.
- Calendar state, disabled date logic, locale formatting, popover state, and roving focus remain in the date picker implementation and primitives.
- Date segment composition remains component-specific.

Required verification:

- Targeted render coverage for single-date and range field shells.
- Targeted browser coverage for popover/open/selection flows and popover input composition.
- Targeted accessibility coverage for wrapper labeling plus segmented/date-specific semantics.

### Phase 5: Add and migrate `ColorPicker` / `ColorInput`

Adopt the same composition model for color selection rather than introducing a one-off API.

Acceptance criteria:

- `ColorPicker` is defined as the color picking surface only, with no baked-in field wrapper chrome.
- `ColorInput` is defined as `Input.Wrapper` + `Popover` + input trigger with left-section color preview + `ColorPicker` in the dropdown.
- `color_input` exists as a first-class registry/install entry separate from `color_picker`.
- Shared field-shell props such as `label`, `description`, `error`, `size`, `radius`, `variant`, sections, and clear behavior live in the input composition rather than the picker surface.
- Color-specific behavior such as color state, hue/alpha interaction, swatches, and picker accessibility stay local or primitive-owned.
- Registry/install metadata includes the required separate `color_input` entry and dependencies.

Required verification:

- Targeted render coverage for color input field-shell states and left-section preview behavior.
- Targeted browser coverage for popover open/close, color selection, clear behavior, keyboard flows, and popover input composition.
- Targeted accessibility coverage for wrapper labeling plus picker-specific semantics.

### Phase 6: Migrate `Select`

Wrap the select trigger surface in `Input` or `InputBase` while keeping select primitive behavior intact.

Acceptance criteria:

- `Select` and `SelectMulti` get the shared sizing, radius, disabled, error, label, description, and section contract.
- The select primitive continues to own listbox state, keyboard navigation, option selection, and `data-state`.
- The chevron becomes a default right section instead of hardcoded trigger-specific layout.
- Clearable select behavior uses the shared clear/section rules if clearable is introduced.

Required verification:

- Targeted render coverage for select trigger shell states.
- Targeted browser coverage for open, highlight, selection, and disabled flows.
- Targeted accessibility coverage for trigger/listbox wiring plus merged wrapper metadata.

### Phase 7: Migrate combobox-backed inputs

This phase depends on the combobox branch/artifacts being landed or rebased after the CSS vars/source ownership normalization.

Affected styled components:

- `Combobox`
- `VirtualizedCombobox`
- `Autocomplete`
- `MultiSelect`
- `TagsInput`
- `PillsInput`

Acceptance criteria:

- `combobox` installer metadata lists `"input"` in `componentDependencies` where shared input code is imported.
- `Combobox` and `VirtualizedCombobox` render their search target through the shared input container, with the chevron supplied as a default `right_section`.
- `Autocomplete` uses the shared field shell for the visible search field rather than standalone field-shell styling.
- `MultiSelect`, `TagsInput`, and `PillsInput` use the shared input container for the pill surface.
- The inner search field inside pill inputs remains visually unstyled and flexible inside the shared container.
- Combobox primitives continue to own query state, target attributes, event target attributes, highlighted option state, option submission, virtualized option ids/indexes, and listbox semantics.
- Component-specific combobox CSS keeps listbox, option, empty state, check icon, pill, and demo styling, but no longer owns generic input height, padding, radius, background, border, focus ring, placeholder, or disabled surface styling.

Required verification:

- Targeted render coverage for each combobox-backed field-shell variant.
- Targeted browser coverage for autocomplete, multi-select, tags input, pills input, and virtualized combobox flows.
- Targeted accessibility coverage for merged wrapper ids plus combobox/listbox-specific aria such as `aria-controls` and `aria-activedescendant`.
- Installer smoke coverage for combobox transitive dependency installation.

### Phase 8: Future components

Require new input-like components to use `InputBase` unless they have a documented reason not to.

Candidate components:

- `NumberInput`
- `MaskInput`
- `Textarea`
- date/time variants with custom triggers

Acceptance criteria:

- New controls do not reimplement label, description, error, sections, clear button, size, radius, or disabled shell styling.
- Component-specific behavior stays local or in primitives.

## Backward compatibility strategy

Avoid a single breaking refactor.

- Keep old component-specific enums as aliases or conversion wrappers during migration.
- Preserve current `TimePickerClearSectionMode` variants and behavior:
  - `Both`: clear plus right section.
  - `Clear`: clear only.
  - `RightSection`: right section only.
- Preserve existing prop names where practical, especially `right_section`, `clearable`, `clear_section_mode`, `size`, `radius`, `variant`, `label`, `description`, and `error`.
- Preserve the existing text-input public API through `TextInput`, a compatibility wrapper, or both until explicit deprecation/removal is separately planned.
- Document migration implications for picker naming clearly: surface-only APIs (`ColorPicker`, `DatePicker`, `TimePicker`) should not promise field chrome, while `ColorInput`, `DateInput`, and `TimeInput` are the styled field-entry variants.
- Keep primitive APIs stable unless a behavior bug requires a primitive change.
- Prefer additive docs/examples before removing old component-specific paths.
- Update installer metadata and dependency metadata in the same phase that first makes any component depend on shared input files.

## Affected component and dependency matrix

| Component | Surface vs input role | Shared `input` dependency | Additional dependency notes | Registry/checklist notes |
| --- | --- | --- | --- | --- |
| `input` | Shared low-level shell, wrapper, and base composition | N/A | Canonical shared foundation | Export `Input`, `Input.Wrapper`, `InputBase`, `TextInput`, `InputClearButton` |
| `text_input` or existing `input` compatibility entry | Public text-entry adapter | Yes | `TextInput` receives the current text-input behavior/API; optional short shim only if it does not compromise low-level `Input` | Ensure docs and manifest reflect the chosen migration strategy |
| `color_picker` | Picker surface only | No for primitive surface; yes only if styled inline demos compose it | `Popover` belongs to `ColorInput`, not the picker surface itself | Add registry manifest entry and acceptance coverage |
| `color_input` | Styled field composition around `ColorPicker` | Yes | Depends on `input`, `popover`, and `color_picker` | Add separate registry/install entry |
| `date_picker` | Picker surface only after migration | No for picker surface | Preserve picker dependencies such as `calendar` | Remains separately installable |
| `date_input` | Styled field composition around `DatePicker` | Yes | Depends on `input`, `popover`, and `date_picker` | Add separate registry/install entry |
| `time_picker` | Picker surface only after migration | No for picker surface | Preserve time-selection primitive dependencies | Remains separately installable |
| `time_input` | Styled field composition around `TimePicker` | Yes | Depends on `input`, `popover`, and `time_picker` where applicable | Add separate registry/install entry |
| `select` | Styled trigger composition over primitives | Yes | Keep select primitive/listbox dependencies intact | Add `input` to manifest once migrated |
| `combobox` | Styled combobox family composition over primitives | Yes | Preserve combobox primitive dependencies | Add `input` to manifest once migrated |

## Registry manifest checklist

When migration reaches registry-backed components, verify all relevant manifests and installer metadata:

- `time_picker`: keep picker-surface-only dependencies unless a temporary migration note explicitly documents a short-lived shared-field import during the split.
- `time_input`: add a separate registry/install entry with `"input"`, `popover`, and `time_picker` dependencies as applicable.
- `date_picker`: keep the picker-surface entry separately installable.
- `date_input`: add a separate registry/install entry with `"input"`, `popover`, and `date_picker` dependencies.
- `select`: add `"input"` to `componentDependencies` when trigger shell styling moves to the shared input foundation.
- `combobox`: add `"input"` to `componentDependencies` for `Combobox`, `VirtualizedCombobox`, `Autocomplete`, `MultiSelect`, `TagsInput`, and `PillsInput` as applicable.
- `color_picker`: add a manifest entry for the picker surface.
- `color_input`: add a separate registry/install entry with `"input"`, `popover`, and `color_picker` dependencies.
- `text_input` or compatibility `input` entry: ensure manifest/export/docs match the chosen migration strategy.

## Risks and mitigations

- Risk: shared base becomes too behavior-aware.
  - Mitigation: enforce that date, time, select, and combobox state remains in primitives or component adapters.

- Risk: picker surfaces drift back into input-shaped APIs because existing components currently mix shell and behavior.
  - Mitigation: enforce the surface-versus-input split explicitly for color, date, and time before implementation begins.

- Risk: implementation starts before the CSS vars/source ownership contract is stable.
  - Mitigation: enforce the pre-implementation gate and treat upstream contract drift as a blocker, not something to paper over locally.

- Risk: CSS regressions across several components.
  - Mitigation: migrate one component at a time and require targeted render, browser, and accessibility coverage for each migrated family.

- Risk: accessibility ids are inconsistent during migration.
  - Mitigation: make `Input.Wrapper` responsible for field-level ids and describedby merging before migrating multiple components.

- Risk: migration confusion because current `Input` is a text input but the target `Input` is a Mantine-style low-level field shell.
  - Mitigation: make `TextInput` the public text-entry target, migrate internal consumers immediately, and use a short shim only if it does not compromise the low-level `Input` API.

## Decided constraints

- This plan is downstream of the CSS vars/source-ownership work and must consume its finalized contracts.
- Canonical styled source is crate-owned under `dioxus-components/src/components/**`; preview is the demo/registry/import surface.
- Shared field-shell styling must use the finalized unprefixed recipe-token groups rather than a new prefixed token family.
- Do not add custom `Input.Wrapper` ordering in this migration.
- Declare shared input dependencies through `componentDependencies`, and verify the transitive install smoke workflow in Phase 0 before migration phases begin.
- Mantine-style composition is the preferred API direction because it improves developer flexibility and keeps picker surfaces reusable outside input wrappers.
- Picker surfaces remain primitive/base behavior; input composition remains a styled-layer concern.
- `Input` is the low-level shell, `TextInput` is the public native text-entry component, and current `Input` text-entry consumers migrate accordingly.
- `ColorInput`, `DateInput`, and `TimeInput` are required first-class composition components with separate registry/install entries from their picker surfaces.

## Suggested implementation order

1. Complete Phase 0 and record the verified install smoke command.
2. Add the shared foundation and its targeted render/browser/a11y coverage.
3. Migrate the generic text-input path conservatively with an explicit compatibility wrapper/story.
4. Split and migrate `TimePicker` / `TimeInput`.
5. Split and migrate `DatePicker`, `DateInput`, and `DateRangePickerInput`.
6. Add and migrate `ColorPicker` / `ColorInput`.
7. Migrate `Select` and `SelectMulti`.
8. Migrate the combobox-backed family after the combobox branch/artifacts are landed or rebased onto the normalized ownership model.
9. Add a contributor note that new input-like components must use the shared base.

## Validation commands

Use concrete repo validation from `AGENTS.md` instead of generic placeholders:

- Styled library build: `cargo check -p dioxus-components`
- Preview build for visual behavior: `scripts/preview-web.sh build`
- Targeted Playwright coverage from `playwright/` for the migrated component family. Files can be chosen during implementation, but coverage must include wrapper aria/describedby wiring, section composition, popover input composition, and at least one fresh install flow.
- CSS validation when CSS changes: from `preview/`, run `npx stylelint "src/**/*.css"`
- Fresh-install smoke command: record the exact verified `dx components add ...` command/workflow in Phase 0 and reuse it in later phases
- Phase 0 fresh-install smoke workflow for this slice: create a temporary fresh Dioxus project, run `dx components add input`, add a minimal use of `TextInput` and `InputBase`, and run the downstream compile check. Once `color_input`, `date_input`, and `time_input` registry entries exist, extend the same workflow with `dx components add color_input date_input time_input` and verify the transitive `input` dependency files are copied from the canonical styled source. Result for 2026-06-11 foundation slice: blocked before install; `dx` is available, but `dx new "$tmpdir/app" --yes --vcs none` exits with `ERROR dx new: IO error: not a terminal`.
- Fresh-install smoke rerun for 2026-06-12 picker/input split slice: `tmpdir="$(mktemp -d)"; dx new "$tmpdir/app" --yes --vcs none` still fails before component install with `dx new: IO error: not a terminal`.
- Approved local registry/dependency smoke substitute for 2026-06-12 picker/input split slice:
  - `dx components schema`: passed with no warnings or errors.
  - `rg -n '"componentDependencies"' preview/src/components/{input,color_input,date_input,time_input,color_picker,date_picker,time_picker,select,combobox}/component.json`: passed and confirmed `color_picker` and `date_picker` do not depend on `input` or `popover`, while `color_input` depends on `input`, `popover`, and `color_picker`, and `date_input` depends on `input`, `popover`, `calendar`, and `date_picker`.
- Latest final completion slice results for 2026-06-12:
  - `cargo check -p dioxus-components`: passed. Existing warnings remained in `dioxus-primitives` for unused schedule/textarea helpers.
  - `scripts/preview-web.sh build`: passed. Existing unused helper warnings remained in primitive/preview schedule code.
  - `cd preview && npx stylelint "src/**/*.css"`: passed.
  - `cd playwright && npx playwright test input.spec.ts combobox.spec.ts --project=chromium`: blocked before tests by Playwright web server preflight because `http://localhost:8080` is already used. The stale project preview server `node ../playwright/serve-preview.mjs ../target/dx/preview/debug/web/public 8080` was stopped, but OrbStack still owns the wildcard `*:8080` listener and is not a project preview process.
  - `cd playwright && npx playwright test input.spec.ts combobox.spec.ts --project=webkit`: blocked by the same non-project `localhost:8080` listener before tests.
  - `dx components schema`: passed and printed the component JSON schema.
  - Registry/dependency smoke substitute: `rg -n '"componentDependencies"' preview/src/components/{input,color_input,date_input,time_input,color_picker,date_picker,time_picker,select,combobox}/component.json` passed and confirmed `time_input` depends on `input`, `popover`, and `time_picker`; `color_input` depends on `input`, `popover`, and `color_picker`; `date_input` depends on `input`, `popover`, `calendar`, and `date_picker`; `select` and `combobox` depend on `input`; picker-only entries do not depend on `input` or `popover`.
  - Fresh install smoke remains non-terminal-blocked at `dx new "$tmpdir/app" --yes --vcs none` with `dx new: IO error: not a terminal`; the registry/dependency smoke above remains the local substitute.
