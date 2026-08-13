#!/usr/bin/env bash
# shellcheck disable=SC1090,SC2030,SC2031,SC2154,SC2329
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
script="$repo_root/scripts/shared-kache-service.sh"
tmp_dir="$(mktemp -d)"
tmp_dir="$(cd "$tmp_dir" && pwd -P)"

test_pid_matches_temporary_service() {
	local pid="$1"
	local command_line

	case "$pid" in
		'' | *[!0-9]*) return 1 ;;
	esac
	command_line="$(ps -p "$pid" -o command= 2>/dev/null || true)"
	case "$command_line" in
		"$tmp_dir"*/kache-shared/rustfs/*/bin/rustfs' server '* | \
			'bash '"$tmp_dir"*/kache-shared/rustfs/*/bin/rustfs' server '* | \
			'/bin/bash '"$tmp_dir"*/kache-shared/rustfs/*/bin/rustfs' server '* | \
			"$tmp_dir"*/kache-shared/kache/*/bin/kache' daemon run' | \
			'bash '"$tmp_dir"*/kache-shared/kache/*/bin/kache' daemon run' | \
			'/bin/bash '"$tmp_dir"*/kache-shared/kache/*/bin/kache' daemon run') return 0 ;;
		*) return 1 ;;
	esac
}

cleanup_test_processes() {
	local pid
	local pid_file

	while IFS= read -r pid_file; do
		pid="$(cat "$pid_file" 2>/dev/null || true)"
		if test_pid_matches_temporary_service "$pid"; then
			kill "$pid" 2>/dev/null || true
		fi
	done < <(find "$tmp_dir" -type f \( -name 'rustfs.pid' -o -name 'daemon.pid' \) 2>/dev/null)
	for _ in $(seq 1 50); do
		local found=0
		while IFS= read -r pid_file; do
			pid="$(cat "$pid_file" 2>/dev/null || true)"
			if test_pid_matches_temporary_service "$pid"; then
				found=1
			fi
		done < <(find "$tmp_dir" -type f \( -name 'rustfs.pid' -o -name 'daemon.pid' \) 2>/dev/null)
		[ "$found" -eq 0 ] && return 0
		sleep 0.02
	done
	while IFS= read -r pid_file; do
		pid="$(cat "$pid_file" 2>/dev/null || true)"
		if test_pid_matches_temporary_service "$pid"; then
			kill -KILL "$pid" 2>/dev/null || true
		fi
	done < <(find "$tmp_dir" -type f \( -name 'rustfs.pid' -o -name 'daemon.pid' \) 2>/dev/null)
}

cleanup() {
	cleanup_test_processes
	if [ -f "$tmp_dir/version-state/kache-shared/rustfs/rustfs.pid" ]; then
		kill "$(cat "$tmp_dir/version-state/kache-shared/rustfs/rustfs.pid")" 2>/dev/null || true
	fi
	rm -rf "$tmp_dir"
}

trap cleanup EXIT

fail() {
	printf 'FAIL: %s\n' "$1" >&2
	exit 1
}

assert_eq() {
	local actual="$1"
	local expected="$2"

	[ "$actual" = "$expected" ] || fail "expected '$expected', got '$actual'"
}

run_test() {
	local test_name="$1"

	printf '== %s ==\n' "$test_name"
	"$test_name"
}

inode_for_path() {
	local path="$1"

	if stat -f '%i' "$path" >/dev/null 2>&1; then
		stat -f '%i' "$path"
	else
		stat -c '%i' "$path"
	fi
}

mode_for_path() {
	local path="$1"

	if stat -f '%Lp' "$path" >/dev/null 2>&1; then
		stat -f '%Lp' "$path"
	else
		stat -c '%a' "$path"
	fi
}

test_local_environment_is_canonical() (
	export HOME="$tmp_dir/local-home"
	export XDG_CONFIG_HOME="$tmp_dir/local-config"
	export XDG_DATA_HOME="$tmp_dir/local-data"
	export XDG_STATE_HOME="$tmp_dir/local-state"
	export KACHE_S3_ENDPOINT='http://127.0.0.1:29999'
	export KACHE_S3_PREFIX='project-specific'
	export KACHE_CACHE_DIR="$tmp_dir/project-cache"
	unset CI SHARED_KACHE_ENDPOINT
	unset AWS_ACCESS_KEY_ID AWS_SECRET_ACCESS_KEY AWS_REGION
	unset KACHE_S3_ACCESS_KEY KACHE_S3_SECRET_KEY
	unset RUSTFS_ACCESS_KEY RUSTFS_SECRET_KEY RUSTFS_REGION

	. "$script"
	shared_kache_export_environment

	assert_eq "$KACHE_S3_ENDPOINT" 'http://127.0.0.1:19000'
	assert_eq "$KACHE_S3_PREFIX" 'shared-local'
	assert_eq "$KACHE_LOCAL_MAX_SIZE" '50GiB'
	assert_eq "$KACHE_CONFIG" "$XDG_CONFIG_HOME/kache/config.toml"
	[ -z "${KACHE_CACHE_DIR+x}" ] || fail 'local environment retained project KACHE_CACHE_DIR'
	[ -z "${AWS_ACCESS_KEY_ID+x}" ] || fail 'local environment set AWS_ACCESS_KEY_ID'
	[ -z "${AWS_SECRET_ACCESS_KEY+x}" ] || fail 'local environment set AWS_SECRET_ACCESS_KEY'
	[ -z "${AWS_REGION+x}" ] || fail 'local environment set AWS_REGION'
	[ -z "${KACHE_S3_ACCESS_KEY+x}" ] || fail 'local environment exported kache access credentials'
	[ -z "${KACHE_S3_SECRET_KEY+x}" ] || fail 'local environment exported kache secret credentials'
	[ -z "${RUSTFS_ACCESS_KEY+x}" ] || fail 'local environment exported RustFS access credentials'
	[ -z "${RUSTFS_SECRET_KEY+x}" ] || fail 'local environment exported RustFS secret credentials'
)

test_local_environment_uses_explicit_shared_endpoint() (
	export HOME="$tmp_dir/explicit-endpoint-home"
	export XDG_CONFIG_HOME="$tmp_dir/explicit-endpoint-config"
	export SHARED_KACHE_ENDPOINT='http://host.docker.internal:19000'
	unset CI

	. "$script"
	shared_kache_export_environment

	assert_eq "$KACHE_S3_ENDPOINT" "$SHARED_KACHE_ENDPOINT"
)

test_ensure_preserves_caller_aws_environment() (
	export HOME="$tmp_dir/aws-home"
	export XDG_CONFIG_HOME="$tmp_dir/aws-config"
	export XDG_DATA_HOME="$tmp_dir/aws-data"
	export XDG_STATE_HOME="$tmp_dir/aws-state"
	export KACHE_SHARED_SERVICE=0
	export AWS_ACCESS_KEY_ID='caller-access'
	export AWS_SECRET_ACCESS_KEY='caller-secret'
	export AWS_REGION='caller-region'
	unset CI

	. "$script"
	shared_kache_ensure '/unused/kache' '/unused/rustfs' '/unused/bucket-helper'

	assert_eq "$AWS_ACCESS_KEY_ID" 'caller-access'
	assert_eq "$AWS_SECRET_ACCESS_KEY" 'caller-secret'
	assert_eq "$AWS_REGION" 'caller-region'
)

test_ensure_keeps_unset_aws_environment_unset() (
	export HOME="$tmp_dir/aws-unset-home"
	export XDG_CONFIG_HOME="$tmp_dir/aws-unset-config"
	export XDG_DATA_HOME="$tmp_dir/aws-unset-data"
	export XDG_STATE_HOME="$tmp_dir/aws-unset-state"
	export KACHE_SHARED_SERVICE=0
	unset CI AWS_ACCESS_KEY_ID AWS_SECRET_ACCESS_KEY AWS_REGION

	. "$script"
	shared_kache_ensure '/unused/kache' '/unused/rustfs' '/unused/bucket-helper'

	[ -z "${AWS_ACCESS_KEY_ID+x}" ] || fail 'ensure set AWS_ACCESS_KEY_ID'
	[ -z "${AWS_SECRET_ACCESS_KEY+x}" ] || fail 'ensure set AWS_SECRET_ACCESS_KEY'
	[ -z "${AWS_REGION+x}" ] || fail 'ensure set AWS_REGION'
)

test_ci_environment_survives() (
	export HOME="$tmp_dir/ci-home"
	export XDG_CONFIG_HOME="$tmp_dir/ci-config"
	export CI=true
	export KACHE_S3_ENDPOINT='http://rustfs.ci.example:9000'
	export KACHE_S3_BUCKET='ci-bucket'
	export KACHE_S3_REGION='ci-region'
	export KACHE_S3_PREFIX='ci-prefix'
	export KACHE_LOCAL_MAX_SIZE='12GiB'
	export KACHE_CACHE_DIR="$tmp_dir/ci-cache"

	. "$script"
	shared_kache_export_environment

	assert_eq "$KACHE_S3_ENDPOINT" 'http://rustfs.ci.example:9000'
	assert_eq "$KACHE_S3_BUCKET" 'ci-bucket'
	assert_eq "$KACHE_S3_REGION" 'ci-region'
	assert_eq "$KACHE_S3_PREFIX" 'ci-prefix'
	assert_eq "$KACHE_LOCAL_MAX_SIZE" '12GiB'
	assert_eq "$KACHE_CACHE_DIR" "$tmp_dir/ci-cache"
)

test_ci_without_remote_skips_config() (
	export HOME="$tmp_dir/ci-empty-home"
	export XDG_CONFIG_HOME="$tmp_dir/ci-empty-config"
	export CI=true
	unset KACHE_S3_ENDPOINT
	export KACHE_S3_BUCKET='stale-local-bucket'
	export KACHE_S3_REGION='stale-local-region'
	export KACHE_S3_PREFIX='stale-local-prefix'
	export KACHE_LOCAL_MAX_SIZE='99GiB'
	export KACHE_CACHE_DIR="$tmp_dir/stale-local-cache"

	. "$script"
	shared_kache_ensure '/missing/kache' '/missing/rustfs' '/missing/bucket-helper.py'

	[ -z "${KACHE_S3_BUCKET+x}" ] || fail 'CI without remote preserved stale KACHE_S3_BUCKET'
	[ -z "${KACHE_S3_REGION+x}" ] || fail 'CI without remote preserved stale KACHE_S3_REGION'
	[ -z "${KACHE_S3_PREFIX+x}" ] || fail 'CI without remote preserved stale KACHE_S3_PREFIX'
	[ -z "${KACHE_LOCAL_MAX_SIZE+x}" ] || fail 'CI without remote preserved stale KACHE_LOCAL_MAX_SIZE'
	[ -z "${KACHE_CACHE_DIR+x}" ] || fail 'CI without remote preserved stale KACHE_CACHE_DIR'
	[ ! -e "$XDG_CONFIG_HOME/kache/config.toml" ] || fail 'CI without remote wrote kache config'
)

test_shared_paths_are_xdg_scoped() (
	export HOME="$tmp_dir/path-home"
	export XDG_CONFIG_HOME="$tmp_dir/path-config"
	export XDG_DATA_HOME="$tmp_dir/path-data"
	export XDG_STATE_HOME="$tmp_dir/path-state"
	unset CI

	. "$script"
	shared_kache_export_environment
	shared_kache_init_paths

	assert_eq "$shared_kache_data_root" "$XDG_DATA_HOME/kache-shared"
	assert_eq "$shared_kache_state_root" "$XDG_STATE_HOME/kache-shared"
	assert_eq "$shared_kache_rustfs_data_dir" "$XDG_DATA_HOME/kache-shared/rustfs/data"
	assert_eq "$shared_kache_rustfs_pid_file" "$XDG_STATE_HOME/kache-shared/rustfs/rustfs.pid"
	assert_eq "$shared_kache_daemon_pid_file" "$XDG_STATE_HOME/kache-shared/kache/daemon.pid"
)

test_daemon_scan_returns_empty_without_managed_process() (
	export HOME="$tmp_dir/daemon-empty-home"
	export XDG_DATA_HOME="$tmp_dir/daemon-empty-data"
	export XDG_STATE_HOME="$tmp_dir/daemon-empty-state"
	unset CI

	. "$script"
	shared_kache_init_paths

	assert_eq "$(shared_kache_find_daemon_pid)" ''
)

test_daemon_scan_rejects_command_containing_target_text() (
	export HOME="$tmp_dir/daemon-decoy-home"
	export XDG_DATA_HOME="$tmp_dir/daemon-decoy-data"
	export XDG_STATE_HOME="$tmp_dir/daemon-decoy-state"
	unset CI

	. "$script"
	shared_kache_init_paths
	bash -c "while :; do sleep 1; done # $shared_kache_kache_bin daemon run" &
	local decoy_pid=$!
	trap 'kill "$decoy_pid" 2>/dev/null || true' EXIT

	assert_eq "$(shared_kache_find_daemon_pid)" ''
	kill "$decoy_pid"
	wait "$decoy_pid" 2>/dev/null || true
	trap - EXIT
)

test_daemon_scan_finds_exact_managed_process() (
	export HOME="$tmp_dir/daemon-real-home"
	export XDG_DATA_HOME="$tmp_dir/daemon-real-data"
	export XDG_STATE_HOME="$tmp_dir/daemon-real-state"
	export SHARED_KACHE_TESTING=1
	unset CI

	. "$script"
	shared_kache_init_paths
	mkdir -p "$(dirname "$shared_kache_kache_bin")" "$shared_kache_daemon_state_dir"
	cat > "$shared_kache_kache_bin" <<'SH'
#!/usr/bin/env bash
trap 'exit 0' TERM INT
while :; do sleep 1; done
SH
	chmod +x "$shared_kache_kache_bin"
	"$shared_kache_kache_bin" daemon run >/dev/null &
	local daemon_pid=$!
	trap 'kill "$daemon_pid" 2>/dev/null || true' EXIT

	assert_eq "$(shared_kache_find_daemon_pid)" "$daemon_pid"
	shared_kache_start_daemon
	assert_eq "$(cat "$shared_kache_daemon_pid_file")" "$daemon_pid"
	shared_kache_daemon_pid_running || fail 'exact managed daemon PID was rejected'
	kill "$daemon_pid"
	wait "$daemon_pid" 2>/dev/null || true
	trap - EXIT
)

test_daemon_ambiguous_exact_orphans_fail_closed() (
	export HOME="$tmp_dir/daemon-ambiguous-home"
	export XDG_DATA_HOME="$tmp_dir/daemon-ambiguous-data"
	export XDG_STATE_HOME="$tmp_dir/daemon-ambiguous-state"
	export SHARED_KACHE_TESTING=1
	unset CI

	. "$script"
	shared_kache_init_paths
	mkdir -p "$(dirname "$shared_kache_kache_bin")" "$shared_kache_daemon_state_dir"
	cat > "$shared_kache_kache_bin" <<'SH'
#!/usr/bin/env bash
trap 'exit 0' TERM INT
while :; do sleep 1; done
SH
	chmod +x "$shared_kache_kache_bin"
	local daemon_pids=()
	local daemon_pid
	for _ in 1 2; do
		"$shared_kache_kache_bin" daemon run &
		daemon_pids+=("$!")
	done
	trap 'kill "${daemon_pids[@]}" 2>/dev/null || true' EXIT

	if shared_kache_start_daemon 2> "$tmp_dir/daemon-ambiguous-error.log"; then
		fail 'ambiguous exact kache daemons were adopted'
	fi
	grep -Fq 'multiple exact shared kache daemons found' "$tmp_dir/daemon-ambiguous-error.log"
	[ ! -e "$shared_kache_daemon_pid_file" ] || fail 'ambiguous kache adoption wrote PID file'
	for daemon_pid in "${daemon_pids[@]}"; do
		shared_kache_pid_alive "$daemon_pid" || fail 'ambiguous kache adoption signaled exact process'
		kill "$daemon_pid"
		wait "$daemon_pid" 2>/dev/null || true
	done
	trap - EXIT
)

test_service_credentials_are_process_scoped() (
	export HOME="$tmp_dir/scoped-home"
	export XDG_CONFIG_HOME="$tmp_dir/scoped-config"
	export XDG_DATA_HOME="$tmp_dir/scoped-data"
	export XDG_STATE_HOME="$tmp_dir/scoped-state"
	export AWS_ACCESS_KEY_ID='caller-access'
	export AWS_SECRET_ACCESS_KEY='caller-secret'
	export AWS_REGION='caller-region'
	export CREDENTIAL_CAPTURE="$XDG_DATA_HOME/scoped-credentials"
	export SHARED_KACHE_TESTING=1
	local bucket_script="$tmp_dir/scoped-bucket.py"
	unset CI KACHE_S3_ACCESS_KEY KACHE_S3_SECRET_KEY
	cat > "$bucket_script" <<'PY'
import os

with open(os.path.join(os.environ["XDG_DATA_HOME"], "scoped-credentials"), "a", encoding="utf-8") as capture:
	capture.write(f"bucket={os.environ['KACHE_S3_ACCESS_KEY']}:{os.environ['KACHE_S3_SECRET_KEY']}\n")
PY

	. "$script"
	shared_kache_export_environment
	shared_kache_init_paths
	mkdir -p "$(dirname "$shared_kache_rustfs_bin")" "$(dirname "$shared_kache_kache_bin")"
	cat > "$shared_kache_rustfs_bin" <<'SH'
#!/usr/bin/env bash
printf 'rustfs=%s:%s:%s:%s\n' "$RUSTFS_ACCESS_KEY" "$RUSTFS_SECRET_KEY" "$RUSTFS_REGION" "${AWS_ACCESS_KEY_ID-unset}" >> "$XDG_DATA_HOME/scoped-credentials"
trap 'exit 0' TERM INT
while :; do sleep 1; done
SH
	chmod +x "$shared_kache_rustfs_bin"
	shared_kache_rustfs_pid_ready() { shared_kache_rustfs_pid_running; }
	shared_kache_start_rustfs

	shared_kache_ensure_bucket "$bucket_script"

	cat > "$shared_kache_kache_bin" <<'SH'
#!/usr/bin/env bash
if [ "${1:-}" = 'daemon' ] && [ "${2:-}" = 'run' ]; then
	printf 'daemon=%s:%s:%s:%s:%s\n' \
		"$AWS_ACCESS_KEY_ID" "$AWS_SECRET_ACCESS_KEY" "$AWS_REGION" \
		"$KACHE_S3_ACCESS_KEY" "$KACHE_S3_SECRET_KEY" >> "$XDG_DATA_HOME/scoped-credentials"
	trap 'exit 0' TERM INT
	while :; do sleep 1; done
fi
SH
	chmod +x "$shared_kache_kache_bin"
	shared_kache_start_daemon

	grep -Fqx 'rustfs=kachelocal:kachelocalsecret:us-east-1:unset' "$CREDENTIAL_CAPTURE"
	grep -Fqx 'bucket=kachelocal:kachelocalsecret' "$CREDENTIAL_CAPTURE"
	grep -Fqx 'daemon=kachelocal:kachelocalsecret:us-east-1:kachelocal:kachelocalsecret' "$CREDENTIAL_CAPTURE"
	assert_eq "$AWS_ACCESS_KEY_ID" 'caller-access'
	assert_eq "$AWS_SECRET_ACCESS_KEY" 'caller-secret'
	assert_eq "$AWS_REGION" 'caller-region'
	[ -z "${KACHE_S3_ACCESS_KEY+x}" ] || fail 'service invocation leaked KACHE_S3_ACCESS_KEY'
	[ -z "${KACHE_S3_SECRET_KEY+x}" ] || fail 'service invocation leaked KACHE_S3_SECRET_KEY'
	local rustfs_pid daemon_pid
	rustfs_pid="$(cat "$shared_kache_rustfs_pid_file")"
	daemon_pid="$(cat "$shared_kache_daemon_pid_file")"
	shared_kache_stop_managed_rustfs "$rustfs_pid" "$shared_kache_rustfs_bin"
	kill "$daemon_pid"
	for _ in $(seq 1 50); do
		shared_kache_pid_alive "$daemon_pid" || break
		sleep 0.02
	done
)

test_local_service_command_uses_configured_ports() (
	export HOME="$tmp_dir/command-home"
	export XDG_CONFIG_HOME="$tmp_dir/command-config"
	export XDG_DATA_HOME="$tmp_dir/command-data"
	export XDG_STATE_HOME="$tmp_dir/command-state"
	unset CI
	. "$script"
	shared_kache_export_environment
	shared_kache_init_paths
	assert_eq "$(shared_kache_rustfs_expected_command)" "$shared_kache_rustfs_bin server --address 127.0.0.1:19000 --console-address 127.0.0.1:19001 --console-enable $shared_kache_rustfs_data_dir"
)

test_config_write_is_secure_and_idempotent() (
	export HOME="$tmp_dir/config-home"
	export XDG_CONFIG_HOME="$tmp_dir/config-root"
	export XDG_DATA_HOME="$tmp_dir/config-data"
	export XDG_STATE_HOME="$tmp_dir/config-state"
	unset CI

	. "$script"
	shared_kache_export_environment
	shared_kache_write_config
	local inode_before
	inode_before="$(inode_for_path "$KACHE_CONFIG")"
	shared_kache_write_config

	assert_eq "$(inode_for_path "$KACHE_CONFIG")" "$inode_before"
	assert_eq "$(mode_for_path "$KACHE_CONFIG")" '600'
	grep -Fq 'endpoint = "http://127.0.0.1:19000"' "$KACHE_CONFIG"
	grep -Fq 'bucket = "shared-local"' "$KACHE_CONFIG"
	grep -Fq 'prefix = "shared-local"' "$KACHE_CONFIG"
)

test_concurrent_ensure_has_single_owner() (
	local guard_dir="$tmp_dir/concurrent-guard"
	local overlap_file="$tmp_dir/concurrent-overlap"
	local completion_file="$tmp_dir/concurrent-completions"
	local worker_script="$tmp_dir/concurrent-worker.sh"

	cat > "$worker_script" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
. "$SERVICE_SCRIPT"
shared_kache_install_binary() { :; }
shared_kache_start_rustfs() {
	if ! mkdir "$GUARD_DIR" 2>/dev/null; then
		: > "$OVERLAP_FILE"
		return 1
	fi
	sleep 0.2
	rmdir "$GUARD_DIR"
}
shared_kache_ensure_bucket() { :; }
shared_kache_start_daemon() { printf '%s\n' "$$" >> "$COMPLETION_FILE"; }
shared_kache_ensure '/unused/kache' '/unused/rustfs' '/unused/bucket-helper'
SH
	chmod +x "$worker_script"

	HOME="$tmp_dir/concurrent-home" \
	XDG_CONFIG_HOME="$tmp_dir/concurrent-config" \
	XDG_DATA_HOME="$tmp_dir/concurrent-data" \
	XDG_STATE_HOME="$tmp_dir/concurrent-state" \
	SERVICE_SCRIPT="$script" \
	GUARD_DIR="$guard_dir" \
	OVERLAP_FILE="$overlap_file" \
	COMPLETION_FILE="$completion_file" \
		"$worker_script" &
	local first_pid=$!
	HOME="$tmp_dir/concurrent-home" \
	XDG_CONFIG_HOME="$tmp_dir/concurrent-config" \
	XDG_DATA_HOME="$tmp_dir/concurrent-data" \
	XDG_STATE_HOME="$tmp_dir/concurrent-state" \
	SERVICE_SCRIPT="$script" \
	GUARD_DIR="$guard_dir" \
	OVERLAP_FILE="$overlap_file" \
	COMPLETION_FILE="$completion_file" \
		"$worker_script" &
	local second_pid=$!
	wait "$first_pid"
	wait "$second_pid"

	[ ! -e "$overlap_file" ] || fail 'concurrent ensure entered critical section twice'
	assert_eq "$(wc -l < "$completion_file" | tr -d ' ')" '2'
)

test_dead_owner_releases_kernel_lock_for_concurrent_reclaimers() (
	local acquired_file="$tmp_dir/reclaim-acquired"
	local completion_file="$tmp_dir/reclaim-completions"
	local guard_dir="$tmp_dir/reclaim-guard"
	local overlap_file="$tmp_dir/reclaim-overlap"
	local owner_script="$tmp_dir/reclaim-owner.sh"
	local worker_script="$tmp_dir/reclaim-worker.sh"

	cat > "$owner_script" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
. "$SERVICE_SCRIPT"
shared_kache_init_paths
shared_kache_acquire_lock
: > "$ACQUIRED_FILE"
kill -STOP "$$"
SH
	cat > "$worker_script" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
. "$SERVICE_SCRIPT"
shared_kache_init_paths
shared_kache_acquire_lock
if ! mkdir "$GUARD_DIR" 2>/dev/null; then
	: > "$OVERLAP_FILE"
	exit 1
fi
sleep 0.1
printf '%s\n' "$$" >> "$COMPLETION_FILE"
rmdir "$GUARD_DIR"
shared_kache_release_lock
SH
	chmod +x "$owner_script" "$worker_script"

	HOME="$tmp_dir/reclaim-home" \
	XDG_DATA_HOME="$tmp_dir/reclaim-data" \
	XDG_STATE_HOME="$tmp_dir/reclaim-state" \
	SERVICE_SCRIPT="$script" \
	ACQUIRED_FILE="$acquired_file" \
		"$owner_script" &
	local owner_pid=$!
	for _ in $(seq 1 100); do
		[ -e "$acquired_file" ] && break
		sleep 0.02
	done
	[ -e "$acquired_file" ] || fail 'lock owner did not acquire kernel lock'
	kill -KILL "$owner_pid"
	wait "$owner_pid" 2>/dev/null || true

	local worker_pids=()
	local worker_pid
	for _ in 1 2 3; do
		HOME="$tmp_dir/reclaim-home" \
		XDG_DATA_HOME="$tmp_dir/reclaim-data" \
		XDG_STATE_HOME="$tmp_dir/reclaim-state" \
		SERVICE_SCRIPT="$script" \
		GUARD_DIR="$guard_dir" \
		OVERLAP_FILE="$overlap_file" \
		COMPLETION_FILE="$completion_file" \
			"$worker_script" &
		worker_pids+=("$!")
	done
	for worker_pid in "${worker_pids[@]}"; do
		wait "$worker_pid"
	done

	[ ! -e "$overlap_file" ] || fail 'concurrent reclaimers overlapped critical sections'
	assert_eq "$(wc -l < "$completion_file" | tr -d ' ')" '3'
)

test_rustfs_pid_file_swap_never_signals_unrelated_process() (
	export HOME="$tmp_dir/swap-home"
	export XDG_DATA_HOME="$tmp_dir/swap-data"
	export XDG_STATE_HOME="$tmp_dir/swap-state"
	export SHARED_KACHE_TESTING=1
	unset CI

	. "$script"
	export RUSTFS_VERSION='1.0.0-old'
	shared_kache_init_paths
	mkdir -p "$(dirname "$shared_kache_rustfs_bin")" "$shared_kache_rustfs_state_dir"
	cat > "$shared_kache_rustfs_bin" <<'SH'
#!/usr/bin/env bash
trap 'exit 0' TERM INT
while :; do sleep 1; done
SH
	chmod +x "$shared_kache_rustfs_bin"
	"$shared_kache_rustfs_bin" server test-data &
	local managed_pid=$!
	sleep 30 &
	local unrelated_pid=$!
	printf '%s\n' "$managed_pid" > "$shared_kache_rustfs_pid_file"

	local alive_checks=0
	shared_kache_pid_alive() {
		local pid="$1"
		local state
		alive_checks=$((alive_checks + 1))
		if [ "$alive_checks" -eq 2 ]; then
			printf '%s\n' "$unrelated_pid" > "$shared_kache_rustfs_pid_file"
		fi
		kill -0 "$pid" 2>/dev/null || return 1
		state="$(ps -p "$pid" -o stat= 2>/dev/null || true)"
		case "$state" in
			'' | *Z*) return 1 ;;
			*) return 0 ;;
		esac
	}
	shared_kache_stop_managed_rustfs "$managed_pid"

	if kill -0 "$managed_pid" 2>/dev/null; then
		fail 'managed RustFS survived exact-PID termination'
	fi
	kill -0 "$unrelated_pid" 2>/dev/null || fail 'PID-file swap killed unrelated process'
	kill "$unrelated_pid"
	wait "$unrelated_pid" 2>/dev/null || true
)

test_rustfs_pid_rejects_command_that_only_mentions_binary_path() (
	export HOME="$tmp_dir/substring-home"
	export XDG_DATA_HOME="$tmp_dir/substring-data"
	export XDG_STATE_HOME="$tmp_dir/substring-state"
	unset CI

	. "$script"
	shared_kache_init_paths
	mkdir -p "$shared_kache_rustfs_state_dir"
	bash -c "while :; do sleep 1; done # $shared_kache_rustfs_bin" &
	local unrelated_pid=$!
	printf '%s\n' "$unrelated_pid" > "$shared_kache_rustfs_pid_file"

	if shared_kache_rustfs_pid_running; then
		kill "$unrelated_pid"
		wait "$unrelated_pid" 2>/dev/null || true
		fail 'RustFS PID validation accepted command containing only binary-path text'
	fi
	kill "$unrelated_pid"
	wait "$unrelated_pid" 2>/dev/null || true
)

test_pid_temp_creation_failure_never_starts_child() (
	local launch_root="$tmp_dir/pid-temp-failure"
	local child_marker="$launch_root/child-started"
	local fake_service="$launch_root/fake-service"

	. "$script"
	mkdir -p "$launch_root/pids" "$launch_root/logs"
	cat > "$fake_service" <<SH
#!/usr/bin/env bash
printf 'started\n' > '$child_marker'
SH
	chmod +x "$fake_service"
	export SHARED_KACHE_TEST_PID_TEMP_FAILURE=1
	if shared_kache_launch_detached rustfs \
		"$launch_root/pids/service.pid" \
		"$launch_root/logs/service.log" \
		"$fake_service" 2> "$launch_root/launch-error.log"; then
		fail 'launch passed injected PID tempfile creation failure'
	fi
	grep -Fq 'injected PID tempfile creation failure' "$launch_root/launch-error.log"
	[ ! -e "$child_marker" ] || fail 'child started before PID tempfile creation succeeded'
	[ -z "$(find "$launch_root/pids" -mindepth 1 -print -quit)" ] || fail 'PID tempfile failure left temporary file'
)

test_log_symlink_is_rejected_without_starting_child() (
	local launch_root="$tmp_dir/log-symlink"
	local child_marker="$launch_root/child-started"
	local victim="$launch_root/victim"
	local fake_service="$launch_root/fake-service"

	. "$script"
	mkdir -p "$launch_root/pids" "$launch_root/logs"
	printf 'victim-content\n' > "$victim"
	ln -s "$victim" "$launch_root/logs/service.log"
	cat > "$fake_service" <<SH
#!/usr/bin/env bash
printf 'started\n' > '$child_marker'
SH
	chmod +x "$fake_service"
	if shared_kache_launch_detached rustfs \
		"$launch_root/pids/service.pid" \
		"$launch_root/logs/service.log" \
		"$fake_service" 2> "$launch_root/launch-error.log"; then
		fail 'launch accepted symlink log destination'
	fi
	assert_eq "$(cat "$victim")" 'victim-content'
	[ ! -e "$child_marker" ] || fail 'child started after symlink log rejection'
	[ ! -e "$launch_root/pids/service.pid" ] || fail 'symlink log rejection wrote PID file'
)

test_listener_ownership_is_bound_to_pid() (
	local listener_root="$tmp_dir/listener-owner"
	local listener_pid
	local port=29989

	. "$script"
	mkdir -p "$listener_root"
	python3 -m http.server "$port" --bind 127.0.0.1 > "$listener_root/server.log" 2>&1 &
	listener_pid=$!
	trap 'kill "$listener_pid" 2>/dev/null || true' EXIT
	export KACHE_S3_ENDPOINT="http://127.0.0.1:$port"
	for _ in $(seq 1 100); do
		shared_kache_rustfs_pid_owns_listener "$listener_pid" && break
		sleep 0.02
	done
	shared_kache_rustfs_pid_owns_listener "$listener_pid" || fail 'listener owner PID was rejected'
	if shared_kache_rustfs_pid_owns_listener "$$"; then
		fail 'non-owner PID accepted listener ownership'
	fi
	kill "$listener_pid"
	wait "$listener_pid" 2>/dev/null || true
	trap - EXIT
)

test_foreign_listener_does_not_bless_exact_orphan() (
	export HOME="$tmp_dir/foreign-home"
	export XDG_DATA_HOME="$tmp_dir/foreign-data"
	export XDG_STATE_HOME="$tmp_dir/foreign-state"
	export SHARED_KACHE_TESTING=1
	export SHARED_KACHE_TEST_ENDPOINT='http://127.0.0.1:29988'
	export SHARED_KACHE_TEST_CONSOLE_ADDRESS='127.0.0.1:29987'
	unset CI

	. "$script"
	shared_kache_export_environment
	shared_kache_init_paths
	mkdir -p "$(dirname "$shared_kache_rustfs_bin")" "$shared_kache_rustfs_data_dir" "$shared_kache_rustfs_state_dir"
	cat > "$shared_kache_rustfs_bin" <<'SH'
#!/usr/bin/env bash
trap 'exit 0' TERM INT
while :; do sleep 1; done
SH
	chmod +x "$shared_kache_rustfs_bin"
	python3 -m http.server 29988 --bind 127.0.0.1 >/dev/null 2>&1 &
	local foreign_pid=$!
	"$shared_kache_rustfs_bin" server \
		--address "${KACHE_S3_ENDPOINT#*://}" \
		--console-address "$SHARED_KACHE_TEST_CONSOLE_ADDRESS" \
		--console-enable \
		"$shared_kache_rustfs_data_dir" &
	local orphan_pid=$!
	trap 'kill "$foreign_pid" "$orphan_pid" 2>/dev/null || true' EXIT
	for _ in $(seq 1 100); do
		shared_kache_rustfs_port_open && break
		command sleep 0.02
	done
	sleep() { command sleep 0.001; }

	if shared_kache_start_rustfs 2> "$tmp_dir/foreign-listener-error.log"; then
		fail 'foreign listener blessed exact non-listening RustFS orphan'
	fi
	grep -Fq 'orphaned shared rustfs process failed readiness verification' "$tmp_dir/foreign-listener-error.log"
	shared_kache_pid_alive "$orphan_pid" && fail 'non-listening managed orphan survived failed readiness'
	shared_kache_pid_alive "$foreign_pid" || fail 'foreign listener was signaled during managed orphan cleanup'
	[ ! -e "$shared_kache_rustfs_pid_file" ] || fail 'failed orphan adoption retained PID file'
	kill "$foreign_pid"
	wait "$foreign_pid" 2>/dev/null || true
	trap - EXIT
)

test_interrupted_rustfs_launch_adopts_exact_orphan_and_rejects_decoy() (
	export HOME="$tmp_dir/orphan-home"
	export XDG_DATA_HOME="$tmp_dir/orphan-data"
	export XDG_STATE_HOME="$tmp_dir/orphan-state"
	export SHARED_KACHE_TESTING=1
	export SHARED_KACHE_TEST_ENDPOINT='http://127.0.0.1:29995'
	export SHARED_KACHE_TEST_CONSOLE_ADDRESS='127.0.0.1:29994'
	unset CI

	. "$script"
	shared_kache_export_environment
	shared_kache_init_paths
	mkdir -p "$(dirname "$shared_kache_rustfs_bin")" "$shared_kache_rustfs_data_dir" "$shared_kache_rustfs_state_dir"
	cat > "$shared_kache_rustfs_bin" <<'SH'
#!/usr/bin/env bash
trap 'exit 0' TERM INT
while :; do sleep 1; done
SH
	chmod +x "$shared_kache_rustfs_bin"
	local expected
	expected="$(shared_kache_rustfs_expected_command)"
	bash -c "while :; do sleep 1; done # $expected" &
	local decoy_pid=$!
	"$shared_kache_rustfs_bin" server \
		--address "${KACHE_S3_ENDPOINT#*://}" \
		--console-address "$SHARED_KACHE_TEST_CONSOLE_ADDRESS" \
		--console-enable \
		"$shared_kache_rustfs_data_dir" &
	local orphan_pid=$!
	trap 'kill "$decoy_pid" "$orphan_pid" 2>/dev/null || true' EXIT
	shared_kache_rustfs_pid_ready() { shared_kache_pid_alive "$orphan_pid"; }

	assert_eq "$(shared_kache_find_rustfs_pid)" "$orphan_pid"
	shared_kache_start_rustfs
	assert_eq "$(cat "$shared_kache_rustfs_pid_file")" "$orphan_pid"
	shared_kache_pid_alive "$decoy_pid" || fail 'orphan adoption signaled command-text decoy'
	shared_kache_stop_managed_rustfs "$orphan_pid" "$shared_kache_rustfs_bin"
	kill "$decoy_pid"
	wait "$decoy_pid" 2>/dev/null || true
	trap - EXIT
)

test_rustfs_ambiguous_exact_orphans_fail_closed() (
	export HOME="$tmp_dir/ambiguous-home"
	export XDG_DATA_HOME="$tmp_dir/ambiguous-data"
	export XDG_STATE_HOME="$tmp_dir/ambiguous-state"
	export SHARED_KACHE_TESTING=1
	export SHARED_KACHE_TEST_ENDPOINT='http://127.0.0.1:29993'
	export SHARED_KACHE_TEST_CONSOLE_ADDRESS='127.0.0.1:29992'
	unset CI

	. "$script"
	shared_kache_export_environment
	shared_kache_init_paths
	mkdir -p "$(dirname "$shared_kache_rustfs_bin")" "$shared_kache_rustfs_data_dir" "$shared_kache_rustfs_state_dir"
	cat > "$shared_kache_rustfs_bin" <<'SH'
#!/usr/bin/env bash
trap 'exit 0' TERM INT
while :; do sleep 1; done
SH
	chmod +x "$shared_kache_rustfs_bin"
	local orphan_pids=()
	local orphan_pid
	for _ in 1 2; do
		"$shared_kache_rustfs_bin" server \
			--address "${KACHE_S3_ENDPOINT#*://}" \
			--console-address "$SHARED_KACHE_TEST_CONSOLE_ADDRESS" \
			--console-enable \
			"$shared_kache_rustfs_data_dir" &
		orphan_pids+=("$!")
	done
	trap 'kill "${orphan_pids[@]}" 2>/dev/null || true' EXIT
	shared_kache_rustfs_pid_ready() { return 0; }

	if shared_kache_start_rustfs 2> "$tmp_dir/ambiguous-error.log"; then
		fail 'ambiguous exact RustFS processes were adopted'
	fi
	grep -Fq 'multiple exact shared rustfs processes found' "$tmp_dir/ambiguous-error.log"
	[ ! -e "$shared_kache_rustfs_pid_file" ] || fail 'ambiguous adoption wrote PID file'
	for orphan_pid in "${orphan_pids[@]}"; do
		shared_kache_pid_alive "$orphan_pid" || fail 'ambiguous adoption signaled exact process'
		kill "$orphan_pid"
		wait "$orphan_pid" 2>/dev/null || true
	done
	trap - EXIT
)

test_unready_adopted_rustfs_is_stopped_after_exact_revalidation() (
	export HOME="$tmp_dir/unready-orphan-home"
	export XDG_DATA_HOME="$tmp_dir/unready-orphan-data"
	export XDG_STATE_HOME="$tmp_dir/unready-orphan-state"
	export SHARED_KACHE_TESTING=1
	export SHARED_KACHE_TEST_ENDPOINT='http://127.0.0.1:29991'
	export SHARED_KACHE_TEST_CONSOLE_ADDRESS='127.0.0.1:29990'
	unset CI

	. "$script"
	shared_kache_export_environment
	shared_kache_init_paths
	mkdir -p "$(dirname "$shared_kache_rustfs_bin")" "$shared_kache_rustfs_data_dir" "$shared_kache_rustfs_state_dir"
	cat > "$shared_kache_rustfs_bin" <<'SH'
#!/usr/bin/env bash
trap 'exit 0' TERM INT
while :; do sleep 1; done
SH
	chmod +x "$shared_kache_rustfs_bin"
	"$shared_kache_rustfs_bin" server \
		--address "${KACHE_S3_ENDPOINT#*://}" \
		--console-address "$SHARED_KACHE_TEST_CONSOLE_ADDRESS" \
		--console-enable \
		"$shared_kache_rustfs_data_dir" &
	local orphan_pid=$!
	trap 'kill "$orphan_pid" 2>/dev/null || true' EXIT
	shared_kache_rustfs_pid_ready() { return 1; }
	sleep() { command sleep 0.001; }

	if shared_kache_start_rustfs 2> "$tmp_dir/unready-orphan-error.log"; then
		fail 'unready exact RustFS orphan was accepted'
	fi
	grep -Fq 'orphaned shared rustfs process failed readiness verification' "$tmp_dir/unready-orphan-error.log"
	shared_kache_pid_alive "$orphan_pid" && fail 'unready adopted RustFS remained alive'
	[ ! -e "$shared_kache_rustfs_pid_file" ] || fail 'unready adopted RustFS left PID file'
	trap - EXIT
)

test_failed_rustfs_start_stops_captured_process() (
	export HOME="$tmp_dir/failed-start-home"
	export XDG_DATA_HOME="$tmp_dir/failed-start-data"
	export XDG_STATE_HOME="$tmp_dir/failed-start-state"
	export SHARED_KACHE_TESTING=1
	export SHARED_KACHE_TEST_ENDPOINT='http://127.0.0.1:29997'
	local failed_pid_file="$XDG_STATE_HOME/failed-start.pid"
	unset CI

	. "$script"
	shared_kache_export_environment
	shared_kache_init_paths
	mkdir -p "$(dirname "$shared_kache_rustfs_bin")"
	cat > "$shared_kache_rustfs_bin" <<'SH'
#!/usr/bin/env bash
printf '%s\n' "$$" > "$XDG_STATE_HOME/failed-start.pid"
trap 'exit 0' TERM INT
while :; do sleep 1; done
SH
	chmod +x "$shared_kache_rustfs_bin"
	shared_kache_rustfs_pid_ready() { return 1; }

	if shared_kache_start_rustfs; then
		fail 'RustFS start unexpectedly passed failed readiness probe'
	fi
	local failed_pid
	failed_pid="$(cat "$failed_pid_file" 2>/dev/null || true)"
	[ -n "$failed_pid" ] || fail 'failed RustFS fake did not capture process PID'
	if shared_kache_pid_alive "$failed_pid"; then
		fail 'failed RustFS start left captured process running'
	fi
	[ ! -e "$shared_kache_rustfs_pid_file" ] || fail 'failed RustFS start left matching PID file'
)

test_rustfs_version_transition_replaces_managed_process() (
	export HOME="$tmp_dir/version-home"
	export XDG_DATA_HOME="$tmp_dir/version-data"
	export XDG_STATE_HOME="$tmp_dir/version-state"
	export SHARED_KACHE_TESTING=1
	export SHARED_KACHE_TEST_ENDPOINT='http://127.0.0.1:29998'
	unset CI

	local old_source="$tmp_dir/rustfs-old"
	local new_source="$tmp_dir/rustfs-new"
	cat > "$old_source" <<'SH'
#!/usr/bin/env bash
if [ "${1:-}" = '--version' ]; then printf 'rustfs 1.0.0-old\n'; exit 0; fi
trap 'exit 0' TERM INT
while :; do sleep 1; done
SH
	cat > "$new_source" <<'SH'
#!/usr/bin/env bash
if [ "${1:-}" = '--version' ]; then printf 'rustfs 1.0.0-new\n'; exit 0; fi
trap 'exit 0' TERM INT
while :; do sleep 1; done
SH
	chmod +x "$old_source" "$new_source"

	. "$script"
	shared_kache_export_environment
	shared_kache_rustfs_pid_ready() {
		local service_pid
		service_pid="$(cat "$shared_kache_rustfs_pid_file" 2>/dev/null || true)"
		[ -n "$service_pid" ] && kill -0 "$service_pid" 2>/dev/null
	}

	export RUSTFS_VERSION='1.0.0-old'
	shared_kache_init_paths
	shared_kache_install_binary "$old_source" "$shared_kache_rustfs_bin" "$RUSTFS_VERSION"
	shared_kache_start_rustfs
	local old_pid
	old_pid="$(cat "$shared_kache_rustfs_pid_file")"

	export RUSTFS_VERSION='1.0.0-new'
	shared_kache_init_paths
	shared_kache_install_binary "$new_source" "$shared_kache_rustfs_bin" "$RUSTFS_VERSION"
	shared_kache_start_rustfs
	local new_pid
	new_pid="$(cat "$shared_kache_rustfs_pid_file")"

	[ "$new_pid" != "$old_pid" ] || fail 'RustFS version transition retained old process'
	if shared_kache_pid_alive "$old_pid"; then
		fail 'RustFS version transition left old managed process running'
	fi
	ps -p "$new_pid" -o command= | grep -Fq '/rustfs/1.0.0-new/bin/rustfs'
	kill "$new_pid"
	wait "$new_pid" 2>/dev/null || true
)

test_services_survive_caller_process_group_teardown() (
	local lifecycle_root="$tmp_dir/detached-lifecycle"
	local caller_pid_file="$lifecycle_root/caller.pid"
	local ready_file="$lifecycle_root/ready"
	local worker_log="$lifecycle_root/worker.log"
	local worker_script="$lifecycle_root/worker.sh"
	local source_kache="$lifecycle_root/source-kache"
	local source_rustfs="$lifecycle_root/source-rustfs"
	local bucket_script="$lifecycle_root/bucket.py"
	. "$script"
	mkdir -p "$lifecycle_root"

	cat > "$source_kache" <<'SH'
#!/usr/bin/env bash
if [ "${1:-}" = '--version' ]; then printf 'kache 0.8.0\n'; exit 0; fi
if [ "${1:-}" = 'daemon' ] && [ "${2:-}" = 'stop' ]; then exit 0; fi
if [ "${1:-}" = 'daemon' ] && [ "${2:-}" = 'run' ]; then
	pwd > "$XDG_STATE_HOME/kache-cwd"
	trap 'exit 0' TERM INT
	while :; do sleep 1; done
fi
exit 1
SH
	cat > "$source_rustfs" <<'SH'
#!/usr/bin/env bash
if [ "${1:-}" = '--version' ]; then printf 'rustfs 1.0.0-beta.8\n'; exit 0; fi
if [ "${1:-}" = 'server' ]; then
	pwd > "$XDG_STATE_HOME/rustfs-cwd"
	trap 'exit 0' TERM INT
	while :; do sleep 1; done
fi
exit 1
SH
	printf '%s\n' 'raise SystemExit(0)' > "$bucket_script"
	chmod +x "$source_kache" "$source_rustfs"

	cat > "$worker_script" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
trap 'exit 0' TERM INT
. "$SERVICE_SCRIPT"
shared_kache_rustfs_pid_ready() {
	shared_kache_rustfs_pid_running
}
shared_kache_ensure "$SOURCE_KACHE" "$SOURCE_RUSTFS" "$BUCKET_SCRIPT"
printf '%s %s %s\n' \
	"$$" \
	"$(cat "$shared_kache_rustfs_pid_file")" \
	"$(cat "$shared_kache_daemon_pid_file")" > "$READY_FILE"
while :; do sleep 1; done
SH
	chmod +x "$worker_script"

	HOME="$lifecycle_root/home" \
	XDG_CONFIG_HOME="$lifecycle_root/config" \
	XDG_DATA_HOME="$lifecycle_root/data" \
	XDG_STATE_HOME="$lifecycle_root/state" \
	SHARED_KACHE_TESTING=1 \
	SHARED_KACHE_TEST_CACHE_DIR="$lifecycle_root/cache" \
	SHARED_KACHE_TEST_ENDPOINT='http://127.0.0.1:29996' \
	SERVICE_SCRIPT="$script" \
	SOURCE_KACHE="$source_kache" \
	SOURCE_RUSTFS="$source_rustfs" \
	BUCKET_SCRIPT="$bucket_script" \
	READY_FILE="$ready_file" \
		python3 - "$worker_script" "$caller_pid_file" "$worker_log" <<'PY'
import os
import subprocess
import sys

worker, pid_path, log_path = sys.argv[1:]
with open(os.devnull, "rb") as stdin_file, open(log_path, "ab", buffering=0) as log_file:
	process = subprocess.Popen(
		[worker],
		stdin=stdin_file,
		stdout=log_file,
		stderr=log_file,
		start_new_session=True,
		close_fds=True,
		env=os.environ.copy(),
	)
with open(pid_path, "w", encoding="utf-8") as pid_file:
	pid_file.write(f"{process.pid}\n")
PY

	for _ in $(seq 1 200); do
		[ -s "$ready_file" ] && break
		sleep 0.05
	done
	[ -s "$ready_file" ] || fail "detached lifecycle worker did not become ready: $(cat "$worker_log" 2>/dev/null || true)"

	local caller_pid rustfs_pid daemon_pid caller_pgid rustfs_pgid daemon_pgid
	read -r caller_pid rustfs_pid daemon_pid < "$ready_file"
	caller_pgid="$(ps -p "$caller_pid" -o pgid= | tr -d ' ')"
	rustfs_pgid="$(ps -p "$rustfs_pid" -o pgid= | tr -d ' ')"
	daemon_pgid="$(ps -p "$daemon_pid" -o pgid= | tr -d ' ')"
	assert_eq "$caller_pgid" "$caller_pid"
	assert_eq "$rustfs_pgid" "$rustfs_pid"
	assert_eq "$daemon_pgid" "$daemon_pid"
	[ "$rustfs_pgid" != "$caller_pgid" ] || fail 'RustFS remained in caller process group'
	[ "$daemon_pgid" != "$caller_pgid" ] || fail 'kache daemon remained in caller process group'
	assert_eq "$(cat "$lifecycle_root/state/rustfs-cwd")" "$lifecycle_root/state/kache-shared/rustfs"
	assert_eq "$(cat "$lifecycle_root/state/kache-cwd")" "$lifecycle_root/state/kache-shared/kache"
	assert_eq "$(mode_for_path "$lifecycle_root/state/kache-shared/rustfs/rustfs.log")" '600'
	assert_eq "$(mode_for_path "$lifecycle_root/state/kache-shared/kache/daemon.log")" '600'

	python3 - "$caller_pgid" <<'PY'
import os
import signal
import sys

os.killpg(int(sys.argv[1]), signal.SIGTERM)
PY
	for _ in $(seq 1 100); do
		shared_kache_pid_alive "$caller_pid" || break
		sleep 0.02
	done
	shared_kache_pid_alive "$caller_pid" && fail 'caller process group survived termination'
	shared_kache_pid_alive "$rustfs_pid" || fail 'RustFS died with caller process group'
	shared_kache_pid_alive "$daemon_pid" || fail 'kache daemon died with caller process group'

	export HOME="$lifecycle_root/home"
	export XDG_CONFIG_HOME="$lifecycle_root/config"
	export XDG_DATA_HOME="$lifecycle_root/data"
	export XDG_STATE_HOME="$lifecycle_root/state"
	export SHARED_KACHE_TESTING=1
	. "$script"
	shared_kache_export_environment
	shared_kache_init_paths
	shared_kache_stop_managed_rustfs "$rustfs_pid" "$shared_kache_rustfs_bin"
	shared_kache_daemon_pid_matches_binary "$daemon_pid" || fail 'kache daemon identity changed before cleanup'
	kill "$daemon_pid"
	for _ in $(seq 1 100); do
		shared_kache_pid_alive "$daemon_pid" || break
		sleep 0.02
	done
	shared_kache_pid_alive "$daemon_pid" && fail 'kache daemon survived scoped cleanup'
	return 0
)

[ -f "$script" ] || fail "missing implementation: $script"
run_test test_local_environment_is_canonical
run_test test_local_environment_uses_explicit_shared_endpoint
run_test test_ensure_preserves_caller_aws_environment
run_test test_ensure_keeps_unset_aws_environment_unset
run_test test_ci_environment_survives
run_test test_ci_without_remote_skips_config
run_test test_shared_paths_are_xdg_scoped
run_test test_daemon_scan_returns_empty_without_managed_process
run_test test_daemon_scan_rejects_command_containing_target_text
run_test test_daemon_scan_finds_exact_managed_process
run_test test_daemon_ambiguous_exact_orphans_fail_closed
run_test test_service_credentials_are_process_scoped
run_test test_local_service_command_uses_configured_ports
run_test test_config_write_is_secure_and_idempotent
run_test test_concurrent_ensure_has_single_owner
run_test test_dead_owner_releases_kernel_lock_for_concurrent_reclaimers
run_test test_rustfs_pid_file_swap_never_signals_unrelated_process
run_test test_rustfs_pid_rejects_command_that_only_mentions_binary_path
run_test test_pid_temp_creation_failure_never_starts_child
run_test test_log_symlink_is_rejected_without_starting_child
run_test test_listener_ownership_is_bound_to_pid
run_test test_foreign_listener_does_not_bless_exact_orphan
run_test test_interrupted_rustfs_launch_adopts_exact_orphan_and_rejects_decoy
run_test test_rustfs_ambiguous_exact_orphans_fail_closed
run_test test_unready_adopted_rustfs_is_stopped_after_exact_revalidation
run_test test_failed_rustfs_start_stops_captured_process
run_test test_rustfs_version_transition_replaces_managed_process
run_test test_services_survive_caller_process_group_teardown
