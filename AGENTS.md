# Agent Instructions

## Repository Boundaries

This workspace has three separate component layers. Keep changes in the correct layer.

| Path | Role | May import from | Must not contain |
| --- | --- | --- | --- |
| `primitives/` | Unstyled base functionality and accessibility behavior. | Dioxus and shared workspace utilities only. | Shadcn styles, theme tokens, preview-only examples, or dependencies on `dioxus-components` / `preview`. |
| `dioxus-components/` | Canonical styled reusable component crate. It wraps/imports primitives and owns reusable component styles. | `primitives/` / `dioxus-primitives`. | Preview app routes, demos, test pages, or preview-only state. |
| `preview/` | Fully styled showcase application, demos, docs, metadata, and registry/import surface. | `dioxus-components/` and primitives when needed for demos. | Core primitive behavior that belongs in `primitives/`, or reusable library APIs/styles that belong in `dioxus-components/`. |

Dependency direction must flow one way:

```text
primitives/ -> dioxus-components/ -> preview/
```

In other words:

- `primitives/` is the foundation. It owns unstyled behavior, accessibility, state management, events, props, and DOM structure needed for correct base functionality.
- `dioxus-components/` is the styled library and implementation source of truth. It imports primitives and applies reusable styling. It should not invent separate interaction behavior when the behavior belongs in primitives.
- `preview/` is the app and gallery. It imports the styled components and shows fully styled examples. It is not the place to define reusable component behavior or reusable component styles.

## Component Workflow

When adding or changing a component:

1. Put base behavior and accessibility in `primitives/`.
2. Put reusable styled component wrappers and component CSS in `dioxus-components/src/components`.
3. Use `preview/src/components` for demo modules, docs, metadata, variants, and re-exports/imports from `dioxus-components`.
4. Use `preview/` for demos, examples, routes, and visual validation.
5. Add or update Playwright coverage when behavior changes.

Do not put canonical styled implementations in `preview/src/components`; preview should consume and display the styled crate implementation.

## Dioxus Props and Class Attributes

Do not add a manual `class` prop for ordinary DOM class forwarding when a component already accepts global attributes.

Use Dioxus attribute extension instead:

```rust
#[props(extends = GlobalAttributes)]
pub attributes: Vec<Attribute>,
```

Then spread or merge those attributes onto the rendered element:

```rust
let base = attributes!(div {
    class: Styles::component_root,
});
let attributes = merge_attributes(vec![base, props.attributes]);
```

Why: a prop like `pub class: Option<String>` makes `class:` a narrow component prop. CSS module identifiers such as `Styles::my_class` then fail with trait errors like `Option<String>: From<__CssIdent>`. Letting `class` flow through `GlobalAttributes` preserves normal Dioxus attribute behavior and allows CSS module identifiers without `.to_string()`.

Only keep a separate class-like prop when it is not the plain DOM `class` attribute, such as router `active_class`, split-pane divider class configuration, or component-specific class metadata. For styled wrappers in `dioxus-components/`, merge the wrapper's CSS module class into `attributes` with `merge_attributes` and pass `attributes` through to the primitive.

## Dioxus Public Props and Signals

For public component-library props that can change over time, prefer reactive prop types:

```rust
pub value: ReadSignal<String>,
pub placeholder: ReadSignal<Option<String>>,
pub disabled: ReadSignal<bool>,
```

Do not use this as the default shape for dynamic public props:

```rust
#[props(default, into)]
pub value: String,
```

Why: Dioxus `ReadSignal<T>` props are accepted through `SuperInto<ReadSignal<T>>`, so downstream users can pass literals, owned values, memos, signals, and other compatible reactive values. Dioxus also auto-detects `ReadSignal<Option<T>>` as optional for props builder ergonomics. A plain `String` prop is an owned non-reactive value; it may avoid call-site `.to_string()`, but it prevents users from passing a signal as the prop itself.

Use plain `String` only when the value is intentionally static owned configuration or data that should not be reactive on its own. For reusable UI component inputs such as value, label text, description text, error text, placeholder text, name, ARIA text, selected/open/checked state, or disabled state, default to `ReadSignal<T>` or `ReadSignal<Option<T>>` unless there is a concrete reason not to.

For string-like props that accept either text or rendered content, use an explicit wrapper type only when the component genuinely supports both text and elements. Do not use a wrapper merely to paper over `.to_string()` at call sites if `ReadSignal<String>` or `ReadSignal<Option<String>>` is the correct API.

When removing unnecessary `.to_string()`, distinguish between:

- CSS module identifiers passed to normal DOM `class:` attributes, which should work without `.to_string()` when class flows through Dioxus attributes.
- Public component props that should become `ReadSignal<T>` instead of requiring owned `String`.
- Real owned-string construction for IDs, keys, formatting, serialization, or stored data, which should usually stay.

## Validation

Run the smallest relevant checks for the layer you changed:

- Primitive behavior: `cargo test -p dioxus-primitives`
- Styled library build: `cargo check -p dioxus-components`
- Preview or visual behavior: `scripts/preview-web.sh build` and targeted Playwright tests from `playwright/`
- CSS changes: from `preview/`, run `npx stylelint "src/**/*.css"`

If you cannot run a relevant check, report exactly which command was skipped and why.

## General Expectations

- Be concise and direct.
- Follow established project conventions.
- Find the root cause instead of masking symptoms.
- Reuse existing helpers and components before adding new ones.
- Keep changes small and focused.
- Never commit secrets or API keys.
