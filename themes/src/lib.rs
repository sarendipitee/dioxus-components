//! Theme assets for styled dioxus-components.
//!
//! Exposes the default theme stylesheet as a compile-time constant so consumers
//! (e.g. the preview build script) depend on it through cargo's crate graph
//! rather than reading the file by a fragile relative path. `include_str!`
//! registers `default.css` as a build input, so any edit recompiles this crate
//! and reruns dependents' build scripts.

/// The default theme variables stylesheet. Import once in your project root.
pub const DEFAULT_CSS: &str = include_str!("../default.css");
