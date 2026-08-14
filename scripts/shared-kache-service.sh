#!/usr/bin/env bash

# Export deterministic local shared-cache settings while preserving CI-provided
# remote configuration. Production local runs deliberately use kache's own
# platform cache-directory resolution instead of constructing an OS path.
shared_kache_export_environment() {
	local config_home="${XDG_CONFIG_HOME:-$HOME/.config}"
	local endpoint="${SHARED_KACHE_ENDPOINT:-http://127.0.0.1:19000}"

	export KACHE_CONFIG="$config_home/kache/config.toml"
	if [ "${CI:-}" = 'true' ] && [ "${SHARED_KACHE_FORCE_LOCAL:-0}" != '1' ]; then
		if [ -n "${KACHE_S3_ENDPOINT:-}" ]; then
			export KACHE_S3_BUCKET="${KACHE_S3_BUCKET:-kache}"
			export KACHE_S3_REGION="${KACHE_S3_REGION:-us-east-1}"
			export KACHE_S3_PREFIX="${KACHE_S3_PREFIX:-ci}"
			export KACHE_LOCAL_MAX_SIZE="${KACHE_LOCAL_MAX_SIZE:-50GiB}"
		else
			unset KACHE_CACHE_DIR
			unset KACHE_S3_BUCKET KACHE_S3_REGION KACHE_S3_PREFIX KACHE_LOCAL_MAX_SIZE
		fi
		return 0
	fi

	if [ "${SHARED_KACHE_TESTING:-0}" = '1' ] && [ -n "${SHARED_KACHE_TEST_CACHE_DIR:-}" ]; then
		export KACHE_CACHE_DIR="$SHARED_KACHE_TEST_CACHE_DIR"
	else
		unset KACHE_CACHE_DIR
	fi

	if [ "${SHARED_KACHE_TESTING:-0}" = '1' ] && [ -n "${SHARED_KACHE_TEST_ENDPOINT:-}" ]; then
		endpoint="$SHARED_KACHE_TEST_ENDPOINT"
	fi
	export KACHE_S3_ENDPOINT="$endpoint"
	export KACHE_S3_BUCKET='shared-local'
	export KACHE_S3_REGION='us-east-1'
	export KACHE_S3_PREFIX='shared-local'
	export KACHE_LOCAL_MAX_SIZE='50GiB'
}

# Resolve shared service paths from XDG data/state roots. These paths never
# depend on project, worktree, or OS-specific cache-directory literals.
shared_kache_init_paths() {
	local data_home="${XDG_DATA_HOME:-$HOME/.local/share}"
	local state_home="${XDG_STATE_HOME:-$HOME/.local/state}"
	local kache_version="${KACHE_VERSION:-0.14.0}"
	local rustfs_version="${RUSTFS_VERSION:-1.0.0-beta.8}"

	shared_kache_data_root="$data_home/kache-shared"
	shared_kache_state_root="$state_home/kache-shared"
	shared_kache_kache_bin="$shared_kache_data_root/kache/$kache_version/bin/kache"
	shared_kache_rustfs_bin="$shared_kache_data_root/rustfs/$rustfs_version/bin/rustfs"
	shared_kache_rustfs_data_dir="$shared_kache_data_root/rustfs/data"
	shared_kache_rustfs_state_dir="$shared_kache_state_root/rustfs"
	shared_kache_rustfs_pid_file="$shared_kache_rustfs_state_dir/rustfs.pid"
	shared_kache_rustfs_log_file="$shared_kache_rustfs_state_dir/rustfs.log"
	shared_kache_lock_dir="$shared_kache_state_root/ensure.lock"
	shared_kache_daemon_state_dir="$shared_kache_state_root/kache"
	shared_kache_daemon_pid_file="$shared_kache_daemon_state_dir/daemon.pid"
	shared_kache_daemon_log_file="$shared_kache_daemon_state_dir/daemon.log"
}

# Start one service in its own session with a deliberately constructed
# environment. Python provides the same detached-process semantics on macOS
# and Linux and atomically records the exact child PID returned by Popen.
shared_kache_launch_detached() {
	local service="$1"
	local pid_file="$2"
	local log_file="$3"
	shift 3

	python3 - "$service" "$pid_file" "$log_file" "$@" <<'PY'
import os
import stat
import subprocess
import sys
import tempfile

service, pid_path, log_path, *command = sys.argv[1:]
allowed_environment = (
	"HOME",
	"KACHE_CACHE_DIR",
	"KACHE_CONFIG",
	"KACHE_LOCAL_MAX_SIZE",
	"KACHE_S3_BUCKET",
	"KACHE_S3_ENDPOINT",
	"KACHE_S3_PREFIX",
	"KACHE_S3_REGION",
	"LANG",
	"LC_ALL",
	"PATH",
	"SSL_CERT_DIR",
	"SSL_CERT_FILE",
	"TEMP",
	"TMP",
	"TMPDIR",
	"XDG_CONFIG_HOME",
	"XDG_DATA_HOME",
	"XDG_STATE_HOME",
)
child_environment = {
	name: os.environ[name]
	for name in allowed_environment
	if name in os.environ
}
if service == "rustfs":
	child_environment.update({
		"RUSTFS_ACCESS_KEY": "kachelocal",
		"RUSTFS_REGION": "us-east-1",
		"RUSTFS_SECRET_KEY": "kachelocalsecret",
	})
elif service == "kache":
	child_environment.update({
		"AWS_ACCESS_KEY_ID": "kachelocal",
		"AWS_REGION": "us-east-1",
		"AWS_SECRET_ACCESS_KEY": "kachelocalsecret",
		"KACHE_S3_ACCESS_KEY": "kachelocal",
		"KACHE_S3_SECRET_KEY": "kachelocalsecret",
	})
else:
	raise SystemExit(f"unsupported shared kache service: {service}")

os.makedirs(os.path.dirname(pid_path), exist_ok=True)
os.makedirs(os.path.dirname(log_path), exist_ok=True)
log_flags = os.O_WRONLY | os.O_APPEND | os.O_CREAT
if hasattr(os, "O_NOFOLLOW"):
	log_flags |= os.O_NOFOLLOW
log_fd = os.open(log_path, log_flags, 0o600)
log_status = os.fstat(log_fd)
if not stat.S_ISREG(log_status.st_mode):
	os.close(log_fd)
	raise OSError(f"shared kache log is not a regular file: {log_path}")
os.fchmod(log_fd, 0o600)
pid_directory = os.path.dirname(pid_path)
if os.environ.get("SHARED_KACHE_TEST_PID_TEMP_FAILURE") == "1":
	os.close(log_fd)
	raise OSError("injected PID tempfile creation failure")
try:
	pid_fd, temporary_path = tempfile.mkstemp(prefix=".pid.", dir=pid_directory, text=True)
except BaseException:
	os.close(log_fd)
	raise
process = None
try:
	os.chmod(temporary_path, 0o600)
	with open(os.devnull, "rb") as stdin_file:
		process = subprocess.Popen(
			command,
			stdin=stdin_file,
			stdout=log_fd,
			stderr=log_fd,
			start_new_session=True,
			close_fds=True,
			cwd=os.path.dirname(log_path),
			env=child_environment,
		)
	with os.fdopen(pid_fd, "w", encoding="utf-8") as pid_file:
		pid_file.write(f"{process.pid}\n")
		pid_file.flush()
		os.fsync(pid_file.fileno())
	os.replace(temporary_path, pid_path)
except BaseException:
	try:
		os.close(pid_fd)
	except OSError:
		pass
	try:
		os.unlink(temporary_path)
	except FileNotFoundError:
		pass
	if process is not None:
		try:
			process.terminate()
		except ProcessLookupError:
			pass
		else:
			try:
				process.wait(timeout=5)
			except subprocess.TimeoutExpired:
				process.kill()
				process.wait()
	raise
finally:
	os.close(log_fd)
PY
}

shared_kache_write_pid_file() {
	local pid_file="$1"
	local pid="$2"
	local temporary="$pid_file.tmp.$$"

	(
		umask 077
		printf '%s\n' "$pid" > "$temporary"
	) || return 1
	chmod 600 "$temporary"
	mv -f "$temporary" "$pid_file"
}

# Write canonical kache config atomically. Identical content keeps existing
# inode/mtime; symlinks and non-regular destinations are rejected.
shared_kache_write_config() {
	local config_dir
	local desired
	local temporary

	local config_value

	for config_value in "$KACHE_LOCAL_MAX_SIZE" "$KACHE_S3_BUCKET" "$KACHE_S3_ENDPOINT" "$KACHE_S3_REGION" "$KACHE_S3_PREFIX"; do
		case "$config_value" in
			*'"'* | *'\'* | *$'\n'* | *$'\r'*)
				printf 'shared kache config value contains unsafe characters\n' >&2
				return 1
				;;
		esac
	done
	config_dir="$(dirname "$KACHE_CONFIG")"
	desired="$(cat <<KACHECFG
[cache]
local_max_size = "${KACHE_LOCAL_MAX_SIZE}"

[cache.remote]
type = "s3"
bucket = "${KACHE_S3_BUCKET}"
endpoint = "${KACHE_S3_ENDPOINT}"
region = "${KACHE_S3_REGION}"
prefix = "${KACHE_S3_PREFIX}"
force_path_style = true
KACHECFG
)"

	if [ -L "$KACHE_CONFIG" ]; then
		printf 'shared kache config rejected symlink: %s\n' "$KACHE_CONFIG" >&2
		return 1
	fi
	if [ -e "$KACHE_CONFIG" ] && [ ! -f "$KACHE_CONFIG" ]; then
		printf 'shared kache config is not a regular file: %s\n' "$KACHE_CONFIG" >&2
		return 1
	fi
	if ! mkdir -p "$config_dir"; then
		printf 'shared kache config directory unavailable: %s\n' "$config_dir" >&2
		return 1
	fi
	if [ -f "$KACHE_CONFIG" ] && [ "$(cat "$KACHE_CONFIG")" = "$desired" ]; then
		chmod 600 "$KACHE_CONFIG"
		return 0
	fi

	temporary="$config_dir/.config.toml.tmp.$$"
	(
		umask 077
		printf '%s\n' "$desired" > "$temporary"
	) || return 1
	chmod 600 "$temporary"
	mv -f "$temporary" "$KACHE_CONFIG"
	printf 'Updated shared kache config: %s\n' "$KACHE_CONFIG" >&2
}

shared_kache_acquire_lock() {
	local control_fifo
	local status
	local status_fifo

	mkdir -p "$shared_kache_state_root"
	if ! command -v python3 >/dev/null 2>&1; then
		printf 'shared kache ensure lock requires python3\n' >&2
		return 1
	fi
	shared_kache_lock_control_dir="$(mktemp -d "$shared_kache_state_root/ensure-control.$$.XXXXXX")"
	control_fifo="$shared_kache_lock_control_dir/control"
	status_fifo="$shared_kache_lock_control_dir/status"
	mkfifo "$control_fifo" "$status_fifo"
	python3 - "$shared_kache_lock_dir" "$control_fifo" "$status_fifo" <<'PY' &
import fcntl
import os
import sys
import time

lock_path, control_path, status_path = sys.argv[1:]
control_fd = os.open(control_path, os.O_RDONLY | os.O_NONBLOCK)
lock_fd = os.open(lock_path, os.O_RDWR | os.O_CREAT, 0o600)
deadline = time.monotonic() + 10
status = "locked"
while True:
	try:
		fcntl.flock(lock_fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
		break
	except BlockingIOError:
		if time.monotonic() >= deadline:
			status = "timeout"
			break
		time.sleep(0.1)
with open(status_path, "w", encoding="utf-8") as status_file:
	status_file.write(status + "\n")
if status == "locked":
	fcntl.fcntl(control_fd, fcntl.F_SETFL, 0)
	while os.read(control_fd, 4096):
		pass
os.close(control_fd)
os.close(lock_fd)
for path in (status_path, control_path):
	try:
		os.unlink(path)
	except FileNotFoundError:
		pass
try:
	os.rmdir(os.path.dirname(control_path))
except FileNotFoundError:
	pass
PY
	shared_kache_lock_helper_pid=$!
	exec 9> "$control_fifo"
	IFS= read -r status < "$status_fifo" || status='error'
	if [ "$status" = 'locked' ]; then
		return 0
	fi
	exec 9>&-
	wait "$shared_kache_lock_helper_pid" 2>/dev/null || true
	rm -rf "$shared_kache_lock_control_dir"
	shared_kache_lock_helper_pid=''
	shared_kache_lock_control_dir=''
	printf 'shared kache ensure lock timed out: %s\n' "$shared_kache_lock_dir" >&2
	return 1
}

shared_kache_release_lock() {
	if [ -n "${shared_kache_lock_helper_pid:-}" ]; then
		exec 9>&-
		wait "$shared_kache_lock_helper_pid" 2>/dev/null || true
	fi
	if [ -n "${shared_kache_lock_control_dir:-}" ]; then
		rm -rf "$shared_kache_lock_control_dir"
	fi
	shared_kache_lock_helper_pid=''
	shared_kache_lock_control_dir=''
}

shared_kache_install_binary() {
	local source_binary="$1"
	local destination_binary="$2"
	local required_version="$3"
	local destination_dir
	local temporary

	if [ -x "$destination_binary" ] && "$destination_binary" --version 2>/dev/null | grep -q "$required_version"; then
		return 0
	fi
	if [ ! -x "$source_binary" ]; then
		printf 'shared kache source binary missing: %s\n' "$source_binary" >&2
		return 1
	fi

	destination_dir="$(dirname "$destination_binary")"
	mkdir -p "$destination_dir"
	temporary="$destination_binary.tmp.$$"
	cp "$source_binary" "$temporary"
	chmod 755 "$temporary"
	if ! "$temporary" --version 2>/dev/null | grep -q "$required_version"; then
		rm -f "$temporary"
		printf 'shared kache binary version mismatch: %s\n' "$source_binary" >&2
		return 1
	fi
	mv -f "$temporary" "$destination_binary"
}

shared_kache_rustfs_port_open() {
	command -v curl >/dev/null 2>&1 && \
		curl -s -o /dev/null --max-time 1 "$KACHE_S3_ENDPOINT"
}

# Verify that exact RustFS PID owns canonical IPv4 listener. macOS uses system
# lsof; Linux resolves socket inodes through procfs without extra dependencies.
shared_kache_rustfs_pid_owns_listener() {
	local pid="$1"
	local address="${RUSTFS_LISTEN_ADDRESS:-${KACHE_S3_ENDPOINT#*://}}"
	local host="${address%:*}"
	local port="${address##*:}"
	local lsof_bin=''

	case "$pid" in
		'' | *[!0-9]*) return 1 ;;
	esac
	[[ "$host" = '127.0.0.1' || "$host" = '0.0.0.0' ]] || return 1
	case "$(uname -s)" in
		Darwin)
			for lsof_bin in /usr/sbin/lsof /usr/bin/lsof; do
				[ -x "$lsof_bin" ] && break
				lsof_bin=''
			done
			[ -n "$lsof_bin" ] || return 1
			"$lsof_bin" -nP -a -p "$pid" -iTCP@"$host":"$port" -sTCP:LISTEN -t 2>/dev/null | grep -qx "$pid"
			;;
		Linux)
			python3 - "$pid" "$host" "$port" <<'PY'
import os
import socket
import sys

pid, host, port_text = sys.argv[1:]
try:
	port = int(port_text)
	if host == "0.0.0.0":
		host_hex = None
		host_v6_hex = None
	else:
		host_hex = socket.inet_aton(host)[::-1].hex().upper()
		host_v6_hex = "0000000000000000FFFF0000" + host_hex
except (OSError, ValueError):
	raise SystemExit(1)

listener_inodes = set()
for table_path in ("/proc/net/tcp", "/proc/net/tcp6"):
	try:
		with open(table_path, encoding="ascii") as table:
			next(table, None)
			for line in table:
				fields = line.split()
				if len(fields) < 10 or fields[3] != "0A":
					continue
				local_address, local_port = fields[1].split(":")
				if int(local_port, 16) != port:
					continue
				if host_hex is not None and table_path.endswith("tcp") and local_address != host_hex:
					continue
				if host_v6_hex is not None and table_path.endswith("tcp6") and local_address != host_v6_hex:
					continue
				listener_inodes.add(fields[9])
	except (FileNotFoundError, PermissionError):
		continue

try:
	for fd_name in os.listdir(f"/proc/{pid}/fd"):
		try:
			target = os.readlink(f"/proc/{pid}/fd/{fd_name}")
		except (FileNotFoundError, PermissionError):
			continue
		if target.startswith("socket:[") and target[8:-1] in listener_inodes:
			raise SystemExit(0)
except (FileNotFoundError, PermissionError):
	pass
raise SystemExit(1)
PY
			;;
		*) return 1 ;;
	esac
}

shared_kache_rustfs_pid_ready() {
	local pid="$1"

	shared_kache_rustfs_pid_matches_expected_command "$pid" &&
		shared_kache_rustfs_pid_owns_listener "$pid" &&
		shared_kache_rustfs_port_open
}

shared_kache_pid_alive() {
	local pid="$1"
	local state

	if ! kill -0 "$pid" 2>/dev/null; then
		return 1
	fi
	state="$(ps -p "$pid" -o stat= 2>/dev/null || true)"
	case "$state" in
		'' | *Z*) return 1 ;;
		*) return 0 ;;
	esac
}

shared_kache_rustfs_pid_matches_binary() {
	local pid="$1"
	local binary="$2"
	local command_line

	case "$pid" in
		'' | *[!0-9]*) return 1 ;;
	esac
	if ! shared_kache_pid_alive "$pid"; then
		return 1
	fi
	command_line="$(ps -ww -p "$pid" -o command= 2>/dev/null || true)"
	case "$command_line" in
		"$binary"' server '*) return 0 ;;
	esac
	if [ "${SHARED_KACHE_TESTING:-0}" = '1' ]; then
		case "$command_line" in
			'bash '"$binary"' server '* | \
				'/bin/bash '"$binary"' server '* | \
				'/usr/bin/env bash '"$binary"' server '*) return 0 ;;
		esac
	fi
	return 1
}

shared_kache_rustfs_expected_command() {
	local address="${RUSTFS_LISTEN_ADDRESS:-${KACHE_S3_ENDPOINT#*://}}"
	local console_address='127.0.0.1:19001'

	if [ "${SHARED_KACHE_TESTING:-0}" = '1' ] && [ -n "${SHARED_KACHE_TEST_CONSOLE_ADDRESS:-}" ]; then
		console_address="$SHARED_KACHE_TEST_CONSOLE_ADDRESS"
	fi
	printf '%s server --address %s --console-address %s --console-enable %s' \
		"$shared_kache_rustfs_bin" "$address" "$console_address" "$shared_kache_rustfs_data_dir"
}

shared_kache_rustfs_pid_matches_expected_command() {
	local pid="$1"
	local command_line
	local expected

	case "$pid" in
		'' | *[!0-9]*) return 1 ;;
	esac
	shared_kache_pid_alive "$pid" || return 1
	expected="$(shared_kache_rustfs_expected_command)"
	command_line="$(ps -ww -p "$pid" -o command= 2>/dev/null || true)"
	[ "$command_line" = "$expected" ] && return 0
	if [ "${SHARED_KACHE_TESTING:-0}" = '1' ]; then
		case "$command_line" in
			'bash '"$expected" | '/bin/bash '"$expected" | '/usr/bin/env bash '"$expected") return 0 ;;
		esac
	fi
	return 1
}

shared_kache_find_rustfs_pid() {
	local command_line
	local expected
	local found_pid=''
	local pid

	expected="$(shared_kache_rustfs_expected_command)"
	while read -r pid command_line; do
		[ "$command_line" = "$expected" ] || {
			if [ "${SHARED_KACHE_TESTING:-0}" != '1' ]; then
				continue
			fi
			case "$command_line" in
				'bash '"$expected" | '/bin/bash '"$expected" | '/usr/bin/env bash '"$expected") ;;
				*) continue ;;
			esac
		}
		if [ -n "$found_pid" ]; then
			return 2
		fi
		found_pid="$pid"
	done < <(ps -ww -axo pid=,command= | sed -E 's/^[[:space:]]*([0-9]+)[[:space:]]+/\1 /')
	printf '%s\n' "$found_pid"
}

shared_kache_rustfs_pid_running() {
	local pid

	pid="$(cat "$shared_kache_rustfs_pid_file" 2>/dev/null || true)"
	shared_kache_rustfs_pid_matches_expected_command "$pid"
}

shared_kache_rustfs_managed_pid_running() {
	local pid="$1"
	local command_line

	case "$pid" in
		'' | *[!0-9]*) return 1 ;;
	esac
	if ! shared_kache_pid_alive "$pid"; then
		return 1
	fi
	command_line="$(ps -ww -p "$pid" -o command= 2>/dev/null || true)"
	case "$command_line" in
		"$shared_kache_data_root"/rustfs/*'/bin/rustfs server '*) return 0 ;;
	esac
	if [ "${SHARED_KACHE_TESTING:-0}" = '1' ]; then
		case "$command_line" in
			'bash '"$shared_kache_data_root"/rustfs/*'/bin/rustfs server '* | \
				'/bin/bash '"$shared_kache_data_root"/rustfs/*'/bin/rustfs server '* | \
				'/usr/bin/env bash '"$shared_kache_data_root"/rustfs/*'/bin/rustfs server '*) return 0 ;;
		esac
	fi
	return 1
}

shared_kache_stop_managed_rustfs() {
	local pid="$1"
	local required_binary="${2:-}"

	if [ -n "$required_binary" ]; then
		shared_kache_rustfs_pid_matches_binary "$pid" "$required_binary" || return 1
	elif ! shared_kache_rustfs_managed_pid_running "$pid"; then
		return 1
	fi
	if [ -n "$required_binary" ]; then
		shared_kache_rustfs_pid_matches_binary "$pid" "$required_binary" || return 1
	else
		shared_kache_rustfs_managed_pid_running "$pid" || return 1
	fi
	kill "$pid" 2>/dev/null || return 1
	for _ in $(seq 1 50); do
		if ! shared_kache_pid_alive "$pid"; then
			wait "$pid" 2>/dev/null || true
			return 0
		fi
		sleep 0.1
	done
	if [ -n "$required_binary" ] && shared_kache_rustfs_pid_matches_binary "$pid" "$required_binary"; then
		kill -KILL "$pid" 2>/dev/null || true
	elif [ -z "$required_binary" ] && shared_kache_rustfs_managed_pid_running "$pid"; then
		kill -KILL "$pid" 2>/dev/null || true
	fi
	for _ in $(seq 1 10); do
		if ! shared_kache_pid_alive "$pid"; then
			wait "$pid" 2>/dev/null || true
			return 0
		fi
		sleep 0.1
	done
	printf 'shared rustfs managed pid did not stop: %s\n' "$pid" >&2
	return 1
}

shared_kache_start_rustfs() {
	local address="${RUSTFS_LISTEN_ADDRESS:-${KACHE_S3_ENDPOINT#*://}}"
	local console_address='127.0.0.1:19001'
	local pid
	local pid_file_pid

	if [ "${SHARED_KACHE_TESTING:-0}" = '1' ] && [ -n "${SHARED_KACHE_TEST_CONSOLE_ADDRESS:-}" ]; then
		console_address="$SHARED_KACHE_TEST_CONSOLE_ADDRESS"
	fi
	if shared_kache_rustfs_pid_running; then
		pid="$(cat "$shared_kache_rustfs_pid_file" 2>/dev/null || true)"
		local readiness_attempts=30
		if [ "${SHARED_KACHE_TESTING:-0}" = '1' ] && [ -n "${SHARED_KACHE_TEST_READINESS_ATTEMPTS:-}" ]; then
			readiness_attempts="$SHARED_KACHE_TEST_READINESS_ATTEMPTS"
		fi
		for _ in $(seq 1 "$readiness_attempts"); do
			shared_kache_rustfs_pid_ready "$pid" && return 0
			sleep 0.1
		done
		if shared_kache_rustfs_pid_matches_expected_command "$pid"; then
			shared_kache_stop_managed_rustfs "$pid" "$shared_kache_rustfs_bin" || true
		fi
		rm -f "$shared_kache_rustfs_pid_file"
		printf 'shared rustfs pid is alive but endpoint is unavailable\n' >&2
		return 1
	fi
	pid="$(cat "$shared_kache_rustfs_pid_file" 2>/dev/null || true)"
	if shared_kache_rustfs_managed_pid_running "$pid"; then
		shared_kache_stop_managed_rustfs "$pid" || return 1
	fi
	rm -f "$shared_kache_rustfs_pid_file"
	if pid="$(shared_kache_find_rustfs_pid)"; then
		if [ -n "$pid" ]; then
			shared_kache_write_pid_file "$shared_kache_rustfs_pid_file" "$pid" || return 1
			local adoption_attempts=100
			if [ "${SHARED_KACHE_TESTING:-0}" = '1' ] && [ -n "${SHARED_KACHE_TEST_READINESS_ATTEMPTS:-}" ]; then
				adoption_attempts="$SHARED_KACHE_TEST_READINESS_ATTEMPTS"
			fi
			for _ in $(seq 1 "$adoption_attempts"); do
				if shared_kache_rustfs_pid_ready "$pid"; then
					printf 'Adopted shared rustfs pid %s at %s\n' "$pid" "$KACHE_S3_ENDPOINT" >&2
					return 0
				fi
				shared_kache_pid_alive "$pid" || break
				sleep 0.1
			done
			if shared_kache_rustfs_pid_matches_expected_command "$pid"; then
				shared_kache_stop_managed_rustfs "$pid" "$shared_kache_rustfs_bin" || true
			fi
			rm -f "$shared_kache_rustfs_pid_file"
			printf 'orphaned shared rustfs process failed readiness verification: %s\n' "$pid" >&2
			return 1
		fi
	else
		printf 'multiple exact shared rustfs processes found; refusing ambiguous adoption\n' >&2
		return 1
	fi
	if shared_kache_rustfs_port_open; then
		printf 'shared rustfs endpoint occupied by unmanaged process: %s\n' "$KACHE_S3_ENDPOINT" >&2
		return 1
	fi

	mkdir -p "$shared_kache_rustfs_data_dir" "$shared_kache_rustfs_state_dir"
	shared_kache_launch_detached rustfs \
		"$shared_kache_rustfs_pid_file" \
		"$shared_kache_rustfs_log_file" \
		"$shared_kache_rustfs_bin" server \
		--address "$address" \
		--console-address "$console_address" \
		--console-enable \
		"$shared_kache_rustfs_data_dir"
	pid="$(cat "$shared_kache_rustfs_pid_file")"

	local startup_attempts=100
	if [ "${SHARED_KACHE_TESTING:-0}" = '1' ] && [ -n "${SHARED_KACHE_TEST_READINESS_ATTEMPTS:-}" ]; then
		startup_attempts="$SHARED_KACHE_TEST_READINESS_ATTEMPTS"
	fi
	for _ in $(seq 1 "$startup_attempts"); do
		if shared_kache_rustfs_pid_ready "$pid"; then
			printf 'Started shared rustfs pid %s at %s\n' "$pid" "$KACHE_S3_ENDPOINT" >&2
			return 0
		fi
		if ! kill -0 "$pid" 2>/dev/null; then
			break
		fi
		sleep 0.1
	done
	if shared_kache_rustfs_pid_matches_binary "$pid" "$shared_kache_rustfs_bin"; then
		shared_kache_stop_managed_rustfs "$pid" "$shared_kache_rustfs_bin" || true
	fi
	pid_file_pid="$(cat "$shared_kache_rustfs_pid_file" 2>/dev/null || true)"
	if [ "$pid_file_pid" = "$pid" ]; then
		rm -f "$shared_kache_rustfs_pid_file"
	fi
	printf 'shared rustfs failed to start; log: %s\n' "$shared_kache_rustfs_log_file" >&2
	return 1
}

shared_kache_ensure_bucket() {
	local bucket_script="$1"

	if ! command -v python3 >/dev/null 2>&1; then
		printf 'shared rustfs bucket helper requires python3\n' >&2
		return 1
	fi
	if [ ! -f "$bucket_script" ]; then
		printf 'shared rustfs bucket helper missing: %s\n' "$bucket_script" >&2
		return 1
	fi
	KACHE_S3_ACCESS_KEY='kachelocal' \
	KACHE_S3_SECRET_KEY='kachelocalsecret' \
	RUSTFS_BUCKET_INIT_ATTEMPTS=10 \
		python3 "$bucket_script" >>"$shared_kache_rustfs_log_file" 2>&1
}

shared_kache_daemon_pid_running() {
	local pid

	pid="$(cat "$shared_kache_daemon_pid_file" 2>/dev/null || true)"
	case "$pid" in
		'' | *[!0-9]*) return 1 ;;
	esac
	shared_kache_daemon_pid_matches_binary "$pid"
}

shared_kache_daemon_pid_matches_binary() {
	local pid="$1"
	local command_line

	if ! shared_kache_pid_alive "$pid"; then
		return 1
	fi
	command_line="$(ps -ww -p "$pid" -o command= 2>/dev/null || true)"
	if [ "$command_line" = "$shared_kache_kache_bin daemon run" ]; then
		return 0
	fi
	if [ "${SHARED_KACHE_TESTING:-0}" = '1' ]; then
		case "$command_line" in
			'bash '"$shared_kache_kache_bin"' daemon run' | \
				'/bin/bash '"$shared_kache_kache_bin"' daemon run' | \
				'/usr/bin/env bash '"$shared_kache_kache_bin"' daemon run') return 0 ;;
		esac
	fi
	return 1
}

shared_kache_find_daemon_pid() {
	local found_pid=''
	local command_line
	local expected="$shared_kache_kache_bin daemon run"
	local pid

	while read -r pid command_line; do
		[ "$command_line" = "$expected" ] || {
			if [ "${SHARED_KACHE_TESTING:-0}" != '1' ]; then
				continue
			fi
			case "$command_line" in
				'bash '"$expected" | '/bin/bash '"$expected" | '/usr/bin/env bash '"$expected") ;;
				*) continue ;;
			esac
		}
		if [ -n "$found_pid" ]; then
			return 2
		fi
		found_pid="$pid"
	done < <(ps -ww -axo pid=,command= | sed -E 's/^[[:space:]]*([0-9]+)[[:space:]]+/\1 /')
	printf '%s\n' "$found_pid"
}

shared_kache_start_daemon() {
	local pid

	mkdir -p "$shared_kache_daemon_state_dir"
	if shared_kache_daemon_pid_running; then
		return 0
	fi
	if pid="$(shared_kache_find_daemon_pid)"; then
		if [ -n "$pid" ]; then
			shared_kache_write_pid_file "$shared_kache_daemon_pid_file" "$pid"
			return 0
		fi
	else
		printf 'multiple exact shared kache daemons found; refusing ambiguous adoption\n' >&2
		return 1
	fi
	"$shared_kache_kache_bin" daemon stop >/dev/null 2>&1 || true
	rm -f "$shared_kache_daemon_pid_file"
	shared_kache_launch_detached kache \
		"$shared_kache_daemon_pid_file" \
		"$shared_kache_daemon_log_file" \
		"$shared_kache_kache_bin" daemon run
	pid="$(cat "$shared_kache_daemon_pid_file")"

	for _ in $(seq 1 50); do
		if shared_kache_daemon_pid_matches_binary "$pid"; then
			return 0
		fi
		if ! shared_kache_pid_alive "$pid"; then
			break
		fi
		sleep 0.1
	done
	if shared_kache_daemon_pid_matches_binary "$pid"; then
		kill "$pid" 2>/dev/null || true
	fi
	rm -f "$shared_kache_daemon_pid_file"
	printf 'shared kache daemon failed to expose manager-owned process\n' >&2
	return 1
}

# Install stable shared binaries, ensure one RustFS process and bucket, then
# start the single kache daemon. CI writes only its isolated env-driven config.
shared_kache_ensure() {
	local source_kache="$1"
	local source_rustfs="$2"
	local bucket_script="$3"
	local result=0

	shared_kache_export_environment
	shared_kache_init_paths
	if [ "${CI:-}" = 'true' ] && [ "${SHARED_KACHE_FORCE_LOCAL:-0}" != '1' ] && [ -z "${KACHE_S3_ENDPOINT:-}" ]; then
		return 0
	fi
	shared_kache_write_config || result=$?
	if [ "$result" -ne 0 ]; then
		return "$result"
	fi
	if [ "${CI:-}" = 'true' ] && [ "${SHARED_KACHE_FORCE_LOCAL:-0}" != '1' ]; then
		return 0
	fi
	if [ "${KACHE_SHARED_SERVICE:-1}" = '0' ]; then
		return 0
	fi

	shared_kache_acquire_lock || return 1
	shared_kache_install_binary "$source_kache" "$shared_kache_kache_bin" "${KACHE_VERSION:-0.14.0}" || result=$?
	if [ "$result" -eq 0 ]; then
		shared_kache_install_binary "$source_rustfs" "$shared_kache_rustfs_bin" "${RUSTFS_VERSION:-1.0.0-beta.8}" || result=$?
	fi
	if [ "$result" -eq 0 ]; then
		shared_kache_start_rustfs || result=$?
	fi
	if [ "$result" -eq 0 ]; then
		shared_kache_ensure_bucket "$bucket_script" || result=$?
	fi
	if [ "$result" -eq 0 ]; then
		shared_kache_start_daemon || result=$?
	fi
	shared_kache_release_lock
	return "$result"
}
