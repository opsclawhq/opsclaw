# Phase 0 Packaging + Install Path (00-03) Design

## Scope
Close roadmap item `00-03` by adding an explicit single-binary packaging and install workflow that supports a curl-based installer path.

1. add release packaging script for `opsclaw` binary tarball output
2. add install script intended for `curl | bash` usage with configurable source and install dir
3. add deterministic local verification path for installer (no network dependency)
4. document compose + install workflow for operators

## Requirement Coverage
- INFRA-01
- INFRA-03

## Options Considered

1. Keep `cargo install --path` as the only install route.
2. Add tarball packaging + standalone install script with release URL defaults (recommended).
3. Distribute only Docker image and skip binary install script.

## Selected Approach
Option 2. It preserves self-hosted flexibility, aligns with roadmap wording (“curl one-liner”), and remains testable in CI/local by using local archive inputs.

## Architecture

- `scripts/release/package-opsclaw.sh`:
  - builds release binary for host target
  - emits `opsclaw-<target>.tar.gz` with binary payload
- `scripts/install/install-opsclaw.sh`:
  - installs from local archive (`OPSCLAW_INSTALL_ARCHIVE`) or default release URL
  - supports target install dir (`OPSCLAW_INSTALL_DIR`, default `~/.local/bin`)
  - runs `opsclaw --version` post-install verification
- docs updates in user/developer guides to show:
  - docker compose startup
  - package + install command path
  - one-liner install template command

## Failure Modes

- install script succeeds but binary is not executable on PATH
- release archive naming mismatch breaks fetch
- network-only install flow blocks local deterministic verification

## Verification Strategy

- RED: installer script missing before implementation
- GREEN: package script emits tarball and installer installs from local archive into temp dir
- full: docker compose config validates and rust workspace tests/lint remain green
