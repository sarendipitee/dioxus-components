#!/usr/bin/env bash
set -euo pipefail

workspace_root="$(git -C "${MISE_PROJECT_ROOT:-.}" rev-parse --show-toplevel 2>/dev/null || (cd "${MISE_PROJECT_ROOT:-.}" && pwd -P))"
shared_kache_script="$workspace_root/scripts/shared-kache-service.sh"
bucket_script="$workspace_root/scripts/rustfs-ensure-bucket.py"

if [ ! -f "$shared_kache_script" ]; then
	printf 'shared kache helper missing: %s\n' "$shared_kache_script" >&2
	exit 1
fi
if [ ! -f "$bucket_script" ]; then
	printf 'rustfs bucket helper missing: %s\n' "$bucket_script" >&2
	exit 1
fi

# shellcheck disable=SC1090
. "$shared_kache_script"

if [ "${CI:-}" = 'true' ] && [ "${SHARED_KACHE_FORCE_LOCAL:-0}" != '1' ]; then
	exit 0
fi
if [ "${KACHE_SHARED_SERVICE:-1}" = '0' ]; then
	exit 0
fi

if ! kache_bin="$(mise -C "$workspace_root" which kache 2>/dev/null || command -v kache)"; then
	printf 'required Mise tool missing: kache; run mise install --locked\n' >&2
	exit 1
fi
if ! rustfs_bin="$(mise -C "$workspace_root" which rustfs 2>/dev/null || command -v rustfs)"; then
	printf 'required Mise tool missing: rustfs; run mise install --locked\n' >&2
	exit 1
fi

if shared_kache_ensure "$kache_bin" "$rustfs_bin" "$bucket_script"; then
	exit 0
else
	initial_result=$?
fi

# Kache can leave legacy socket locks after an unclean daemon exit. Its
# official restart command clears them, but starts an unmanaged process. Run
# restart followed by stop under the shared lifecycle lock, then let the
# manager retry and capture the new detached process itself.
shared_kache_init_paths
if [ ! -x "$shared_kache_kache_bin" ] || \
	! shared_kache_rustfs_pid_running || \
	shared_kache_daemon_pid_running; then
	exit "$initial_result"
fi

printf 'recovering stale Kache daemon locks\n' >&2
shared_kache_acquire_lock || exit "$initial_result"
recovery_result=0
if ! shared_kache_daemon_pid_running; then
	AWS_ACCESS_KEY_ID='kachelocal' \
	AWS_REGION='us-east-1' \
	AWS_SECRET_ACCESS_KEY='kachelocalsecret' \
	KACHE_S3_ACCESS_KEY='kachelocal' \
	KACHE_S3_SECRET_KEY='kachelocalsecret' \
		"$shared_kache_kache_bin" daemon restart >/dev/null 2>&1 || recovery_result=$?
	if [ "$recovery_result" -eq 0 ]; then
		"$shared_kache_kache_bin" daemon stop >/dev/null 2>&1 || recovery_result=$?
	fi
fi
shared_kache_release_lock
[ "$recovery_result" -eq 0 ] || exit "$recovery_result"

shared_kache_ensure "$kache_bin" "$rustfs_bin" "$bucket_script"
