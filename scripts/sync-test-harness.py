#!/usr/bin/env python3
import os
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
COMPONENTS_DIR = os.path.join(REPO_ROOT, "preview", "src", "components")
HARNESS_BIN_DIR = os.path.join(REPO_ROOT, "test-harness", "src", "bin")

os.makedirs(HARNESS_BIN_DIR, exist_ok=True)

def generate_harness_binary(comp_name):
    demos_dir = os.path.join(COMPONENTS_DIR, comp_name, "demos")
    if not os.path.exists(demos_dir):
        return

    demos = []
    for item in sorted(os.listdir(demos_dir)):
        demo_mod = os.path.join(demos_dir, item, "mod.rs")
        if os.path.isfile(demo_mod):
            demos.append(item)

    if not demos:
        return

    # Normalize binary name (e.g. color_picker -> color_picker)
    bin_name = comp_name.replace("-", "_")

    code_lines = [
        "use dioxus::prelude::*;",
        "",
    ]

    for d in demos:
        mod_ident = f"demo_{d}"
        path_str = f"../../../preview/src/components/{comp_name}/demos/{d}/mod.rs"
        code_lines.append(f'#[path = "{path_str}"]')
        code_lines.append(f"mod {mod_ident};")
        code_lines.append("")

    code_lines.extend([
        "fn main() {",
        "    dioxus::launch(App);",
        "}",
        "",
        "#[component]",
        "fn App() -> Element {",
        '    rsx! {',
        '        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }',
        '        document::Link {',
        '            rel: "stylesheet",',
        '            href: asset!("/assets/dx-components-theme.css"),',
        '        }',
        '        div { id: "dx-preview-block-root", style: "min-height: 100vh;",',
        '            BlockView {}',
        '        }',
        '    }',
        "}",
        "",
        "#[component]",
        "fn BlockView() -> Element {",
        "    let hash = use_signal(|| {",
        '        #[cfg(target_arch = "wasm32")]',
        "        {",
        "            let window = web_sys::window().unwrap();",
        "            let location = window.location();",
        "            let h = location.hash().unwrap_or_default();",
        "            h.trim_start_matches('#').to_string()",
        "        }",
        '        #[cfg(not(target_arch = "wasm32"))]',
        "        {",
        '            "".to_string()',
        "        }",
        "    });",
        "",
        "    match hash().as_str() {",
    ])

    for d in demos:
        mod_ident = f"demo_{d}"
        code_lines.append(f'        "{d}" => rsx! {{ {mod_ident}::Demo {{}} }},')

    first_demo = demos[0]
    code_lines.append(f'        _ => rsx! {{ demo_{first_demo}::Demo {{}} }},')
    code_lines.append("    }")
    code_lines.append("}")
    code_lines.append("")

    out_file = os.path.join(HARNESS_BIN_DIR, f"{bin_name}.rs")
    with open(out_file, "w", encoding="utf-8") as f:
        f.write("\n".join(code_lines))

def main():
    if not os.path.exists(COMPONENTS_DIR):
        print(f"Error: {COMPONENTS_DIR} does not exist", file=sys.stderr)
        sys.exit(1)

    for item in os.listdir(COMPONENTS_DIR):
        comp_dir = os.path.join(COMPONENTS_DIR, item)
        if os.path.isdir(comp_dir):
            generate_harness_binary(item)

    print(f"Successfully synchronized micro-binaries in {HARNESS_BIN_DIR}")

if __name__ == "__main__":
    main()
