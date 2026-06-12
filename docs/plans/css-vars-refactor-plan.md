# CSS Vars Refactor Plan

## Goal

Replace the current palette-scale and component-local variable drift with a small semantic CSS variable system that components consume directly.

This is a full rewrite. The plan intentionally does not preserve legacy CSS variable names, palette scales, or the current `var(--dark, ...) var(--light, ...)` switching pattern.

The target shape is:

- Theme source variables switch at `:root`, `:root[data-theme="dark"]`, and `@media (prefers-color-scheme: dark)`.
- Components consume shared semantic variables directly.
- Variables are unprefixed recipe groups: `--surface`, `--surface-fg`, `--surface-border`, `--accent`, `--accent-fg`, etc.
- Component-specific variables are rare and reserved for true runtime or domain-specific values that cannot be represented by shared tokens.
- `--primary-color-*`, `--secondary-color-*`, status palette pairs, `--focused-border-color`, `--dark`, and `--light` are removed.
- No Tailwind dependency.

## Non-Goals

- Do not maintain compatibility aliases for old variables.
- Do not keep old palette scales during migration.
- Do not make every component define its own design tokens.
- Do not move theme responsibility into component styles.
- Do not replace useful `data-*` state and variant hooks with variables.
- Do not tokenize every pixel value, animation value, layout calculation, or runtime measurement.

## Current State

### Global Theme Assets

The styled theme source of truth now lives in the `dioxus_components` crate at:

- `dioxus-components/assets/dx-components-theme.css`

`test-harness/assets/dx-components-theme.css` is a consumer-side mirror/compatibility asset, not a source of truth.

The preview app is now a consumer of the crate theme asset. `preview/build.rs` reads `../dioxus-components/assets/dx-components-theme.css` for code display, but `preview` is not the styling source of truth.

Both define a palette-oriented system:

- `--primary-color`, `--primary-color-1` through `--primary-color-7`
- `--secondary-color`, `--secondary-color-1` through `--secondary-color-6`
- `--focused-border-color`
- `--primary-success-color`, `--secondary-success-color`
- `--primary-warning-color`, `--secondary-warning-color`
- `--primary-error-color`, `--secondary-error-color`, `--contrast-error-color`
- `--primary-info-color`, `--secondary-info-color`

The crate-owned theme asset also uses mode sentinels today:

```css
--token: var(--dark, dark-value) var(--light, light-value);
```

That pattern is the wrong responsibility boundary for this library. Every component that wants a themed value has to carry both branches of the theme. The rewrite should instead switch the variable source once and let component declarations remain simple:

```css
:root {
  --surface: 96% 0.012 75;
  --surface-fg: 18% 0.012 75;
  --surface-border: 80% 0.012 75;
}

:root[data-theme="dark"] {
  --surface: 16% 0.008 75;
  --surface-fg: 92% 0.006 75;
  --surface-border: 28% 0.01 75;
}

.dx-card {
  background: var(--surface);
  color: var(--surface-fg);
  border-color: var(--surface-border);
}
```

### Component Patterns

The component layering is now split across three surfaces. The bare primitive source of truth lives in the `dioxus_primitives` crate under `primitives/src/**`. The styled component source lives in the `dioxus_components` crate under `dioxus-components/src/components/**`, where each styled component imports the primitive and applies the component CSS. The preview tree is only the registry/demo/import surface.

The current ownership split is:

- `primitives/src/**` contains bare behavior, state, accessibility, and functionality. It should not own the design-token contract or component styling.
- `dioxus-components/src/components/**` contains styled wrappers that import `dioxus_primitives` components or props, apply `#[css_module(".../style.css")]`, and own the component `style.css` files.
- `dioxus-components/src/lib.rs` re-exports `components::*` and `styles::{COMPONENT_CSS, THEME_CSS}`.
- `preview/src/components/**` is a consumer/import surface. Its `mod.rs` files re-export from `dioxus_components`, and preview variants import crate components for demos.
- `preview/build.rs` renders highlighted code by reading crate-owned `style.css` files and the crate-owned theme asset.

Styled components generally use:

- `dx-*` root and part classes.
- `data-*` attributes for state and variants, including `data-state`, `data-disabled`, `data-size`, `data-style`, `data-variant`, `data-selected`, `data-color`, `data-orientation`, and `data-slot`.
- CSS modules via `#[css_module("...")]` in crate-owned component implementations, plus preview-local demo CSS where a variant needs presentation-only framing.
- `component.json` metadata in preview as registry/demo metadata, not as the styling source of truth.
- Static CSS files per component in the crate.

Current custom property usage is fragmented:

- Global palette scales: `--primary-color-*`, `--secondary-color-*`.
- Planned prefixed semantic vars in prior docs: `--dxc-*`.
- Component-local design tokens: `--schedule-*`, `--dx-tabs-*`, `--dx-time-picker-radius`, sidebar vars, textarea vars.
- Runtime variables: `--progress-value`, `--toast-index`, `--toast-count`, `--event-color`, `--area-color`, `--swatch-color`, `--depth`, `--depth-offset`.

Inventory findings:

- `schedule` has the most complete local token layer, but many of those tokens are just local reinventions of surface, border, focus, muted foreground, radius, and state colors.
- `tabs` has its own `--dx-tabs-*` system, which should collapse into shared variables plus a few true tabs-only layout values if needed.
- `input`, `textarea`, `select`, `combobox`, `date_picker`, and `time_picker` duplicate a field-shell pattern: surface, foreground, placeholder/muted text, border, focus ring, disabled state, invalid/error state, radius, and hover surface.
- `progress`, `toast`, `table_of_contents`, `color_picker`, and schedule primitives use custom properties as runtime plumbing. These are not theme tokens and should not be forced into the global token set.
- Some split pane examples reference `var(--primary-color-11)`, while the shared theme inventory only showed `--primary-color` through `--primary-color-7`. The rewrite should remove this entire palette-scale dependency rather than add more scale values.

## Downstream Consumer Model

There are two consumer paths:

1. Linked theme consumers:
   Consumers install styled components and link the generated global theme asset, documented as `/assets/dx-components.css`.

2. Copied component consumers:
   `dx components add` copies the styled component source that is owned by the `dioxus_components` crate. That styled source composes primitives from `dioxus_primitives` and applies the crate-owned CSS. The preview tree is the registry/demo surface, not the canonical implementation.

Because this is a full breaking rewrite, copied components and linked theme consumers should migrate together. The new contract is not “old vars keep working.” The new contract is:

- Include the new theme CSS once.
- Components consume the unprefixed semantic variables.
- Consumers override variables globally, in a subtree, or on a wrapper around one component.
- Existing copied components using old palette vars are out of date and should be regenerated or manually migrated.

## Proposed Token Architecture

### Color Value Contract

Use OKLCH triplets for public color variables.

```css
--surface: 96% 0.012 75;
background: var(--surface);
box-shadow: 0 0 0 3px oklch(var(--accent) / 0.24);
```

Triplets are preferred because they allow alpha composition without inventing paired alpha tokens. Do not mix full color functions and triplets in the same public token set.

If browser support for OKLCH is not acceptable, this rewrite should pause and choose a different color contract before implementation. The rest of the architecture still stands: theme values switch at the source, components consume semantic roles directly, and old palette scales are removed.

### Global Token Set

Use visual recipe groups instead of independent color channels. A recipe is a small set of variables that travel together:

- Fill/background: `--<role>`
- Foreground: `--<role>-fg`
- Border/outline: `--<role>-border`

This is more explicit than a tiny token set, but it avoids the worse failure mode where every component invents its own state border and foreground variables. Components should pick the closest shared recipe for a visual state.

These names intentionally have no library prefix.

Surface recipes:

- `--fg`
- `--fg-muted`
- `--fg-faint`
- `--surface`
- `--surface-fg`
- `--surface-border`
- `--surface-muted`
- `--surface-muted-fg`
- `--surface-muted-border`
- `--surface-hover`
- `--surface-hover-fg`
- `--surface-hover-border`
- `--surface-active`
- `--surface-active-fg`
- `--surface-active-border`
- `--surface-selected`
- `--surface-selected-fg`
- `--surface-selected-border`
- `--surface-disabled`
- `--surface-disabled-fg`
- `--surface-disabled-border`

Input recipes:

- `--input`
- `--input-fg`
- `--input-fg-muted`
- `--input-fg-faint`
- `--input-border`
- `--input-hover`
- `--input-hover-fg`
- `--input-hover-border`
- `--input-focus`
- `--input-focus-fg`
- `--input-focus-border`
- `--input-disabled`
- `--input-disabled-fg`
- `--input-disabled-border`

Overlay recipes:

- `--overlay`
- `--overlay-fg`
- `--overlay-border`

Accent/action recipes:

- `--accent`
- `--accent-fg`
- `--accent-border`
- `--accent-hover`
- `--accent-hover-fg`
- `--accent-hover-border`
- `--accent-active`
- `--accent-active-fg`
- `--accent-active-border`
- `--accent-subtle`
- `--accent-subtle-fg`
- `--accent-subtle-border`

Status recipes:

- `--success`
- `--success-fg`
- `--success-border`
- `--success-subtle`
- `--success-subtle-fg`
- `--success-subtle-border`
- `--warning`
- `--warning-fg`
- `--warning-border`
- `--warning-subtle`
- `--warning-subtle-fg`
- `--warning-subtle-border`
- `--danger`
- `--danger-fg`
- `--danger-border`
- `--danger-hover`
- `--danger-hover-fg`
- `--danger-hover-border`
- `--danger-active`
- `--danger-active-fg`
- `--danger-active-border`
- `--danger-subtle`
- `--danger-subtle-fg`
- `--danger-subtle-border`
- `--info`
- `--info-fg`
- `--info-border`
- `--info-subtle`
- `--info-subtle-fg`
- `--info-subtle-border`

Focus:

- `--focus`

Shape and density:

- `--radius`
- `--radius-surface`
- `--radius-input`
- `--radius-control`
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

This replaces `primary` and `secondary` scales. There should not be `--primary-color-1` through `--primary-color-9`, or equivalent numbered scales under a different name. If a role is needed, name the recipe: `--surface-hover`, `--surface-hover-fg`, `--surface-hover-border`, `--accent-subtle`, `--accent-subtle-fg`, `--accent-subtle-border`.

Radius should follow a role-alias model instead of a size scale:

```css
:root {
  --radius: 0.5rem; /* Base radius. Change this to make the whole system sharper or softer. */
  --radius-surface: var(--radius); /* Panels, cards, popovers, menus, and other container surfaces. */
  --radius-input: var(--radius); /* Text fields, textareas, selects, combobox triggers, and segmented inputs. */
  --radius-control: var(--radius); /* Buttons, toggles, checkboxes, radios, switches, and compact action controls. */
}
```

This gives consumers one global radius knob by default while still allowing a useful split:

```css
:root {
  --radius: 0.5rem;
  --radius-surface: 0.75rem;
  --radius-input: 0.375rem;
  --radius-control: 999px;
}
```

Spacing and sizing should follow the same model: one base unit, then role aliases for the common component structures. Do not add a generic `--space-1` through `--space-12` scale unless the implementation proves that role aliases cannot cover real component needs.

```css
:root {
  --space: 0.25rem; /* Base spacing unit. Change this to make the whole system denser or looser. */

  --surface-padding: calc(var(--space) * 4); /* Cards, popovers, dialogs, menus, and panels. */
  --surface-gap: calc(var(--space) * 3); /* Gaps between sections inside surfaces. */
  --content-gap: calc(var(--space) * 2); /* Gaps between label/description/content groups. */
  --item-gap: calc(var(--space) * 2); /* Gaps between icon + text, checkbox + label, and inline item content. */

  --control-height-sm: 2rem; /* Compact buttons, toolbar controls, and dense selects. */
  --control-height-md: 2.5rem; /* Default buttons, inputs, selects, and combobox triggers. */
  --control-height-lg: 3rem; /* Large controls used in prominent forms or touch-heavy layouts. */

  --control-padding-x-sm: calc(var(--space) * 2.5); /* Horizontal padding for small buttons and controls. */
  --control-padding-x-md: calc(var(--space) * 3); /* Horizontal padding for default buttons and controls. */
  --control-padding-x-lg: calc(var(--space) * 4); /* Horizontal padding for large buttons and controls. */
  --control-gap: calc(var(--space) * 2); /* Gap between icon and label inside buttons and compact controls. */

  --input-padding-x-sm: calc(var(--space) * 2.5); /* Horizontal padding for small field controls. */
  --input-padding-x-md: calc(var(--space) * 3); /* Horizontal padding for default field controls. */
  --input-padding-x-lg: calc(var(--space) * 4); /* Horizontal padding for large field controls. */
  --input-padding-y: calc(var(--space) * 2); /* Vertical padding for multiline inputs and textareas. */

  --list-item-padding-x: calc(var(--space) * 2.5); /* Menu items, combobox options, select options, and command rows. */
  --list-item-padding-y: calc(var(--space) * 2); /* Menu items, combobox options, select options, and command rows. */
}
```

This is intentionally not a general layout system. It covers the repeated component-library needs: controls, inputs, surfaces, menus/lists, and small internal gaps. Page layout, marketing sections, grids, and app-specific spacing should stay outside the component token contract.

### Spacing Authoring Rules

Component authors should not use a Tailwind-style public spacing scale such as `--space-xs`, `--space-sm`, `--space-md`, `--space-lg`, or `--space-1` through `--space-12`.

Use this order:

1. Use an existing role token when one matches the job.
   Examples: `--surface-padding`, `--content-gap`, `--item-gap`, `--control-height-md`, `--input-padding-x-md`, `--list-item-padding-y`.

2. Use `calc(var(--space) * n)` for component-internal spacing that does not deserve a public token.
   Examples: one-off icon offsets, small animation travel distances, internal decorative gaps, and layout math inside one component.

3. Promote a repeated spacing value to a named role token only when it appears across multiple components with the same semantic purpose.
   Good promotion: repeated option/menu row padding becomes `--list-item-padding-x` and `--list-item-padding-y`.
   Bad promotion: a one-off nested badge offset becomes `--space-badge-offset` or `--space-7`.

This keeps all component spacing tied to the same density knob while avoiding a public spacing scale that consumers have to learn. The base `--space` is the consistency mechanism; named role tokens are the customization API.

Allowed:

```css
.dx-menu-item {
  gap: var(--item-gap);
  padding: var(--list-item-padding-y) var(--list-item-padding-x);
}

.dx-combobox-check {
  margin-inline-start: calc(var(--space) * -1);
}
```

Avoid:

```css
:root {
  --space-xs: 0.25rem;
  --space-sm: 0.5rem;
  --space-md: 0.75rem;
  --space-lg: 1rem;
}

.dx-menu-item {
  padding: var(--space-sm) var(--space-md);
}
```

### Token Comments And Intended Usage

The theme file should keep comments next to the variables. The comments are part of the design contract, not decoration. They explain when a component should use a token and prevent contributors from inventing local aliases for already-covered roles.

```css
:root {
  /* Default foreground roles. Use for text and icons when a component does
     not need a full foreground/border recipe. These are especially useful
     inside rich surfaces where individual text/icon elements need different
     emphasis levels. */
  --fg: 18% 0.012 75;
  --fg-muted: 42% 0.014 75;
  --fg-faint: 52% 0.014 75;

  /* Base surface recipe. Use for pages, cards, neutral panels, and default
     component backgrounds. */
  --surface: 96% 0.012 75;
  --surface-fg: 18% 0.012 75;
  --surface-border: 80% 0.012 75;

  /* Muted surface recipe. Use for secondary panels, quiet grouped regions,
     subtle callouts, and low-emphasis rows. */
  --surface-muted: 92% 0.014 75;
  --surface-muted-fg: 42% 0.014 75;
  --surface-muted-border: 78% 0.012 75;

  /* Hover surface recipe. Use for neutral hover states on ghost controls,
     menu items, list rows, tabs, and selectable items. */
  --surface-hover: 90% 0.014 75;
  --surface-hover-fg: 12% 0.014 75;
  --surface-hover-border: 72% 0.014 75;

  /* Active surface recipe. Use for pressed neutral controls and active rows
     where the state is temporary or interaction-driven. */
  --surface-active: 86% 0.014 75;
  --surface-active-fg: 8% 0.016 75;
  --surface-active-border: 64% 0.014 75;

  /* Selected surface recipe. Use for current tabs, selected menu options,
     selected list rows, and persistent neutral selection. */
  --surface-selected: 84% 0.018 75;
  --surface-selected-fg: 8% 0.016 75;
  --surface-selected-border: 58% 0.018 75;

  /* Disabled surface recipe. Use for disabled controls and disabled rows.
     Disabled foreground is intentionally lower contrast and should not be
     used for required readable content. */
  --surface-disabled: 90% 0.006 75;
  --surface-disabled-fg: 58% 0.008 75;
  --surface-disabled-border: 84% 0.006 75;

  /* Input recipe. Use for text fields, textareas, selects, combobox triggers,
     date/time inputs, and other field shells. */
  --input: 96% 0.012 75;
  --input-fg: 18% 0.012 75;
  --input-fg-muted: 42% 0.014 75;
  --input-fg-faint: 52% 0.014 75;
  --input-border: 76% 0.012 75;

  /* Input hover recipe. Use when a field shell is hovered but not focused. */
  --input-hover: 94% 0.014 75;
  --input-hover-fg: 12% 0.014 75;
  --input-hover-border: 66% 0.014 75;

  /* Input focus recipe. Use for the field shell while focus-visible or
     focus-within is active. The focus ring still uses --focus. */
  --input-focus: 96% 0.012 75;
  --input-focus-fg: 18% 0.012 75;
  --input-focus-border: 48% 0.13 25;

  /* Input disabled recipe. Use for disabled field shells. */
  --input-disabled: 90% 0.006 75;
  --input-disabled-fg: 58% 0.008 75;
  --input-disabled-border: 84% 0.006 75;

  /* Overlay recipe. Use for popovers, dropdown menus, command palettes,
     tooltips, floating panels, and dialogs. */
  --overlay: 98% 0.01 75;
  --overlay-fg: 18% 0.012 75;
  --overlay-border: 78% 0.012 75;

  /* Accent recipe. Use for primary actions, selected accent controls, brand
     highlights, and links that are intentionally accent-colored. */
  --accent: 40% 0.13 25;
  --accent-fg: 98% 0.01 75;
  --accent-border: 36% 0.13 25;

  /* Accent hover recipe. Use for hovered primary/accent actions. */
  --accent-hover: 36% 0.13 25;
  --accent-hover-fg: 98% 0.01 75;
  --accent-hover-border: 32% 0.13 25;

  /* Accent active recipe. Use for pressed primary/accent actions. */
  --accent-active: 32% 0.13 25;
  --accent-active-fg: 98% 0.01 75;
  --accent-active-border: 28% 0.13 25;

  /* Accent subtle recipe. Use for low-emphasis accent badges, selected
     rows that should read as branded, and quiet accent callouts. */
  --accent-subtle: 92% 0.035 25;
  --accent-subtle-fg: 34% 0.13 25;
  --accent-subtle-border: 78% 0.045 25;

  /* Focus indicator color. Use for focus-visible rings and outlines only.
     Do not use as a general brand/accent color. */
  --focus: 48% 0.13 25;

  /* Success filled recipe. Use for high-emphasis positive status. */
  --success: 50% 0.085 150;
  --success-fg: 98% 0.01 75;
  --success-border: 42% 0.085 150;

  /* Success subtle recipe. Use for success alerts, badges, toasts, and
     validation messages that should not be filled with the strong color. */
  --success-subtle: 92% 0.035 150;
  --success-subtle-fg: 34% 0.10 150;
  --success-subtle-border: 76% 0.05 150;

  /* Warning filled recipe. Use for high-emphasis cautionary status. */
  --warning: 60% 0.14 60;
  --warning-fg: 18% 0.012 75;
  --warning-border: 50% 0.13 60;

  /* Warning subtle recipe. Use for warning alerts, badges, toasts, and
     validation messages that should not be filled with the strong color. */
  --warning-subtle: 94% 0.045 75;
  --warning-subtle-fg: 34% 0.12 70;
  --warning-subtle-border: 78% 0.07 70;

  /* Danger filled recipe. Use for destructive actions and high-emphasis
     error states. */
  --danger: 52% 0.16 25;
  --danger-fg: 98% 0.01 75;
  --danger-border: 44% 0.16 25;

  /* Danger hover recipe. Use for hovered destructive actions. */
  --danger-hover: 46% 0.16 25;
  --danger-hover-fg: 98% 0.01 75;
  --danger-hover-border: 38% 0.16 25;

  /* Danger active recipe. Use for pressed destructive actions. */
  --danger-active: 40% 0.16 25;
  --danger-active-fg: 98% 0.01 75;
  --danger-active-border: 32% 0.16 25;

  /* Danger subtle recipe. Use for validation messages, error alerts, danger
     badges, and invalid field background treatments. */
  --danger-subtle: 93% 0.035 25;
  --danger-subtle-fg: 38% 0.16 25;
  --danger-subtle-border: 78% 0.06 25;

  /* Info filled recipe. Use for high-emphasis informational status. */
  --info: 52% 0.11 245;
  --info-fg: 98% 0.01 75;
  --info-border: 44% 0.11 245;

  /* Info subtle recipe. Use for informational alerts, badges, and toasts. */
  --info-subtle: 93% 0.035 245;
  --info-subtle-fg: 38% 0.11 245;
  --info-subtle-border: 78% 0.055 245;

  /* Base radius. Change this to make the whole system sharper or softer. */
  --radius: 0.5rem;

  /* Surface radius. Use for panels, cards, popovers, menus, and dialogs. */
  --radius-surface: var(--radius);

  /* Input radius. Use for text fields, textareas, selects, combobox triggers,
     and segmented date/time inputs. */
  --radius-input: var(--radius);

  /* Control radius. Use for buttons, toggles, chips, checkboxes, radios, and
     compact controls. */
  --radius-control: var(--radius);

  /* Base spacing unit. Change this to make component spacing denser or looser
     without changing page layout. */
  --space: 0.25rem;

  /* Surface padding. Use for cards, popovers, dialogs, menus, and panels. */
  --surface-padding: calc(var(--space) * 4);

  /* Surface gap. Use between sections inside cards, popovers, dialogs, and
     other contained surfaces. */
  --surface-gap: calc(var(--space) * 3);

  /* Content gap. Use between labels, descriptions, validation messages, and
     related field content. */
  --content-gap: calc(var(--space) * 2);

  /* Item gap. Use between icon + text, checkbox + label, and inline content
     inside a single row/control. */
  --item-gap: calc(var(--space) * 2);

  /* Small control height. Use for dense buttons, toolbar controls, and compact
     selects. */
  --control-height-sm: 2rem;

  /* Default control height. Use for normal buttons, inputs, selects, and
     combobox triggers. */
  --control-height-md: 2.5rem;

  /* Large control height. Use for prominent form controls and touch-heavy
     layouts. */
  --control-height-lg: 3rem;

  /* Horizontal padding for small/default/large buttons and compact controls. */
  --control-padding-x-sm: calc(var(--space) * 2.5);
  --control-padding-x-md: calc(var(--space) * 3);
  --control-padding-x-lg: calc(var(--space) * 4);

  /* Gap between icon and label inside buttons and compact controls. */
  --control-gap: calc(var(--space) * 2);

  /* Horizontal padding for small/default/large field controls. */
  --input-padding-x-sm: calc(var(--space) * 2.5);
  --input-padding-x-md: calc(var(--space) * 3);
  --input-padding-x-lg: calc(var(--space) * 4);

  /* Vertical padding for multiline inputs and textareas. Single-line inputs
     should usually use height tokens instead. */
  --input-padding-y: calc(var(--space) * 2);

  /* Option/menu row padding. Use for select options, combobox options, menu
     items, command rows, and similar list actions. */
  --list-item-padding-x: calc(var(--space) * 2.5);
  --list-item-padding-y: calc(var(--space) * 2);
}
```

### Theme Switching

Theme switching happens only by redefining the source variables.

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
  accent-color: var(--accent);
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

The real theme block should define every recipe listed above. This excerpt is intentionally short to show the switching pattern without duplicating the full commented contract.

The existing runtime theme toggle can keep writing `html[data-theme]`, but the CSS should move from `--dark` / `--light` sentinels to direct source-token overrides.

### Component Usage Rules

Components should use global variables directly first:

```css
.dx-input {
  background: var(--input);
  color: var(--input-fg);
  border: 1px solid var(--input-border);
  border-radius: var(--radius-input);
  min-height: var(--control-height-md);
  padding-inline: var(--input-padding-x-md);
}

.dx-input::placeholder {
  color: var(--input-fg-muted);
}

.dx-input-icon,
.dx-input-clear {
  color: var(--input-fg-faint);
}

.dx-input:hover {
  background: var(--input-hover);
  color: var(--input-hover-fg);
  border-color: var(--input-hover-border);
}

.dx-input:focus-visible {
  background: var(--input-focus);
  color: var(--input-focus-fg);
  border-color: var(--input-focus-border);
  outline: 2px solid var(--focus);
  outline-offset: 2px;
}

.dx-button[data-variant="ghost"],
.dx-link {
  background: transparent;
  color: var(--surface-fg);
}

.dx-button[data-variant="ghost"]:hover,
.dx-link:hover {
  background: var(--surface-hover);
  color: var(--surface-hover-fg);
  border-color: var(--surface-hover-border);
}

.dx-button[data-variant="ghost"]:active,
.dx-link:active,
.dx-link[aria-current="page"] {
  background: var(--surface-active);
  color: var(--surface-active-fg);
  border-color: var(--surface-active-border);
}

.dx-popover {
  background: var(--overlay);
  color: var(--overlay-fg);
  border: 1px solid var(--overlay-border);
  border-radius: var(--radius-surface);
  padding: var(--surface-padding);
}

.dx-select-option {
  background: transparent;
  color: var(--surface-fg);
  border: 1px solid transparent;
}

.dx-select-option[data-highlighted="true"] {
  background: var(--surface-hover);
  color: var(--surface-hover-fg);
  border-color: var(--surface-hover-border);
}

.dx-select-option[data-selected="true"] {
  background: var(--surface-selected);
  color: var(--surface-selected-fg);
  border-color: var(--surface-selected-border);
}

.dx-select-option {
  display: flex;
  align-items: center;
  gap: var(--item-gap);
  padding: var(--list-item-padding-y) var(--list-item-padding-x);
}
```

Use `data-*` selectors to select states, then apply the shared variables:

```css
.dx-input[data-invalid="true"] {
  background: var(--danger-subtle);
  color: var(--danger-subtle-fg);
  border-color: var(--danger-subtle-border);
}
```

Only add a component variable when the value is genuinely component-specific or runtime-driven:

- Keep runtime geometry/state values such as `--progress-value`, `--toast-index`, `--depth`, and `--swatch-color`.
- Prefer unprefixed, role-like names only if the value can be meaningfully overridden by a consumer in context.
- Do not add `--input-bg`, `--tabs-fg`, `--schedule-border`, or similar aliases when an existing recipe already covers the state.

Recipe foregrounds do not replace all standalone foreground roles. Use `--<recipe>-fg` for the primary content color that belongs to a filled visual state. Use `--fg`, `--fg-muted`, and `--fg-faint` for general text/icon emphasis inside ordinary surfaces. Use `--input-fg-muted` and `--input-fg-faint` for placeholders, field icons, clear buttons, right sections, units, and other muted content inside input/control shells.

## Rationale And Options Considered

### Why Recipe Groups

The recipe-group model is the best fit for this library because components usually need to theme a state as a complete visual treatment, not as unrelated color channels.

A muted surface does not just have a different background. It usually has a different foreground and border too. The same is true for hover, active, selected, disabled, input focus, accent actions, subtle status alerts, and destructive actions. If the plan only exposes `--surface`, `--fg`, and `--border`, consumers will quickly need per-state border and foreground customization. If every component solves that locally, the library returns to the current drift.

Recipes make that relationship explicit:

```css
--surface-muted
--surface-muted-fg
--surface-muted-border
```

Components choose the recipe that matches the state. Consumers can then tune the full state treatment without searching for component-specific escape hatches.

### Options Rejected

1. Minimal global tokens only.

   A tiny set like `--surface`, `--fg`, `--border`, `--accent`, and `--danger` is attractive, but it fails common UI states. Hover borders, selected borders, disabled foreground, muted borders, input focus borders, and subtle status backgrounds all become ambiguous. The result would be either poor theming or a wave of local component variables.

2. Per-component variables for every state.

   Variables like `--button-hover-border`, `--input-disabled-fg`, `--tabs-active-border`, and `--alert-danger-bg` are easy to understand in isolation, but they create a large component-specific API. Consumers must learn every component's token matrix and cannot theme common states globally.

3. Independent channel tokens for every state.

   A global matrix such as `--border-hover`, `--fg-hover`, `--surface-hover`, `--border-muted`, and `--fg-muted` is better than per-component variables, but it still treats fill, foreground, and border as unrelated. In practice, a component wants the border that belongs to the selected surface or muted surface, not a generic selected border that may not match that recipe.

4. Relative color syntax and dynamic derivation.

   Deriving hover, active, subtle, and border values with relative color syntax would reduce token count, but current browser support is still too new for this to be the core contract. It also pushes color-generation logic into component CSS and makes contrast harder to reason about. `color-mix()` and relative colors can still be used later as progressive enhancement, but important theme states should be explicit tokens.

5. Numbered palette scales.

   `--primary-color-1` through `--primary-color-9` and equivalent scales are too indirect for component authors and consumers. They encode relative position in a palette instead of UI intent. The rewrite should name the role directly.

### Relationship To Popular UI Libraries

The plan borrows different lessons without copying any one system:

- shadcn/ui shows the value of plain CSS custom properties and semantic roles instead of runtime theme objects.
- MUI's `palette.action` and state colors show that hover, selected, disabled, and focus states are real API surface, not incidental component details.
- Mantine's component variables and variants show why component-level theming is powerful, but this plan intentionally keeps component theme variables rare to avoid a large per-component API.
- DaisyUI's radius approach motivates one base radius with role aliases, so consumers can split surface/input/control shape without a size scale.
- Chakra and Panda semantic tokens reinforce the idea that tokens should describe UI intent and theme switching should happen at the token source.

## Variable Scope Contract

Unprefixed variables are intentionally ergonomic, but they carry collision risk. The scope contract is:

- The generated theme asset defines the tokens on `:root`.
- Components read inherited tokens and must not redefine global recipe tokens on component roots.
- Downstream apps that already use generic variables can wrap Dioxus components in a theme scope and define the same variables there.
- Scoped theme examples should use a wrapper such as `[data-dx-theme]` or an app-owned class. The variable names remain unprefixed; the scope prevents collision.
- Component runtime variables may still be component-scoped and prefixed when they represent layout math, prop-derived values, user data, or internal state. Component theme aliases remain forbidden unless a component has a true domain concept that shared recipes cannot cover.

## Rewrite Plan

### Phase 1: Token Spike

Before rewriting every component, map representative hard cases to the proposed recipes:

- One input field shell.
- One ghost/text button or link.
- One menu/list item with highlighted and selected states.
- One tabs trigger variant.
- One sidebar item.
- One schedule event.
- One badge or alert for each status tone used by the library.

The spike should answer one question: does the recipe vocabulary cover the component without inventing theme aliases such as `--tabs-active-border` or `--schedule-muted-fg`?

If a missing role appears in more than one component family, add it to the global recipe set. If it is truly domain-specific, keep it as a runtime/domain variable and document why it is not global.

### Phase 1.5: Visual Snapshot Baseline

Before rewriting the theme contract or migrating component CSS, add a programmatic visual-regression harness around the preview site.

Use the existing `playwright/` package and preview web server rather than introducing Storybook or a hosted visual testing service. The harness should:

- Enumerate preview components and variants from `preview/src/components/**/component.json` and `variants/*`.
- Visit stable preview routes such as `/component/block/?name=<component>&variant=<variant>` for every component variant.
- Capture screenshots of the stable demo frame or component block, not the full docs page.
- Run in fixed viewports with deterministic settings: animations disabled, consistent color scheme, stable dates/timers where needed, and known font loading behavior.
- Capture at least light and dark theme baselines. Add mobile or high-risk responsive baselines for components whose layout changes materially by viewport.
- Store Playwright snapshot baselines under the `playwright/` snapshot tree so `expect(locator).toHaveScreenshot()` can compare future runs.
- Provide separate commands for updating baselines and checking drift, for example `cd playwright && npx playwright test preview-visual.spec.ts --update-snapshots` and `cd playwright && npx playwright test preview-visual.spec.ts`.

This is a drift detector, not a replacement for semantic tests. The first baseline should be captured from the current implementation before Phase 2 begins. During migration, agents should run either the affected component snapshots or the full visual suite before claiming a component family is visually preserved.

### Phase 2: Replace The Theme Contract

- Rewrite `dioxus-components/assets/dx-components-theme.css` around the new unprefixed semantic tokens.
- Rewrite `test-harness/assets/dx-components-theme.css` to use the same token contract.
- Keep preview consuming the crate theme asset and verifying that the import/rendering path still works.
- Remove `--primary-color-*`, `--secondary-color-*`, `--focused-border-color`, status palette pairs, `--dxc-light-on`, `--dxc-dark-on`, `--dark`, and `--light`.
- Keep `html[data-theme]` and `prefers-color-scheme` as the theme selectors, but make them switch source variables directly.
- Add base defaults for body, default border color, `accent-color`, and selection using the new recipes.

This phase is intentionally breaking. Components that still read old variables should be migrated in the same rewrite branch before release.

### Phase 3: Migrate Shared Control Shells

Start with components that duplicate the same control shell:

1. `input`
2. `textarea`
3. `select`
4. `combobox`
5. `date_picker`
6. `time_picker`

For each component:

- Replace palette-scale reads with direct global semantic tokens.
- Replace hardcoded repeated field heights, input padding, row padding, and inline gaps with shared density/spacing tokens.
- Remove component-specific color aliases when the shared token already describes the role.
- Keep component-specific runtime variables only for values that come from props or component state.
- Preserve class, style, attribute spread, and `data-*` state hooks.
- Use the same hover, focus-visible, disabled, invalid, placeholder, and open-state token roles across the family.
- Migrate the crate-owned styled wrapper and `style.css` implementation. Preserve its primitive imports and behavior contract unless the primitive API itself must change. Preview should only need import-surface updates or demo verification if API or styling output changes.

### Phase 4: Migrate Common Display And Action Components

Next migrate lower-risk components that mostly need color, border, focus, radius, and spacing roles:

- `button`
- `badge`
- `item`
- `toolbar`
- `scroll_area`
- `card`
- `popover`
- `dropdown_menu`

Rules:

- Variants should map to semantic roles, not numbered palette positions.
- `primary` / `secondary` visual language should be renamed or internally mapped to role concepts such as accent, muted, outline, danger, ghost, and link.
- Hover and active states should use complete recipes such as `--surface-hover`, `--accent-hover`, `--danger-hover`, or `--surface-active`, including each recipe's `-fg` and `-border` tokens, not generated palette steps.
- Size variants should use shared height, padding, and gap tokens unless a component has a real domain-specific layout need.

### Phase 5: Migrate High-Risk Components

Migrate high-risk components after the shared token vocabulary has proven sufficient:

- `schedule`, because it currently has a large local token layer that mixes theme roles with layout and event-state roles.
- `tabs`, because it has its own `--dx-tabs-*` token family.
- `sidebar`, because it combines layout, collapse state, borders, and nested variants.
- `color_picker`, because runtime color values are part of the component behavior.

For high-risk components, split theme tokens from runtime variables:

- Theme roles should become direct uses of surface, input, overlay, accent, and status recipes.
- Runtime or user-data values should remain local variables, for example event colors, selected color swatches, progress values, resize preview counts, and depth offsets.

### Phase 6: Documentation And Installer Surface

Update docs and installer expectations:

- Document the new global token list.
- Document the theme switching model: override source variables, not component branches.
- Show global override, scoped override, and one component-instance override.
- State clearly that old variables are removed in this rewrite.
- Update any generated or copied component guidance so `dx components add` consumers know they need the new theme asset and migrated component files together.
- Update prior docs that recommend `--dxc-*` or palette-scale variables.

## Removal Matrix

| Current Var Pattern | Rewrite Action |
| --- | --- |
| `--primary-color*` | Remove; replace usages with recipes such as `--accent`, `--accent-hover`, `--accent-subtle`, or `--surface-selected` |
| `--secondary-color*` | Remove; replace usages with recipes such as `--surface`, `--surface-muted`, `--surface-hover`, `--surface-active`, or `--overlay` |
| `--focused-border-color` | Remove; replace with `--focus` |
| `--primary-success-color` / `--secondary-success-color` | Remove; replace with success filled or subtle recipes |
| `--primary-warning-color` / `--secondary-warning-color` | Remove; replace with warning filled or subtle recipes |
| `--primary-error-color` / `--secondary-error-color` / `--contrast-error-color` | Remove; replace with danger filled, hover, active, or subtle recipes |
| `--primary-info-color` / `--secondary-info-color` | Remove; replace with info filled or subtle recipes |
| `--dark` / `--light` | Remove; replace with source-token overrides under `:root[data-theme]` and media queries |
| `--dxc-*` semantic variables | Remove or rename to unprefixed equivalents |
| `--dx-*` component color/theme variables | Remove where shared tokens cover the role |
| `--schedule-*` theme variables | Remove where shared tokens cover the role; keep only true schedule runtime/layout variables |
| `--progress-value`, `--toast-index`, `--toast-count`, `--depth`, `--depth-offset`, `--area-color`, `--swatch-color`, `--event-color` | Keep only if they remain runtime/component-data plumbing rather than theme tokens |

## Acceptance Criteria

- No source file under `dioxus-components`, `preview`, `test-harness`, or generated component templates references removed variables, except historical docs explicitly marked obsolete.
- No component CSS defines color aliases just to rename shared recipes.
- Components use unprefixed global recipe variables directly for common theme roles from the crate-owned implementation.
- Theme switching is implemented by redefining source variables under `:root[data-theme]` and `prefers-color-scheme`.
- Downstream docs present the rewrite as breaking, with no compatibility alias promise.
- Copied component files and the global theme asset are treated as one migration unit.
- Preview is treated as a consumer/import surface: it re-exports/imports crate components and verifies demo rendering, but does not become a parallel styling source.
- Runtime custom properties remain only where they carry component state, prop-derived values, user data, or layout math.
- The theme has no numbered primary/secondary palette scale.
- Repeated component spacing uses the shared spacing/density roles instead of hardcoded one-off padding and gap values.
- Visual states remain covered as recipes: hover, active, focus-visible, disabled, selected/open, invalid/error, subtle status, filled status, and destructive hover/active.
- Recipe foreground/background pairs meet WCAG contrast targets for their intended text sizes, including `--surface-fg` on `--surface`, `--accent-fg` on `--accent`, status `-fg` pairs, and subtle status `-fg` pairs.
- Focus indicators using `--focus` meet WCAG 2.2 focus appearance expectations where practical.
- A Playwright visual snapshot harness exists for preview component variants, has a baseline captured before the CSS variable rewrite, and can compare post-refactor output programmatically.

## Tests And Validation

Run the existing checks after implementation:

```sh
cargo test -p dioxus-primitives
cd preview && npx stylelint "src/**/*.css"
cd playwright && npx playwright test tabs.spec.ts
cd playwright && npx playwright test schedule.spec.ts
cd playwright && npx playwright test color-picker.spec.ts
cd playwright && npx playwright test navbar.spec.ts
cd playwright && npx playwright test sidebar.spec.ts
```

Run visual snapshot checks before and after each migration slice:

```sh
cd playwright && npx playwright test preview-visual.spec.ts --update-snapshots
cd playwright && npx playwright test preview-visual.spec.ts
```

Use `--update-snapshots` only when intentionally establishing or accepting a new baseline. Refactor agents should normally run the compare command and investigate diffs before updating snapshots.

Search checks should include CSS, Rust inline `style:` strings, docs examples, and generated component templates:

```sh
rg -- '--(primary-color|secondary-color|focused-border-color|dark|light|dxc-)'
rg -- '--dx-.*(background|foreground|border|accent|ring|surface|fg)'
```

Programmatic visual checks are required because current Playwright coverage is mostly behavioral.

Minimum visual checks:

- Default light theme.
- Default dark theme.
- `prefers-color-scheme` dark with no explicit `data-theme`.
- Explicit `data-theme="light"` and `data-theme="dark"`.
- Scoped theme override on a subtree.
- One component-instance override through a wrapper or inline style.
- Input, textarea, select, combobox, date picker, and time picker states: default, hover, focus-visible, disabled, invalid/error, placeholder, open, highlighted option, selected option, and size/radius variants where applicable.
- Control density checks: small/default/large controls, menu/list item rows, popover/card padding, icon+label gaps, and multiline input padding.
- High-risk components after migration: schedule responsive layout and event states, sidebar collapsed state, tabs variants/orientation, color picker selected swatches.

Preview-specific validation should confirm the new ownership boundary still holds:

- Preview `mod.rs` re-exports continue to resolve to `dioxus_components::*`.
- Preview demos still render correctly while reading crate-owned `style.css` and `dx-components-theme.css` for highlighted source display.

## Open Decisions Before Implementation

- Confirm OKLCH browser support. If OKLCH is not acceptable, choose a different value format before edits.
- Confirm the final recipe names. The plan currently uses surface, input, overlay, accent, success, warning, danger, and info recipes.
- Confirm whether spacing/density belongs in the first rewrite. This plan recommends including it because otherwise every migrated control will keep or reinvent hardcoded paddings, heights, and gaps.
- Confirm whether old copied components are simply unsupported after the rewrite, or whether the CLI should provide an explicit migration command or warning.
