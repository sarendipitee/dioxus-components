# Playwright CI harness alignment

## Root cause

`scripts/pre-push.sh` runs each harness-backed component spec against its matching
`test-harness` binary, then runs remaining specs against `preview`. GitHub Actions
still runs every spec in one sharded invocation against `preview`, where test-only
fixtures intentionally do not exist.

## Change

1. Add shared Playwright suite runner that partitions harness-backed components
   across optional CI shards and runs preview-only tests on the same shard.
2. Replace duplicated pre-push loop with shared runner.
3. Update Playwright workflow to call shared runner with matrix shard metadata.
   Shared runner allocates one port per invocation so Cargo artifacts are reused
   across component binaries.
4. Validate runner argument handling, formatting, targeted Playwright execution,
   and repository pre-push script as practical.

## Validation

- `node --check playwright/run-suite.mjs`
- Workflow YAML parsed with PyYAML
- 28 harness components partitioned exactly once across four shards
- Targeted shared runner: Select 9 passed; preview-only shard 8 passed
- `scripts/pre-push.sh`: passed, including 230 preview-only Playwright tests
