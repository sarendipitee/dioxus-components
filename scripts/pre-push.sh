#!/usr/bin/env bash
set -euo pipefail

repo_root=$(git rev-parse --show-toplevel)
cd "$repo_root"

PATH=/usr/bin:/bin:$PATH SHARED_KACHE_TEST_ENDPOINT=http://127.0.0.1:29989 scripts/shared-kache-service.spec.sh
scripts/mise-env.spec.sh

cache_wrapper_added=0
if [ "${KACHE_SHARED_SERVICE:-1}" != '0' ] && command -v mise >/dev/null 2>&1; then
  if kache_bin=$(mise -C "$repo_root" which kache 2>/dev/null) && rustfs_bin=$(mise -C "$repo_root" which rustfs 2>/dev/null); then
    if [ -z "${RUSTC_WRAPPER:-}" ]; then
      export RUSTC_WRAPPER=$kache_bin
      cache_wrapper_added=1
    fi
    export KACHE_VERSION="${KACHE_VERSION:-0.8.0}"
    export RUSTFS_VERSION="${RUSTFS_VERSION:-1.0.0-beta.8}"
    # shellcheck disable=SC1091
    . "$repo_root/scripts/mise-env.sh"
    if ! "$repo_root/scripts/mise-ensure-shared-kache.sh"; then
      printf '%s\n' 'warning: shared Kache service unavailable; continuing without Kache wrapper' >&2
      if [ "$cache_wrapper_added" -eq 1 ]; then
        unset RUSTC_WRAPPER
      fi
    fi
  else
    printf '%s\n' 'warning: Mise cache tools missing; run mise install --locked' >&2
  fi
fi

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
  # preview gallery. Shared runner selects each matching isolated binary, then
  # runs remaining specs against the full preview app.
  node run-suite.mjs

)
printf '%s\n' 'Pre-push checks passed.'
