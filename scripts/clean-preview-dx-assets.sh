#!/usr/bin/env sh
set -eu

repo_root="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
target_dir="${CARGO_TARGET_DIR:-"$repo_root/target"}"

rm -rf \
  "$target_dir/dx/preview/debug/web/public/assets" \
  "$target_dir/dx/preview/release/web/public/assets" \
  "$target_dir"/*/dx/preview/debug/web/public/assets \
  "$target_dir"/*/dx/preview/release/web/public/assets
