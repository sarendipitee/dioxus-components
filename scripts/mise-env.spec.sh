#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "$0")" && pwd -P)"
repo_root="$(cd "$script_dir/.." && pwd -P)"
test_root="$(mktemp -d "${TMPDIR:-/tmp}/dioxus-components-mise-env.XXXXXX")"
trap 'rm -rf "$test_root"' EXIT

mkdir -p "$test_root/bin" "$test_root/config"
printf '#!/usr/bin/env sh\nexit 0\n' > "$test_root/bin/kache"
printf '#!/usr/bin/env sh\nexit 0\n' > "$test_root/bin/not-kache"
chmod +x "$test_root/bin/kache" "$test_root/bin/not-kache"

fail() {
	printf 'mise-env spec failed: %s\n' "$1" >&2
	exit 1
}

unset RUSTC_WRAPPER CARGO_INCREMENTAL CI KACHE_S3_ENDPOINT KACHE_S3_BUCKET KACHE_S3_PREFIX
export PATH="$test_root/bin:$PATH"
export XDG_CONFIG_HOME="$test_root/config"
export MISE_PROJECT_ROOT="$repo_root"
# shellcheck disable=SC1091
. "$script_dir/mise-env.sh"

[ "$RUSTC_WRAPPER" = "$test_root/bin/kache" ] || fail 'default wrapper is not absolute Mise kache path'
[ "$CARGO_INCREMENTAL" = '0' ] || fail 'Cargo incremental compilation not disabled'
[ "$KACHE_S3_ENDPOINT" = 'http://127.0.0.1:19000' ] || fail 'local RustFS endpoint not exported'
[ "$KACHE_S3_BUCKET" = 'shared-local' ] || fail 'shared bucket not exported'
[ "$KACHE_CONFIG" = "$test_root/config/kache/config.toml" ] || fail 'XDG config path not exported'

RUSTC_WRAPPER='/caller/wrapper'
CARGO_INCREMENTAL='1'
export RUSTC_WRAPPER CARGO_INCREMENTAL
. "$script_dir/mise-env.sh"
[ "$RUSTC_WRAPPER" = '/caller/wrapper' ] || fail 'caller wrapper overwritten'
[ "$CARGO_INCREMENTAL" = '1' ] || fail 'caller incremental setting overwritten'

unset RUSTC_WRAPPER
CARGO_INCREMENTAL='1'
DIOXUS_COMPONENTS_REQUIRE_KACHE='1'
export CARGO_INCREMENTAL DIOXUS_COMPONENTS_REQUIRE_KACHE
. "$script_dir/mise-env.sh"
[ "$RUSTC_WRAPPER" = "$test_root/bin/kache" ] || fail 'required mode did not resolve Kache'
[ "$CARGO_INCREMENTAL" = '0' ] || fail 'required mode preserved incremental compilation'
[ -z "${DIOXUS_COMPONENTS_REQUIRE_KACHE:-}" ] || fail 'required mode leaked its control variable'

RUSTC_WRAPPER='kache'
CARGO_INCREMENTAL='1'
DIOXUS_COMPONENTS_REQUIRE_KACHE='1'
export RUSTC_WRAPPER CARGO_INCREMENTAL DIOXUS_COMPONENTS_REQUIRE_KACHE
. "$script_dir/mise-env.sh"
[ "$RUSTC_WRAPPER" = "$test_root/bin/kache" ] || fail 'compatible caller wrapper was not resolved'
[ "$CARGO_INCREMENTAL" = '0' ] || fail 'required mode did not override caller incremental setting'

RUSTC_WRAPPER="$test_root/bin/not-kache"
DIOXUS_COMPONENTS_REQUIRE_KACHE='1'
export RUSTC_WRAPPER DIOXUS_COMPONENTS_REQUIRE_KACHE
if (. "$script_dir/mise-env.sh") 2>"$test_root/rejected-wrapper.log"; then
	fail 'required mode accepted a non-Kache wrapper'
fi
case "$(cat "$test_root/rejected-wrapper.log")" in
	*'Playwright builds require Kache'*) ;;
	*) fail 'required mode did not diagnose a non-Kache wrapper' ;;
esac

printf 'mise-env specs passed\n'
