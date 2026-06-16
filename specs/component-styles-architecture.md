# Component Styles Architecture

## Purpose

Component styles provide the canonical styling pipeline for `dioxus-components`.

The system must give component implementations type-safe class references without turning public class names into private hashed selectors. Consumers should be able to override stable selectors such as `.dx-button`, inspect rendered markup, and customize theme variables without wrapping every component or reverse-engineering generated class names.

The design replaces canonical `dioxus-components` use of Manganis `#[css_module]` with a local `#[component_styles]` macro plus one combined stylesheet injected by the library.

## Acceptance Summary

An implementation satisfies this specification only if:

- `dioxus-components` canonical component styles use stable public `.dx-*` selectors.
- Component implementations keep type-safe class access through `Styles::dx_button`-style associated constants.
- `#[component_styles("...")]` does not hash or rewrite emitted class names.
- Component CSS is combined into one generated stylesheet for `dioxus-components`.
- The combined stylesheet is generated into `OUT_DIR`, not into the crate source tree.
- Downstream consumers can use the crate through a single root-level style hook.
- Preview-local CSS modules may continue using `#[css_module]` for demo-only styling.
- The styling system does not add render-time per-component stylesheet checks.
- Future per-component feature gating or tree-shakeable style loading can be added without changing component class call sites.

## Rationale

CSS modules are useful for application-private styles, but they are a poor default for a reusable component library with a documented customization surface.

With Manganis `#[css_module]`, a selector such as:

```css
.dx-button {
  color: var(--dx-button-fg);
}
```

is emitted as a hashed class such as:

```css
.dx-button-<hash> {
  color: var(--dx-button-fg);
}
```

That breaks the public styling contract consumers expect from a component library:

- Consumers cannot reliably target `.dx-button`.
- Rendered markup contains implementation-specific class names.
- Overrides require adding extra classes, wrapping components, or depending on unstable hashes.
- Selector ordering and specificity become harder to reason about.

The library still benefits from type-safe class references in Rust. The goal is therefore not to remove style constants, but to make those constants point at stable public class names.

## Layering

### dioxus-attributes/

`dioxus-attributes/` owns the `#[component_styles]` proc macro.

The macro:

- reads the referenced CSS file at compile time
- extracts class selectors
- converts CSS class names to Rust associated constant names
- generates stable `&'static str` class constants
- does not inject styles
- does not hash class names

The macro must stay independent from `dioxus-components` so it can remain a reusable compile-time helper.

### dioxus-components/

`dioxus-components/` owns:

- canonical component CSS files under `src/components/*/style.css`
- component implementations that use `#[component_styles]`
- the build script that combines component CSS
- the public root-level style hook

`dioxus-components` must not rely on preview routes, preview demos, or preview assets for canonical style delivery.

### preview/

`preview/` owns demo-only styling, documentation examples, and visual validation.

Preview-local `#[css_module]` usage is allowed because those styles are not part of the reusable component library API.

## Public Consumer API

Consumers include the combined stylesheet once near the app root:

```rust
use dioxus::prelude::*;
use dioxus_components::{Button, DioxusComponentsStyles};

fn App() -> Element {
    rsx! {
        DioxusComponentsStyles {}
        Button { "Save" }
    }
}
```

After the root style hook is installed, components should be used normally.

The style hook is intentionally explicit. Rust and Dioxus do not provide a reliable global hook that runs only because a dependency component appears somewhere in RSX. A single root-level component is the transparent compromise: the app opts into the library stylesheet once, and individual component renders do not perform stylesheet checks.

## Component Authoring API

Canonical styled components declare their local styles with `#[component_styles]`:

```rust
use crate::component_styles;

#[component_styles("./style.css")]
struct Styles;

#[component]
pub fn Button(children: Element) -> Element {
    rsx! {
        button {
            class: Styles::dx_button,
            {children}
        }
    }
}
```

For a CSS selector:

```css
.dx-button {
  display: inline-flex;
}
```

the generated Rust constant is:

```rust
impl Styles {
    pub const dx_button: &'static str = "dx-button";
}
```

Requirements:

- The macro accepts unit structs only.
- Generic structs are rejected with a compile error.
- The original struct item is preserved rather than reconstructed into a different item.
- Class constants use lower-snake-case names derived from CSS class names.
- CSS class values remain the original class strings without hashes.
- Component call sites should continue using `Styles::...` constants rather than raw string literals when a class exists in the component stylesheet.

## Combined Stylesheet Generation

`dioxus-components/build.rs` combines component styles during crate compilation.

Requirements:

- Read component styles from `dioxus-components/src/components/*/style.css`.
- Skip component directories that do not contain `style.css`.
- Sort input paths deterministically before concatenation.
- Emit `cargo:rerun-if-changed` for the build script, component directory, and each input stylesheet.
- Write the generated stylesheet to `OUT_DIR/dioxus-components.css`.
- Do not write generated CSS into `CARGO_MANIFEST_DIR`, `target/` inside the crate source tree, or any other source-controlled path.

Writing only to `OUT_DIR` matters for downstream consumers. Dependencies may be built from immutable registry, vendor, or sandboxed source directories. A dependency build script must not assume it can create new files in the dependency source tree.

## Style Injection

`dioxus-components` exposes a root-level style component:

```rust
#[component]
pub fn DioxusComponentsStyles() -> Element {
    rsx! {
        document::Style {
            {DIOXUS_COMPONENTS_STYLESHEET}
        }
    }
}
```

The stylesheet content is loaded with:

```rust
include_str!(concat!(env!("OUT_DIR"), "/dioxus-components.css"))
```

This keeps generated output in Cargo's build output directory while still allowing downstream apps to receive the full library stylesheet.

The current implementation intentionally uses inline document CSS rather than a generated Manganis asset path. Manganis asset paths are best suited to files that already exist in the crate source tree. Using `OUT_DIR` through `include_str!` avoids source-tree writes and works for dependency builds.

## Performance Model

The current system optimizes for one load-time style installation:

- Component renders do not call `OnceLock`.
- A large component such as `DataTable` does not pay per-cell or per-row style injection checks.
- The browser receives one combined stylesheet rather than many component stylesheet links.
- The CSS customization surface remains stable and inspectable.

This is a deliberate tradeoff. Until true usage-based tree shaking exists, loading one combined stylesheet is preferable to repeated render-time checks and many small stylesheet loads.

## Future Feature Gating

The combined stylesheet can later become feature-aware.

A future build script may include only styles for enabled component features:

```text
button -> src/components/button/style.css
badge -> src/components/badge/style.css
data-table -> src/components/data_table/style.css
```

That future change must preserve:

- `#[component_styles]` at component call sites
- stable `.dx-*` selector values
- `DioxusComponentsStyles` as the root consumer hook

Feature-gated CSS inclusion is still not the same as usage-based tree shaking. Cargo features are unified across the dependency graph, so the generated stylesheet would include the union of enabled component features.

## Future Tree-Shakeable Loading

If Dioxus or the library later supports true usage-based component style loading, the internal implementation may change from one combined stylesheet to per-component style assets.

That enhancement must not require component authors to replace:

```rust
class: Styles::dx_button
```

or downstream consumers to rewrite component markup.

The stable contract is:

- `Styles::...` returns public class names.
- Public selectors remain stable.
- Style delivery can evolve behind the macro and root hook.

## Non-Goals

This system does not attempt to:

- provide CSS selector isolation
- prevent all class name collisions
- parse every valid CSS edge case
- make preview demo styles part of the public component API
- infer actual component usage from downstream source code
- remove the need for one explicit root-level library style hook

Selector isolation is intentionally not the priority for canonical component styles. The public `.dx-*` namespace is the collision boundary.

## Validation

Relevant checks after changing this system:

- `cargo check -p dioxus-attributes`
- `cargo check -p dioxus-components`
- `scripts/preview-web.sh build`
- `rg -n "#\\[css_module" dioxus-components/src/components preview`

Expected `rg` result:

- no `#[css_module]` under `dioxus-components/src/components`
- preview-local `#[css_module]` usage may remain under `preview/src`

When changing selector extraction behavior, add focused macro tests or compile-time fixtures that cover:

- ordinary `.dx-*` selectors
- nested selectors and media queries
- comments
- `:global(...)`
- identifier normalization
- missing CSS file diagnostics

## Current Limitations

The `#[component_styles]` selector extraction is intentionally small. It is adequate for the current component styles, but it is not a full CSS parser.

If component styles begin using syntax that the scanner cannot recognize reliably, the macro should move to a real CSS parser or a shared parser implementation rather than accumulating ad hoc selector rules.
