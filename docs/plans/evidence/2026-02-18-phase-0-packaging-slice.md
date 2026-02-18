# Verification Evidence: Phase 0 Packaging + Install Path Slice

## RED (expected failure before implementation)

Command:

```bash
test -f scripts/install/install-opsclaw.sh
```

Result:

- failed before implementation because installer script did not exist.

## GREEN (packaging + local installer)

Commands:

```bash
test -f scripts/install/install-opsclaw.sh
bash scripts/release/package-opsclaw.sh
bash scripts/install/verify-local-install.sh
```

Result:

- installer script exists
- package script emits `dist/opsclaw-<target>.tar.gz`
- local install verifier installs binary from local archive and confirms `opsclaw --version`

## Full Verification

Commands:

```bash
test -f scripts/install/install-opsclaw.sh
bash scripts/release/package-opsclaw.sh
bash scripts/install/verify-local-install.sh
docker compose -f docker/docker-compose.yaml config
bash scripts/docs/validate-release-docs.sh
cargo test --workspace
cargo clippy --workspace --all-targets
```

Result:

- packaging and local install checks pass
- docker compose configuration validates
- release docs validation script passes
- rust workspace tests pass
- clippy reports no issues
