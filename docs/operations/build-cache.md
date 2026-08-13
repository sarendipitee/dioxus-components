# Rust Build Cache

This repository uses Kache `0.8.0` as `RUSTC_WRAPPER` and RustFS
`1.0.0-beta.8` as its local S3-compatible remote store. Mise installs both
release binaries from the checksummed `mise.lock`.

## Setup

Install the pinned tools and start the shared service:

```sh
mise install --locked
mise exec -- scripts/build-cache.sh ensure
```

Mise environment activation starts the service on a best-effort basis. Direct
Git-hook execution also activates the cache before repository checks. Cache
startup failure prints a warning and does not prevent compilation.

Service configuration, data, and process state live under user XDG directories
rather than this checkout. RustFS binds
only to `127.0.0.1:19000` and uses the `shared-local` bucket and prefix. Static
development credentials are passed only to managed child processes; they are
not exported into the interactive shell.

Disable automatic local service use for one command with:

```sh
KACHE_SHARED_SERVICE=0 scripts/pre-push.sh
```

## Operations

```sh
mise exec -- scripts/build-cache.sh status
mise exec -- scripts/build-cache.sh doctor
mise exec -- scripts/build-cache.sh logs
mise exec -- kache stats
mise exec -- kache monitor
```

Kache runs as a repository-managed detached process, not an installed system
service. Therefore `kache doctor` reports "Daemon service not installed" even
when this setup is healthy; use `scripts/build-cache.sh doctor` instead.
Bootstrap also performs one locked restart/stop recovery when an unclean Kache
exit leaves legacy socket locks, then returns process ownership to the shared
lifecycle manager.

## Dioxus

Cargo checks and tests use Kache. Dioxus CLI `0.7.10` does not: its compiler
probe inserts `dx` between Kache and `rustc`, which Kache `0.8.0` rejects as an
unknown subcommand. `scripts/preview-web.sh` therefore removes `RUSTC_WRAPPER`
and disables Mise environment reloading only for `dx` subprocess trees. No
`rustc-wrapper` is set in `.cargo/config.toml`; environment activation remains
the single integration point.

## CI

Local RustFS is intentionally not started in GitHub Actions. Independent
ephemeral runners cannot share its artifacts. CI remote caching requires one
durable S3-compatible endpoint and secret-managed credentials before enabling
Kache there.
