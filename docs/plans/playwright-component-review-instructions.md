# Playwright Component Review Instructions

## Goal

Review one assigned component end to end. Strengthen Playwright coverage so core observable behavior, accessibility semantics, and public state transitions are proven. Fix component functionality only when review or tests demonstrate a real defect.

## Repository boundaries

Follow root `AGENTS.md`.

- `primitives/`: unstyled behavior, state, events, accessibility, and required DOM structure.
- `dioxus-components/`: canonical styled wrappers and reusable component CSS.
- `preview/`: demos, fixtures, docs, metadata, and visual validation only.
- `playwright/`: end-to-end observable behavior coverage.
- Preserve dependency direction: `primitives -> dioxus-components -> preview`.

Do not put reusable behavior in preview or canonical styling in primitives.

## Review procedure

1. Read root `AGENTS.md`, this document, component docs, primitive implementation, styled wrapper, preview demos, and existing Playwright spec.
2. Inventory existing coverage before editing. Reuse nearby test patterns and selectors.
3. Identify missing core contracts. Prioritize:
   - semantic roles and accessible names;
   - ARIA state and relationships;
   - keyboard navigation, focus movement/restoration, and dismissal;
   - pointer/touch interaction where behavior differs;
   - controlled state and callbacks through visible demo output;
   - disabled, read-only, required, invalid, loading, and boundary states supported by component;
   - form association and forwarded global attributes when applicable;
   - open/closed, selected/current/checked, min/max, and other component-specific invariants.
4. Add focused deterministic Playwright tests for real gaps. Test observable contracts, not source text, CSS implementation details, or plumbing.
5. Add or adjust preview fixtures only when needed to expose a supported public contract. Keep fixture state preview-only.
6. Fix proven behavior/accessibility defects at owning layer. Avoid speculative features, compatibility shims, unrelated cleanup, and duplicate abstractions.
7. Have independent quick review inspect changed files. Resolve every concrete finding.
8. Run targeted Chromium spec against current tree through repository-configured preview/browser setup. Re-run after corrections until clean; an earlier failed or stale run is not verification.

## Test conventions

- Prefer role, label, accessible name, and stable public `data-*` selectors over generated classes or DOM position.
- Exercise real user actions with Playwright `click`, `tap`, `press`, `fill`, pointer, and keyboard APIs. Do not invoke DOM methods through `evaluate` to bypass actionability.
- Assert state before and after interaction.
- Verify disabled controls cannot activate or mutate.
- Verify relationships (`aria-controls`, `aria-labelledby`, `aria-describedby`) point to actual elements when exposed.
- Keep tests deterministic and full-suite safe. Avoid arbitrary sleeps.
- Preserve concurrent changes; never overwrite another component review.

## Validation

Use repository scripts and configured browser environment per root `AGENTS.md`. Do not run raw `cargo check` or unconfigured Playwright. Targeted component verification must use Chromium selected by repository setup after preview build. Skip formatter, lint, and full repository suite inside component slice; integration validation runs once after all reviews.

## Required result

Report:

- exact files changed;
- coverage added or confirmed sufficient;
- behavior/accessibility fixes and owning layer;
- quick-review findings and resolutions;
- exact targeted command and final pass/fail counts;
- remaining risks or skipped checks.

No-edit verdict is valid only with concrete evidence that host/shared specs already cover component contracts or component has no independently routable observable surface.
