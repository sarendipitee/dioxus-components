#!/usr/bin/env bash

dioxus_components_mise_workspace_root() {
	git -C "${MISE_PROJECT_ROOT:-.}" rev-parse --show-toplevel 2>/dev/null || pwd -P
}

dioxus_components_resolve_kache() {
	local workspace_root="$1"
	local kache_bin

	if kache_bin="$(command -v kache 2>/dev/null)"; then
		printf '%s\n' "$kache_bin"
		return 0
	fi
	if command -v mise >/dev/null 2>&1 &&
		kache_bin="$(mise -C "$workspace_root" which kache 2>/dev/null)" &&
		[ -n "$kache_bin" ]; then
		printf '%s\n' "$kache_bin"
		return 0
	fi

	printf 'Kache is required but unavailable; run `mise install --locked` or add kache to PATH\n' >&2
	return 1
}

dioxus_components_mise_export_environment() {
	local workspace_root
	local shared_kache_script
	local kache_bin

	workspace_root="${MISE_PROJECT_ROOT:-$(dioxus_components_mise_workspace_root)}"
	shared_kache_script="$workspace_root/scripts/shared-kache-service.sh"
	if [ ! -f "$shared_kache_script" ]; then
		printf 'shared kache helper missing: %s\n' "$shared_kache_script" >&2
		return 1
	fi

	# shellcheck disable=SC1090
	. "$shared_kache_script"
	shared_kache_export_environment
	export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
	export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"
	if [ -z "${RUSTC_WRAPPER:-}" ]; then
		kache_bin="$(dioxus_components_resolve_kache "$workspace_root")" || return 1
		export RUSTC_WRAPPER="$kache_bin"
	fi
}

# Build launchers that cannot assume Mise was sourced use this stricter entry
# point. It preserves compatible caller cache configuration, but never permits
# a Playwright build to fall through to raw rustc or incremental compilation.
dioxus_components_export_required_kache_environment() {
	local resolved_wrapper
	local wrapper_name

	dioxus_components_mise_export_environment || return 1

	resolved_wrapper="$RUSTC_WRAPPER"
	if [ "${resolved_wrapper#*/}" = "$resolved_wrapper" ]; then
		resolved_wrapper="$(command -v "$resolved_wrapper" 2>/dev/null)" || {
			printf 'Configured RUSTC_WRAPPER is not executable: %s\n' "$RUSTC_WRAPPER" >&2
			return 1
		}
	elif [ ! -x "$resolved_wrapper" ]; then
		printf 'Configured RUSTC_WRAPPER is not executable: %s\n' "$resolved_wrapper" >&2
		return 1
	fi

	wrapper_name="${resolved_wrapper##*/}"
	if [ "$wrapper_name" != 'kache' ]; then
		printf 'Playwright builds require Kache; RUSTC_WRAPPER resolved to %s\n' "$resolved_wrapper" >&2
		return 1
	fi

	export RUSTC_WRAPPER="$resolved_wrapper"
	export CARGO_INCREMENTAL=0
}

if [ "${DIOXUS_COMPONENTS_REQUIRE_KACHE:-0}" = '1' ]; then
	unset DIOXUS_COMPONENTS_REQUIRE_KACHE
	dioxus_components_export_required_kache_environment
else
	dioxus_components_mise_export_environment
fi
