#!/usr/bin/env bash

dioxus_components_mise_workspace_root() {
	git -C "${MISE_PROJECT_ROOT:-.}" rev-parse --show-toplevel 2>/dev/null || pwd -P
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
		if ! kache_bin="$(command -v kache 2>/dev/null)"; then
			printf 'Mise-managed kache missing from PATH\n' >&2
			return 1
		fi
		export RUSTC_WRAPPER="$kache_bin"
	fi
}

dioxus_components_mise_export_environment
