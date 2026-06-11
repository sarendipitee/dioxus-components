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
