# CSS Variables Architecture

## Purpose

The `dioxus-components` theme system provides a small semantic CSS variable contract that styled components consume directly.

The architecture replaces palette-scale variables, component-local theme drift, and `var(--dark, ...) var(--light, ...)` sentinels with source-level theme switching. Theme values are defined once on `:root`, `:root[data-theme="dark"]`, and `prefers-color-scheme`; components then read inherited semantic variables without carrying light and dark branches in component CSS.

This is a breaking contract. It does not preserve compatibility aliases for legacy palette variables, `--dxc-*` variables, or component theme aliases that only rename shared roles.

## Acceptance Summary

An implementation satisfies this specification only if:

- The crate-owned theme source of truth is `dioxus-components/assets/dx-components-theme.css`.
- Theme switching redefines source variables under `:root`, explicit `data-theme`, and `prefers-color-scheme`.
- Public color variables use one consistent value contract, currently OKLCH triplets.
- Components consume unprefixed semantic recipe variables directly for shared theme roles.
- Components keep `data-*`, pseudo-class, and ARIA selectors for state, then apply shared variables for the visual treatment.
- `--primary-color*`, `--secondary-color*`, `--focused-border-color`, old status palette pairs, `--dark`, `--light`, and `--dxc-*` are removed from active implementation files.
- Component-specific custom properties are limited to runtime values, prop-derived values, user data, layout math, or true domain concepts that shared recipes cannot represent.
- The theme has no numbered primary, secondary, or equivalent public palette scale.
- Repeated control, input, surface, list, and item spacing uses shared density roles or `calc(var(--space) * n)`.
- Preview remains a consumer/import/demo surface and does not become a parallel styling source.
- Linked-theme consumers and copied-component consumers migrate the new theme asset and migrated component files together.
- Visual regression coverage exists for preview component variants before the full rewrite proceeds.

## Layering

### primitives/

`primitives/` owns unstyled behavior, accessibility, state, events, props, and DOM structure. It must not own the design-token contract or styled component CSS.

### dioxus-components/

`dioxus-components/` owns the canonical styled reusable components and the public theme contract:

- `assets/dx-components-theme.css`
- `src/components/**/style.css`
- styled wrappers that import primitives and apply stable component CSS
- public style exports from the crate

Component CSS should read semantic variables directly. It should not invent local theme variables when a shared recipe covers the role.

### preview/

`preview/` owns demos, docs, metadata, routes, visual validation, and highlighted source display. It consumes the crate-owned theme asset and crate-owned component styles. Preview-local CSS is allowed only for demo framing or documentation examples.

### test-harness/

`test-harness/assets/dx-components-theme.css` is a consumer-side mirror or compatibility asset. It is not the source of truth and must follow the crate-owned contract.

## Consumer Model

There are two supported consumer paths:

- Linked theme consumers include the generated global theme asset once, documented as `/assets/dx-components.css`.
- Copied component consumers use `dx components add` to copy migrated styled component source that composes primitives and reads the new theme variables.

Because the rewrite is breaking, old copied components that still read removed variables are out of date. Consumers should regenerate or manually migrate copied components and include the new theme CSS once.

Consumers may override variables globally, inside a subtree, or around one component instance:

```css
:root {
  --accent: 42% 0.14 250;
  --accent-fg: 98% 0.01 75;
}

[data-dx-theme="brand"] {
  --surface: 98% 0.01 250;
  --surface-fg: 18% 0.02 250;
}
```

## Color Value Contract

Public color variables use OKLCH triplets:

```css
:root {
  --surface: 96% 0.012 75;
  --surface-fg: 18% 0.012 75;
  --accent: 40% 0.13 25;
}

.dx-card {
  background: oklch(var(--surface));
  color: oklch(var(--surface-fg));
  box-shadow: 0 0 0 3px oklch(var(--accent) / 0.24);
}
```

Triplets allow alpha composition without paired alpha tokens. Do not mix full color functions and triplets in the public token set.

If OKLCH support is not acceptable for the target browser matrix, implementation must pause and choose a different color format before changing component CSS. The architectural requirements still hold: source-level theme switching, direct semantic consumption, and no legacy palette scales.

## Global Token Contract

Tokens are visual recipe groups. A recipe is a small set of variables that travel together:

- Fill/background: `--<role>`
- Foreground: `--<role>-fg`
- Border/outline: `--<role>-border`

Recipe names intentionally have no library prefix. Components should choose the closest shared recipe for a visual state instead of creating local aliases.

### Foreground Roles

- `--fg`
- `--fg-muted`
- `--fg-faint`

Use standalone foreground roles for general text and icon emphasis inside ordinary surfaces. Use recipe foregrounds for content that belongs to a filled visual state.

### Surface Recipes

- `--surface`, `--surface-fg`, `--surface-border`
- `--surface-muted`, `--surface-muted-fg`, `--surface-muted-border`
- `--surface-hover`, `--surface-hover-fg`, `--surface-hover-border`
- `--surface-active`, `--surface-active-fg`, `--surface-active-border`
- `--surface-selected`, `--surface-selected-fg`, `--surface-selected-border`
- `--surface-disabled`, `--surface-disabled-fg`, `--surface-disabled-border`

Use surface recipes for pages, cards, neutral panels, list rows, tabs, selectable items, disabled rows, and neutral interaction states.

### Input Recipes

- `--input`, `--input-fg`, `--input-fg-muted`, `--input-fg-faint`, `--input-border`
- `--input-hover`, `--input-hover-fg`, `--input-hover-border`
- `--input-focus`, `--input-focus-fg`, `--input-focus-border`
- `--input-disabled`, `--input-disabled-fg`, `--input-disabled-border`

Use input recipes for text fields, textareas, selects, combobox triggers, date/time inputs, placeholders, field icons, units, clear buttons, and focused field shells.

### Overlay Recipes

- `--overlay`, `--overlay-fg`, `--overlay-border`

Use overlay recipes for popovers, dropdown menus, command palettes, tooltips, floating panels, and dialogs.

### Accent Recipes

- `--accent`, `--accent-fg`, `--accent-border`
- `--accent-hover`, `--accent-hover-fg`, `--accent-hover-border`
- `--accent-active`, `--accent-active-fg`, `--accent-active-border`
- `--accent-subtle`, `--accent-subtle-fg`, `--accent-subtle-border`

Use accent recipes for primary actions, branded selections, links, selected accent rows, and low-emphasis accent callouts.

### Status Recipes

- `--success`, `--success-fg`, `--success-border`
- `--success-subtle`, `--success-subtle-fg`, `--success-subtle-border`
- `--warning`, `--warning-fg`, `--warning-border`
- `--warning-subtle`, `--warning-subtle-fg`, `--warning-subtle-border`
- `--danger`, `--danger-fg`, `--danger-border`
- `--danger-hover`, `--danger-hover-fg`, `--danger-hover-border`
- `--danger-active`, `--danger-active-fg`, `--danger-active-border`
- `--danger-subtle`, `--danger-subtle-fg`, `--danger-subtle-border`
- `--info`, `--info-fg`, `--info-border`
- `--info-subtle`, `--info-subtle-fg`, `--info-subtle-border`

Use filled status recipes for high-emphasis status states. Use subtle status recipes for alerts, badges, toasts, validation messages, and other lower-emphasis status treatments.

### Focus

- `--focus`

Use `--focus` for focus-visible rings and outlines only. Do not use it as a general accent or brand color.

### Shape

- `--radius`
- `--radius-surface`
- `--radius-input`
- `--radius-control`

Radius follows a role-alias model:

```css
:root {
  --radius: 0.5rem;
  --radius-surface: var(--radius);
  --radius-input: var(--radius);
  --radius-control: var(--radius);
}
```

Consumers can tune all shape through `--radius` or split surface, input, and control shape independently.

### Density And Spacing

- `--space`
- `--surface-padding`
- `--surface-gap`
- `--content-gap`
- `--item-gap`
- `--control-height-sm`
- `--control-height-md`
- `--control-height-lg`
- `--control-padding-x-sm`
- `--control-padding-x-md`
- `--control-padding-x-lg`
- `--control-gap`
- `--input-padding-x-sm`
- `--input-padding-x-md`
- `--input-padding-x-lg`
- `--input-padding-y`
- `--list-item-padding-x`
- `--list-item-padding-y`

Spacing is not a general layout system. It covers repeated component-library needs: controls, inputs, surfaces, menus, lists, and small internal gaps. Page layout, marketing sections, grids, and app-specific spacing stay outside this contract.

Component authors should use spacing in this order:

1. Use an existing role token when one matches the job.
2. Use `calc(var(--space) * n)` for component-internal spacing that does not deserve a public token.
3. Promote a repeated spacing value only when it appears across multiple components with the same semantic purpose.

Do not add Tailwind-style public spacing scales such as `--space-xs`, `--space-sm`, `--space-md`, or `--space-1` through `--space-12`.

## Theme Switching

Theme switching happens by redefining source variables:

```css
:root {
  --surface: 96% 0.012 75;
  --surface-fg: 18% 0.012 75;
  --surface-border: 80% 0.012 75;
  --accent: 40% 0.13 25;
  --accent-fg: 98% 0.01 75;
  --accent-border: 36% 0.13 25;
  --focus: 48% 0.13 25;
  color-scheme: light;
  accent-color: oklch(var(--accent));
}

:root[data-theme="dark"] {
  --surface: 16% 0.008 75;
  --surface-fg: 92% 0.006 75;
  --surface-border: 28% 0.01 75;
  --accent: 72% 0.12 25;
  --accent-fg: 14% 0.008 75;
  --accent-border: 78% 0.12 25;
  --focus: 72% 0.12 25;
  color-scheme: dark;
}

@media (prefers-color-scheme: dark) {
  :root:not([data-theme="light"]):not([data-theme="dark"]) {
    --surface: 16% 0.008 75;
    --surface-fg: 92% 0.006 75;
    --surface-border: 28% 0.01 75;
    --accent: 72% 0.12 25;
    --accent-fg: 14% 0.008 75;
    --accent-border: 78% 0.12 25;
    --focus: 72% 0.12 25;
    color-scheme: dark;
  }
}
```

The real theme block must define every public token in both light and dark source contexts. Runtime theme toggles may keep writing `html[data-theme]`.

## Component Authoring Rules

Components should use global variables directly:

```css
.dx-input {
  background: oklch(var(--input));
  color: oklch(var(--input-fg));
  border: 1px solid oklch(var(--input-border));
  border-radius: var(--radius-input);
  min-height: var(--control-height-md);
  padding-inline: var(--input-padding-x-md);
}

.dx-input::placeholder {
  color: oklch(var(--input-fg-muted));
}

.dx-input:hover {
  background: oklch(var(--input-hover));
  color: oklch(var(--input-hover-fg));
  border-color: oklch(var(--input-hover-border));
}

.dx-input:focus-visible {
  background: oklch(var(--input-focus));
  color: oklch(var(--input-focus-fg));
  border-color: oklch(var(--input-focus-border));
  outline: 2px solid oklch(var(--focus));
  outline-offset: 2px;
}

.dx-input[data-invalid="true"] {
  background: oklch(var(--danger-subtle));
  color: oklch(var(--danger-subtle-fg));
  border-color: oklch(var(--danger-subtle-border));
}
```

Rules:

- Use `data-*`, ARIA, and pseudo-class selectors to identify state.
- Apply shared recipes for hover, active, selected, open, disabled, invalid, subtle status, filled status, destructive, and focus-visible states.
- Use recipe foregrounds for content that belongs to a filled state.
- Use `--fg`, `--fg-muted`, and `--fg-faint` for general text or icon emphasis inside ordinary surfaces.
- Use `--input-fg-muted` and `--input-fg-faint` inside input/control shells.
- Keep runtime plumbing variables such as `--progress-value`, `--toast-index`, `--toast-count`, `--depth`, `--depth-offset`, `--area-color`, `--swatch-color`, and `--event-color` only when they carry state, props, user data, or layout math.
- Do not add aliases such as `--input-bg`, `--tabs-fg`, `--schedule-border`, or `--button-hover-border` when an existing recipe already covers the role.
- Components must not redefine global recipe tokens on component roots.

## Token Comments

The theme file should keep comments next to variables. Comments are part of the design contract because they tell component authors when to use a token and reduce local alias drift.

Each recipe group should document intended usage. For example:

```css
/* Hover surface recipe. Use for neutral hover states on ghost controls,
   menu items, list rows, tabs, and selectable items. */
--surface-hover: 90% 0.014 75;
--surface-hover-fg: 12% 0.014 75;
--surface-hover-border: 72% 0.014 75;
```

## Variable Scope Contract

Unprefixed variables are intentionally ergonomic and intentionally scoped by CSS inheritance.

- The generated theme asset defines tokens on `:root`.
- Components read inherited tokens.
- Components must not redefine shared recipe tokens on component roots.
- Apps that already use generic variable names can wrap Dioxus components in a theme scope and define the same variables there.
- Scoped theme examples should use a wrapper such as `[data-dx-theme]` or an app-owned class.
- Runtime component variables may be component-scoped and prefixed when they represent layout math, prop-derived values, user data, or internal state.

## Migration Plan

### Phase 1: Token Spike

Before rewriting every component, map representative hard cases to the proposed recipes:

- one input field shell
- one ghost/text button or link
- one menu/list item with highlighted and selected states
- one tabs trigger variant
- one sidebar item
- one schedule event
- one badge or alert for each status tone used by the library

If a missing role appears in more than one component family, add it to the global recipe set. If it is truly domain-specific, keep it as a runtime or domain variable and document why it is not global.

### Phase 1.5: Visual Snapshot Baseline

Before migrating component CSS, add a Playwright visual-regression harness around the preview site:

- enumerate preview components and variants from `preview/src/components/**/component.json` and `variants/*`
- visit stable preview routes such as `/component/block/?name=<component>&variant=<variant>`
- capture the stable demo frame or component block
- run deterministic fixed viewports with animations disabled, stable color scheme, stable dates/timers where needed, and known font loading behavior
- capture at least light and dark baselines
- add mobile or high-risk responsive baselines where layout materially changes
- store snapshots under the `playwright/` snapshot tree
- provide separate commands for baseline updates and drift checks

The first baseline must be captured from the current implementation before Phase 2 begins.

### Phase 2: Replace The Theme Contract

- Rewrite `dioxus-components/assets/dx-components-theme.css`.
- Mirror the same token contract in `test-harness/assets/dx-components-theme.css`.
- Keep preview consuming the crate theme asset.
- Remove legacy palette variables and mode sentinels.
- Keep `html[data-theme]` and `prefers-color-scheme`, but switch source variables directly.
- Add base defaults for body, default border color, `accent-color`, and selection using the new recipes.

### Phase 3: Migrate Shared Control Shells

Migrate the duplicated field-shell family first:

- `input`
- `textarea`
- `select`
- `combobox`
- `date_picker`
- `time_picker`

For each component, replace palette reads with semantic tokens, use shared density roles, remove local color aliases, preserve `data-*` hooks, and keep primitive imports and behavior contracts intact.

### Phase 4: Migrate Common Display And Action Components

Migrate lower-risk display and action components next:

- `button`
- `badge`
- `item`
- `toolbar`
- `scroll_area`
- `card`
- `popover`
- `dropdown_menu`

Variants should map to semantic roles such as accent, muted, outline, danger, ghost, and link. Hover and active states should use complete recipes, including foreground and border tokens.

### Phase 5: Migrate High-Risk Components

Migrate high-risk components after the shared vocabulary has been proven:

- `schedule`
- `tabs`
- `sidebar`
- `color_picker`

Split theme roles from runtime variables. Theme roles become direct uses of surface, input, overlay, accent, and status recipes. Runtime or user-data values remain local variables.

### Phase 6: Documentation And Installer Surface

Update docs and installer expectations:

- document the new global token list
- document source-level theme switching
- show global override, scoped override, and one component-instance override
- state that old variables are removed
- update copied-component guidance so `dx components add` consumers migrate theme asset and component files together
- update prior docs that recommend `--dxc-*` or palette-scale variables

## Removal Matrix

| Current Variable Pattern | Rewrite Action |
| --- | --- |
| `--primary-color*` | Remove; replace with recipes such as `--accent`, `--accent-hover`, `--accent-subtle`, or `--surface-selected` |
| `--secondary-color*` | Remove; replace with recipes such as `--surface`, `--surface-muted`, `--surface-hover`, `--surface-active`, or `--overlay` |
| `--focused-border-color` | Remove; replace with `--focus` |
| `--primary-success-color` / `--secondary-success-color` | Remove; replace with success filled or subtle recipes |
| `--primary-warning-color` / `--secondary-warning-color` | Remove; replace with warning filled or subtle recipes |
| `--primary-error-color` / `--secondary-error-color` / `--contrast-error-color` | Remove; replace with danger filled, hover, active, or subtle recipes |
| `--primary-info-color` / `--secondary-info-color` | Remove; replace with info filled or subtle recipes |
| `--dark` / `--light` | Remove; replace with source-token overrides under `:root[data-theme]` and media queries |
| `--dxc-*` | Remove or rename to unprefixed equivalents |
| `--dx-*` component color/theme variables | Remove where shared recipes cover the role |
| `--schedule-*` theme variables | Remove where shared recipes cover the role; keep only true schedule runtime/layout variables |
| `--progress-value`, `--toast-index`, `--toast-count`, `--depth`, `--depth-offset`, `--area-color`, `--swatch-color`, `--event-color` | Keep only when they remain runtime/component-data plumbing rather than theme tokens |

## Rationale

Recipe groups fit this library because components usually theme a state as a complete visual treatment, not as unrelated color channels. A muted, hover, active, selected, disabled, input focus, accent, subtle status, or destructive state usually needs fill, foreground, and border values that belong together.

Rejected alternatives:

- Minimal global tokens fail common state needs and push authors back toward local variables.
- Per-component variables create a large component-specific API that prevents global theming of common states.
- Independent channel tokens make related state values ambiguous.
- Relative color derivation and `color-mix()` are useful future enhancements but are too browser-support-dependent for the core contract.
- Numbered palette scales encode palette position instead of UI intent.

## Validation

Implementation should run the smallest relevant checks for changed layers, then broader checks before completing the full migration.

Existing checks:

```sh
cargo test -p dioxus-primitives
cargo check -p dioxus-components
cd preview && npx stylelint "src/**/*.css"
cd playwright && npx playwright test tabs.spec.ts
cd playwright && npx playwright test schedule.spec.ts
cd playwright && npx playwright test color-picker.spec.ts
cd playwright && npx playwright test navbar.spec.ts
cd playwright && npx playwright test sidebar.spec.ts
```

Visual snapshot checks:

```sh
cd playwright && npx playwright test preview-visual.spec.ts --update-snapshots
cd playwright && npx playwright test preview-visual.spec.ts
```

Use `--update-snapshots` only when intentionally establishing or accepting a baseline. Migration work should normally run the compare command and investigate diffs before updating snapshots.

Search checks should include CSS, Rust inline `style:` strings, docs examples, and generated component templates:

```sh
rg -- '--(primary-color|secondary-color|focused-border-color|dark|light|dxc-)'
rg -- '--dx-.*(background|foreground|border|accent|ring|surface|fg)'
```

Minimum visual validation:

- default light theme
- default dark theme
- `prefers-color-scheme` dark with no explicit `data-theme`
- explicit `data-theme="light"` and `data-theme="dark"`
- scoped theme override on a subtree
- one component-instance override through a wrapper or inline style
- field states for input, textarea, select, combobox, date picker, and time picker
- small, default, and large density variants
- menu/list item rows
- popover/card padding
- icon and label gaps
- multiline input padding
- schedule responsive layout and event states
- sidebar collapsed state
- tabs variants and orientation
- color picker selected swatches

Accessibility validation:

- Recipe foreground/background pairs meet WCAG contrast targets for intended text sizes.
- Focus indicators using `--focus` meet WCAG 2.2 focus appearance expectations where practical.

Preview ownership validation:

- Preview `mod.rs` re-exports resolve to `dioxus_components::*`.
- Preview demos render while reading crate-owned `style.css` and `dx-components-theme.css` for highlighted source display.

## Open Decisions Before Implementation

- Confirm OKLCH browser support for the supported browser matrix.
- Confirm final recipe names before component migration starts.
- Confirm spacing and density are part of the first rewrite. This spec recommends including them because field/control migration otherwise preserves or reinvents hardcoded values.
- Confirm whether old copied components are unsupported after the rewrite or whether the CLI should provide an explicit migration command or warning.
