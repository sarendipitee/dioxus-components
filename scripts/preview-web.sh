#!/usr/bin/env sh
set -eu

expected_dx_version='0.7.10'
script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo_root=$(CDPATH= cd -- "$script_dir/.." && pwd)
target_dir=${CARGO_TARGET_DIR:-"$repo_root/target"}
wasm_split=true

case "${1:-}" in
  build | serve)
    command=$1
    shift
    ;;
  *)
    command=serve
    ;;
esac

if [ "${1:-}" = --no-wasm-split ]; then
  wasm_split=false
  shift
fi

if [ "$wasm_split" = true ]; then
  public_dir="$target_dir/dx/preview/release/web/public"
  set -- --wasm-split --features wasm-split --release --debug-symbols=false "$@"
else
  public_dir="$target_dir/dx/preview/debug/web/public"
fi

is_supported_dx() {
  [ -x "$1" ] || return 1
  version_output=$("$1" --version 2>/dev/null) || return 1
  for version_token in $version_output; do
    [ "$version_token" = "$expected_dx_version" ] && return 0
  done
  return 1
}

if [ -n "${DIOXUS_CLI:-}" ]; then
  if ! is_supported_dx "$DIOXUS_CLI"; then
    printf 'DIOXUS_CLI must name an executable Dioxus CLI version %s: %s\n' "$expected_dx_version" "$DIOXUS_CLI" >&2
    exit 127
  fi
  dx=$DIOXUS_CLI
elif path_dx=$(command -v dx 2>/dev/null) && is_supported_dx "$path_dx"; then
  dx=$path_dx
elif mise_dx="${HOME:-}/.local/share/mise/installs/cargo-dioxus-cli/$expected_dx_version/bin/dx" && is_supported_dx "$mise_dx"; then
  dx=$mise_dx
else
  printf 'Unable to find Dioxus CLI version %s. Set DIOXUS_CLI to its executable path.\n' "$expected_dx_version" >&2
  exit 127
fi

"$script_dir/clean-preview-dx-assets.sh"

if [ "$command" = build ]; then
  rm -rf "${public_dir%/public}"
  if "$dx" build -p preview --web "$@"; then
    :
  else
    exit $?
  fi

  index="$public_dir/index.html"
  if [ ! -s "$index" ]; then
    printf 'Dioxus build completed without a nonempty preview index: %s\n' "$index" >&2
    exit 1
  fi

  js_src=$(sed -n 's/.*src=["'"'"']\([^"'"'"']*\.js\)["'"'"'].*/\1/p' "$index" | sed -n '1p')
  if [ -z "$js_src" ]; then
    printf 'Dioxus build completed without a JavaScript entrypoint in %s\n' "$index" >&2
    exit 1
  fi

  case "$js_src" in
    *://* | //*)
      printf 'Dioxus build produced a non-local JavaScript entrypoint: %s\n' "$js_src" >&2
      exit 1
      ;;
  esac
  js_relative=$js_src
  while [ "${js_relative#/}" != "$js_relative" ]; do js_relative=${js_relative#/}; done
  while [ "${js_relative#./}" != "$js_relative" ]; do js_relative=${js_relative#./}; done
  case "$js_relative" in
    "" | .. | ../* | */../* | */..)
      printf 'Dioxus build produced an unsafe JavaScript entrypoint: %s\n' "$js_src" >&2
      exit 1
      ;;
  esac
  js_path="$public_dir/$js_relative"
  if [ ! -s "$js_path" ]; then
    printf 'Dioxus build completed without nonempty JavaScript entrypoint: %s\n' "$js_path" >&2
    exit 1
  fi

  if ! find "$public_dir/wasm" -type f -name '*.wasm' -size +0c -print -quit 2>/dev/null | grep -q .; then
    printf 'Dioxus build completed without a nonempty WASM payload under %s/wasm\n' "$public_dir" >&2
    exit 1
  fi
else
  exec "$dx" serve -p preview --web "$@"
fi
