# Release Packaging

Phase 0 packaging scripts provide a deterministic path to build and validate the `opsclaw` single-binary install flow.

## Package a target archive

```bash
bash scripts/release/package-opsclaw.sh
```

Output:
- `dist/opsclaw-<target>.tar.gz`

Optional overrides:
- `OPSCLAW_TARGET` (for `cargo build --target`)
- `OPSCLAW_DIST_DIR` (archive output directory)

## Install binary from local or remote source

Local archive install:

```bash
OPSCLAW_INSTALL_ARCHIVE="$(pwd)/dist/opsclaw-$(rustc -vV | awk '/^host:/ {print $2}').tar.gz" \
OPSCLAW_INSTALL_DIR="$(pwd)/.tmp/bin" \
  bash scripts/install/install-opsclaw.sh
```

Remote release install:

```bash
OPSCLAW_VERSION=latest bash scripts/install/install-opsclaw.sh
```

Optional install overrides:
- `OPSCLAW_TARGET`
- `OPSCLAW_INSTALL_DIR`
- `OPSCLAW_RELEASE_BASE_URL`

## Verify end-to-end installer behavior

```bash
bash scripts/install/verify-local-install.sh
```

This is the primary Phase 0 TDD green-path check for INFRA-01 and INFRA-03.
