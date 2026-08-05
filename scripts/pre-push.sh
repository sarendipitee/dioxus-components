#!/usr/bin/env sh
set -eu

repo_root=$(git rev-parse --show-toplevel)
cd "$repo_root"

# Linux desktop builds use GTK/WebKit system libraries. Prefer the distro
# pkg-config metadata so Homebrew's Linux pkg-config does not hide libffi,
# bzip2, and the GTK dependency files.
if [ "$(uname -s)" = "Linux" ] && [ -x /usr/bin/pkg-config ]; then
  PKG_CONFIG=/usr/bin/pkg-config
  export PKG_CONFIG
  PKG_CONFIG_PATH="/usr/lib/$(uname -m)-linux-gnu/pkgconfig:/usr/lib64/pkgconfig:/usr/share/pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
  export PKG_CONFIG_PATH
fi

printf '%s\n' 'Running pre-push checks...'
cargo check --workspace --all-features
cargo test --workspace
cargo fmt --all -- --check
RUSTDOCFLAGS='--document-private-items' cargo doc --workspace --no-deps --all-features --document-private-items

(
  cd preview
  npx stylelint "src/**/*.css" --max-warnings=0
)

node scripts/validate-theme-contrast.mjs
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

  # Run the Chromium project explicitly: Firefox and WebKit downloads are not
  # available on every Linux distribution supported by Playwright.
  npx playwright test --project=chromium

)
printf '%s\n' 'Pre-push checks passed.'
