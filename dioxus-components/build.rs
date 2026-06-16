use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("missing CARGO_MANIFEST_DIR"));
    let components_dir = manifest_dir.join("src/components");
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("missing OUT_DIR"));
    let output_path = out_dir.join("dioxus-components.css");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", components_dir.display());

    let mut style_files = component_style_files(&components_dir);
    style_files.sort();

    let mut combined = String::new();
    for style_path in style_files {
        println!("cargo:rerun-if-changed={}", style_path.display());

        if !combined.is_empty() {
            combined.push('\n');
        }

        let contents = fs::read_to_string(&style_path)
            .unwrap_or_else(|err| panic!("failed to read {}: {err}", style_path.display()));
        combined.push_str(&contents);
        if !contents.ends_with('\n') {
            combined.push('\n');
        }
    }

    fs::write(&output_path, combined)
        .unwrap_or_else(|err| panic!("failed to write {}: {err}", output_path.display()));
}

fn component_style_files(components_dir: &Path) -> Vec<PathBuf> {
    let mut style_files = Vec::new();

    let entries = fs::read_dir(components_dir)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", components_dir.display()));

    for entry in entries {
        let entry = entry.unwrap_or_else(|err| panic!("failed to read directory entry: {err}"));
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let style_path = path.join("style.css");
        if style_path.is_file() {
            style_files.push(style_path);
        }
    }

    style_files
}
