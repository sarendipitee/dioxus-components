#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "$0")" && pwd -P)"
repo_root="$(cd "$script_dir/.." && pwd -P)"

export KACHE_VERSION="${KACHE_VERSION:-0.8.0}"
export RUSTFS_VERSION="${RUSTFS_VERSION:-1.0.0-beta.8}"

# shellcheck disable=SC1091
. "$script_dir/shared-kache-service.sh"
shared_kache_export_environment
shared_kache_init_paths

usage() {
	printf 'usage: %s {ensure|status|doctor|logs}\n' "$0" >&2
}

case "${1:-}" in
	ensure)
		exec "$script_dir/mise-ensure-shared-kache.sh"
		;;
	status)
		printf 'RustFS endpoint: %s\n' "$KACHE_S3_ENDPOINT"
		if shared_kache_rustfs_pid_running; then
			printf 'RustFS: running (PID %s)\n' "$(cat "$shared_kache_rustfs_pid_file")"
		else
			printf 'RustFS: stopped\n'
		fi
		if shared_kache_daemon_pid_running; then
			printf 'Kache: running (PID %s)\n' "$(cat "$shared_kache_daemon_pid_file")"
		else
			printf 'Kache: stopped\n'
		fi
		;;
	doctor)
		"$script_dir/mise-ensure-shared-kache.sh"
		[ -x "$shared_kache_kache_bin" ] || {
			printf 'shared Kache binary missing: %s\n' "$shared_kache_kache_bin" >&2
			exit 1
		}
		RUSTC_WRAPPER="$shared_kache_kache_bin" "$shared_kache_kache_bin" daemon
		KACHE_S3_ACCESS_KEY='kachelocal' \
		KACHE_S3_SECRET_KEY='kachelocalsecret' \
			python3 "$script_dir/rustfs-ensure-bucket.py"
		printf 'shared Kache and RustFS checks passed\n'
		;;
	logs)
		for log_file in "$shared_kache_rustfs_log_file" "$shared_kache_daemon_log_file"; do
			printf '==> %s <==\n' "$log_file"
			if [ -f "$log_file" ]; then
				tail -n 100 "$log_file"
			else
				printf '(missing)\n'
			fi
		done
		;;
	*)
		usage
		exit 2
		;;
esac
