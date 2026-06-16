//! Proc macro for creating dynamic attribute lists in Dioxus
//!
//! This crate provides the `attributes!` macro which allows creating
//! `Vec<Attribute>` for use with element spread patterns.
//!
//! # Example
//!
//! ```rust,ignore
//! use dioxus::prelude::*;
//! use dioxus_attributes::attributes;
//!
//! fn MyComponent() -> Element {
//!     let attrs = attributes!(div {
//!         class: "my-class",
//!         "data-custom": "value",
//!         onclick: |_| println!("clicked"),
//!     });
//!
//!     rsx! {
//!         div { ..attrs }
//!     }
//! }
//! ```

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::collections::BTreeSet;
use std::path::PathBuf;
use syn::{parse_macro_input, spanned::Spanned, ItemStruct, LitStr};

mod attribute_list;

use attribute_list::AttributeList;

/// Create a `Vec<Attribute>` from RSX attribute syntax.
///
/// The macro requires an element name followed by braced attributes.
/// This allows proper namespace and volatility lookup for attributes.
///
/// Accepts the same syntax as attributes inside rsx! elements:
/// - Built-in attributes: `class: "value"`
/// - Custom attributes: `"data-custom": value`
/// - Event handlers: `onclick: |_| {}`
/// - Shorthand attributes: `class,` (uses variable named `class`)
/// - Spreads: `..existing_attrs`
///
/// # Example
///
/// ```rust,ignore
/// let attrs = attributes!(button {
///     class: "btn btn-primary",
///     onclick: move |_| println!("clicked"),
///     "data-testid": "my-button",
/// });
///
/// rsx! {
///     button { ..attrs, "Click me" }
/// }
/// ```
#[proc_macro]
pub fn attributes(tokens: TokenStream) -> TokenStream {
    match syn::parse::<AttributeList>(tokens) {
        Ok(list) => list.into_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Generate stable CSS class constants from a stylesheet.
///
/// This macro applies to a unit struct and generates associated `pub const`
/// values for each CSS class selector found in the referenced file.
///
/// # Path resolution
///
/// - `"./style.css"` or `"../shared.css"` — resolved relative to the source
///   file that contains the attribute (requires Rust 1.88+, stable).
/// - `"src/components/button/style.css"` or `"/src/components/button/style.css"` —
///   resolved relative to the crate root (`CARGO_MANIFEST_DIR`).
///
/// # Example
///
/// ```rust,ignore
/// use dioxus_attributes::component_styles;
///
/// #[component_styles("./style.css")]
/// struct Styles;
///
/// let button_class = Styles::dx_button;
/// ```
#[proc_macro_attribute]
pub fn component_styles(attr: TokenStream, item: TokenStream) -> TokenStream {
    let path = parse_macro_input!(attr as LitStr);
    let input = parse_macro_input!(item as ItemStruct);

    match expand_component_styles(path, input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn expand_component_styles(
    path: LitStr,
    input: ItemStruct,
) -> syn::Result<proc_macro2::TokenStream> {
    if !matches!(input.fields, syn::Fields::Unit) {
        return Err(syn::Error::new(
            input.span(),
            "#[component_styles] only supports unit structs",
        ));
    }
    if !input.generics.params.is_empty() || input.generics.where_clause.is_some() {
        return Err(syn::Error::new(
            input.generics.span(),
            "#[component_styles] does not support generic structs",
        ));
    }

    let css_path = resolve_css_path(&path)?;
    let css = std::fs::read_to_string(&css_path).map_err(|err| {
        syn::Error::new(
            path.span(),
            format!("failed to read CSS file `{}`: {err}", css_path.display()),
        )
    })?;

    let struct_ident = &input.ident;
    let constants = collect_classes(&css)
        .into_iter()
        .map(|class_name| {
            let const_ident = format_ident!("{}", class_name_to_ident(&class_name));
            quote! {
                pub const #const_ident: &'static str = #class_name;
            }
        })
        .collect::<Vec<_>>();

    Ok(quote! {
        #input

        #[allow(non_upper_case_globals)]
        impl #struct_ident {
            #(#constants)*
        }
    })
}

/// Resolve a CSS path argument to an absolute filesystem path.
///
/// - Paths starting with `./` or `../` are resolved relative to the calling
///   source file via the path literal's span.
/// - All other paths are resolved relative to `CARGO_MANIFEST_DIR`, with any
///   leading `/` stripped so both `"src/foo.css"` and `"/src/foo.css"` work.
fn resolve_css_path(path: &LitStr) -> syn::Result<PathBuf> {
    let raw_path = path.value();

    if raw_path.starts_with("./") || raw_path.starts_with("../") {
        if let Some(source_dir) = path_literal_source_dir(path) {
            return Ok(source_dir.join(&raw_path));
        }

        return Err(syn::Error::new(
            path.span(),
            "#[component_styles] could not resolve a relative CSS path because the proc-macro host did not provide the source file for this attribute",
        ));
    }

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").map_err(|err| {
        syn::Error::new(
            path.span(),
            format!("CARGO_MANIFEST_DIR is not available: {err}"),
        )
    })?;
    Ok(PathBuf::from(manifest_dir).join(raw_path.trim_start_matches('/')))
}

fn path_literal_source_dir(path: &LitStr) -> Option<PathBuf> {
    let span = path.span().unwrap();

    let local_file = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| span.local_file()))
        .ok()
        .flatten();
    if let Some(source_dir) = local_file.and_then(|file| file.parent().map(PathBuf::from)) {
        return Some(source_dir);
    }

    let display_file =
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| span.file())).ok()?;
    source_dir_from_display_file(&display_file)
}

fn source_dir_from_display_file(display_file: &str) -> Option<PathBuf> {
    let display_path = PathBuf::from(display_file);

    if display_path.is_absolute() && display_path.is_file() {
        return display_path.parent().map(PathBuf::from);
    }

    if display_path.is_relative() {
        if let Ok(current_dir) = std::env::current_dir() {
            let current_path = current_dir.join(&display_path);
            if current_path.is_file() {
                return current_path.parent().map(PathBuf::from);
            }
        }

        if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
            let manifest_path = PathBuf::from(&manifest_dir).join(&display_path);
            if manifest_path.is_file() {
                return manifest_path.parent().map(PathBuf::from);
            }

            if let Some(workspace_path) = PathBuf::from(manifest_dir)
                .parent()
                .map(|workspace| workspace.join(&display_path))
            {
                if workspace_path.is_file() {
                    return workspace_path.parent().map(PathBuf::from);
                }
            }
        }
    }

    None
}

/// Strip `/* ... */` block comments from CSS source.
fn strip_css_comments(css: &str) -> String {
    let mut result = String::with_capacity(css.len());
    let mut chars = css.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '/' && chars.peek() == Some(&'*') {
            chars.next(); // consume '*'
            let mut prev = ' ';
            for c in chars.by_ref() {
                if prev == '*' && c == '/' {
                    break;
                }
                prev = c;
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Extract unique CSS class names from a stylesheet.
///
/// Handles comments, string literals in attribute selectors and declarations,
/// `@`-rules without braces (`@charset`, `@import`), and nested at-rules
/// (`@media`, `@supports`, `@layer`) that wrap regular rule blocks.
fn collect_classes(css: &str) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut classes = Vec::new();
    let css_stripped = strip_css_comments(css);

    let mut current = String::new();
    // true = at-rule body (children are rules); false = rule body (children are declarations)
    let mut block_stack: Vec<bool> = Vec::new();
    let mut chars = css_stripped.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            // Skip quoted string literals (attribute selector values, content property, etc.)
            '"' | '\'' => {
                let quote = ch;
                let mut escaped = false;
                for c in chars.by_ref() {
                    if escaped {
                        escaped = false;
                    } else if c == '\\' {
                        escaped = true;
                    } else if c == quote {
                        break;
                    }
                }
            }
            '{' => {
                let is_at_rule = current.trim_start().starts_with('@');
                if !is_at_rule {
                    push_classes_from_selector(&current, &mut seen, &mut classes);
                }
                current.clear();
                block_stack.push(is_at_rule);
            }
            '}' => {
                block_stack.pop();
                current.clear();
            }
            // @-rules without a brace body (e.g. `@charset "UTF-8";`, `@import url(...);`)
            ';' if block_stack.is_empty() => {
                current.clear();
            }
            _ => {
                // Only accumulate when in selector context:
                // - top level (stack empty), or
                // - directly inside an at-rule body (stack top is true)
                let in_selector_ctx = block_stack.is_empty() || block_stack.last() == Some(&true);
                if in_selector_ctx {
                    current.push(ch);
                }
            }
        }
    }

    classes
}

fn push_classes_from_selector(
    selector: &str,
    seen: &mut BTreeSet<String>,
    classes: &mut Vec<String>,
) {
    let trimmed = selector.trim();
    if trimmed.is_empty() || trimmed.starts_with('@') {
        return;
    }

    let bytes = trimmed.as_bytes();
    let mut idx = 0;
    while idx < bytes.len() {
        if bytes[idx] != b'.' {
            idx += 1;
            continue;
        }

        let start = idx + 1;
        let mut end = start;
        while end < bytes.len() {
            let ch = bytes[end] as char;
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                end += 1;
            } else {
                break;
            }
        }

        if end > start {
            let class_name = &trimmed[start..end];
            if seen.insert(class_name.to_string()) {
                classes.push(class_name.to_string());
            }
        }

        idx = end.max(start + 1);
    }
}

fn class_name_to_ident(class_name: &str) -> String {
    let mut ident = String::with_capacity(class_name.len());

    for (idx, ch) in class_name.chars().enumerate() {
        let sanitized = if ch.is_ascii_alphanumeric() || ch == '_' {
            ch
        } else {
            '_'
        };

        if idx == 0 && sanitized.is_ascii_digit() {
            ident.push('_');
        }

        ident.push(sanitized);
    }

    ident
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_comments_removes_block_comment() {
        assert_eq!(
            strip_css_comments("/* .fake { */\n.real { }"),
            "\n.real { }"
        );
    }

    #[test]
    fn strip_comments_preserves_plain_css() {
        let css = ".dx-button { color: red; }";
        assert_eq!(strip_css_comments(css), css);
    }

    #[test]
    fn strip_comments_inline_comment() {
        assert_eq!(
            strip_css_comments(".dx-button /* comment */ .child { }"),
            ".dx-button  .child { }"
        );
    }

    #[test]
    fn collect_classes_basic() {
        let css = ".dx-button { color: red; }";
        assert_eq!(collect_classes(css), vec!["dx-button"]);
    }

    #[test]
    fn collect_classes_multiple() {
        let css = ".dx-button { } .dx-input { }";
        assert_eq!(collect_classes(css), vec!["dx-button", "dx-input"]);
    }

    #[test]
    fn collect_classes_deduplicates() {
        let css = ".dx-button { } .dx-button:hover { }";
        assert_eq!(collect_classes(css), vec!["dx-button"]);
    }

    #[test]
    fn collect_classes_comment_with_brace_and_dot() {
        // A comment containing { and . must not produce fake class names.
        let css = "/* .fake-class { */\n.dx-button { }";
        let classes = collect_classes(css);
        assert!(!classes.contains(&"fake-class".to_string()));
        assert!(classes.contains(&"dx-button".to_string()));
    }

    #[test]
    fn collect_classes_at_charset_does_not_swallow_next_selector() {
        // @charset is a top-level @-rule without braces; the ; must reset the accumulator.
        let css = "@charset \"UTF-8\";\n.dx-button { }";
        assert_eq!(collect_classes(css), vec!["dx-button"]);
    }

    #[test]
    fn collect_classes_at_import_does_not_swallow_next_selector() {
        let css = "@import url(\"base.css\");\n.dx-card { }";
        assert_eq!(collect_classes(css), vec!["dx-card"]);
    }

    #[test]
    fn collect_classes_content_string_not_extracted() {
        // A dot inside a quoted string in a declaration must not become a class.
        let css = ".dx-icon::before { content: \".not-a-class\"; }";
        let classes = collect_classes(css);
        assert!(!classes.contains(&"not-a-class".to_string()));
    }

    #[test]
    fn collect_classes_attribute_selector_string_not_extracted() {
        // Dots inside attribute value strings must not become class names.
        let css = "div[data-v=\"v1.2\"] .dx-button { }";
        let classes = collect_classes(css);
        assert!(!classes.contains(&"2".to_string()));
        assert!(classes.contains(&"dx-button".to_string()));
    }

    #[test]
    fn collect_classes_at_media_nested() {
        // Selectors nested inside @media blocks must be captured.
        let css = "@media (min-width: 768px) {\n  .dx-responsive { }\n}";
        assert!(collect_classes(css).contains(&"dx-responsive".to_string()));
    }

    #[test]
    fn collect_classes_at_media_content_not_extracted() {
        // Declarations inside a media-nested rule must not produce false classes.
        let css = "@media print { .dx-print { content: \".fake\"; } }";
        let classes = collect_classes(css);
        assert!(!classes.contains(&"fake".to_string()));
        assert!(classes.contains(&"dx-print".to_string()));
    }

    #[test]
    fn collect_classes_compound_selector() {
        let css = ".dx-button.dx-button--primary { }";
        let classes = collect_classes(css);
        assert!(classes.contains(&"dx-button".to_string()));
        assert!(classes.contains(&"dx-button--primary".to_string()));
    }

    #[test]
    fn source_dir_from_display_file_accepts_absolute_paths() {
        let source_file = std::env::current_dir()
            .unwrap()
            .join("src")
            .join("lib.rs");

        assert_eq!(
            source_dir_from_display_file(&source_file.display().to_string()),
            source_file.parent().map(PathBuf::from)
        );
    }
}
