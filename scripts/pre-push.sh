#!/usr/bin/env sh
set -eu

repo_root=$(git rev-parse --show-toplevel)
cd "$repo_root"

# Rust doctests invoke linkers that use TMPDIR for large temporary outputs. Keep
# those outputs on the repository filesystem rather than a constrained /tmp tmpfs.
TMPDIR="$repo_root/target/tmp/pre-push"
mkdir -p "$TMPDIR"
export TMPDIR

# Linux desktop builds use GTK/WebKit system libraries. Prefer the distro
# pkg-config metadata so Homebrew's Linux pkg-config does not hide libffi,
# bzip2, and the GTK dependency files.
if [ "$(uname -s)" = "Linux" ] && [ -x /usr/bin/pkg-config ]; then
  PKG_CONFIG=/usr/bin/pkg-config
  export PKG_CONFIG
  PKG_CONFIG_PATH="/usr/lib/$(uname -m)-linux-gnu/pkgconfig:/usr/lib64/pkgconfig:/usr/share/pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
  export PKG_CONFIG_PATH
fi

# Limit Cargo parallel compilation concurrency to prevent rustdoc/rustc process
# fan-out and system memory exhaustion on high-core CPUs.
CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}"
export CARGO_BUILD_JOBS

printf '%s\n' 'Running pre-push checks...'
cargo check --workspace --all-features
cargo test --workspace --lib --bins
cargo fmt --all -- --check

(
  cd preview
  npx stylelint "src/**/*.css" --max-warnings=0
)

node scripts/validate-theme-contrast.mjs
scripts/preview-web.sh build --no-wasm-split

(
  cd playwright

  # Playwright cannot install its bundled Chromium on every supported Linux
  # distribution. Reuse a compatible system browser when one is available.
  chromium_executable=${PLAYWRIGHT_CHROMIUM_EXECUTABLE:-}
  if [ -n "$chromium_executable" ] && [ ! -x "$chromium_executable" ]; then
    chromium_executable=
  fi

  if [ -z "$chromium_executable" ]; then
    for chromium_command in chromium chromium-browser google-chrome google-chrome-stable chrome chrome-browser; do
      chromium_executable=$(command -v "$chromium_command" 2>/dev/null || true)
      if [ -n "$chromium_executable" ] && [ -x "$chromium_executable" ]; then
        break
      fi
      chromium_executable=
    done
  fi

  if [ -n "$chromium_executable" ]; then
    export PLAYWRIGHT_CHROMIUM_EXECUTABLE=$chromium_executable
    printf '%s\n' "Using system Chromium: $chromium_executable"
  else
    printf '%s\n' 'No system Chromium found; installing the Playwright Chromium browser...'
    npx playwright install chromium
  fi

  # Harness-backed specs require fixtures that intentionally do not ship in the
  # preview gallery. Run each against its matching isolated binary, then run the
  # remaining preview specs against the full preview app.
  node --input-type=module -e '
    import { spawnSync } from "node:child_process";
    import { existsSync } from "node:fs";
    import { MICRO_HARNESS_COMPONENTS } from "./micro-harness-policy.mjs";

    for (const component of MICRO_HARNESS_COMPONENTS) {
      const spec = existsSync(`${component}.spec.ts`)
        ? `${component}.spec.ts`
        : `${component.replaceAll("_", "-")}.spec.ts`;
      const result = spawnSync(
        "npx",
        ["playwright", "test", spec, "--project=chromium"],
        { stdio: "inherit", env: process.env },
      );
      if (result.status !== 0) process.exit(result.status ?? 1);
    }
  '
  PLAYWRIGHT_PREVIEW_ONLY=1 npx playwright test --project=chromium

)
printf '%s\n' 'Pre-push checks passed.'
