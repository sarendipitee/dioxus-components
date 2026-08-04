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
  npx playwright install
  npx playwright test
)
printf '%s\n' 'Pre-push checks passed.'
