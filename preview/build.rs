use dioxus::prelude::*;
use quote::ToTokens;
fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = std::path::PathBuf::from(out_dir);
    let generated_assets_dir = std::path::PathBuf::from("assets/generated");
    if generated_assets_dir.exists() {
        std::fs::remove_dir_all(&generated_assets_dir).unwrap();
    }
    std::fs::create_dir_all(&generated_assets_dir).unwrap();
    println!("cargo:rerun-if-changed=src/components");
    // Process all markdown files in each component folder.
    for folder in std::fs::read_dir("src/components").unwrap().flatten() {
        if !folder.file_type().unwrap().is_dir() {
            continue;
        }
        let folder_path = folder.path();
        walk_markdown_dir(&folder_path, &out_dir, &generated_assets_dir).unwrap();
    }
    render_theme_css(&out_dir, &generated_assets_dir).unwrap();
}

fn crate_component_source_folder(folder_name: &str) -> &str {
    match folder_name {
        "autocomplete" | "multi_select" | "pills_input" | "tags_input" => "combobox",
        "text_input" => "input",
        "schedule_day_view"
        | "schedule_week_view"
        | "schedule_month_view"
        | "schedule_year_view"
        | "schedule_mobile_month_view"
        | "schedule_recurring"
        | "schedule_events" => "schedule",
        _ => folder_name,
    }
}

/// Returns the named plain structs a preview folder should document as its props table.
///
/// Schedule sub-pages document a focused subset of the shared `Schedule` API (view config or
/// data-model structs) rather than the full `ScheduleProps` table. These types are plain
/// structs, not `#[derive(Props)]`, so they are extracted by name (see `extract_named_structs`).
fn curated_prop_types(folder_name: &str) -> Option<&'static [&'static str]> {
    let types: &'static [&'static str] = match folder_name {
        "schedule_day_view" => &["ScheduleDayViewConfig", "ScheduleTimeGridConfig"],
        "schedule_week_view" => &["ScheduleWeekViewConfig", "ScheduleTimeGridConfig"],
        "schedule_month_view" => &["ScheduleMonthViewConfig"],
        "schedule_year_view" => &["ScheduleYearViewConfig"],
        "schedule_mobile_month_view" => &["ScheduleMobileMonthViewConfig"],
        "schedule_recurring" => &["ScheduleRecurrence", "ScheduleRecurrenceExpansionLimit"],
        "schedule_events" => &["ScheduleEvent"],
        _ => return None,
    };
    Some(types)
}

fn write_generated_html(
    out_path: &std::path::Path,
    asset_path: &std::path::Path,
    html: String,
) -> std::io::Result<()> {
    if let Some(parent) = asset_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(out_path, &html)?;
    std::fs::write(asset_path, html)
}

fn walk_markdown_dir(
    dir: &std::path::Path,
    out_dir: &std::path::Path,
    asset_dir: &std::path::Path,
) -> std::io::Result<()> {
    let folder_name = dir.file_name().unwrap();
    let folder_name = folder_name.to_string_lossy();
    let out_folder = out_dir.join(&*folder_name);
    let asset_folder = asset_dir.join(&*folder_name);
    std::fs::create_dir_all(&out_folder).unwrap();
    std::fs::create_dir_all(&asset_folder).unwrap();
    for file in std::fs::read_dir(dir).unwrap().flatten() {
        if file.file_type().unwrap().is_dir() {
            walk_markdown_dir(&file.path(), &out_folder, &asset_folder)?;
            continue;
        }
        if file.file_name().to_string_lossy().starts_with('.') {
            continue;
        }
        if file.path().extension() == Some(std::ffi::OsStr::new("md")) {
            let markdown = process_markdown_to_html(&file.path());
            let out_file_path = out_folder.join(file.file_name()).with_extension("html");
            let asset_file_path = asset_folder.join(file.file_name()).with_extension("html");
            write_generated_html(&out_file_path, &asset_file_path, markdown).unwrap();
            continue;
        }
        if file.file_name() == "component.json" {
            let description = read_component_description(&file.path());
            let out_file_path = out_folder.join("description.txt");
            std::fs::write(out_file_path, description).unwrap();
        }
        if file.file_name() == "component.rs" {
            let source = std::fs::read_to_string(file.path())?;
            let out_file_path = out_folder.join("component.rs.html");
            write_generated_html(
                &out_file_path,
                &asset_folder.join("component.rs.html"),
                render_source_html(dioxus_code::Language::Rust, &source),
            )?;
        }
        if file.file_name() == "mod.rs" {
            let source = std::fs::read_to_string(file.path())?;
            let out_file_path = out_folder.join("mod.rs.html");
            write_generated_html(
                &out_file_path,
                &asset_folder.join("mod.rs.html"),
                render_source_html(dioxus_code::Language::Rust, &source),
            )?;
        }
        if file.file_name() == "demo.css" {
            let source = std::fs::read_to_string(file.path())?;
            let out_file_path = out_folder.join("demo.css.html");
            write_generated_html(
                &out_file_path,
                &asset_folder.join("demo.css.html"),
                render_source_html(dioxus_code::Language::Css, &source),
            )?;
        }
    }

    if dir
        .parent()
        .and_then(std::path::Path::file_name)
        .is_some_and(|name| name == "components")
    {
        let crate_folder_name = crate_component_source_folder(&folder_name);
        let mut wrote_props_metadata = false;
        let crate_component = std::path::Path::new("../dioxus-components/src/components")
            .join(crate_folder_name)
            .join("component.rs");
        if crate_component.exists() {
            println!("cargo:rerun-if-changed={}", crate_component.display());
            let source = std::fs::read_to_string(crate_component)?;
            let out_file_path = out_folder.join("component.rs.html");
            write_generated_html(
                &out_file_path,
                &asset_folder.join("component.rs.html"),
                render_source_html(dioxus_code::Language::Rust, &source),
            )?;
            if let Some(types) = curated_prop_types(&folder_name) {
                write_curated_props_metadata(crate_folder_name, types, &out_folder)?;
            } else {
                write_component_props_metadata(crate_folder_name, &source, &out_folder)?;
            }
            wrote_props_metadata = true;
        }

        let crate_style = std::path::Path::new("../dioxus-components/src/components")
            .join(crate_folder_name)
            .join("style.css");
        if crate_style.exists() {
            println!("cargo:rerun-if-changed={}", crate_style.display());
            let source = std::fs::read_to_string(crate_style)?;
            let out_file_path = out_folder.join("style.css.html");
            write_generated_html(
                &out_file_path,
                &asset_folder.join("style.css.html"),
                render_source_html(dioxus_code::Language::Css, &source),
            )?;
        } else {
            write_generated_html(
                &out_folder.join("style.css.html"),
                &asset_folder.join("style.css.html"),
                render_plain_code_block(""),
            )?;
        }

        if !wrote_props_metadata {
            std::fs::write(out_folder.join("props.rs"), "&[]\n")?;
        }
    }

    Ok(())
}

#[derive(Debug)]
struct PropMetadata {
    component: String,
    name: String,
    ty: String,
    value: String,
    docs: String,
}

fn write_component_props_metadata(
    component_name: &str,
    source: &str,
    out_folder: &std::path::Path,
) -> std::io::Result<()> {
    let mut props = extract_props_metadata(source);

    for primitive_source in read_primitive_sources(component_name)? {
        props.extend(extract_props_metadata(&primitive_source));
    }

    props.sort_by(|a, b| {
        a.component
            .cmp(&b.component)
            .then_with(|| a.name.cmp(&b.name))
    });

    write_props_rs(&props, out_folder)
}

/// Writes a curated props table built from a named allowlist of plain structs.
///
/// Used by schedule sub-pages so each documents only the structs relevant to it (e.g.
/// `ScheduleDayViewConfig`), pulling field names, types, and doc-comments from the same
/// primitive/crate sources as the full props extraction.
fn write_curated_props_metadata(
    component_name: &str,
    types: &[&str],
    out_folder: &std::path::Path,
) -> std::io::Result<()> {
    let mut sources = Vec::new();
    let crate_component = std::path::Path::new("../dioxus-components/src/components")
        .join(component_name)
        .join("component.rs");
    if crate_component.exists() {
        sources.push(std::fs::read_to_string(crate_component)?);
    }
    sources.extend(read_primitive_sources(component_name)?);

    let mut props = Vec::new();
    for source in &sources {
        props.extend(extract_named_structs(source, types));
    }

    props.sort_by(|a, b| {
        a.component
            .cmp(&b.component)
            .then_with(|| a.name.cmp(&b.name))
    });
    props.dedup_by(|a, b| a.component == b.component && a.name == b.name);

    write_props_rs(&props, out_folder)
}

fn write_props_rs(props: &[PropMetadata], out_folder: &std::path::Path) -> std::io::Result<()> {
    let mut output = String::from("&[\n");
    for prop in props {
        output.push_str("    PropMetadata {\n");
        output.push_str(&format!(
            "        component: {},\n",
            rust_string(&prop.component)
        ));
        output.push_str(&format!("        name: {},\n", rust_string(&prop.name)));
        output.push_str(&format!("        ty: {},\n", rust_string(&prop.ty)));
        output.push_str(&format!("        value: {},\n", rust_string(&prop.value)));
        output.push_str(&format!("        docs: {},\n", rust_string(&prop.docs)));
        output.push_str("    },\n");
    }
    output.push_str("]\n");

    std::fs::write(out_folder.join("props.rs"), output)
}

/// Extracts props from plain structs whose identifier is in `types`.
///
/// Unlike `extract_props_metadata`, this does not require a `#[derive(Props)]` gate, so it can
/// document config and data-model structs (which are not Dioxus props) by name.
fn extract_named_structs(source: &str, types: &[&str]) -> Vec<PropMetadata> {
    let file = match syn::parse_file(source) {
        Ok(file) => file,
        Err(_) => return Vec::new(),
    };

    let mut props = Vec::new();
    for item in file.items {
        if let syn::Item::Struct(item) = item {
            if types.contains(&item.ident.to_string().as_str()) {
                props.extend(extract_struct_props(&item));
            }
        }
    }
    props
}

fn read_primitive_sources(component_name: &str) -> std::io::Result<Vec<String>> {
    let mut sources = Vec::new();
    let path = std::path::Path::new("../primitives/src").join(format!("{component_name}.rs"));
    if path.exists() {
        println!("cargo:rerun-if-changed={}", path.display());
        sources.push(std::fs::read_to_string(path)?);
    }

    let folder = std::path::Path::new("../primitives/src").join(component_name);
    if folder.exists() {
        read_primitive_folder_sources(&folder, &mut sources)?;
    }

    Ok(sources)
}

fn read_primitive_folder_sources(
    folder: &std::path::Path,
    sources: &mut Vec<String>,
) -> std::io::Result<()> {
    for entry in std::fs::read_dir(folder)?.flatten() {
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            read_primitive_folder_sources(&path, sources)?;
        } else if path.extension() == Some(std::ffi::OsStr::new("rs")) {
            println!("cargo:rerun-if-changed={}", path.display());
            sources.push(std::fs::read_to_string(path)?);
        }
    }
    Ok(())
}

fn extract_props_metadata(source: &str) -> Vec<PropMetadata> {
    let file = match syn::parse_file(source) {
        Ok(file) => file,
        Err(_) => return Vec::new(),
    };

    let mut props = Vec::new();

    for item in file.items {
        match item {
            syn::Item::Struct(item) if has_props_derive(&item.attrs) => {
                props.extend(extract_struct_props(&item));
            }
            syn::Item::Fn(item) if has_component_attr(&item.attrs) => {
                props.extend(extract_function_props(&item));
            }
            _ => {}
        }
    }

    props
}

fn has_props_derive(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("derive") {
            return false;
        }

        let mut found = false;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("Props") {
                found = true;
            }
            Ok(())
        });
        found
    })
}

fn has_component_attr(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident("component"))
}

fn extract_struct_props(item: &syn::ItemStruct) -> Vec<PropMetadata> {
    let component = item
        .ident
        .to_string()
        .strip_suffix("Props")
        .unwrap_or(&item.ident.to_string())
        .to_string();
    let syn::Fields::Named(fields) = &item.fields else {
        return Vec::new();
    };

    fields
        .named
        .iter()
        .filter_map(|field| {
            let name = field.ident.as_ref()?.to_string();
            let ty = normalize_type(&field.ty.to_token_stream().to_string());
            Some(PropMetadata {
                component: component.clone(),
                name,
                value: prop_value(&field.attrs, &field.ty),
                ty,
                docs: doc_string(&field.attrs),
            })
        })
        .collect()
}

fn extract_function_props(item: &syn::ItemFn) -> Vec<PropMetadata> {
    let component = item.sig.ident.to_string();

    item.sig
        .inputs
        .iter()
        .filter_map(|input| {
            let syn::FnArg::Typed(pat_ty) = input else {
                return None;
            };
            let syn::Pat::Ident(pat_ident) = pat_ty.pat.as_ref() else {
                return None;
            };

            let ty = normalize_type(&pat_ty.ty.to_token_stream().to_string());
            Some(PropMetadata {
                component: component.clone(),
                name: pat_ident.ident.to_string(),
                value: prop_value(&pat_ty.attrs, &pat_ty.ty),
                ty,
                docs: doc_string(&pat_ty.attrs),
            })
        })
        .collect()
}

fn prop_value(attrs: &[syn::Attribute], ty: &syn::Type) -> String {
    let mut has_default = false;
    let mut has_extends = false;

    for attr in attrs {
        if !attr.path().is_ident("props") {
            continue;
        }

        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("default") {
                has_default = true;
            }
            if meta.path.is_ident("extends") {
                has_extends = true;
            }
            Ok(())
        });

        if let syn::Meta::List(list) = &attr.meta {
            let tokens = list.tokens.to_string();
            if let Some(value) = default_expr_from_props_tokens(&tokens) {
                return value;
            }
        }
    }

    if has_default {
        "Default::default()".to_string()
    } else if has_extends {
        "none".to_string()
    } else if is_option_type(ty) {
        "none".to_string()
    } else {
        "required".to_string()
    }
}

fn default_expr_from_props_tokens(tokens: &str) -> Option<String> {
    let default_idx = tokens.find("default")?;
    let eq_idx = tokens[default_idx..].find('=')?;
    let after_eq = tokens[default_idx + eq_idx + 1..].trim_start();

    let mut depth_paren = 0usize;
    let mut depth_bracket = 0usize;
    let mut depth_brace = 0usize;
    let mut in_string = false;
    let mut prev_escape = false;

    for (idx, ch) in after_eq.char_indices() {
        match ch {
            '"' if !prev_escape => in_string = !in_string,
            '(' if !in_string => depth_paren += 1,
            ')' if !in_string => depth_paren = depth_paren.saturating_sub(1),
            '[' if !in_string => depth_bracket += 1,
            ']' if !in_string => depth_bracket = depth_bracket.saturating_sub(1),
            '{' if !in_string => depth_brace += 1,
            '}' if !in_string => depth_brace = depth_brace.saturating_sub(1),
            ',' if !in_string && depth_paren == 0 && depth_bracket == 0 && depth_brace == 0 => {
                return Some(normalize_type(&after_eq[..idx]));
            }
            _ => {}
        }
        prev_escape = ch == '\\' && !prev_escape;
        if ch != '\\' {
            prev_escape = false;
        }
    }

    Some(normalize_type(after_eq))
}

fn is_option_type(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "Option"),
        _ => false,
    }
}

fn doc_string(attrs: &[syn::Attribute]) -> String {
    let mut docs = Vec::new();

    for attr in attrs {
        if !attr.path().is_ident("doc") {
            continue;
        }

        if let syn::Meta::NameValue(name_value) = &attr.meta {
            if let syn::Expr::Lit(expr_lit) = &name_value.value {
                if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                    let value = lit_str.value();
                    let trimmed = value.trim();
                    if !trimmed.is_empty() {
                        docs.push(trimmed.to_string());
                    }
                }
            }
        }
    }

    docs.join(" ")
}

fn normalize_type(ty: &str) -> String {
    let mut normalized = ty.trim().trim_end_matches(',').trim().to_string();

    let mut result = String::with_capacity(normalized.len());
    let mut segment_start = 0usize;
    let bytes = normalized.as_bytes();
    let mut idx = 0usize;

    while idx < bytes.len() {
        let literal_len = rust_literal_len(&normalized[idx..]);
        if literal_len == 0 {
            idx += 1;
            continue;
        }

        result.push_str(&compact_rust_spacing(&normalized[segment_start..idx]));
        result.push_str(&normalized[idx..idx + literal_len]);
        idx += literal_len;
        segment_start = idx;
    }

    result.push_str(&compact_rust_spacing(&normalized[segment_start..]));
    normalized = result;

    normalized
}

fn compact_rust_spacing(source: &str) -> String {
    let mut normalized = source.to_string();

    for (from, to) in [
        (" :: ", "::"),
        (" ::", "::"),
        (":: ", "::"),
        (" < ", "<"),
        (" <", "<"),
        ("< ", "<"),
        (" > ", ">"),
        (" >", ">"),
        ("> ", ">"),
        (" ( ", "("),
        (" (", "("),
        ("( ", "("),
        (" ) ", ")"),
        (" )", ")"),
        (") ", ")"),
        (" [ ", "["),
        (" [", "["),
        ("[ ", "["),
        (" ] ", "]"),
        (" ]", "]"),
        ("] ", "]"),
        (" ! ", "!"),
        (" !", "!"),
        ("! ", "!"),
        (" & ", "&"),
        ("& ", "&"),
    ] {
        normalized = normalized.replace(from, to);
    }

    normalized
}

fn rust_literal_len(source: &str) -> usize {
    if source.is_empty() {
        return 0;
    }

    if let Some(len) = cooked_string_literal_len(source) {
        return len;
    }

    if let Some(len) = raw_string_literal_len(source) {
        return len;
    }

    cooked_char_literal_len(source).unwrap_or(0)
}

fn cooked_string_literal_len(source: &str) -> Option<usize> {
    let mut chars = source.char_indices();
    if chars.next()?.1 != '"' {
        return None;
    }

    let mut escaped = false;
    for (idx, ch) in chars {
        if escaped {
            escaped = false;
            continue;
        }

        match ch {
            '\\' => escaped = true,
            '"' => return Some(idx + ch.len_utf8()),
            _ => {}
        }
    }

    Some(source.len())
}

fn raw_string_literal_len(source: &str) -> Option<usize> {
    let rest = source.strip_prefix('r')?;
    let hashes = rest.chars().take_while(|&ch| ch == '#').count();
    let rest = &rest[hashes..];
    if !rest.starts_with('"') {
        return None;
    }

    let terminator = format!("\"{}", "#".repeat(hashes));
    let body = &rest[1..];
    let end = body.find(&terminator)?;
    Some(1 + hashes + 1 + end + terminator.len())
}

fn cooked_char_literal_len(source: &str) -> Option<usize> {
    let mut chars = source.char_indices();
    if chars.next()?.1 != '\'' {
        return None;
    }

    let (content_idx, content) = chars.next()?;
    let closing_idx = if content == '\\' {
        cooked_char_escape_len(&source[content_idx..]).and_then(|escape_len| {
            source[content_idx + escape_len..]
                .chars()
                .next()
                .filter(|&ch| ch == '\'')
                .map(|ch| content_idx + escape_len + ch.len_utf8())
        })?
    } else {
        if content == '\'' || content == '\n' || content == '\r' {
            return None;
        }

        source[content_idx + content.len_utf8()..]
            .chars()
            .next()
            .filter(|&ch| ch == '\'')
            .map(|ch| content_idx + content.len_utf8() + ch.len_utf8())?
    };

    Some(closing_idx)
}

fn cooked_char_escape_len(source: &str) -> Option<usize> {
    let mut chars = source.char_indices();
    if chars.next()?.1 != '\\' {
        return None;
    }

    let (_, escape) = chars.next()?;
    match escape {
        '\'' | '"' | '\\' | 'n' | 'r' | 't' | '0' => Some(1 + escape.len_utf8()),
        'x' => {
            let mut len = 1 + escape.len_utf8();
            for _ in 0..2 {
                let (idx, ch) = chars.next()?;
                if !ch.is_ascii_hexdigit() {
                    return None;
                }
                len = idx + ch.len_utf8();
            }
            Some(len)
        }
        'u' => {
            let (_, ch) = chars.next()?;
            if ch != '{' {
                return None;
            }

            let mut digits = 0usize;
            for (idx, ch) in chars {
                if ch == '}' {
                    return (digits > 0).then_some(idx + ch.len_utf8());
                }
                if !ch.is_ascii_hexdigit() || digits >= 6 {
                    return None;
                }
                digits += 1;
            }
            None
        }
        _ => None,
    }
}

fn rust_string(value: &str) -> String {
    format!("{value:?}")
}

fn process_markdown_to_html(markdown_path: &std::path::Path) -> String {
    println!("cargo:rerun-if-changed={}", markdown_path.display());
    use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd};
    let markdown_input =
        std::fs::read_to_string(markdown_path).expect("Failed to read markdown file");
    let mut options = Options::empty();
    options.insert(Options::ENABLE_GFM);
    let parser = Parser::new_ext(&markdown_input, options);
    let mut events = Vec::new();
    let mut code_block: Option<(CodeBlockKind<'_>, String)> = None;

    for event in parser {
        match (&mut code_block, event) {
            (None, Event::Start(Tag::CodeBlock(kind))) => {
                code_block = Some((kind, String::new()));
            }
            (Some((_, source)), Event::Text(text)) => {
                source.push_str(&text);
            }
            (Some((kind, source)), Event::End(TagEnd::CodeBlock)) => {
                events.push(Event::Html(CowStr::Boxed(
                    render_code_block_html(kind, source).into_boxed_str(),
                )));
                code_block = None;
            }
            (None, event) => events.push(event),
            (Some((_, source)), Event::Code(text)) => {
                source.push_str(&text);
            }
            (Some((_, source)), Event::Html(html) | Event::InlineHtml(html)) => {
                source.push_str(&html);
            }
            (Some(_), _) => {}
        }
    }

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, events.into_iter());
    html_output
}

fn read_component_description(component_json_path: &std::path::Path) -> String {
    println!("cargo:rerun-if-changed={}", component_json_path.display());
    let input =
        std::fs::read_to_string(component_json_path).expect("Failed to read component metadata");
    let json: serde_json::Value =
        serde_json::from_str(&input).expect("Failed to parse component metadata");
    json.get("description")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("A Dioxus component.")
        .trim()
        .to_string()
}

fn render_code_block_html(kind: &pulldown_cmark::CodeBlockKind<'_>, source: &str) -> String {
    let language = match kind {
        pulldown_cmark::CodeBlockKind::Fenced(info) => {
            let slug = info.split_whitespace().next().unwrap_or_default();
            language_from_slug(slug)
        }
        pulldown_cmark::CodeBlockKind::Indented => None,
    };

    let Some(language) = language else {
        return render_plain_code_block(source);
    };

    let source = source.trim_end_matches('\n');
    let highlighted: dioxus_code::advanced::HighlightedSource =
        dioxus_code::SourceCode::new(language, source).into();

    dioxus_ssr::render_element(rsx! {
        div {
            class: "dx-preview-code-theme",
            tabindex: "0",
            dioxus_code::Code {
                src: highlighted,
                theme: dioxus_code::CodeTheme::system(
                    dioxus_code::Theme::GITHUB_LIGHT,
                    dioxus_code::Theme::GITHUB_DARK,
                ),
            }
        }
    })
}

fn render_plain_code_block(source: &str) -> String {
    let source = source.trim_end_matches('\n');

    dioxus_ssr::render_element(rsx! {
        pre {
            code { "{source}" }
        }
    })
}

fn render_source_html(language: dioxus_code::Language, source: &str) -> String {
    let highlighted: dioxus_code::advanced::HighlightedSource =
        dioxus_code::SourceCode::new(language, source.trim_end_matches('\n')).into();

    dioxus_ssr::render_element(rsx! {
        div {
            class: "dx-preview-code-theme",
            tabindex: "0",
            dioxus_code::Code {
                src: highlighted,
                theme: dioxus_code::CodeTheme::system(
                    dioxus_code::Theme::GITHUB_LIGHT,
                    dioxus_code::Theme::GITHUB_DARK,
                ),
            }
        }
    })
}

fn render_theme_css(
    out_dir: &std::path::Path,
    generated_assets_dir: &std::path::Path,
) -> std::io::Result<()> {
    let theme_path = std::path::Path::new("../themes/default.css");
    println!("cargo:rerun-if-changed={}", theme_path.display());
    let source = std::fs::read_to_string(theme_path)?;
    let app_assets = std::path::Path::new("assets");
    std::fs::create_dir_all(app_assets)?;
    std::fs::write(app_assets.join("dx-components-theme.css"), &source)?;

    let out_assets = out_dir.join("assets");
    std::fs::create_dir_all(&out_assets)?;
    let generated_assets = generated_assets_dir.join("assets");
    std::fs::create_dir_all(&generated_assets)?;
    write_generated_html(
        &out_assets.join("dx-components-theme.css.html"),
        &generated_assets.join("dx-components-theme.css.html"),
        render_source_html(dioxus_code::Language::Css, &source),
    )
}

fn language_from_slug(slug: &str) -> Option<dioxus_code::Language> {
    match slug {
        "" => Some(dioxus_code::Language::Rust),
        "rs" => Some(dioxus_code::Language::Rust),
        "rust" => Some(dioxus_code::Language::Rust),
        "css" => Some(dioxus_code::Language::Css),
        slug => dioxus_code::Language::from_slug(slug),
    }
}

#[cfg(test)]
mod tests {
    use super::{default_expr_from_props_tokens, extract_props_metadata, normalize_type};

    #[test]
    fn default_expr_preserves_nested_default_expression_parens() {
        assert_eq!(
            default_expr_from_props_tokens("default = Some(Default::default())"),
            Some("Some(Default::default())".to_string())
        );
        assert_eq!(
            default_expr_from_props_tokens("default = vec![format!(\"x{}\", 1)]"),
            Some("vec![format!(\"x{}\", 1)]".to_string())
        );
    }

    #[test]
    fn default_expr_keeps_simple_defaults_working() {
        assert_eq!(
            default_expr_from_props_tokens("default = true"),
            Some("true".to_string())
        );
        assert_eq!(
            default_expr_from_props_tokens("default = \"hello\""),
            Some("\"hello\"".to_string())
        );
    }

    #[test]
    fn default_expr_preserves_raw_string_literals() {
        assert_eq!(
            default_expr_from_props_tokens("default = r#\"Vec < Attribute >\"#"),
            Some("r#\"Vec < Attribute >\"#".to_string())
        );
    }

    #[test]
    fn normalize_type_compacts_generic_spacing_from_token_streams() {
        assert_eq!(normalize_type("Vec < Attribute >"), "Vec<Attribute>");
        assert_eq!(
            normalize_type("Option < EventHandler < MouseEvent > >"),
            "Option<EventHandler<MouseEvent>>"
        );
        assert_eq!(normalize_type("Option < &'a str >"), "Option<&'a str>");
        assert_eq!(
            normalize_type("Some ( Default :: default () )"),
            "Some(Default::default())"
        );
    }

    #[test]
    fn normalize_type_preserves_string_and_char_literal_contents() {
        assert_eq!(
            normalize_type("format! ( \"a :: b\" , Some ( \"Vec < Attribute >\" ) , '<' )"),
            "format!(\"a :: b\", Some(\"Vec < Attribute >\"), '<')"
        );
        assert_eq!(
            default_expr_from_props_tokens("default = Some(\"Vec < Attribute >\")"),
            Some("Some(\"Vec < Attribute >\")".to_string())
        );
    }

    #[test]
    fn extract_props_metadata_separates_docs_from_field_names() {
        let source = r#"
            #[derive(Props, Clone, PartialEq)]
            pub struct TimeInputProps {
                /// Accessibility labels for segments and clear affordances.
                #[props(default)]
                pub labels: TimePickerLabels,
            }
        "#;

        let props = extract_props_metadata(source);
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].name, "labels");
        assert_eq!(
            props[0].docs,
            "Accessibility labels for segments and clear affordances."
        );
    }
}
